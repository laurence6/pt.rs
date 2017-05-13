use std::fs::OpenOptions;
use std::io::BufWriter;

use camera::Camera;
use film::Film;
use ray::Ray;
use sampler::Sampler;
use scene::Scene;
use spectrum::Spectrum;
use vector::Point2u;

// SamplerIntegrator
pub struct Integrator {
    scene: Scene,
    sampler: Box<Sampler>,
    camera: Box<Camera>,
    film: Film,
}

impl Integrator {
    /// Sampler generates a sequence of sample, point on image. Camera turns a sample into ray.
    /// Call li() to compute the radiance along the ray arriving at the film.
    pub fn render(&mut self) {
        let mut file = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("output.ppm")
                .unwrap()
        );

        let (mut x, mut y) = (0, 0);
        loop {
            let pixel = Point2u::new(x, y);
            self.sampler.start_pixel(pixel);

            loop {
                let camera_sample = self.sampler.get_camera_sample(pixel);
                let ray = self.camera.generate_ray(&camera_sample);
                let l = self.li(&ray, 0);
                self.film.add_sample(camera_sample.p_film, &l);

                if !self.sampler.start_next_sample() {
                    break;
                }
            }

            if x < self.film.resolution.x {
                x += 1;
            } else {
                x = 0;
                y += 1;
                if y > self.film.resolution.y {
                    break;
                }
            };
        }

        self.film.write_image_ppm(&mut file);
    }

    fn li(&mut self, ray: &Ray, depth: u16) -> Spectrum {
        unimplemented!()
    }
}
