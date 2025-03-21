//! Star rendering functionality.
//!
//! This module provides specialized rendering functionality for stars.

use cgmath::{Vector2 as Vec2, Vector3 as Vec3, Vector4 as Vec4};
use std::collections::VecDeque;

use crate::error::Result;
use crate::debug_renderer::DebugRenderer;
use crate::primitives::{Point, Vertex};

/// Maximum number of stars that can be rendered in a batch.
const MAX_STARS_PER_BATCH: usize = 1000;

/// A star that can be rendered.
#[derive(Debug, Clone, Copy)]
pub struct Star {
    /// Position of the star (x, y, z).
    pub position: Vec3<f32>,
    /// Color of the star (r, g, b, a).
    pub color: Vec4<f32>,
    /// Size of the star.
    pub size: f32,
    /// Brightness of the star (affects rendering).
    pub brightness: f32,
}

impl Star {
    /// Create a new star.
    pub fn new(position: Vec3<f32>, color: Vec4<f32>, size: f32, brightness: f32) -> Self {
        Self {
            position,
            color,
            size,
            brightness,
        }
    }

    /// Convert the star to a point for rendering.
    pub fn to_point(&self) -> Point {
        // Project 3D position to 2D (simple Z-based scaling for now)
        let z_factor = if self.position.z > 0.0 {
            1.0 / (1.0 + self.position.z * 0.01)
        } else {
            1.0
        };

        // Apply brightness to color
        let brightness_adjusted_color = Vec4::new(
            self.color.x * self.brightness,
            self.color.y * self.brightness,
            self.color.z * self.brightness,
            self.color.w,
        );

        Point::new(
            Vec2::new(self.position.x, self.position.y),
            self.size * z_factor,
            brightness_adjusted_color,
        )
    }
}

impl Default for Star {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            color: Vec4::new(1.0, 1.0, 1.0, 1.0), // White
            size: 2.0,
            brightness: 1.0,
        }
    }
}

/// Manages rendering of multiple stars efficiently.
pub struct StarRenderer {
    /// The stars to render.
    stars: Vec<Star>,
    /// The batched vertices for rendering.
    vertex_batches: Vec<Vec<Vertex>>,
    /// Whether the star data has changed and needs rebatching.
    dirty: bool,
}

impl StarRenderer {
    /// Create a new star renderer.
    pub fn new() -> Self {
        Self {
            stars: Vec::new(),
            vertex_batches: Vec::new(),
            dirty: false,
        }
    }

    /// Add a star to the renderer.
    pub fn add_star(&mut self, star: Star) {
        self.stars.push(star);
        self.dirty = true;
    }

    /// Add multiple stars to the renderer.
    pub fn add_stars(&mut self, stars: &[Star]) {
        self.stars.extend_from_slice(stars);
        self.dirty = true;
    }

    /// Clear all stars from the renderer.
    pub fn clear(&mut self) {
        self.stars.clear();
        self.vertex_batches.clear();
        self.dirty = false;
    }

    /// Set the stars in the renderer.
    pub fn set_stars(&mut self, stars: Vec<Star>) {
        self.stars = stars;
        self.dirty = true;
    }

    /// Get the number of stars in the renderer.
    pub fn star_count(&self) -> usize {
        self.stars.len()
    }

    /// Prepare the stars for rendering by batching vertices.
    fn prepare(&mut self) {
        if !self.dirty {
            return;
        }

        self.vertex_batches.clear();
        
        // Sort stars by z-position for proper depth ordering
        let mut sorted_stars = self.stars.clone();
        sorted_stars.sort_by(|a, b| b.position.z.partial_cmp(&a.position.z).unwrap_or(std::cmp::Ordering::Equal));

        // Group stars into batches
        let mut current_batch = Vec::with_capacity(MAX_STARS_PER_BATCH * 6); // 6 vertices per star (2 triangles)
        
        for star in &sorted_stars {
            let point = star.to_point();
            let vertices = point.to_vertices();
            
            // Check if adding these vertices would exceed the batch size
            if current_batch.len() + vertices.len() > MAX_STARS_PER_BATCH * 6 {
                // Start a new batch
                self.vertex_batches.push(current_batch);
                current_batch = Vec::with_capacity(MAX_STARS_PER_BATCH * 6);
            }
            
            // Add vertices to the current batch
            current_batch.extend_from_slice(&vertices);
        }
        
        // Add the last batch if it's not empty
        if !current_batch.is_empty() {
            self.vertex_batches.push(current_batch);
        }
        
        self.dirty = false;
    }

    /// Render the stars using the debug renderer.
    pub fn render(&mut self, renderer: &mut DebugRenderer, frame: &wgpu::SurfaceTexture) -> Result<()> {
        // Prepare batches if needed
        self.prepare();
        
        // Render each batch
        for batch in &self.vertex_batches {
            renderer.draw_vertices(batch, frame)?;
        }
        
        Ok(())
    }

    /// Update an existing star's properties.
    pub fn update_star(&mut self, index: usize, position: Option<Vec3<f32>>, color: Option<Vec4<f32>>, 
                      size: Option<f32>, brightness: Option<f32>) -> Result<()> {
        if index >= self.stars.len() {
            return Err(crate::error::Error::IndexOutOfBounds(format!("Star index {} out of bounds (max {})", index, self.stars.len() - 1)));
        }
        
        let star = &mut self.stars[index];
        
        if let Some(position) = position {
            star.position = position;
            self.dirty = true;
        }
        
        if let Some(color) = color {
            star.color = color;
            self.dirty = true;
        }
        
        if let Some(size) = size {
            star.size = size;
            self.dirty = true;
        }
        
        if let Some(brightness) = brightness {
            star.brightness = brightness;
            self.dirty = true;
        }
        
        Ok(())
    }

    /// Create a star field with randomly positioned stars.
    pub fn create_star_field(count: usize, width: f32, height: f32, depth: f32) -> Vec<Star> {
        use rand::{Rng, SeedableRng};
        let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Fixed seed for reproducibility
        
        let mut stars = Vec::with_capacity(count);
        
        for _ in 0..count {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let z = rng.gen_range(0.0..depth);
            
            // Vary star properties
            let brightness = rng.gen_range(0.5..1.0);
            let size = rng.gen_range(1.0..5.0);
            
            // Create color variations (white with slight tints)
            let r = rng.gen_range(0.8..1.0);
            let g = rng.gen_range(0.8..1.0);
            let b = rng.gen_range(0.8..1.0);
            
            stars.push(Star::new(
                Vec3::new(x, y, z),
                Vec4::new(r, g, b, 1.0),
                size,
                brightness,
            ));
        }
        
        stars
    }
}

/// Add an error variant for index out of bounds errors
impl crate::error::Error {
    /// Create an index out of bounds error
    pub fn index_out_of_bounds(message: impl Into<String>) -> Self {
        Self::IndexOutOfBounds(message.into())
    }
}

/// Add an index out of bounds variant to the error enum
#[doc(hidden)]
pub enum ErrorKind {
    /// Index out of bounds error
    IndexOutOfBounds,
}
