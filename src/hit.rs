use common::Float;

//pub struct Hit<'a> {
//    pub Shape: &'a (Shape + 'a),
//    pub T: Float,
// }
//
//impl<'a> Hit<'a> {
//    pub fn New(shape: &'a Shape, t: Float) -> Hit {
//        return Hit { Shape: shape, T: t };
//    }
// }

pub struct Hit {
    pub T: Float,
}

impl Hit {
    pub fn New(t: Float) -> Hit {
        return Hit { T: t };
    }
}
