use crate::schedule::Schedule;
use crate::stage::Stage;
use crate::error::SchedulerError;
use crate::system::SystemFn;
use rustica_ecs::World;

#[cfg(test)]
mod schedule_tests {
    use super::*;

    // Helper function
    fn get_or_insert_execution_order(world: &mut World) -> &mut Vec<String> {
        if world.get_resource::<Vec<String>>().is_none() {
            world.insert_resource(Vec::<String>::new());
        }
        world.get_resource_mut::<Vec<String>>().unwrap()
    }

    // Test systems
    fn system_a(world: &mut World) {
        let mut execution_order = get_or_insert_execution_order(world);
        execution_order.push("A".to_string());
    }
    
    fn system_b(world: &mut World) {
        let mut execution_order = get_or_insert_execution_order(world);
        execution_order.push("B".to_string());
    }
    
    fn system_c(world: &mut World) {
        let mut execution_order = get_or_insert_execution_order(world);
        execution_order.push("C".to_string());
    }

    #[test]
    fn test_schedule_creation() {
        let schedule = Schedule::new();
        // Empty schedule should not have any systems
        // Just verifying that it can be created without panic
    }
    
    #[test]
    fn test_add_system() {
        let mut schedule = Schedule::new();
        let result = schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update);
        
        assert!(result.is_ok(), "Adding a system should succeed");
    }
    
    #[test]
    fn test_add_system_duplicate() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let result = schedule.add_system(SystemFn::new(system_b, "system_a"), "system_a", Stage::Update);
        assert!(result.is_err(), "Adding a duplicate system should fail");
        
        match result {
            Err(SchedulerError::SystemAlreadyExists(name)) => {
                assert_eq!(name, "system_a", "Error should contain the duplicate system name");
            },
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_add_dependency() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_b, "system_b"), "system_b", Stage::Update).unwrap();
        
        let result = schedule.add_dependency("system_b", "system_a");
        assert!(result.is_ok(), "Adding a valid dependency should succeed");
    }
    
    #[test]
    fn test_add_dependency_unknown_system() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let result = schedule.add_dependency("unknown", "system_a");
        assert!(result.is_err(), "Adding a dependency for an unknown system should fail");
        
        match result {
            Err(SchedulerError::SystemNotFound(name)) => {
                assert_eq!(name, "unknown", "Error should contain the unknown system name");
            },
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_add_dependency_unknown_dependency() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let result = schedule.add_dependency("system_a", "unknown");
        assert!(result.is_err(), "Adding an unknown dependency should fail");
        
        match result {
            Err(SchedulerError::DependencyNotFound(name)) => {
                assert_eq!(name, "unknown", "Error should contain the unknown dependency name");
            },
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_dependency_cycle_detection() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_b, "system_b"), "system_b", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_c, "system_c"), "system_c", Stage::Update).unwrap();
        
        // Create a dependency chain: A -> B -> C
        schedule.add_dependency("system_b", "system_a").unwrap();
        schedule.add_dependency("system_c", "system_b").unwrap();
        
        // Trying to create a cycle: C -> A would make A -> B -> C -> A
        let result = schedule.add_dependency("system_a", "system_c");
        assert!(result.is_err(), "Adding a cyclic dependency should fail");
        
        match result {
            Err(SchedulerError::DependencyCycle) => {
                // Good, we caught the cycle
            },
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_run_empty_schedule() {
        let mut schedule = Schedule::new();
        let mut world = World::new();
        
        // This should not panic
        schedule.run(&mut world);
    }
    
    #[test]
    fn test_run_single_system() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let mut world = World::new();
        schedule.run(&mut world);
        
        let execution_order = world.get_resource::<Vec<String>>().unwrap();
        assert_eq!(execution_order.len(), 1, "System should have been executed once");
        assert_eq!(execution_order[0], "A", "System A should have been executed");
    }
    
    #[test]
    fn test_run_systems_in_stage_order() {
        let mut schedule = Schedule::new();
        
        // Add systems in reverse order of expected execution
        schedule.add_system(SystemFn::new(system_c, "system_c"), "system_c", Stage::LateUpdate).unwrap();
        schedule.add_system(SystemFn::new(system_b, "system_b"), "system_b", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::EarlyUpdate).unwrap();
        
        let mut world = World::new();
        schedule.run(&mut world);
        
        let execution_order = world.get_resource::<Vec<String>>().unwrap();
        assert_eq!(execution_order.len(), 3, "All systems should have been executed");
        assert_eq!(execution_order[0], "A", "System A (PreUpdate) should run first");
        assert_eq!(execution_order[1], "B", "System B (Update) should run second");
        assert_eq!(execution_order[2], "C", "System C (PostUpdate) should run last");
    }
    
    #[test]
    fn test_run_systems_with_dependencies() {
        let mut schedule = Schedule::new();
        
        // Add all systems to the same stage
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_b, "system_b"), "system_b", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_c, "system_c"), "system_c", Stage::Update).unwrap();
        
        // Create dependencies: C -> B -> A
        // This means A should run first, then B, then C
        schedule.add_dependency("system_b", "system_a").unwrap();
        schedule.add_dependency("system_c", "system_b").unwrap();
        
        let mut world = World::new();
        schedule.run(&mut world);
        
        let execution_order = world.get_resource::<Vec<String>>().unwrap();
        assert_eq!(execution_order.len(), 3, "All systems should have been executed");
        assert_eq!(execution_order[0], "A", "System A should run first (dependency of B)");
        assert_eq!(execution_order[1], "B", "System B should run second (dependency of C)");
        assert_eq!(execution_order[2], "C", "System C should run last");
    }
    
    #[test]
    fn test_get_system() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let system = schedule.get_system("system_a");
        assert!(system.is_some(), "Should be able to get a system by name");
        assert_eq!(system.unwrap().name(), "system_a", "Retrieved system should have correct name");
        
        let nonexistent = schedule.get_system("nonexistent");
        assert!(nonexistent.is_none(), "Should get None for nonexistent system");
    }
    
    #[test]
    fn test_get_system_mut() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let system = schedule.get_system_mut("system_a");
        assert!(system.is_some(), "Should be able to get a mutable system by name");
        assert_eq!(system.unwrap().name(), "system_a", "Retrieved system should have correct name");
    }
    
    #[test]
    fn test_remove_system() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        
        let result = schedule.remove_system("system_a");
        assert!(result.is_ok(), "Removing a system should succeed");
        
        let system = schedule.get_system("system_a");
        assert!(system.is_none(), "System should no longer exist after removal");
    }
    
    #[test]
    fn test_remove_nonexistent_system() {
        let mut schedule = Schedule::new();
        
        let result = schedule.remove_system("nonexistent");
        assert!(result.is_err(), "Removing a nonexistent system should fail");
        
        match result {
            Err(SchedulerError::SystemNotFound(name)) => {
                assert_eq!(name, "nonexistent", "Error should contain the nonexistent system name");
            },
            _ => panic!("Unexpected error type"),
        }
    }
    
    #[test]
    fn test_clear_schedule() {
        let mut schedule = Schedule::new();
        schedule.add_system(SystemFn::new(system_a, "system_a"), "system_a", Stage::Update).unwrap();
        schedule.add_system(SystemFn::new(system_b, "system_b"), "system_b", Stage::Update).unwrap();
        
        schedule.clear();
        
        let system_a = schedule.get_system("system_a");
        let system_b = schedule.get_system("system_b");
        
        assert!(system_a.is_none(), "System A should be removed after clear");
        assert!(system_b.is_none(), "System B should be removed after clear");
    }
}
