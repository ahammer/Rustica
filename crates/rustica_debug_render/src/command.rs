//! Render commands for the debug renderer.
//!
//! This module provides a command-based interface for the debug renderer,
//! allowing systems to generate render commands that will be processed by
//! the renderer.

use cgmath::{Vector2 as Vec2, Vector3 as Vec3, Vector4 as Vec4};

use crate::star_renderer::Star;

/// A list of rendering commands to be processed by the renderer.
#[derive(Default)]
pub struct RenderCommandList {
    /// Commands to draw points
    pub point_commands: Vec<DrawPointCommand>,
    /// Commands to draw lines
    pub line_commands: Vec<DrawLineCommand>,
    /// Commands to draw rectangles
    pub rect_commands: Vec<DrawRectCommand>,
    /// Stars to render
    pub stars: Vec<Star>,
}

/// Command to draw a point in 3D space.
#[derive(Debug, Clone)]
pub struct DrawPointCommand {
    /// Position in 3D space
    pub position: Vec3<f32>,
    /// Color (RGBA)
    pub color: Vec4<f32>,
    /// Size of the point
    pub size: f32,
    /// Brightness multiplier
    pub brightness: f32,
}

/// Command to draw a line.
#[derive(Debug, Clone)]
pub struct DrawLineCommand {
    /// Start position
    pub start: Vec2<f32>,
    /// End position
    pub end: Vec2<f32>,
    /// Color (RGBA)
    pub color: Vec4<f32>,
    /// Line thickness
    pub thickness: f32,
}

/// Command to draw a rectangle.
#[derive(Debug, Clone)]
pub struct DrawRectCommand {
    /// Position of the top-left corner
    pub position: Vec2<f32>,
    /// Size of the rectangle
    pub size: Vec2<f32>,
    /// Color (RGBA)
    pub color: Vec4<f32>,
    /// Whether the rectangle is filled
    pub filled: bool,
}

impl RenderCommandList {
    /// Create a new empty render command list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear all commands from the list.
    pub fn clear(&mut self) {
        self.point_commands.clear();
        self.line_commands.clear();
        self.rect_commands.clear();
        self.stars.clear();
    }
    
    /// Clear only the star commands, leaving other commands intact.
    pub fn clear_stars(&mut self) {
        self.stars.clear();
    }
    
    /// Add a star to the render list.
    pub fn add_star(&mut self, star: Star) {
        self.stars.push(star);
    }
    
    /// Get the number of stars in the command list.
    pub fn star_count(&self) -> usize {
        self.stars.len()
    }

    /// Add a command to draw a point.
    pub fn add_point(&mut self, position: Vec3<f32>, color: Vec4<f32>, size: f32, brightness: f32) {
        self.point_commands.push(DrawPointCommand {
            position,
            color,
            size,
            brightness,
        });
    }

    /// Add a command to draw a line.
    pub fn add_line(&mut self, start: Vec2<f32>, end: Vec2<f32>, color: Vec4<f32>, thickness: f32) {
        self.line_commands.push(DrawLineCommand {
            start,
            end,
            color,
            thickness,
        });
    }

    /// Add a command to draw a rectangle.
    pub fn add_rect(&mut self, position: Vec2<f32>, size: Vec2<f32>, color: Vec4<f32>, filled: bool) {
        self.rect_commands.push(DrawRectCommand {
            position,
            size,
            color,
            filled,
        });
    }
}
