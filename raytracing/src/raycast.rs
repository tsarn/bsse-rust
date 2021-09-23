use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3,
    pub specular_exponent: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub intensity: f32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RayIntersection<'a> {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Renderable {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection>;
}

pub struct Scene {
    pub objects: Vec<Box<dyn Renderable>>,
    pub lights: Vec<PointLight>,
    pub clear_color: Vec3,
}

fn reflect(i: &Vec3, n: Vec3) -> Vec3 {
    *i - n * 2.0 * i.dot(n)
}

fn refract(i: &Vec3, n: Vec3, eta_t: f32, eta_i: f32) -> Vec3 {
    let cosi = -i.dot(n).clamp(-1.0, 1.0);
    if cosi < 0.0 {
        refract(i, n * -1.0, eta_i, eta_t)
    } else {
        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            *i * eta + n * (eta * cosi - k.sqrt())
        }
    }
}

impl Renderable for Vec<Box<dyn Renderable>> {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        self.iter()
            .map(|obj| obj.ray_intersect(ray))
            .flatten()
            .min_by(|i, j| i.distance.partial_cmp(&j.distance).unwrap())
    }
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray, depth: i32) -> Vec3 {
        let intersection = self.objects.ray_intersect(ray);
        if depth <= 0 || intersection.is_none() {
            return self.clear_color;
        }

        let RayIntersection {
            point,
            normal,
            material,
            ..
        } = intersection.unwrap();

        let eps = 1e-3;
        let reflect_dir = reflect(&ray.direction, normal).normalize();
        let reflect_color = self.cast_ray(
            &Ray {
                origin: if reflect_dir.dot(normal) < 0.0 {
                    point - normal * eps
                } else {
                    point + normal * eps
                },
                direction: reflect_dir,
            },
            depth - 1,
        );

        let refract_dir =
            refract(&ray.direction, normal, material.refractive_index, 1.0).normalize();
        let refract_color = self.cast_ray(
            &Ray {
                origin: if refract_dir.dot(normal) < 0.0 {
                    point - normal * eps
                } else {
                    point + normal * eps
                },
                direction: refract_dir,
            },
            depth - 1,
        );

        let mut diffuse_light = 0.0;
        let mut specular_light = 0.0;

        for light in self.lights.iter() {
            let light_dir = (light.position - point).normalize();

            let to_light = self.objects.ray_intersect(&Ray {
                origin: if light_dir.dot(normal) < 0.0 {
                    point - normal * eps
                } else {
                    point + normal * eps
                },
                direction: light_dir,
            });

            if let Some(RayIntersection { distance, .. }) = to_light {
                if distance < (light.position - point).length() {
                    continue;
                }
            }

            diffuse_light += light.intensity * light_dir.dot(normal).max(0.0);
            specular_light += (reflect(&(light_dir * -1.0), normal) * -1.0)
                .dot(ray.direction)
                .max(0.0)
                .powf(material.specular_exponent)
                * light.intensity;
        }

        material.diffuse_color * diffuse_light * material.albedo[0]
            + Vec3::new(1.0, 1.0, 1.0) * specular_light * material.albedo[1]
            + reflect_color * material.albedo[2]
            + refract_color * material.albedo[3]
    }
}

// --- concrete objects ---

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Renderable for Sphere {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        let l = self.center - ray.origin;
        let tca = l.dot(ray.direction);
        let d2 = l.dot(l) - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }

        let thc = (self.radius * self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        let d = if t0 < 0.0 { t1 } else { t0 };

        if d < 0.0 {
            None
        } else {
            let hit = ray.origin + ray.direction * d;

            Some(RayIntersection {
                distance: t0,
                point: hit,
                normal: (hit - self.center).normalize(),
                material: &self.material,
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct DemoCheckerboard;

const DEMO_CHECKERBOARD_MATERIAL_1: Material = Material {
    refractive_index: 1.0,
    albedo: [0.3, 0.0, 0.0, 0.0],
    diffuse_color: Vec3::new(1.0, 1.0, 1.0),
    specular_exponent: 1.0,
};

const DEMO_CHECKERBOARD_MATERIAL_2: Material = Material {
    refractive_index: 1.0,
    albedo: [0.3, 0.0, 0.0, 0.0],
    diffuse_color: Vec3::new(1.0, 0.7, 0.3),
    specular_exponent: 1.0,
};

impl Renderable for DemoCheckerboard {
    fn ray_intersect(&self, ray: &Ray) -> Option<RayIntersection> {
        if ray.direction.y.abs() < 1e-3 {
            return None;
        }

        let d = -(ray.origin.y + 4.0) / ray.direction.y;
        let hit = ray.origin + ray.direction * d;

        if d > 0.0 && hit.x.abs() < 10.0 && hit.z < -10.0 && hit.z > -30.0 {
            let even = ((hit.x * 0.5 + 1000.0) as i32 + (0.5 * hit.z) as i32) % 2 == 0;
            Some(RayIntersection {
                distance: d,
                point: hit,
                normal: Vec3::new(0.0, 1.0, 0.0),
                material: if even {
                    &DEMO_CHECKERBOARD_MATERIAL_1
                } else {
                    &DEMO_CHECKERBOARD_MATERIAL_2
                },
            })
        } else {
            None
        }
    }
}
