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
    pub NNode: usize, // TODO: remove?
    pub Nodes: Vec<Node>,

    bBox: BBox,
}

impl<'a> Tree<'a> {
    pub fn New(shapes: Vec<&'a Shape>, maxDepth: u8) -> Tree {
        print!("Building k-d tree ({} shapes) ... ", shapes.len());

        // Compute BBox
        let bbox = BBox::BBoxOfShapes(&shapes);
        let mut tree = Tree {
            Shapes: shapes,
            ShapeIndices: Vec::<usize>::new(),
            NNode: 0,
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


        let tree = buildTree(tree, shapes, &nodeBBox, 0, maxDepth);

        return tree;
    }

    pub fn Intersect(&self, r: &Ray) -> Option<Hit> {
        unimplemented!()
    }
}

const MAX_SHAPES_IN_NODE: usize = 8;
const ISECT_COST: Float = 80.0; // TODO
const TRAV_COST: Float = 1.0; // TODO
const EMPTY_BONUS: Float = 0.5;

#[derive(PartialEq)]
enum bEdgeType {
    start,
    end,
}

// bounding edge
struct bEdge {
    t: Float,
    shapeNum: usize,
    edgeType: bEdgeType,
}

impl bEdge {
    fn New(t: Float, shapeNum: usize, edgeType: bEdgeType) -> bEdge {
        return bEdge { t: t, shapeNum: shapeNum, edgeType: edgeType };
    }
}

// Recursive construction
//   Decide if the node should be an interior node or leaf
//   Update the data structures appropriately
fn buildTree<'a>(
    mut tree: Tree<'a>,
    mut shapes: Vec<usize>,
    nodeBBox: &BBox,
    mut badRefines: u8,
    depth: u8) -> Tree<'a> {

    if shapes.len() <= MAX_SHAPES_IN_NODE || depth == 0 {
        tree.Nodes.push(Node::NewLeaf(&mut shapes, &mut tree.ShapeIndices));
        return tree;
    }

    let d = nodeBBox.Diagonal();
    let totSA = nodeBBox.SurfaceArea();
    let invTotSA = 1.0 / totSA;

    let mut bestAxis: Option<Axis> = None;
    let mut bestIndex = 0;
    let mut bestCost = FLOAT_MAX;
    let mut oldCost = ISECT_COST * shapes.len() as Float;

    let mut edges = [Vec::<bEdge>::new(), Vec::<bEdge>::new(), Vec::<bEdge>::new()];
    let mut axis = nodeBBox.MaximumExtent();

    // try different axes
    for _ in 0..2 {
        for i in 0..shapes.len() {
            let s = tree.Shapes[i];
            let bbox = s.BBox();
            // 0: X 1: Y 2: Z
            edges[axis as usize].push(bEdge::New(bbox.Min[axis], i, bEdgeType::start));
            edges[axis as usize].push(bEdge::New(bbox.Min[axis], i, bEdgeType::end));
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
                    bestIndex = i;
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

    if bestCost > oldCost {
        badRefines += 1;
    }

    if (bestCost > 4.0 * oldCost && shapes.len() < 16) || bestAxis.is_none() || badRefines == 3 {
        tree.Nodes.push(Node::NewLeaf(&mut shapes, &mut tree.ShapeIndices));
        return tree;
    }

    {
        let n0 = 0;
        let n1 = 1;
    }

    return buildTree(tree, shapes, nodeBBox, badRefines, depth-1);
}


pub enum SplitOrShape {
    Split(Axis, Float), // Splite Point
    Shape(u32),         // Number of Shapes
}

pub struct Node {
    pub SplitOrShape: SplitOrShape,
    pub Index: usize, // Child or Shape Index
}

impl Node {
    pub fn NewInterior(axis: Axis, point: Float) -> Node {
        return Node {
            SplitOrShape: SplitOrShape::Split(axis, point),
            Index: 0,
        };
    }

    // shapes: Indices of shapes
    pub fn NewLeaf(mut shapes: &mut Vec<usize>, shapeIndices: &mut Vec<usize>) -> Node {
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
                (SplitOrShape::Shape(n as u32), i)
            },
        };
        return Node {
            SplitOrShape: Shape,
            Index: Index,
        };
    }

    pub fn IsLeaf(&self) -> bool {
        return match self.SplitOrShape {
            SplitOrShape::Split(_, _) => false,
            SplitOrShape::Shape(_) => true,
        };
    }
}
