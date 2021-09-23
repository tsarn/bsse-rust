use crate::raycast::{Ray, Scene};
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct RenderParams {
    pub field_of_view: f32,
    pub width: u32,
    pub height: u32,
    pub depth: i32,
}

pub fn render_scene(scene: &Scene, params: &RenderParams) -> image::DynamicImage {
    let width = params.width as f32;
    let height = params.height as f32;

    let mut framebuffer = image::DynamicImage::new_rgb8(params.width, params.height);

    for j in 0..params.height {
        for i in 0..params.width {
            let x =
                (2.0 * (i as f32 + 0.5) / width - 1.0) * (params.field_of_view / 2.0).tan() * width
                    / height;
            let y = -(2.0 * (j as f32 + 0.5) / height - 1.0) * (params.field_of_view / 2.0).tan();
            let dir = Vec3::new(x, y, -1.0).normalize();

            let color = scene.cast_ray(
                &Ray {
                    origin: Vec3::zero(),
                    direction: dir,
                },
                params.depth,
            );

            framebuffer.as_mut_rgb8().unwrap().put_pixel(
                i,
                j,
                image::Rgb([
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                ]),
            );
        }
    }

    framebuffer
}
