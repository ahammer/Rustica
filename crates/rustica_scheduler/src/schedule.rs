//! Schedule for organizing and executing systems.

use std::collections::{HashMap, HashSet};
use crate::error::SchedulerError;
use crate::system::System;
use crate::stage::Stage;
use rustica_ecs::World;

/// A system with its execution stage and dependency information.
struct ScheduledSystem {
    /// The system to execute.
    system: Box<dyn System>,
    
    /// The stage this system belongs to.
    stage: Stage,
    
    /// Whether this system has been executed in the current run.
    executed: bool,
}

/// The schedule is responsible for organizing and executing systems.
///
/// Systems are organized by stage and dependencies, and the schedule ensures
/// they are executed in the correct order.
#[derive(Default)]
pub struct Schedule {
    /// Systems by name.
    systems: HashMap<String, ScheduledSystem>,
    
    /// Systems by stage.
    systems_by_stage: HashMap<Stage, Vec<String>>,
}

impl Schedule {
    /// Create a new empty schedule.
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            systems_by_stage: HashMap::new(),
        }
    }
    
    /// Add a system to the schedule.
    pub fn add_system<S>(&mut self, system: S, name: &str, stage: Stage) -> Result<&mut Self, SchedulerError>
    where
        S: System,
    {
        if self.systems.contains_key(name) {
            return Err(SchedulerError::SystemAlreadyExists(name.to_string()));
        }
        
        let boxed_system = Box::new(system);
        
        // Register the system in the stage
        self.systems_by_stage
            .entry(stage)
            .or_insert_with(Vec::new)
            .push(name.to_string());
        
        // Add the system to the system map
        self.systems.insert(
            name.to_string(),
            ScheduledSystem {
                system: boxed_system,
                stage,
                executed: false,
            },
        );
        
        Ok(self)
    }
    
    /// Add a dependency between two systems.
    pub fn add_dependency(&mut self, system: &str, dependency: &str) -> Result<&mut Self, SchedulerError> {
        // Check if both systems exist
        if !self.systems.contains_key(system) {
            return Err(SchedulerError::SystemNotFound(system.to_string()));
        }
        
        if !self.systems.contains_key(dependency) {
            return Err(SchedulerError::DependencyNotFound(dependency.to_string()));
        }
        
        // Check if this would create a cycle
        if self.would_create_cycle(system, dependency) {
            return Err(SchedulerError::DependencyCycle);
        }
        
        // Add the dependency
        if let Some(system) = self.systems.get_mut(system) {
            system.system.add_dependency(dependency);
        }
        
        Ok(self)
    }
    
    /// Check if adding a dependency would create a cycle.
    fn would_create_cycle(&self, system: &str, new_dependency: &str) -> bool {
        // If the dependency depends on the system (directly or indirectly), adding this
        // dependency would create a cycle.
        
        // Set of systems we've already checked
        let mut visited = HashSet::new();
        
        // Stack of systems to check
        let mut to_check = vec![new_dependency.to_string()];
        
        while let Some(current) = to_check.pop() {
            if current == system {
                // We found a cycle
                return true;
            }
            
            if visited.contains(&current) {
                // Already checked this system
                continue;
            }
            
            visited.insert(current.clone());
            
            // Add all dependencies of the current system to the check stack
            if let Some(scheduled_system) = self.systems.get(&current) {
                for dep in scheduled_system.system.dependencies() {
                    to_check.push(dep.clone());
                }
            }
        }
        
        false
    }
    
    /// Run all systems in the schedule on the given world.
    pub fn run(&mut self, world: &mut World) {
        // Reset the executed flag for all systems
        for system in self.systems.values_mut() {
            system.executed = false;
        }
        
        // Run all stages in order
        for stage in Stage::all() {
            if let Some(systems) = self.systems_by_stage.get(&stage) {
                for system_name in systems {
                    self.run_system(system_name, world);
                }
            }
        }
    }
    
    /// Run a specific system, ensuring its dependencies are run first.
    fn run_system(&mut self, system_name: &str, world: &mut World) {
        // Get the system
        let system = match self.systems.get(system_name) {
            Some(s) => s,
            None => return, // System not found
        };
        
        // If the system has already been executed, we're done
        if system.executed {
            return;
        }
        
        // Run all dependencies first
        let deps = system.system.dependencies().to_vec();
        for dep in deps {
            self.run_system(&dep, world);
        }
        
        // Now run the system
        if let Some(system) = self.systems.get_mut(system_name) {
            system.system.run(world);
            system.executed = true;
        }
    }
    
    /// Get a reference to a system by name.
    pub fn get_system(&self, name: &str) -> Option<&dyn System> {
        self.systems.get(name).map(|s| &*s.system)
    }
    
    /// Get a mutable reference to a system by name.
    pub fn get_system_mut(&mut self, name: &str) -> Option<&mut dyn System> {
        self.systems.get_mut(name).map(|s| &mut *s.system)
    }
    
    /// Remove a system from the schedule.
    pub fn remove_system(&mut self, name: &str) -> Result<(), SchedulerError> {
        let system = match self.systems.remove(name) {
            Some(s) => s,
            None => return Err(SchedulerError::SystemNotFound(name.to_string())),
        };
        
        // Remove from stage list
        if let Some(systems) = self.systems_by_stage.get_mut(&system.stage) {
            systems.retain(|s| s != name);
        }
        
        Ok(())
    }
    
    /// Clear all systems from the schedule.
    pub fn clear(&mut self) {
        self.systems.clear();
        self.systems_by_stage.clear();
    }
}
