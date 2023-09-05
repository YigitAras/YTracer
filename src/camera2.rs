use crate::{vector3::*, utils::*};

pub struct Camera {
    pub aspect_ratio: f64,          // Width Height ratio
    pub image_width: i64,           // Rendered image width in pixel count
    pub samples_per_pixel: u64,     // Random samples for each pixel
    pub max_depth: u64,             // Maximum number of ray bounces

    pub vfov: f64,                  // Vertical view angle (field of view)
    pub lookfrom: Vec3,             // Point camera is looking from
    pub lookat: Vec3,               // Point camera is looking at
    pub vup: Vec3,                  // Camera-relative up vector

    pub defocus_angle: f64,         // Variation angle of rays through each pixel (?)
    pub focus_dist: f64,            // Distance from camera lookfrom point to plane of perfect focus

    image_height: u64,              // Rendered image height
    center: Vec3,                   // Camera center
    pixel00_loc: Vec3,              // Location of pixel 0,0
    pixel_delta_u: Vec3,            // Offset to pixel to the right
    pixel_delta_v: Vec3,            // Offset to pixel below
    u: Vec3,                        // Camera basis vector
    v: Vec3,                        // Camera basis vector
    w: Vec3,                        // Camera basis vector
    defocus_disk_u: Vec3,           // Defocus disk horizontal radius
    defous_disk_v: Vec3             // Defocus disk vertical radius
}

impl Camera {
    pub fn init(aspect_ratio: f64,
                image_width: u64,
                samples_per_pixel: u64,
                max_depth: u64,
                vfov: f64,
                lookfrom: Vec3,
                lookat: Vec3,
        ) {
        // Will be mostly this
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        let image_height = (image_width as f64/aspect_ratio) as u64;

        let center = lookfrom;

        // Determine the viewport dims
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the unit basis vecs for camera coord frame
        let w = Vec3::unit_vector(lookfrom-lookat);
        let u = Vec3::unit_vector(vup.cross(w));
        let v = w.cross(u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u =  u * viewport_width;
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors to the next pixel
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (w * focus_dist) - viewport_u/2.0 - viewport_v/2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u+pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle/2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
    }
    pub fn adjust_focus_dist(&mut self,new_dist: f64) {
        self.focus_dist = new_dist;
    }
}