use gdmath::{
    Vector3,
    ScalarFloat,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight<S> {
    pub ambient: Vector3<S>,
    pub diffuse: Vector3<S>,
    pub specular: Vector3<S>,
}

impl<S> PointLight<S> where S: ScalarFloat {
    pub fn new(ambient: Vector3<S>, diffuse: Vector3<S>, specular: Vector3<S>) -> PointLight<S> {
        PointLight {
            ambient: ambient,
            diffuse: diffuse,
            specular: specular,
        }
    }
}
