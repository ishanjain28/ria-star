use {
    crate::{
        demos::Demo,
        types::{Hitable, HitableList, Ray, Sphere, Vec3},
        Camera,
    },
    rand::Rng,
};

pub struct DiffuseMaterials;

impl Demo for DiffuseMaterials {
    fn name(&self) -> &'static str {
        "Diffuse Materials"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, samples: u8) {
        let world = HitableList {
            list: vec![
                Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
                Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
            ],
        };

        let camera: Camera = Default::default();
        let mut rng = rand::thread_rng();
        let mut offset = 0;

        for j in (0..height).rev() {
            for i in 0..width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);

                for _s in 0..samples {
                    let u = (i as f64 + rng.gen::<f64>()) / width as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / height as f64;

                    let r = camera.get_ray(u, v);
                    color += calc_color(r, &world, &mut rng);
                }

                color /= samples as f64;
                // Without taking square root of each color, we get a picture that
                // is quite dark
                // Spheres in this case are absorbing 50% of the light casted on them
                // So, IRL, It *should* look a bit lighter in color
                // To do that, We apply gamma correction by a factor of 2
                // which means multiple rgb values by 1/gamma aka 1/2
                buf[offset] = (255.99 * color.r().sqrt()) as u8;
                buf[offset + 1] = (255.99 * color.g().sqrt()) as u8;
                buf[offset + 2] = (255.99 * color.b().sqrt()) as u8;
                offset += 4;
            }
        }
    }
}

fn calc_color(ray: Ray, world: &HitableList, rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    // The value of t_min here could've been 0.0 but since f32/f64 can only be
    // partially compared, It may cause shadow acne effect.
    // To combat this problem, We set a bias
    // More information here, https://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/#shadow-acne
    if let Some(hit_rec) = world.hit(&ray, 0.001, std::f64::MAX) {
        let target = hit_rec.p + hit_rec.normal + random_point_in_unit_space(rng);
        calc_color(Ray::new(hit_rec.p, target - hit_rec.p), &world, rng) * 0.5
    } else {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn random_point_in_unit_space(rng: &mut rand::rngs::ThreadRng) -> Vec3 {
    let mut point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
        - Vec3::new(1.0, 1.0, 1.0);
    while point.sq_len() >= 1.0 {
        point = Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - Vec3::new(1.0, 1.0, 1.0);
    }
    point
}