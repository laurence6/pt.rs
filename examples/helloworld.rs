use std::sync::Arc;

extern crate pt;

use pt::api::*;

const NTHREADS: u8 = 4;

const RESOLUTION: Point2u = Point2u { x: 1600, y: 1280 };

const SAMPLES_PER_PIXEL: u32 = 500;

fn main() {
    let mut builder = Builder::new();

    create_light(&mut builder);
    create_sphere(&mut builder);
    create_floor(&mut builder);

    let scene = builder.construct(|shapes| KdTree::new(shapes, None));

    let film = Film::new(RESOLUTION, GaussianFilter::new(2., 2.));
    let sampler = HaltonSampler::new(SAMPLES_PER_PIXEL, film.sample_bbox());

    let camera = {
        PerspectiveCamera::new(
            Transform::look_at(Point3f::new(-10., -30., 10.), Point3f::default(), Vector3f::new(0., 0., 1.)),
            Transform::perspective(30., 0.01, 1000.),
            BBox2f::new(Point2f::new(-1.25, -1.), Point2f::new(1.25, 1.)),
            RESOLUTION,
        )
    };

    let integrator = Integrator::new(scene, sampler, camera, film);
    integrator.render(NTHREADS);
}

fn create_light(builder: &mut Builder) {
    let texture = Arc::new(ConstantTexture::new(Spectrum::from(0.)));
    let material = Arc::new(LambertianReflectionMaterial::new(texture));

    let triangle1 = Triangle::new([Point3f::new(-1., -1., 3.), Point3f::new(1., -1., 3.), Point3f::new(1., 1., 3.)], None, true, material.clone());
    builder.add_area_light(AreaLight::new(Spectrum::from(0.9), triangle1));

    let triangle2 = Triangle::new([Point3f::new(1., 1., 3.), Point3f::new(-1., 1., 3.), Point3f::new(-1., -1., 3.)], None, true, material.clone());
    builder.add_area_light(AreaLight::new(Spectrum::from(0.9), triangle2));
}

fn create_sphere(builder: &mut Builder) {
    let texture = Arc::new(ConstantTexture::new(Spectrum::from(1.)));
    let material = Arc::new(LambertianReflectionMaterial::new(texture));
    let sphere = Sphere::new(Point3f::new(0., 0., 1.), 1., material);
    builder.add_shape(sphere);
}

fn create_floor(builder: &mut Builder) {
    let texture = Arc::new(ConstantTexture::new(Spectrum::from(0.5)));
    let material = Arc::new(LambertianReflectionMaterial::new(texture));

    let triangle1 = Triangle::new([Point3f::new(-10., -10., 0.), Point3f::new(10., -10., 0.), Point3f::new(10., 10., 0.)], None, false, material.clone());
    builder.add_shape(triangle1);

    let triangle2 = Triangle::new([Point3f::new(10., 10., 0.), Point3f::new(-10., 10., 0.), Point3f::new(-10., -10., 0.)], None, false, material.clone());
    builder.add_shape(triangle2);
}
