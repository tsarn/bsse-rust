mod raycast;
mod render;
mod vec3;

use raycast::{DemoCheckerboard, Material, PointLight, Renderable, Scene, Sphere};
use render::{render_scene, RenderParams};
use vec3::Vec3;

fn main() {
    let ivory = Material {
        refractive_index: 1.0,
        albedo: [0.6, 0.3, 0.1, 0.0],
        diffuse_color: Vec3::new(0.4, 0.4, 0.3),
        specular_exponent: 50.0,
    };

    let glass = Material {
        refractive_index: 1.5,
        albedo: [0.0, 0.5, 0.1, 0.8],
        diffuse_color: Vec3::new(0.6, 0.7, 0.8),
        specular_exponent: 125.0,
    };

    let red_rubber = Material {
        refractive_index: 1.0,
        albedo: [0.9, 0.1, 0.0, 0.0],
        diffuse_color: Vec3::new(0.3, 0.1, 0.1),
        specular_exponent: 10.0,
    };

    let mirror = Material {
        refractive_index: 1.0,
        albedo: [0.0, 10.0, 0.8, 0.0],
        diffuse_color: Vec3::new(1.0, 1.0, 1.0),
        specular_exponent: 1425.0,
    };

    let spheres: Vec<Box<dyn Renderable>> = vec![
        Box::new(Sphere {
            center: Vec3::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        }),
        Box::new(Sphere {
            center: Vec3::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        }),
        Box::new(Sphere {
            center: Vec3::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
        }),
        Box::new(DemoCheckerboard),
    ];

    let lights = vec![
        PointLight {
            position: Vec3::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        PointLight {
            position: Vec3::new(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        PointLight {
            position: Vec3::new(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];

    let scene = Scene {
        objects: spheres,
        lights,
        clear_color: Vec3::new(0.2, 0.7, 0.8),
    };

    let params = RenderParams {
        field_of_view: std::f32::consts::PI / 3.0,
        width: 1024,
        height: 768,
        depth: 5,
    };

    render_scene(&scene, &params).save("out.png").unwrap();
}
