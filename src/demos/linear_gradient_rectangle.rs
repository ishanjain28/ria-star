pub struct LinearGradientRectangle;

use crate::{
    types::{Ray, Vec3},
    HORIZONTAL_PARTITION, VERTICAL_PARTITION,
    {demos::Chunk, Demo},
};

impl Demo for LinearGradientRectangle {
    fn name(&self) -> &'static str {
        "Linear Gradient Rectangle"
    }

    fn render(&self, buf: &mut [u8], width: usize, height: usize, _samples: u8) {
        let nx = width / VERTICAL_PARTITION;
        let ny = height / HORIZONTAL_PARTITION;

        for j in 0..VERTICAL_PARTITION {
            for i in 0..HORIZONTAL_PARTITION {
                let start_y = j * ny;
                let start_x = i * nx;
                let chunk = Chunk {
                    x: width,
                    y: height,
                    nx,
                    ny,
                    start_x,
                    start_y,
                };

                self.render_chunk(buf, chunk);
            }
        }
    }
    fn render_chunk(&self, buf: &mut [u8], meta: Chunk) {
        let Chunk {
            x,
            y,
            nx,
            ny,
            start_x,
            start_y,
        } = meta;
        // -2.0 and 4.0 in lower_left_corner and horizontal respectively
        // because our canvas is in 2:1 ratio
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

        for j in start_y..start_y + ny {
            for i in start_x..start_x + nx {
                let u = i as f64 / x as f64;
                let v = j as f64 / y as f64;
                let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

                let c = color(ray);

                let offset = (j * x + i) * 4;
                buf[offset] = (255.99 * c.r()) as u8;
                buf[offset + 1] = (255.99 * c.g()) as u8;
                buf[offset + 2] = (255.99 * c.b()) as u8;
            }
        }
    }
}

fn color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * unit_direction.y() + 1.0;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
