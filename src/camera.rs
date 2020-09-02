use gdmath;
use gdmath::{
    Degrees,
    Vector3,
    Vector4,
    Matrix4, 
    Quaternion,
    ScalarFloat,
};
use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CameraSpecification<S> {
    near: S,
    far: S,
    fovy: Degrees<S>,
    aspect: S,
}

impl<S> CameraSpecification<S> where S: ScalarFloat {
    pub fn new(near: S, far: S, fovy: Degrees<S>, aspect: S) -> CameraSpecification<S> {
        CameraSpecification {
            near: near,
            far: far,
            fovy: fovy,
            aspect: aspect,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraKinematics<S> {
    speed: S,
    yaw_speed: S,
    position: Vector3<S>,
    forward: Vector4<S>,
    right: Vector4<S>,
    up: Vector4<S>,
    axis: Quaternion<S>,
}

impl<S> CameraKinematics<S> where S: ScalarFloat {
    pub fn new(
        speed: S, 
        yaw_speed: S, 
        position: Vector3<S>, 
        forward: Vector4<S>, 
        right: Vector4<S>, 
        up: Vector4<S>, 
        axis: Quaternion<S>) -> CameraKinematics<S> {

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CameraAttitude<S> {
    pub roll: S,
    pub pitch: S,
    pub yaw: S,
}

impl<S> CameraAttitude<S> where S: ScalarFloat {
    #[inline]
    pub fn new(roll: S, pitch: S, yaw: S) -> CameraAttitude<S> {
        CameraAttitude {
            roll: roll,
            pitch: pitch,
            yaw: yaw,
        }
    }
}


#[derive(Clone, Debug)]
pub struct Camera<S> {
    // Camera specification parameters.
    pub near: S,
    pub far: S,
    pub fovy: Degrees<S>,
    pub aspect: S,

    // Camera kinematics.
    pub speed: S,
    pub yaw_speed: S,
    position: Vector3<S>,
    forward: Vector4<S>,
    right: Vector4<S>,
    up: Vector4<S>,
    pub axis: Quaternion<S>,

    // Camera matrices.
    pub proj_mat: Matrix4<S>,
    pub trans_mat: Matrix4<S>,
    pub rot_mat: Matrix4<S>,
    pub view_mat: Matrix4<S>,
}

impl<S> fmt::Display for Camera<S> where S: ScalarFloat + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("Camera model:\n");
        output.push_str(&format!("near: {}\n", self.near));
        output.push_str(&format!("far: {}\n", self.far));
        output.push_str(&format!("aspect: {}\n", self.aspect));
        output.push_str(&format!("speed: {}\n", self.speed));
        output.push_str(&format!("yaw_speed: {}\n", self.yaw_speed));
        output.push_str(&format!("position: {}\n", self.position));
        output.push_str(&format!("forward: {}\n", self.forward));
        output.push_str(&format!("right: {}\n", self.right));
        output.push_str(&format!("up: {}\n", self.up));
        output.push_str(&format!("axis: {}\n", self.axis));
        output.push_str(&format!("proj_mat: {}\n", self.proj_mat));
        output.push_str(&format!("trans_mat: {}\n", self.trans_mat));
        output.push_str(&format!("rot_mat: {}\n", self.rot_mat));
        output.push_str(&format!("view_mat: {}\n", self.view_mat));
        writeln!(f, "{}", output)
    }
}

impl<S> Camera<S> where S: gdmath::ScalarFloat {
    /// Construct a new camera.
    pub fn new(spec: CameraSpecification<S>, kinematics: CameraKinematics<S>) -> Camera<S> {
        let proj_mat = gdmath::perspective((spec.fovy, spec.aspect, spec.near, spec.far));
        let trans_mat = Matrix4::from_translation(-kinematics.position);
        let rot_mat = Matrix4::from(kinematics.axis);
        let view_mat = rot_mat * trans_mat;

        Camera {
            near: spec.near,
            far: spec.far,
            fovy: spec.fovy,
            aspect: spec.aspect,

            speed: kinematics.speed,
            yaw_speed: kinematics.yaw_speed,
            position: kinematics.position,
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

    /// Get the camera's eye position in world space.
    #[inline]
    pub fn position(&self) -> Vector3<S> { 
        self.position
    }

    /// Get the camera's up direction in world space.
    #[inline]
    pub fn up_axis(&self) -> Vector3<S> {
        Vector3::new(self.up.x, self.up.y, self.up.z)
    }

    /// Get the camera's right axis in world space.
    #[inline]
    pub fn right_axis(&self) -> Vector3<S> {
        Vector3::new(self.right.x, self.right.y, self.right.z)
    }

    /// Get the camera's forward axis in world space.
    #[inline]
    pub fn forward_axis(&self) -> Vector3<S> {
        Vector3::new(self.forward.x, self.forward.y, self.forward.z)
    }

    /// Get the camera's up direction in camera space.
    #[inline]
    pub fn up_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();
        Vector3::new(zero, one, zero)
    }
    
    /// Get the camera's right axis in camera space.
    #[inline]
    pub fn right_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();
        Vector3::new(one, zero ,zero)
    }
    
    /// Get the camera's forward axis in camera space.
    #[inline]
    pub fn forward_axis_eye(&self) -> Vector3<S> {
        let zero = S::zero();
        let one = S::one();
        Vector3::new(zero, zero, -one)
    }

    /// Update the camera viewport dimensions in case the viewport dimensions have changed.
    #[inline]
    pub fn update_viewport(&mut self, width: u32, height: u32) {
        let width_float = gdmath::num_traits::cast::<u32, S>(width).unwrap();
        let height_float = gdmath::num_traits::cast::<u32, S>(height).unwrap();
        self.aspect = width_float / height_float;
        self.proj_mat = gdmath::perspective((self.fovy, self.aspect, self.near, self.far));
    }

    /// Update the camera position and attitude in world space.
    #[inline]
    pub fn update(&mut self, delta_position: Vector3<S>, delta_attitude: CameraAttitude<S>) {
        // Update the axis of rotation of the camera.
        let q_yaw = Quaternion::from_axis_deg(
            Degrees(delta_attitude.yaw), gdmath::vec3(self.up)
        );
        self.axis = q_yaw * self.axis;
        let q_pitch = Quaternion::from_axis_deg(
            Degrees(delta_attitude.pitch), gdmath::vec3(self.right)
        );
        self.axis = q_pitch * self.axis;
        let q_roll = Quaternion::from_axis_deg(
            Degrees(delta_attitude.roll), gdmath::vec3(self.forward)
        );
        self.axis = q_roll * self.axis;

        // Update the camera axes so we can rotate the camera about the new rotation axes.
        let zero = S::zero();
        let rot_mat_inv = Matrix4::from(self.axis);
        self.forward = rot_mat_inv * gdmath::vec4((self.forward_axis_eye(), zero));
        self.right   = rot_mat_inv * gdmath::vec4((self.right_axis_eye(), zero));
        self.up      = rot_mat_inv * gdmath::vec4((self.up_axis_eye(), zero));

        // Update the camera position.
        self.position += gdmath::vec3(self.forward) * -delta_position.z;
        self.position += gdmath::vec3(self.up)      *  delta_position.y;
        self.position += gdmath::vec3(self.right)   *  delta_position.x;

        // Update the camera matrices.
        let trans_mat_inv = Matrix4::from_translation(self.position);
        self.rot_mat = rot_mat_inv.inverse().unwrap();
        self.trans_mat = trans_mat_inv.inverse().unwrap();
        self.view_mat = self.rot_mat * self.trans_mat;
    }
}
