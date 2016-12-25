use ray::Ray;
use hit::Hit;
use shape::Shape;
use common::Float;

// k-d tree
pub struct Tree<'a> {
    pub Shapes: Vec<&'a Shape>,
    pub ShapeIndices: Vec<usize>,
    pub NNode: usize, // TODO: remove?
    pub Root: Node,
}

impl<'a> Tree<'a> {
    pub fn New(shapes: Vec<&'a Shape>) -> Tree {
        print!("Building k-d tree ({} shapes) ... ", shapes.len());

        let mut shapeIndices = Vec::<usize>::new();
        let node = Node::NewLeaf(Vec::<usize>::new(), &mut shapeIndices);

        println!("Done");

        return Tree {
            Shapes: shapes,
            ShapeIndices: shapeIndices,
            NNode: 0,
            Root: node,
        };
    }

    pub fn Intersect(&self, r: &Ray) -> Option<Hit> {
        unimplemented!()
    }
}

pub enum SplitAxis { X, Y, Z }

pub enum SplitOrShape {
    Split(SplitAxis, Float), // Splite Point
    Shape(u32),             // Number of Shapes
}

pub struct Node {
    pub SplitOrShape: SplitOrShape,
    pub Index: usize, // Child or Shape Index
}

impl Node {
    pub fn NewInterior(axis: SplitAxis, point: Float) -> Node {
        return Node {
            SplitOrShape: SplitOrShape::Split(axis, point),
            Index: 0,
        };
    }

    pub fn NewLeaf(mut shapes: Vec<usize>, shapeIndices: &mut Vec<usize>) -> Node {
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

    //pub Shapes: Vec<&'a Shape>, // 24
    //pub Left: Option<Box<Node<'a>>>, // 8
    //pub Right: Option<Box<Node<'a>>>, // 8
