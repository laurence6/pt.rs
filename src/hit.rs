use common::Float;

pub struct Hit {
    pub T: Float,
}

impl Hit {
    pub fn New(t: Float) -> Hit {
        return Hit { T: t };
    }
}
