use std::cmp::Ordering;
use std::sync::Arc;

use axis::Axis;
use bbox::BBox3f;
use common::FLOAT_MAX;
use container::Container;
use interaction::Interaction;
use ray::Ray;
use shape::{Shape, intersect};

const MAX_SHAPES_IN_NODE: usize = 8;
const ISECT_COST: f32 = 80.;
const TRAV_COST: f32 = 1.;
const EMPTY_BONUS: f32 = 0.5;

/// k-d tree.
pub struct KdTree {
    bbox: BBox3f,
    nodes: Vec<Node>,
}

enum Node {
    Split(Axis, f32, usize), // split axis, split point, the index of the upper child node
    Empty,
    Shape(Arc<Shape>),
    Shapes(Box<[Arc<Shape>]>),
}

impl KdTree {
    /// If max_depth is None, max_depth will be calculated based on the number of shapes.
    pub fn new(shapes: Box<[Arc<Shape>]>, max_depth: Option<u32>) -> KdTree {
        let mut tree = KdTree {
            bbox: BBox3f::bbox_of_shapes(&shapes),
            nodes: Vec::new(),
        };

        let node_bbox = tree.bbox;
        let max_depth = max_depth.unwrap_or((8. + 1.3 * (shapes.len() as f32).log(2.)).round() as u32);

        tree.build(shapes, node_bbox, 0, max_depth);

        return tree;
    }

    // Recursive construction
    //   Decide if the node should be an interior node or leaf
    //   Update the data structures appropriately
    fn build(&mut self, shapes: Box<[Arc<Shape>]>, node_bbox: BBox3f, mut bad_refines: u8, depth: u32) {
        // Bounding edge
        struct BEdge {
            edge_type: BEdgeType,
            t: f32,
            shape: Arc<Shape>,
        }

        impl BEdge {
            fn new(edge_type: BEdgeType, t: f32, shape: Arc<Shape>) -> BEdge {
                BEdge { edge_type, t, shape }
            }
        }

        #[derive(PartialEq)]
        enum BEdgeType {
            Start,
            End,
        }

        let n_shapes = shapes.len();

        // Create a leaf node if
        //   - shapes are few
        //   - reach the maximum depth
        if n_shapes <= MAX_SHAPES_IN_NODE || depth == 0 {
            self.nodes.push(Node::new_leaf(shapes));
            return;
        }

        let d = node_bbox.diagonal();
        let tot_sa = node_bbox.surface_area();
        let old_cost = ISECT_COST * n_shapes as f32; // cost if not split

        let mut best_axis: Option<Axis> = None;
        let mut best_edge: Option<usize> = None;
        let mut best_cost = FLOAT_MAX;
        let mut edges = [
            Vec::with_capacity(n_shapes * 2), // X
            Vec::with_capacity(n_shapes * 2), // Y
            Vec::with_capacity(n_shapes * 2), // Z
        ];

        let mut axis = node_bbox.max_extent();
        // Try different axes to find the best split point
        for _ in 0..3 {
            for s in shapes.iter() {
                let bbox = s.bbox();
                edges[axis as usize].push(BEdge::new(BEdgeType::Start, bbox.min[axis], s.clone()));
                edges[axis as usize].push(BEdge::new(BEdgeType::End, bbox.max[axis], s.clone()));
            }
            edges[axis as usize].sort_by(|a, b| {
                if a.t < b.t {
                    return Ordering::Less;
                }
                if a.t > b.t {
                    return Ordering::Greater;
                }
                match (&a.edge_type, &b.edge_type) {
                    (&BEdgeType::Start, &BEdgeType::End) => return Ordering::Less,
                    (&BEdgeType::End, &BEdgeType::Start) => return Ordering::Greater,
                    _                                    => return Ordering::Equal,
                };
            });

            let mut n_below = 0;
            let mut n_above = n_shapes;

            for i in 0..(n_shapes * 2) {
                let edge = &edges[axis as usize][i];

                if edge.edge_type == BEdgeType::End {
                    n_above -= 1;
                }

                let t = edge.t;
                if node_bbox.min[axis] < t && t < node_bbox.max[axis] {
                    let (p_below, p_above) = {
                        let (axis1, axis2) = axis.others();
                        let below_sa = 2. * (d[axis1] * d[axis2] + (d[axis1] + d[axis2]) * (t - node_bbox.min[axis]));
                        let above_sa = 2. * (d[axis1] * d[axis2] + (d[axis1] + d[axis2]) * (node_bbox.max[axis] - t));
                        (below_sa / tot_sa, above_sa / tot_sa)
                    };
                    let empty_bonus = if n_below == 0 || n_above == 0 { EMPTY_BONUS } else { 0. };
                    let cost = TRAV_COST + ISECT_COST * (1. - empty_bonus) * (p_below * n_below as f32 + p_above * n_above as f32);
                    if cost < best_cost {
                        best_cost = cost;
                        best_axis = Some(axis);
                        best_edge = Some(i);
                    }
                }

                if edge.edge_type == BEdgeType::Start {
                    n_below += 1;
                }
            }

            if best_axis.is_none() {
                axis = axis.next();
            } else {
                break;
            }
        }

        if best_cost > old_cost {
            bad_refines += 1;
        }

        // Create a leaf node if
        //   - the best cost is much higher than the cost when not splitting this node and there are not very many shapes
        //   - no split point found
        //   - too many times of bad splits
        if (best_cost > 4. * old_cost && n_shapes < MAX_SHAPES_IN_NODE * 2) || best_axis.is_none() || bad_refines == 3 {
            self.nodes.push(Node::new_leaf(shapes));
            return;
        }

        let best_axis = best_axis.unwrap();
        let best_edge = best_edge.unwrap();
        let t = edges[best_axis as usize][best_edge].t;
        // Classify shapes
        let (shapes_below, shapes_above) = {
            let mut s_below = Vec::new();
            let mut s_above = Vec::new();
            for edge in edges[best_axis as usize][0..best_edge].iter() {
                if edge.edge_type == BEdgeType::Start {
                    s_below.push(edge.shape.clone());
                }
            }
            for edge in edges[best_axis as usize][(best_edge + 1)..(n_shapes * 2)].iter() {
                if edge.edge_type == BEdgeType::End {
                    s_above.push(edge.shape.clone());
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

        let split_node_index = self.nodes.len();
        self.nodes.push(Node::Empty); // placeholder
        self.build(shapes_below.into_boxed_slice(), bbox_below, bad_refines, depth - 1);
        self.nodes[split_node_index] = Node::Split(best_axis, t, self.nodes.len());
        self.build(shapes_above.into_boxed_slice(), bbox_above, bad_refines, depth - 1);
    }
}

impl Container for KdTree {
    fn bbox(&self) -> BBox3f {
        self.bbox
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        #[derive(Clone, Copy)]
        struct Todo {
            node: usize,
            t_min: f32,
            t_max: f32,
        }

        impl Todo {
            fn new(node: usize, t_min: f32, t_max: f32) -> Todo {
                Todo { node, t_min, t_max }
            }
        }

        let (mut t_min, mut t_max) =
            if let Some((t_min, t_max)) = self.bbox.intersect(&ray) {
                (t_min, t_max)
            } else {
                return false;
            };

        let mut todos = Vec::new();

        let mut node_i = 0;
        loop {
            if ray.t_max < t_min {
                break;
            }

            let node = &self.nodes[node_i];
            if let &Node::Split(axis, t, above_index) = node {
                let t_split = (t - ray.origin[axis]) / ray.direction[axis];

                let (child_1, child_2) =
                    if ray.origin[axis] < t || ray.origin[axis] == t && ray.direction[axis] <= 0. {
                        (node_i + 1, above_index)
                    } else {
                        (above_index, node_i + 1)
                    };

                if t_split > t_max || t_split <= 0. { // ray does not intersect child_2
                    node_i = child_1;
                } else if t_split < t_min {           // ray does not intersect child_1
                    node_i = child_2;
                } else {
                    todos.push(Todo::new(child_2, t_split, t_max));
                    node_i = child_1;
                    t_max = t_split;
                }
            } else {
                match node {
                    &Node::Empty => (),
                    &Node::Shape(ref shape) => {
                        if shape.intersect_p(&ray) {
                            return true;
                        }
                    },
                    &Node::Shapes(ref shapes) => {
                        for shape in shapes.iter() {
                            if shape.intersect_p(&ray) {
                                return true;
                            }
                        }
                    },
                    _ => panic!(),
                }

                if let Some(todo) = todos.pop() {
                    node_i = todo.node;
                    t_min = todo.t_min;
                    t_max = todo.t_max;
                } else {
                    break;
                }
            }
        }

        return false;
    }

    fn intersect(&self, ray: &Ray) -> Option<Interaction> {
        #[derive(Clone, Copy)]
        struct Todo {
            node: usize,
            t_min: f32,
            t_max: f32,
        }

        impl Todo {
            fn new(node: usize, t_min: f32, t_max: f32) -> Todo {
                Todo { node, t_min, t_max }
            }
        }

        let mut ray = ray.clone();

        let (mut t_min, mut t_max) =
            if let Some((t_min, t_max)) = self.bbox.intersect(&ray) {
                (t_min, t_max)
            } else {
                return None;
            };

        let mut todos = Vec::new();

        let mut interaction = None;
        let mut node_i = 0;
        loop {
            if ray.t_max < t_min {
                break;
            }

            let node = &self.nodes[node_i];
            if let &Node::Split(axis, t, above_index) = node {
                let t_split = (t - ray.origin[axis]) / ray.direction[axis];

                let (child_1, child_2) =
                    if ray.origin[axis] < t || ray.origin[axis] == t && ray.direction[axis] <= 0. {
                        (node_i + 1, above_index)
                    } else {
                        (above_index, node_i + 1)
                    };

                if t_split > t_max || t_split <= 0. { // ray does not intersect child_2
                    node_i = child_1;
                } else if t_split < t_min {           // ray does not intersect child_1
                    node_i = child_2;
                } else {
                    todos.push(Todo::new(child_2, t_split, t_max));
                    node_i = child_1;
                    t_max = t_split;
                }
            } else {
                match node {
                    &Node::Empty => (),
                    &Node::Shape(ref shape) => {
                        let i = intersect(shape, &mut ray);
                        if i.is_some() {
                            interaction = i;
                        }
                    },
                    &Node::Shapes(ref shapes) => {
                        for shape in shapes.iter() {
                            let i = intersect(shape, &mut ray);
                            if i.is_some() {
                                interaction = i;
                            }
                        }
                    },
                    _ => panic!(),
                }

                if let Some(todo) = todos.pop() {
                    node_i = todo.node;
                    t_min = todo.t_min;
                    t_max = todo.t_max;
                } else {
                    break;
                }
            }
        }

        return interaction;
    }
}

impl Node {
    fn new_leaf(shapes: Box<[Arc<Shape>]>) -> Node {
        match shapes.len() {
            0 => Node::Empty,
            1 => Node::Shape(shapes[0].clone()),
            _ => Node::Shapes(shapes),
        }
    }
}
