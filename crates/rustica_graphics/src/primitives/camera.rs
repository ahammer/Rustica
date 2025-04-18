// Camera primitive - Represents a camera in 3D space

use glam::{Mat4, Vec3};

/// A camera in 3D space
pub struct Camera {
    /// Position of the camera
    pub position: Vec3,
    /// Target point the camera is looking at
    pub target: Vec3,
    /// Up vector of the camera
    pub up: Vec3,
    /// Field of view in degrees
    pub fov: f32,
    /// Aspect ratio (width / height)
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Camera {
    /// Create a new camera
    pub fn new(
        position: Vec3,
        target: Vec3,
        up: Vec3,
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            position,
            target,
            up,
            fov,
            aspect,
            near,
            far,
        }
    }

    /// Create a perspective camera
    pub fn perspective(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 5.0),
            target: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: 45.0,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }    /// Get the view matrix for this camera
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get the projection matrix for this camera
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov.to_radians(), self.aspect, self.near, self.far)
    }    /// Move the camera to a new position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Look at a specific target
    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;
    }

    /// Position the camera and look at a specific target
    pub fn look_at_from(&mut self, position: Vec3, target: Vec3) {
        self.position = position;
        self.target = target;
    }

    /// Move the camera forward
    pub fn move_forward(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        self.position += direction * distance;
        self.target += direction * distance;
    }

    /// Move the camera backward
    pub fn move_backward(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        self.position -= direction * distance;
        self.target -= direction * distance;
    }

    /// Move the camera right
    pub fn move_right(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        let right = direction.cross(self.up).normalize();
        self.position += right * distance;
        self.target += right * distance;
    }

    /// Move the camera left
    pub fn move_left(&mut self, distance: f32) {
        let direction = (self.target - self.position).normalize();
        let right = direction.cross(self.up).normalize();
        self.position -= right * distance;
        self.target -= right * distance;
    }

    /// Move the camera up
    pub fn move_up(&mut self, distance: f32) {
        self.position += self.up * distance;
        self.target += self.up * distance;
    }

    /// Move the camera down
    pub fn move_down(&mut self, distance: f32) {
        self.position -= self.up * distance;
        self.target -= self.up * distance;
    }    /// Get the view-projection matrix (combined view and projection matrices)
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
    
    /// Get the camera's position as a Vec3
    pub fn position_vector(&self) -> Vec3 {
        self.position
    }
    
    /// Get the camera's forward direction
    pub fn forward_vector(&self) -> Vec3 {
        (self.target - self.position).normalize()
    }
    
    /// Get camera matrices packaged for rendering
    pub fn get_render_matrices(&self) -> CameraMatrices {
        CameraMatrices {
            view: self.view_matrix(),
            projection: self.projection_matrix(),
            view_projection: self.view_projection_matrix(),
            camera_position: self.position_vector(),
            camera_direction: self.forward_vector(),
        }
    }
}

/// Camera matrices packaged for rendering
#[derive(Debug, Clone, Copy)]
pub struct CameraMatrices {
    /// View matrix
    pub view: Mat4,
    /// Projection matrix
    pub projection: Mat4,
    /// Combined view-projection matrix
    pub view_projection: Mat4,
    /// Camera position in world space
    pub camera_position: Vec3,
    /// Direction the camera is facing
    pub camera_direction: Vec3,
}
