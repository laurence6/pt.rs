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
        println!("Building k-d tree...");
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
    NotSplit,
    X,
    Y,
    Z,
}

pub enum SplitOrShape {
    Split(Float), // Splite Point
    Shape(usize), // Shape Index
}

pub enum ChildOrNShape {
    Child(usize), // Child Index
    NShape(u32), // Number of Shapes
}

pub struct Node {
    pub SplitAxis: SplitAxis,
    pub SplitOrShape: SplitOrShape,
    pub ChildOrNShape: ChildOrNShape,
}

impl Node {
    pub fn New() -> Node {
        return Node {
            SplitAxis: SplitAxis::NotSplit,
            SplitOrShape: SplitOrShape::Shape(0),
            ChildOrNShape: ChildOrNShape::NShape(1),
        };
    }

    pub fn InitLeaf() {

    }
}

    //pub Shapes: Vec<&'a Shape>, // 24
    //pub Left: Option<Box<Node<'a>>>, // 8
    //pub Right: Option<Box<Node<'a>>>, // 8
