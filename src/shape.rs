use std::sync::Arc;

use bbox::BBox3f;
use interaction::Interaction;
use material::Material;
use ray::Ray;
use reflection::BSDF;

pub trait Shape : Sync + Send {
    fn bbox(&self) -> BBox3f;

    /// intersect_p returns whether or not the ray intersects this shape.
    fn intersect_p(&self, ray: &Ray) -> bool;

    /// intersect determines whether the ray intersects this shape.
    /// If an intersection occurs, it returns the detail of the intersection (Interaction.shape is NOT initialized in this method) and t value of the ray at the intersection point.
    fn intersect(&self, ray: &Ray) -> Option<(Interaction, f32)>;

    fn material(&self) -> Arc<Material>;

    fn compute_scattering(&self, i: &Interaction) -> BSDF {
        self.material().compute_scattering(i)
    }
}

/// intersect determines whether the ray intersects this shape.
/// If an intersection occurs, the detail of the intersection (Interaction.shape IS initialized in this method) is returned and Ray.t_max is updated with the t value at the intersection point.
pub fn intersect(shape: &Arc<Shape>, ray: &mut Ray) -> Option<Interaction> {
    if let Some((mut i, t)) = shape.intersect(ray) {
        ray.t_max = t;
        i.shape = Some(shape.clone());
        return Some(i);
    }
    return None;
}
