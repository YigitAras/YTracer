use crate::utils::*;
use crate::hittable::*;



pub trait Material {
    fn scatter(&self);
}