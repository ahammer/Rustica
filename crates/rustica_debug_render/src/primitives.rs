//! Primitive 2D shapes for debug rendering.
//!
//! This module provides basic primitive shapes that can be used for debug rendering.

use cgmath::{Vector2 as Vec2, Vector4 as Vec4};
use bytemuck::{Pod, Zeroable};

/// A vertex used in the debug renderer.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    /// Position of the vertex.
    pub position: [f32; 2],
    /// Color of the vertex.
    pub color: [f32; 4],
}

/// A point in 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// Position of the point.
    pub position: Vec2<f32>,
    /// Size of the point.
    pub size: f32,
    /// Color of the point.
    pub color: Vec4<f32>,
}

impl Point {
    /// Create a new point.
    pub fn new(position: Vec2<f32>, size: f32, color: Vec4<f32>) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    /// Convert the point to vertices (makes a small quad).
    pub fn to_vertices(&self) -> [Vertex; 6] {
        let half_size = self.size / 2.0;
        let color_array = [self.color.x, self.color.y, self.color.z, self.color.w];
        
        // Create a quad centered at the point position
        let vertices = [
            // First triangle
            Vertex {
                position: [self.position.x - half_size, self.position.y - half_size],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + half_size, self.position.y - half_size],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + half_size, self.position.y + half_size],
                color: color_array,
            },
            
            // Second triangle
            Vertex {
                position: [self.position.x - half_size, self.position.y - half_size],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + half_size, self.position.y + half_size],
                color: color_array,
            },
            Vertex {
                position: [self.position.x - half_size, self.position.y + half_size],
                color: color_array,
            },
        ];
        
        vertices
    }
}

/// A line between two points.
#[derive(Debug, Clone, Copy)]
pub struct Line {
    /// Start point of the line.
    pub start: Vec2<f32>,
    /// End point of the line.
    pub end: Vec2<f32>,
    /// Width of the line.
    pub width: f32,
    /// Color of the line.
    pub color: Vec4<f32>,
}

impl Line {
    /// Create a new line.
    pub fn new(start: Vec2<f32>, end: Vec2<f32>, width: f32, color: Vec4<f32>) -> Self {
        Self {
            start,
            end,
            width,
            color,
        }
    }

    /// Convert the line to vertices (creates a rectangle along the line).
    pub fn to_vertices(&self) -> Vec<Vertex> {
        let direction = Vec2::new(self.end.x - self.start.x, self.end.y - self.start.y);
        let length = (direction.x * direction.x + direction.y * direction.y).sqrt();
        
        // If the line has no length, return an empty array
        if length < std::f32::EPSILON {
            return Vec::new();
        }
        
        // Normalize the direction
        let direction = Vec2::new(direction.x / length, direction.y / length);
        
        // Calculate the perpendicular direction (rotate 90 degrees)
        let perpendicular = Vec2::new(-direction.y, direction.x);
        
        // Calculate the half width
        let half_width = self.width / 2.0;
        
        // Calculate the four corners of the line rectangle
        let p1 = Vec2::new(
            self.start.x + perpendicular.x * half_width,
            self.start.y + perpendicular.y * half_width,
        );
        let p2 = Vec2::new(
            self.start.x - perpendicular.x * half_width,
            self.start.y - perpendicular.y * half_width,
        );
        let p3 = Vec2::new(
            self.end.x - perpendicular.x * half_width,
            self.end.y - perpendicular.y * half_width,
        );
        let p4 = Vec2::new(
            self.end.x + perpendicular.x * half_width,
            self.end.y + perpendicular.y * half_width,
        );
        
        let color_array = [self.color.x, self.color.y, self.color.z, self.color.w];
        
        // Create two triangles to form a quad
        vec![
            // First triangle
            Vertex {
                position: [p1.x, p1.y],
                color: color_array,
            },
            Vertex {
                position: [p2.x, p2.y],
                color: color_array,
            },
            Vertex {
                position: [p3.x, p3.y],
                color: color_array,
            },
            
            // Second triangle
            Vertex {
                position: [p1.x, p1.y],
                color: color_array,
            },
            Vertex {
                position: [p3.x, p3.y],
                color: color_array,
            },
            Vertex {
                position: [p4.x, p4.y],
                color: color_array,
            },
        ]
    }
}

/// A rectangle in 2D space.
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    /// Position of the top-left corner.
    pub position: Vec2<f32>,
    /// Size of the rectangle.
    pub size: Vec2<f32>,
    /// Color of the rectangle.
    pub color: Vec4<f32>,
    /// Whether the rectangle is filled or just an outline.
    pub filled: bool,
    /// Width of the outline if not filled.
    pub outline_width: f32,
}

impl Rectangle {
    /// Create a new filled rectangle.
    pub fn new(position: Vec2<f32>, size: Vec2<f32>, color: Vec4<f32>) -> Self {
        Self {
            position,
            size,
            color,
            filled: true,
            outline_width: 1.0,
        }
    }

    /// Create a new outline rectangle.
    pub fn new_outline(position: Vec2<f32>, size: Vec2<f32>, color: Vec4<f32>, width: f32) -> Self {
        Self {
            position,
            size,
            color,
            filled: false,
            outline_width: width,
        }
    }

    /// Convert the rectangle to vertices.
    pub fn to_vertices(&self) -> Vec<Vertex> {
        if self.filled {
            self.to_filled_vertices()
        } else {
            self.to_outline_vertices()
        }
    }

    /// Convert to vertices for a filled rectangle.
    fn to_filled_vertices(&self) -> Vec<Vertex> {
        let color_array = [self.color.x, self.color.y, self.color.z, self.color.w];
        
        vec![
            // First triangle
            Vertex {
                position: [self.position.x, self.position.y],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + self.size.x, self.position.y],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + self.size.x, self.position.y + self.size.y],
                color: color_array,
            },
            
            // Second triangle
            Vertex {
                position: [self.position.x, self.position.y],
                color: color_array,
            },
            Vertex {
                position: [self.position.x + self.size.x, self.position.y + self.size.y],
                color: color_array,
            },
            Vertex {
                position: [self.position.x, self.position.y + self.size.y],
                color: color_array,
            },
        ]
    }

    /// Convert to vertices for an outline rectangle.
    fn to_outline_vertices(&self) -> Vec<Vertex> {
        // Create four lines for the rectangle edges
        let top = Line::new(
            self.position,
            Vec2::new(self.position.x + self.size.x, self.position.y),
            self.outline_width,
            self.color,
        );
        
        let right = Line::new(
            Vec2::new(self.position.x + self.size.x, self.position.y),
            Vec2::new(self.position.x + self.size.x, self.position.y + self.size.y),
            self.outline_width,
            self.color,
        );
        
        let bottom = Line::new(
            Vec2::new(self.position.x, self.position.y + self.size.y),
            Vec2::new(self.position.x + self.size.x, self.position.y + self.size.y),
            self.outline_width,
            self.color,
        );
        
        let left = Line::new(
            self.position,
            Vec2::new(self.position.x, self.position.y + self.size.y),
            self.outline_width,
            self.color,
        );
        
        // Combine all the vertices from the four lines
        let mut vertices = Vec::new();
        vertices.extend(top.to_vertices());
        vertices.extend(right.to_vertices());
        vertices.extend(bottom.to_vertices());
        vertices.extend(left.to_vertices());
        
        vertices
    }
}
