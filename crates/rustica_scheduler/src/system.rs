//! System definition and execution.

use std::fmt::{Debug, Formatter, Result as FmtResult};
use rustica_ecs::World;

/// A system function that can be executed by the scheduler.
///
/// Systems are functions that take a mutable reference to the world
/// and perform some operations on it. They are the core unit of logic
/// in the engine.
pub trait System: Send + Sync + 'static {
    /// Execute the system on the given world.
    fn run(&mut self, world: &mut World);
    
    /// Get the name of the system.
    fn name(&self) -> &str;
    
    /// Get the dependencies of this system.
    fn dependencies(&self) -> &[String];
    
    /// Adds a dependency to this system.
    fn add_dependency(&mut self, dependency: &str);
}

/// Implementation of the System trait for functions.
pub struct SystemFn<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    /// The function to execute.
    pub(crate) func: F,
    
    /// The name of the system.
    pub(crate) name: String,
    
    /// The names of systems this system depends on.
    pub(crate) dependencies: Vec<String>,
}

impl<F> SystemFn<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    /// Create a new system from a function and name.
    pub fn new(func: F, name: &str) -> Self {
        Self {
            func,
            name: name.to_string(),
            dependencies: Vec::new(),
        }
    }
    
    /// Add a dependency to this system.
    pub fn with_dependency(mut self, dependency: &str) -> Self {
        self.dependencies.push(dependency.to_string());
        self
    }
    
    /// Add multiple dependencies to this system.
    pub fn with_dependencies(mut self, dependencies: &[&str]) -> Self {
        for dep in dependencies {
            self.dependencies.push(dep.to_string());
        }
        self
    }
}

impl<F> System for SystemFn<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn run(&mut self, world: &mut World) {
        (self.func)(world);
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
    
    fn add_dependency(&mut self, dependency: &str) {
        self.dependencies.push(dependency.to_string());
    }
}

impl<F> Debug for SystemFn<F>
where
    F: FnMut(&mut World) + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("SystemFn")
            .field("name", &self.name)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}
