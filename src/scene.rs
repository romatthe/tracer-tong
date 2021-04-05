use crate::vec::{Point, Vec3};
use crate::ray::Ray;

pub trait SceneObject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool;
}

pub struct Hit {
    front_face: bool,
    point: Point,
    normal: Vec3,
    t: f32,
}

impl Hit {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        }  else {
            -outward_normal
        }
    }
}

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere {
            center,
            radius
        }
    }
}

impl SceneObject for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        let oc = ray.origin() - &self.center;

        let a = ray.direction().len_squared();
        let b = oc.dot(ray.direction());
        let c = oc.len_squared() - (self.radius * self.radius);

        let discriminant = (b * b) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt = discriminant.sqrt();

        // Find the nearest root that lies in acceptable range
        let mut root = (-b - sqrt) / a;
        if root < t_min || t_max < root {
            root = (-b + sqrt) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        let outward_normal = (ray.at(root) - &self.center) / self.radius;

        hit.t = root;
        hit.point = ray.at(root);
        hit.set_face_normal(ray, outward_normal);

        return true;
    }
}

impl SceneObject for Vec<Box<dyn SceneObject>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit: &mut Hit) -> bool {
        todo!()
    }
}