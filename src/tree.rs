use ray::Ray;
use hit::Hit;
use shape::Shape;
use common::Float;

// k-d tree
pub struct Tree<'a> {
    pub Root: Node<'a>,
}

impl<'a> Tree<'a> {
    pub fn New(shapes: Vec<&'a Shape>) -> Tree<'a> {
        println!("Building k-d tree...");
        let node = Node::New(shapes);
        println!("Done");
        return Tree { Root: node };
    }

    pub fn Intersect(&self, r: &Ray) -> Option<Hit> {
        unimplemented!()
    }
}

pub enum NodeType {
    Split(Float),
    OneShape,
    MulShapes,
}

pub struct Node<'a> { // 48 - 56
    pub Type: NodeType, // 1
    pub Point: Float, // 4 - 8
    //pub Shapes: Vec<&'a Shape>, // 24
    //pub Left: Option<Box<Node<'a>>>, // 8
    //pub Right: Option<Box<Node<'a>>>, // 8
}

impl<'a> Node<'a> {
    pub fn New(shapes: Vec<&'a Shape>) -> Node {
        return Node {
            Type: NodeType::OneShape,
            Point: 0.0,
            //Shapes: shapes,
            //Left: None,
            //Right: None,
        };
    }
}
