// Camera primitive - Represents a camera in 3D space

use cgmath::{perspective, Deg, EuclideanSpace, InnerSpace, Matrix4, Point3, Vector3};

/// A camera in 3D space
pub struct Camera {
    /// Position of the camera
    pub position: Point3<f32>,
    /// Target point the camera is looking at
    pub target: Point3<f32>,
    /// Up vector of the camera
    pub up: Vector3<f32>,
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
        position: Point3<f32>,
        target: Point3<f32>,
        up: Vector3<f32>,
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
            position: Point3::new(0.0, 0.0, 5.0),
            target: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            fov: 45.0,
            aspect,
            near: 0.1,
            far: 100.0,
        }
    }

    /// Get the view matrix for this camera
    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(self.position, self.target, self.up)
    }

    /// Get the projection matrix for this camera
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        perspective(Deg(self.fov), self.aspect, self.near, self.far)
    }

    /// Move the camera to a new position
    pub fn set_position(&mut self, position: Point3<f32>) {
        self.position = position;
    }

    /// Look at a specific target
    pub fn look_at(&mut self, target: Point3<f32>) {
        self.target = target;
    }

    /// Position the camera and look at a specific target
    pub fn look_at_from(&mut self, position: Point3<f32>, target: Point3<f32>) {
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
    }

    /// Get the view-projection matrix (combined view and projection matrices)
    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        self.projection_matrix() * self.view_matrix()
    }
    
    /// Get the camera's position as a Vector3
    pub fn position_vector(&self) -> Vector3<f32> {
        self.position.to_vec()
    }
    
    /// Get the camera's forward direction
    pub fn forward_vector(&self) -> Vector3<f32> {
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
    pub view: Matrix4<f32>,
    /// Projection matrix
    pub projection: Matrix4<f32>,
    /// Combined view-projection matrix
    pub view_projection: Matrix4<f32>,
    /// Camera position in world space
    pub camera_position: Vector3<f32>,
    /// Direction the camera is facing
    pub camera_direction: Vector3<f32>,
}
