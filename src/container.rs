use bbox::BBox3f;
use interaction::Interaction;
use ray::Ray;

/// Container stores the references to Shapes in the Scene. It's also responsible for initializing the shape field of the returned Interaction.
pub trait Container {
    fn bbox(&self) -> BBox3f;
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn intersect(&self, ray: &Ray) -> Option<Interaction>;
}
