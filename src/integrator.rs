use std::cmp::min;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use camera::Camera;
use film::{Film, FilmTile};
use interaction::Interaction;
use ray::Ray;
use reflection::{SPECULAR, BSDF};
use sampler::Sampler;
use scene::Scene;
use spectrum::Spectrum;
use vector::Vector3f;

/// Path Integrator.
pub struct Integrator<S, C> where S: 'static + Sampler, C: 'static + Camera {
    scene: Arc<Scene>,
    sampler: S,
    camera: C,
    film: Arc<Film>,
}

impl<S, C> Integrator<S, C> where S: 'static + Sampler, C: 'static + Camera {
    pub fn new(scene: Scene, sampler: S, camera: C, film: Film) -> Integrator<S, C> {
        Integrator {
            scene: Arc::new(scene),
            sampler,
            camera,
            film: Arc::new(film),
        }
    }

    pub fn render(&self, max_threads: u8) {
        let film_tile_iter = Arc::new(Mutex::new(Film::iter(self.film.clone())));

        let mut handles = Vec::new();
        for _ in 0..max_threads {
            let scene = self.scene.clone();
            let sampler = self.sampler.clone();
            let camera = self.camera.clone();
            let film = self.film.clone();
            let film_tile_iter = film_tile_iter.clone();

            let handle = spawn(move || {
                let mut integrator = IntegratorLocal {
                    scene,
                    sampler,
                    camera,
                    film,
                };

                while let Some(tile) = {
                    let mut iter = film_tile_iter.lock().unwrap();
                    iter.next()
                } {
                    integrator.render(tile);

                    integrator.sampler = integrator.sampler.clone();
                }
            });
            handles.push(handle);
        }

        for handle in handles.into_iter() {
            handle.join().unwrap();
        }

        let mut file = BufWriter::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .open("output.ppm")
                .unwrap()
        );
        self.film.write_image_ppm(&mut file);
    }
}

struct IntegratorLocal<S, C> where S: 'static + Sampler, C: 'static + Camera {
    scene: Arc<Scene>,
    sampler: S,
    camera: C,
    film: Arc<Film>,
}

impl<S, C> IntegratorLocal<S, C> where S: 'static + Sampler, C: 'static + Camera {
    /// Sampler generates a sequence of sample, point on image. Camera turns a sample into ray.
    /// Call li() to compute the radiance along the ray arriving at the film.
    fn render(&mut self, mut tile: FilmTile) {
        for pixel in tile.bbox.iter() {
            self.sampler.start_pixel(pixel);

            loop {
                let camera_sample = self.sampler.get_camera_sample(pixel);
                let ray = self.camera.generate_ray(&camera_sample);
                let l = self.li(ray);
                tile.add_sample(camera_sample.p_film, l);

                if !self.sampler.start_next_sample() {
                    break;
                }
            }
        }

        self.film.merge_film_tile(tile);
    }

    fn estimate_direct(&mut self, light: usize, i: &Interaction, bsdf: &BSDF) -> Spectrum {
        let mut ld = Spectrum::default();
        let (wi, li, pdf, visibility) = self.scene.lights()[light].sample_li(i, self.sampler.get_2d());
        if pdf > 0. && !li.is_black() {
            let f = bsdf.f(i.wo, wi) * (Vector3f::from(i.n).dot(wi).abs());
            if !f.is_black() {
                if visibility.unoccluded(&self.scene) {
                    ld += f * li / pdf;
                }
            }
        }
        return ld;
    }

    fn uniform_sample_one_light(&mut self, i: &Interaction, bsdf: &BSDF) -> Spectrum {
        let n_lights = self.scene.lights().len();
        if n_lights == 0 {
            return Spectrum::default();
        }
        let light_i = min(
            (n_lights as f32 * self.sampler.get_1d()) as usize,
            n_lights - 1,
        );
        return self.estimate_direct(light_i, i, bsdf) * n_lights as f32;
    }

    fn li(&mut self, mut ray: Ray) -> Spectrum {
        let mut l = Spectrum::default();
        let mut beta = Spectrum::new(1., 1., 1.);
        let mut specular_bounce = false;
        let mut bounces = 0;
        loop {
            let i = self.scene.intersect(&ray);

            if bounces == 0 || specular_bounce {
                if let Some(ref i) = i {
                } else {
                    for light in self.scene.lights().iter() {
                        l += beta * light.le(&ray);
                    }
                }
            }

            if i.is_none() {
                break;
            }

            let i = i.unwrap();
            let bsdf = i.compute_scattering();

            l += beta * self.uniform_sample_one_light(&i, &bsdf);

            let (wi, f, pdf, bxdf_flag) = bsdf.sample_f(i.wo, self.sampler.get_2d());
            if pdf == 0. || f.is_black() {
                break;
            }
            beta *= f * Vector3f::from(i.n).dot(wi).abs() / pdf;
            specular_bounce = bxdf_flag & SPECULAR != 0;
            ray = i.spawn_ray(wi);

            bounces += 1;
            if bounces > 3 {
                let q = beta.y().min(0.95);
                if self.sampler.get_1d() > q {
                    break;
                }
                beta /= q;
            }
        }
        return l;
    }
}
