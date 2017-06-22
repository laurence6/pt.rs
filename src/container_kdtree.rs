use std::cmp::Ordering;
use std::rc::Rc;

use axis::Axis;
use bbox::BBox3f;
use common::FLOAT_MAX;
use common::FLOAT_MIN_POS;
use common::Float;
use shape::Shape;

/// k-d tree.
pub struct Tree {
    shapes: Vec<Rc<Shape>>,
    shape_indices: Vec<usize>,

    nodes: Vec<Node>,

    bbox: BBox3f,
}

//impl Container for Tree {} // FIXME

struct Node {
    split_or_shape: SplitOrShape,
    index: usize,
}

enum SplitOrShape {
    Split(Axis, Float), // Splite Point
    Shape(usize),       // Number of shapes
}

impl Tree {
    pub fn new(shapes: Vec<Rc<Shape>>, max_depth: u8) -> Tree {
        print!("Building k-d tree ({} shapes) ... ", shapes.len());

        // Compute BBox
        let bbox = BBox3f::bbox_of_shapes(&shapes.clone().into_boxed_slice());
        let tree = Tree {
            shapes: shapes,
            shape_indices: Vec::<usize>::new(),
            nodes: Vec::<Node>::new(),

            bbox: bbox,
        };

        let mut shapes = vec![0; tree.shapes.len()];
        for i in 0..shapes.len() {
            shapes[i] = i;
        }
        let node_bbox = tree.bbox;
        let max_depth = if max_depth > 0 {
            max_depth
        } else {
            (8. + 1.3 * (shapes.len() as f32).log(2.)).round() as u8
        };

        println!("Done");

        return build_tree(tree, shapes, node_bbox, 0, max_depth);
    }
}

impl Node {
    fn new_interior(axis: Axis, point: Float) -> Node {
        return Node {
            split_or_shape: SplitOrShape::Split(axis, point),
            index: 0,
        };
    }

    // shapes: Indices of shapes
    fn new_leaf(mut shapes: &mut Vec<usize>, shape_indices: &mut Vec<usize>) -> Node {
        let (shape, index) = match shapes.len() {
            0 => {
                (SplitOrShape::Shape(0), 0)
            },
            1 => {
                (SplitOrShape::Shape(1), shapes[0])
            },
            n => {
                let i = shape_indices.len();
                shape_indices.append(&mut shapes);
                (SplitOrShape::Shape(n), i)
            },
        };
        return Node {
            split_or_shape: shape,
            index: index,
        };
    }
}

const MAX_TODO: usize = 64;

const MAX_SHAPES_IN_NODE: usize = 8;
const ISECT_COST: Float = 80.;
const TRAV_COST: Float = 1.;
const EMPTY_BONUS: Float = 0.5;

#[derive(PartialEq)]
enum Bedgetype {
    Start,
    End,
}

// bounding edge
struct Bedge {
    t: Float,
    shapeindex: usize,
    edge_type: Bedgetype,
}

impl Bedge {
    fn new(t: Float, shapeindex: usize, edge_type: Bedgetype) -> Bedge {
        return Bedge { t: t, shapeindex: shapeindex, edge_type: edge_type };
    }
}

// Recursive construction
//   Decide if the node should be an interior node or leaf
//   Update the data structures appropriately
fn build_tree(
    mut tree: Tree,
    mut shapes: Vec<usize>,
    node_bbox: BBox3f,
    mut bad_refines: u8,
    depth: u8) -> Tree {

    // Create leaf
    if shapes.len() <= MAX_SHAPES_IN_NODE || depth == 0 {
        tree.nodes.push(Node::new_leaf(&mut shapes, &mut tree.shape_indices));
        return tree;
    }

    let d = node_bbox.diagonal();
    let inv_tot_sa = 1. / node_bbox.surface_area();
    let old_cost = ISECT_COST * shapes.len() as Float;

    let mut best_axis: Option<Axis> = None;
    let mut bestindex: Option<usize> = None;
    let mut best_cost = FLOAT_MAX;
    // 0: X 1: Y 2: Z
    let mut edges = [Vec::<Bedge>::new(), Vec::<Bedge>::new(), Vec::<Bedge>::new()];

    // try different axes
    {
        let mut axis = node_bbox.maximum_extent();
        for _ in 0..3 {
            for i in 0..shapes.len() {
                let s = &tree.shapes[i];
                let bbox = s.bbox();
                edges[axis as usize].push(Bedge::new(bbox.min[axis], i, Bedgetype::Start));
                edges[axis as usize].push(Bedge::new(bbox.min[axis], i, Bedgetype::End));
            }
            edges[axis as usize].sort_by(|a, b| {
                match (a.t < b.t, a.t > b.t) {
                    (true, false) => return Ordering::Less,
                    (false, true) => return Ordering::Greater,
                    _ => (),
                };
                match (&a.edge_type, &b.edge_type) {
                    (&Bedgetype::Start, &Bedgetype::End) => return Ordering::Less,
                    (&Bedgetype::End, &Bedgetype::Start) => return Ordering::Greater,
                    _                                    => return Ordering::Equal,
                };
            });

            let mut n_below = 0;
            let mut n_above = shapes.len();

            for i in 0..(shapes.len() * 2) {
                if edges[axis as usize][i].edge_type == Bedgetype::End {
                    n_above -= 1;
                }

                let t = edges[axis as usize][i].t;
                if node_bbox.min[axis] < t && t < node_bbox.max[axis] {
                    let (p_below, p_above) = {
                        let (axis1, axis2) = axis.others();
                        (
                            2. * (d[axis1] * d[axis2] + (t - node_bbox.min[axis]) * (d[axis1] + d[axis2])) * inv_tot_sa,
                            2. * (d[axis1] * d[axis2] + (node_bbox.min[axis] - t) * (d[axis1] + d[axis2])) * inv_tot_sa,
                        )
                    };
                    let bonus = if p_below < FLOAT_MIN_POS || p_above < FLOAT_MIN_POS {
                        EMPTY_BONUS
                    } else {
                        0.
                    };
                    let cost = TRAV_COST + ISECT_COST * (1. - bonus) * (p_below * n_below as Float + p_above * n_above as Float);
                    if cost < best_cost {
                        best_cost = cost;
                        best_axis = Some(axis);
                        bestindex = Some(i);
                    }
                }

                if edges[axis as usize][i].edge_type == Bedgetype::Start {
                    n_below += 1;
                }
            }
            debug_assert!(n_below == shapes.len() && n_above == 0);

            if best_axis.is_none() {
                axis = axis.next();
            } else {
                break
            }
        }
    }

    if best_cost > old_cost {
        bad_refines += 1;
    }

    // Create leaf
    if (best_cost > 4. * old_cost && shapes.len() < 16) || best_axis.is_none() || bad_refines == 3 {
        tree.nodes.push(Node::new_leaf(&mut shapes, &mut tree.shape_indices));
        return tree;
    }

    let best_axis = best_axis.unwrap();
    let bestindex = bestindex.unwrap();
    let t = edges[best_axis as usize][bestindex].t;
    // Classify shapes
    let (shapes_below, shapes_above) = {
        let mut s_below = Vec::<usize>::new();
        let mut s_above = Vec::<usize>::new();
        for i in 0..bestindex {
            if edges[best_axis as usize][i].edge_type == Bedgetype::Start {
                s_below.push(edges[best_axis as usize][i].shapeindex);
            }
        }
        for i in (bestindex + 1)..(shapes.len() * 2) {
            if edges[best_axis as usize][i].edge_type == Bedgetype::End {
                s_above.push(edges[best_axis as usize][i].shapeindex);
            }
        }
        (s_below, s_above)
    };
    let (bbox_below, bbox_above) = {
        let mut b1 = node_bbox;
        let mut b2 = node_bbox;
        b1.max[best_axis] = t;
        b2.min[best_axis] = t;
        (b1, b2)
    };

    let mut tree = build_tree(tree, shapes_below, bbox_below, bad_refines, depth-1);
    tree.nodes.push(Node::new_interior(best_axis, t));
    let     tree = build_tree(tree, shapes_above, bbox_above, bad_refines, depth-1);

    return tree;
}
