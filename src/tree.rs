use std::cmp::Ordering;
use bbox::BBox;
use common::Axis;
use common::Float;
use common::FLOAT_MAX;
use common::FLOAT_MIN_POS;
use hit::Hit;
use ray::Ray;
use shape::Shape;

// k-d tree
pub struct Tree<'a> {
    pub Shapes: Vec<&'a Shape>, // TODO: use slice?
    pub ShapeIndices: Vec<usize>,

    pub Nodes: Vec<Node>,

    bBox: BBox,
}

#[derive(Debug)]
pub struct Node {
    pub SplitOrShape: SplitOrShape, // 16B
    pub Index: usize,               // 8 B Child or Shape Index
}

#[derive(Debug)]
pub enum SplitOrShape {
    Split(Axis, Float), // Splite Point
    Shape(usize),       // Number of Shapes
}

impl<'a> Tree<'a> {
    pub fn New(shapes: Vec<&'a Shape>, maxDepth: u8) -> Tree {
        print!("Building k-d tree ({} shapes) ... ", shapes.len());

        // Compute BBox
        let bbox = BBox::BBoxOfShapes(&shapes);
        let tree = Tree {
            Shapes: shapes,
            ShapeIndices: Vec::<usize>::new(),
            Nodes: Vec::<Node>::new(),

            bBox: bbox,
        };

        let mut shapes = vec![0; tree.Shapes.len()];
        for i in 0..shapes.len() {
            shapes[i] = i;
        }
        let nodeBBox = tree.bBox;
        let maxDepth = if maxDepth > 0 {
            maxDepth
        } else {
            (8.0 + 1.3 * (shapes.len() as f32).log(2.0)).round() as u8
        };

        println!("Done");

        return buildTree(tree, shapes, nodeBBox, 0, maxDepth);
    }

    pub fn Intersect(&self, ray: &Ray) -> Option<Hit> {
        let isec = self.bBox.IntersectP(ray);
        if isec.is_none() {
            return None;
        }
        let (mut tMin, mut tMax) = isec.unwrap();

        let invDir = ray.Direction.Inv(); // to save division (TODO remove?)

        let mut todos = [todo::new(); MAX_TODO];
        let mut todoI = 0;

        let hit: Option<Hit> = None;
        let mut nodeIndex = 0;
        loop {
            if ray.TMax < tMin {
                break;
            }
            let node = &self.Nodes[nodeIndex];
            match node.SplitOrShape {
                SplitOrShape::Split(axis, point) => {
                    let tPlane = (point - ray.Origin[axis]) * invDir[axis];
                    // below first?
                    let (child1, child2) = if ray.Origin[axis] < point || ray.Origin[axis] == point && ray.Direction[axis] <= 0.0 {
                        (nodeIndex+1, node.Index)
                    } else {
                        (node.Index, nodeIndex+1)
                    };
                    if tPlane > tMax || tPlane <= 0.0 {
                        nodeIndex = child1;
                    } else if tPlane < tMin {
                        nodeIndex = child2;
                    } else {
                        nodeIndex = child1;
                        tMax = tPlane;
                        // put child2 into todo
                        todos[todoI].node = child2;
                        todos[todoI].tMin = tPlane;
                        todos[todoI].tMax = tMax;
                        todoI += 1;
                    }
                },
                SplitOrShape::Shape(n) => {
                    if n == 1 {
                        let shape = &self.Shapes[node.Index];
                        let hit = shape.IntersectP(ray);
                        if hit.is_some() {
                            return hit;
                        }
                    } else {
                        for i in 0..n {
                            let shape = &self.Shapes[self.ShapeIndices[node.Index + i]];
                            let hit = shape.IntersectP(ray);
                            if hit.is_some() {
                                return hit;
                            }
                        }
                    }

                    if todoI > 0 {
                        todoI -= 1;
                        nodeIndex = todos[todoI].node;
                        tMin = todos[todoI].tMin;
                        tMax = todos[todoI].tMax;
                    } else {
                        break;
                    }
                },
            }
        }
        return hit;
    }
}

impl Node {
    fn newInterior(axis: Axis, point: Float) -> Node {
        return Node {
            SplitOrShape: SplitOrShape::Split(axis, point),
            Index: 0,
        };
    }

    // shapes: Indices of shapes
    fn newLeaf(mut shapes: &mut Vec<usize>, shapeIndices: &mut Vec<usize>) -> Node {
        let (Shape, Index) = match shapes.len() {
            0 => {
                (SplitOrShape::Shape(0), 0)
            },
            1 => {
                (SplitOrShape::Shape(1), shapes[0])
            },
            n => {
                let i = shapeIndices.len();
                shapeIndices.append(&mut shapes);
                (SplitOrShape::Shape(n), i)
            },
        };
        return Node {
            SplitOrShape: Shape,
            Index: Index,
        };
    }
}

const MAX_TODO: usize = 64;

#[derive(Clone, Copy)]
struct todo {
    node: usize,
    tMin: Float,
    tMax: Float,
}

impl todo {
    fn new() -> todo {
        return todo { node: 0, tMin: 0.0, tMax: 0.0 };
    }
}

const MAX_SHAPES_IN_NODE: usize = 8;
const ISECT_COST: Float = 80.0; // TODO
const TRAV_COST: Float = 1.0;   // TODO
const EMPTY_BONUS: Float = 0.5; // TODO

#[derive(PartialEq)]
enum bEdgeType {
    start,
    end,
}

// bounding edge
struct bEdge {
    t: Float,
    shapeIndex: usize,
    edgeType: bEdgeType,
}

impl bEdge {
    fn new(t: Float, shapeIndex: usize, edgeType: bEdgeType) -> bEdge {
        return bEdge { t: t, shapeIndex: shapeIndex, edgeType: edgeType };
    }
}

// Recursive construction
//   Decide if the node should be an interior node or leaf
//   Update the data structures appropriately
fn buildTree<'a>(
    mut tree: Tree<'a>,
    mut shapes: Vec<usize>,
    nodeBBox: BBox,
    mut badRefines: u8,
    depth: u8) -> Tree<'a> {

    // Create leaf
    if shapes.len() <= MAX_SHAPES_IN_NODE || depth == 0 {
        tree.Nodes.push(Node::newLeaf(&mut shapes, &mut tree.ShapeIndices));
        return tree;
    }

    let d = nodeBBox.Diagonal();
    let invTotSA = 1.0 / nodeBBox.SurfaceArea();
    let oldCost = ISECT_COST * shapes.len() as Float;

    let mut bestAxis: Option<Axis> = None;
    let mut bestIndex: Option<usize> = None;
    let mut bestCost = FLOAT_MAX;
    // 0: X 1: Y 2: Z
    let mut edges = [Vec::<bEdge>::new(), Vec::<bEdge>::new(), Vec::<bEdge>::new()];

    // try different axes
    {
        let mut axis = nodeBBox.MaximumExtent();
        for _ in 0..3 {
            for i in 0..shapes.len() {
                let s = tree.Shapes[i];
                let bbox = s.BBox();
                edges[axis as usize].push(bEdge::new(bbox.Min[axis], i, bEdgeType::start));
                edges[axis as usize].push(bEdge::new(bbox.Min[axis], i, bEdgeType::end));
            }
            edges[axis as usize].sort_by(|a, b| {
                match (a.t < b.t, a.t > b.t) {
                    (true, false) => return Ordering::Less,
                    (false, true) => return Ordering::Greater,
                    _ => (),
                };
                match (&a.edgeType, &b.edgeType) {
                    (&bEdgeType::start, &bEdgeType::end) => return Ordering::Less,
                    (&bEdgeType::end, &bEdgeType::start) => return Ordering::Greater,
                    _                                    => return Ordering::Equal,
                };
            });

            let mut nBelow = 0;
            let mut nAbove = shapes.len();

            for i in 0..(shapes.len() * 2) {
                if edges[axis as usize][i].edgeType == bEdgeType::end {
                    nAbove -= 1;
                }

                let t = edges[axis as usize][i].t;
                if nodeBBox.Min[axis] < t && t < nodeBBox.Max[axis] {
                    let (pBelow, pAbove) = {
                        let (axis1, axis2) = axis.OtherAxes();
                        (
                            2.0 * (d[axis1] * d[axis2] + (t - nodeBBox.Min[axis]) * (d[axis1] + d[axis2])) * invTotSA,
                            2.0 * (d[axis1] * d[axis2] + (nodeBBox.Min[axis] - t) * (d[axis1] + d[axis2])) * invTotSA,
                        )
                    };
                    let bonus = if pBelow < FLOAT_MIN_POS || pAbove < FLOAT_MIN_POS {
                        EMPTY_BONUS
                    } else {
                        0.0
                    };
                    let cost = TRAV_COST + ISECT_COST * (1.0 - bonus) * (pBelow * nBelow as Float + pAbove * nAbove as Float);
                    if cost < bestCost {
                        bestCost = cost;
                        bestAxis = Some(axis);
                        bestIndex = Some(i);
                    }
                }

                if edges[axis as usize][i].edgeType == bEdgeType::start {
                    nBelow += 1;
                }
            }
            debug_assert!(nBelow == shapes.len() && nAbove == 0);

            if bestAxis.is_none() {
                axis = axis.NextAxis();
            } else {
                break
            }
        }
    }

    if bestCost > oldCost {
        badRefines += 1;
    }

    // Create leaf
    if (bestCost > 4.0 * oldCost && shapes.len() < 16) || bestAxis.is_none() || badRefines == 3 {
        tree.Nodes.push(Node::newLeaf(&mut shapes, &mut tree.ShapeIndices));
        return tree;
    }

    let bestAxis = bestAxis.unwrap();
    let bestIndex = bestIndex.unwrap();
    let t = edges[bestAxis as usize][bestIndex].t;
    // Classify shapes
    let (shapesBelow, shapesAbove) = {
        let mut sBelow = Vec::<usize>::new();
        let mut sAbove = Vec::<usize>::new();
        for i in 0..bestIndex {
            if edges[bestAxis as usize][i].edgeType == bEdgeType::start {
                sBelow.push(edges[bestAxis as usize][i].shapeIndex);
            }
        }
        for i in (bestIndex + 1)..(shapes.len() * 2) {
            if edges[bestAxis as usize][i].edgeType == bEdgeType::end {
                sAbove.push(edges[bestAxis as usize][i].shapeIndex);
            }
        }
        (sBelow, sAbove)
    };
    let (bboxBelow, bboxAbove) = {
        let mut b1 = nodeBBox;
        let mut b2 = nodeBBox;
        b1.Max[bestAxis] = t;
        b2.Min[bestAxis] = t;
        (b1, b2)
    };

    let mut tree = buildTree(tree, shapesBelow, bboxBelow, badRefines, depth-1);
    tree.Nodes.push(Node::newInterior(bestAxis, t));
    let     tree = buildTree(tree, shapesAbove, bboxAbove, badRefines, depth-1);

    return tree;
}
