use ray::Ray;
use hit::Hit;
use shape::Shape;
use common::Float;

// k-d tree
pub struct Tree {
    pub Shapes: Vec<Box<Shape>>,
    pub Root: Node,
}

impl Tree {
    pub fn New(shapes: Vec<Box<Shape>>) -> Tree {
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

pub enum NodeType {
    Split(Float),
    Shape(usize),
}

pub struct Node { // 48 - 56
    pub Type: NodeType, // 8
    //pub Shapes: Vec<&'a Shape>, // 24
    //pub Left: Option<Box<Node<'a>>>, // 8
    //pub Right: Option<Box<Node<'a>>>, // 8
}

impl Node {
    pub fn New() -> Node {
        return Node {
            Type: NodeType::Shape(0),
            //Shapes: shapes,
            //Left: None,
            //Right: None,
        };
    }
}
