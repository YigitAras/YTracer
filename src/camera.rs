use crate::utils::*;


use crate::ray::*;
use crate::vector3::*;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new() -> Self{
        let aspect_ration = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ration * viewport_height;
        let focal_lenght = 1.0;

        let origin  = Vec3::new(0.0,0.0,0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0); 
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self{
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0, focal_lenght),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)    
    }
}