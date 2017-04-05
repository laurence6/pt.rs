use common::Float;
use common::clamp;
use vector::{Vector3f, Point2f};
use spectrum::Spectrum;

type BxDFType = u8;
pub const REFLECTION:   BxDFType = 1 << 0;
pub const TRANSMISSION: BxDFType = 1 << 1;
pub const DIFFUSE:      BxDFType = 1 << 2;
pub const GLOSSY:       BxDFType = 1 << 3;
pub const SPECULAR:     BxDFType = 1 << 4;
pub const ALL:          BxDFType = REFLECTION | TRANSMISSION | DIFFUSE | GLOSSY | SPECULAR;

pub struct BxDF {
    Type: BxDFType,
}

impl BxDF {
    pub fn MatchType(&self, t: BxDFType) -> bool {
        debug_assert!(self.Type <= ALL);

        (self.Type & t) == self.Type
    }

    pub fn F(&self, wo: Vector3f, wi: Vector3f) -> Spectrum {
        unimplemented!()
    }

    pub fn SampleF(&self, wo: Vector3f, wi: &mut Vector3f, sample: Point2f, pdf: &mut Float, sampledType: &mut BxDFType) -> Spectrum {
        unimplemented!()
    }

    pub fn rho(&self) {}
}


fn CosTheta(w: Vector3f) -> Float {
    w.Z
}

fn Cos2Theta(w: Vector3f) -> Float {
    w.Z * w.Z
}

fn AbsCosTheta(w: Vector3f) -> Float {
    w.Z.abs()
}

fn SinTheta(w: Vector3f) -> Float {
    Sin2Theta(w).sqrt()
}

fn Sin2Theta(w: Vector3f) -> Float {
    (1.0 - Cos2Theta(w)).max(0.0)
}

fn TanTheta(w: Vector3f) -> Float {
    SinTheta(w) / CosTheta(w)
}

fn Tan2Theta(w: Vector3f) -> Float {
    Sin2Theta(w) / Cos2Theta(w)
}

fn CosPhi(w: Vector3f) -> Float {
    let sinTheta = SinTheta(w);
    if sinTheta == 0.0 {
        return 1.0;
    } else {
        return clamp(w.X / sinTheta, -1.0, 1.0);
    }
}

fn Cos2Phi(w: Vector3f) -> Float {
    CosPhi(w) * CosPhi(w)
}

fn SinPhi(w: Vector3f) -> Float {
    let sinTheta = SinTheta(w);
    if sinTheta == 0.0 {
        return 0.0;
    } else {
        return clamp(w.Y / sinTheta, -1.0, 1.0);
    }
}

fn Sin2Phi(w: Vector3f) -> Float {
    SinPhi(w) * SinPhi(w)
}

fn CosDPhi(wa: Vector3f, wb: Vector3f) -> Float {
    clamp((wa.X * wb.X + wa.Y * wb.Y)
        /((wa.X * wa.X + wa.Y * wa.Y) * (wb.X * wb.X + wb.Y * wb.Y)).sqrt(),
        -1.0, 1.0)
}
