//! A scene that can be rendered, contains information about camera,
//! objects, materials, image size and so on.
//!
//! # Example
//! Here's a simple example scene of a sphere on a plane illuminated by a
//! light to the right side of the scene. You can also see some other examples
//! under [`scenes/`](scenes/).
//!
//! ```json
//! {
//! 	"camera": {
//! 		"pos": [0, 0, -3],
//! 		"target": [0, 0, 0],
//! 		"up": [0, 1, 0],
//! 		"fovy": 60
//! 	},
//! 	"geometry": [
//! 		{
//! 			"type": "sphere",
//! 			"center": [0, 0, 0],
//! 			"radius": 0.5,
//! 			"lambertian": [0.8, 0.1, 0.1]
//! 		},
//! 		{
//! 			"type": "plane",
//! 			"center": [0, -0.5, 0],
//! 			"normal": [0, 1, 0],
//! 			"lambertian": [0.9, 0.9, 0.9]
//! 		}
//! 	],
//! 	"light": {
//! 		"pos": [0.75, 0.75, -2],
//! 		"intensity": [10, 10, 10]
//! 	},
//! 	"width": 512,
//! 	"height": 512,
//! 	"n_samples": 8
//! }
//! ```

use std::io::prelude::*;
use std::fs::File;

use serde_json::{self, Value};

use vec3f::Vec3f;
use camera::Camera;
use geom::{Sphere, Plane, ISPCGeometry};
use lights::PointLight;
use material::Lambertian;

pub struct Scene {
    /// Image width
    pub width: usize,
    /// Image height
    pub height: usize,
    pub n_samples: usize,
    pub camera: Camera,
    pub geometry: Vec<Box<ISPCGeometry>>,
    pub light: PointLight,
}

impl Scene {
    pub fn load(file: &str) -> Scene {
        let mut f = match File::open(file) {
            Ok(f) => f,
            Err(e) => panic!("Failed to open scene file: {}", e),
        };
        let mut content = String::new();
        if let Err(e) = f.read_to_string(&mut content) {
            panic!("Failed to read scene file: {}", e);
        }
        let data: Value = match serde_json::from_str(&content[..]) {
            Ok(d) => d,
            Err(e) => panic!("JSON parsing error: {}", e),
        };
        if !data.is_object() {
            panic!("Expected a root JSON object. See example scenes");
        }
        let img_width = data.find("width").expect("image width must be set")
            .as_u64().expect("image width must be a uint") as usize;
        let img_height = data.find("height").expect("image height must be set")
            .as_u64().expect("image height must be a uint") as usize;
        let n_samples = data.find("n_samples").expect("n_samples must be set")
            .as_u64().expect("n_samples must be a uint") as usize;
        let camera = Scene::load_camera(data.find("camera").expect("A camera must be specified"),
                                        img_width, img_height);
        let geom = Scene::load_geometry(data.find("geometry").expect("A list of geometry must be set"));
        let light = Scene::load_light(data.find("light").expect("A light must be specified"));
        Scene { width: img_width, height: img_height, n_samples: n_samples,
                camera: camera, geometry: geom, light: light }
                
    }
    fn load_camera(e: &Value, width: usize, height: usize) -> Camera {
        let pos = Scene::load_vec3f(e.find("pos").expect("Camera view position must be set"))
            .expect("Invalid camera position");
        let target = Scene::load_vec3f(e.find("target").expect("Camera view target must be set"))
            .expect("Invalid camera target");
        let up = Scene::load_vec3f(e.find("up").expect("Camera up vector must be set"))
            .expect("Invalid camera up");
        let fovy = e.find("fovy").expect("Camera FOV Y must be set").as_f64()
            .expect("FOV Y must be a float") as f32;
        Camera::new(pos, target, up, fovy, width, height)
    }
    fn load_geometry(e: &Value) -> Vec<Box<ISPCGeometry>> {
        let geom = e.as_array().expect("Geometry must be an array of objects");
        geom.iter().map(|x| {
            if !x.is_object() {
                panic!("Geometry must be specified as JSON objects, see the examples");
            }
            let ty = x.find("type").expect("A geometry type must be set").as_string()
                .expect("Geometry type must be a string");
            let lambertian = Scene::load_vec3f(x.find("lambertian").expect("A lambertian color must be set"))
                            .unwrap();
            let mat = Lambertian::new(lambertian);
            if ty == "sphere" {
                let center = Scene::load_vec3f(x.find("center").expect("A sphere center must be set")).unwrap();
                let radius = x.find("radius").expect("A sphere radius must be set").as_f64().unwrap() as f32;
                Box::new(Sphere::new(center, radius, mat)) as Box<ISPCGeometry>
            } else if ty == "plane" {
                let center = Scene::load_vec3f(x.find("center").expect("A plane center must be set")).unwrap();
                let normal = Scene::load_vec3f(x.find("normal").expect("A plane normal must be set")).unwrap();
                Box::new(Plane::new(center, normal, mat)) as Box<ISPCGeometry>
            } else {
                panic!("Unrecognized geometry type {}", ty);
            }
        }).collect()
    }
    fn load_light(e: &Value) -> PointLight {
        let pos = Scene::load_vec3f(e.find("pos").expect("A light position must be set")).unwrap();
        let intensity = Scene::load_vec3f(e.find("intensity").expect("A light intensity must be set")).unwrap();
        PointLight::new(pos, intensity)
    }
    fn load_vec3f(e: &Value) -> Option<Vec3f> {
        e.as_array().map(|x| {
            assert_eq!(x.len(), 3);
            Vec3f::new(x[0].as_f64().unwrap() as f32,
                       x[1].as_f64().unwrap() as f32,
                       x[2].as_f64().unwrap() as f32)
        })
    }
}

