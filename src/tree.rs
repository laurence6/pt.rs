use ray::Ray;
use hit::Hit;
use shape::Shape;
use common::Float;

// k-d tree
pub struct Tree<'a> {
    pub Shapes: Vec<&'a Shape>,
    pub Root: Node,
}

impl<'a> Tree<'a> {
    pub fn New(shapes: Vec<&'a Shape>) -> Tree {
        print!("Building k-d tree ({} shapes) ... ", shapes.len());
        let node = Node::New();
        println!("Done");
        return Tree {
            Shapes: shapes,
            Root: node,
        };
    }

    pub fn Intersect(&self, r: &Ray) -> Option<Hit> {
        unimplemented!()
    }
}

pub enum SplitAxis {
    X,
    Y,
    Z,
}

pub enum SplitOrShape {
    Split(SplitAxis, Float), // Splite Point
    NShape(u32),             // Number of Shapes
}

pub struct Node {
    pub SplitOrShape: SplitOrShape,
    pub Index: usize, // Child or Shape Index
}

impl Node {
    pub fn New() -> Node {
        return Node {
            SplitOrShape: SplitOrShape::NShape(0),
            Index: 0,
        };
    }

    pub fn InitLeaf(&self) {

    }
}

    //pub Shapes: Vec<&'a Shape>, // 24
    //pub Left: Option<Box<Node<'a>>>, // 8
    //pub Right: Option<Box<Node<'a>>>, // 8
