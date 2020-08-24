use gdmath;
use gdmath::{Vector3, Vector4, Matrix4, Quaternion};

use std::fmt;


#[derive(Copy, Clone, Debug)]
pub struct CameraSpecification<S> {
    near: S,
    far: S,
    fov: S,
    aspect: S,
}

impl<S> CameraSpecification<S> where S: gdmath::ScalarFloat {
    pub fn new(near: S, far: S, fov: S, aspect: S) -> CameraSpecification<S> {
        CameraSpecification {
            near: near,
            far: far,
            fov: fov,
            aspect: aspect,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CameraKinematics<S> {
    speed: S,
    yaw_speed: S,
    position: Vector3<S>,
    forward: Vector4<S>,
    right: Vector4<S>,
    up: Vector4<S>,
    axis: Quaternion<S>,
}

impl<S> CameraKinematics<S> where S: gdmath::ScalarFloat {
    pub fn new(
        speed: S, 
        yaw_speed: S, 
        position: Vector3<S>, 
        forward: Vector4<S>, 
        right: Vector4<S>, 
        up: Vector4<S>, axis: Quaternion<S>) -> CameraKinematics<S> {

        CameraKinematics {
            speed: speed,
            yaw_speed: yaw_speed,
            position: position,
            forward: forward,
            right: right,
            up: up,
            axis: axis,
        }

    }
}


#[derive(Clone, Debug)]
pub struct Camera<S> {
    // Camera specification parameters.
    pub near: S,
    pub far: S,
    pub fov: S,
    pub aspect: S,

    // Camera kinematics.
    pub speed: S,
    pub yaw_speed: S,
    pub position: Vector3<S>,
    pub forward: Vector4<S>,
    pub right: Vector4<S>,
    pub up: Vector4<S>,
    pub axis: Quaternion<S>,

    // Camera matrices.
    pub proj_mat: Matrix4<S>,
    pub trans_mat: Matrix4<S>,
    pub rot_mat: Matrix4<S>,
    pub view_mat: Matrix4<S>,
}

impl<S> Camera<S> where S: gdmath::ScalarFloat {
    pub fn new(spec: CameraSpecification<S>, kinematics: CameraKinematics<S>) -> Camera<S> {
        let proj_mat = gdmath::perspective((spec.fov, spec.aspect, spec.near, spec.far));
        let trans_mat = Matrix4::from_translation(-kinematics.pos);
        let rot_mat = Matrix4::from(kinematics.axis);
        let view_mat = rot_mat * trans_mat;

        Camera {
            near: spec.near,
            far: spec.far,
            fov: spec.fov,
            aspect: spec.aspect,

            speed: kinematics.speed,
            yaw_speed: kinematics.yaw_speed,
            pos: kinematcs.position,
            forward: kinematics.forward,
            right: kinematics.right,
            up: kinematics.up,
            axis: kinematics.axis,

            proj_mat: proj_mat,
            trans_mat: trans_mat,
            rot_mat: rot_mat,
            view_mat: view_mat,
        }
    }
}

impl<S> fmt::Display for Camera<S> where S: gdmath::ScalarFloat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("Camera model:\n");
        output.append(&format!("near: {}\n", self.near));
        output.append(&format!("far: {}\n", self.far));
        output.append(&format!("aspect: {}\n", self.aspect));
        output.append(&format!("speed: {}\n", self.speed));
        output.append(&format!("yaw_speed: {}\n", self.yaw_speed));
        output.append(&format!("position: {}\n", self.position));
        output.append(&format!("forward: {}\n", self.forward));
        output.append(&format!("right: {}\n", self.right));
        output.append(&format!("up: {}\n". self.up));
        output.append(&format!("axis: {}\n", self.axis));
        output.append(&format!("proj_mat: {}\n", self.proj_mat));
        output.append(&format!("trans_mat: {}\n", self.trans_mat));
        output.append(&format!("rot_mat: {}\n", self.rot_mat));
        output.append(&format!("view_mat: {}\n", self.view_mat));
        writeln!(f, "{}", output)
    }
}

