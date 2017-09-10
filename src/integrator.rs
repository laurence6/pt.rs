use std::cmp::min;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use std::thread::spawn;

use camera::Camera;
use container::Container;
use film::{Film, FilmTile};
use interaction::Interaction;
use light::Light;
use ray::Ray;
use reflection::{SPECULAR, BSDF};
use sampler::Sampler;
use scene::Scene;
use spectrum::Spectrum;
use vector::Vector3f;

/// Path Integrator.
pub struct Integrator<Co, Cam, S> where Co: 'static + Container, Cam: 'static + Camera, S: 'static + Sampler {
    scene: Arc<Scene<Co>>,
    camera: Cam,
    sampler: S,
    film: Arc<Film>,
}

impl<Co, Cam, S> Integrator<Co, Cam, S> where Co: 'static + Container, Cam: 'static + Camera, S: 'static + Sampler {
    pub fn new(scene: Scene<Co>, sampler: S, camera: Cam, film: Film) -> Integrator<Co, Cam, S> {
        Integrator {
            scene: Arc::new(scene),
            camera,
            sampler,
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
                let integrator = IntegratorLocal {
                    scene,
                    camera,
                    film,
                };
                let mut sampler = sampler;

                while let Some(tile) = {
                    let mut iter = film_tile_iter.lock().unwrap();
                    iter.next()
                } {
                    integrator.render(&mut sampler, tile);

                    sampler = sampler.clone();
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

struct IntegratorLocal<Co, Cam> where Co: 'static + Container, Cam: 'static + Camera {
    scene: Arc<Scene<Co>>,
    camera: Cam,
    film: Arc<Film>,
}

impl<Co, Cam> IntegratorLocal<Co, Cam> where Co: 'static + Container, Cam: 'static + Camera {
    /// Sampler generates a sequence of sample, point on image. Camera turns a sample into ray.
    /// Call li() to compute the radiance along the ray arriving at the film.
    fn render<S>(&self, sampler: &mut S, mut tile: FilmTile) where S: Sampler {
        for pixel in tile.bbox.iter() {
            sampler.start_pixel(pixel);

            loop {
                let camera_sample = sampler.get_camera_sample(pixel);
                let ray = self.camera.generate_ray(&camera_sample);
                let l = self.li(sampler, ray);
                tile.add_sample(camera_sample.p_film, l);

                if !sampler.start_next_sample() {
                    break;
                }
            }
        }

        self.film.merge_film_tile(tile);
    }

    fn estimate_direct<S>(&self, sampler: &mut S, light: &Light, i: &Interaction, bsdf: &BSDF) -> Spectrum where S: Sampler {
        let mut ld = Spectrum::default();
        let (wi, li, pdf, visibility) = light.sample_li(i, sampler.get_2d());
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

    fn uniform_sample_one_light<S>(&self, sampler: &mut S, i: &Interaction, bsdf: &BSDF) -> Spectrum where S: Sampler {
        let n_lights = self.scene.lights().len();
        if n_lights == 0 {
            return Spectrum::default();
        }
        let light_i = min(
            (n_lights as f32 * sampler.get_1d()) as usize,
            n_lights - 1,
        );
        let light = &*self.scene.lights()[light_i];
        return self.estimate_direct(sampler, light, i, bsdf) * n_lights as f32;
    }

    fn li<S>(&self, sampler: &mut S, mut ray: Ray) -> Spectrum where S: Sampler {
        let mut l = Spectrum::default();
        let mut beta = Spectrum::new(1., 1., 1.);
        let mut specular_bounce = false;
        let mut bounces = 0;
        loop {
            let i = self.scene.intersect(&ray);

            if bounces == 0 || specular_bounce {
                if let Some(ref i) = i {
                    l += beta * i.le(-ray.direction);
                }
            }

            if i.is_none() {
                break;
            }

            let i = i.unwrap();
            let bsdf = i.compute_scattering();

            l += beta * self.uniform_sample_one_light(sampler, &i, &bsdf);

            let (wi, f, pdf, bxdf_flag) = bsdf.sample_f(i.wo, sampler.get_2d());
            if pdf == 0. || f.is_black() {
                break;
            }
            beta *= f * Vector3f::from(i.n).dot(wi).abs() / pdf;
            specular_bounce = bxdf_flag & SPECULAR != 0;
            ray = i.spawn_ray(wi);

            bounces += 1;
            if bounces > 3 {
                let q = beta.y().min(0.95);
                if sampler.get_1d() > q {
                    break;
                }
                beta /= q;
            }
        }
        return l;
    }
}
