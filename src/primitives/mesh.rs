use std::sync::Arc;

use crate::accelerators::{aabb::*, bvh::*};
use crate::core::hittable::*;
use crate::core::hittable_list::*;
use crate::geometry::ray::*;
use crate::geometry::vector3::*;
use crate::material::*;

use crate::primitives::triangle::*;

pub struct Mesh {
    pub triangles: Bvh,
    pub num_triangles: usize,
    pub name: String,
}

impl Mesh {
    pub fn new(mesh_file: &str, mesh_mat: Arc<dyn Material + Sync + Send>, scaling: Vec3) -> Self {
        let mut list = HittableList::default();
        let (models, _) = tobj::load_obj(mesh_file, &tobj::LoadOptions::default())
            .expect("Failed to OBJ load file");

        let model = &models[0].mesh;
        let pos_ind = &model.indices;

        // TODO: Can read the OBJ Model materials here too...
        for chunk_verts in pos_ind.chunks(3) {
            let a = Vec3::new(
                model.positions[chunk_verts[0] as usize * 3] as f64 * scaling.x,
                model.positions[chunk_verts[0] as usize * 3 + 1] as f64 * scaling.y,
                model.positions[chunk_verts[0] as usize * 3 + 2] as f64 * scaling.z,
            );
            let b = Vec3::new(
                model.positions[chunk_verts[1] as usize * 3] as f64 * scaling.x,
                model.positions[chunk_verts[1] as usize * 3 + 1] as f64 * scaling.y,
                model.positions[chunk_verts[1] as usize * 3 + 2] as f64 * scaling.z,
            );
            let c = Vec3::new(
                model.positions[chunk_verts[2] as usize * 3] as f64 * scaling.x,
                model.positions[chunk_verts[2] as usize * 3 + 1] as f64 * scaling.y,
                model.positions[chunk_verts[2] as usize * 3 + 2] as f64 * scaling.z,
            );

            list.add(Arc::new(Triangle::from_points(
                a,
                b,
                c,
                Arc::clone(&mesh_mat),
            )));
        }

        let list_len = list.objects.len();
        let triangles = Bvh::new(&mut list, 0, list_len, 0.0, 0.0);
        println!("Loaded mesh: {}", mesh_file);
        Self {
            triangles,
            num_triangles: list_len,
            name: mesh_file.to_string(),
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        //println!("Hit one mesh: {}", self.name);
        self.triangles.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.triangles.bbox;
        true
    }
}
