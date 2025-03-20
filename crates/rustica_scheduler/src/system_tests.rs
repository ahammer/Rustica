use crate::system::{System, SystemFn};
use rustica_ecs::World;

#[cfg(test)]
mod system_tests {
    use super::*;
    
    // Helper functions for tests
    fn get_or_insert_counter(world: &mut World) -> &mut i32 {
        if world.get_resource::<i32>().is_none() {
            world.insert_resource(0i32);
        }
        world.get_resource_mut::<i32>().unwrap()
    }

    // A simple test system that increments a counter in the world
    fn test_system_fn(world: &mut World) {
        let counter = get_or_insert_counter(world);
        *counter += 1;
    }

    #[test]
    fn test_system_fn_creation() {
        let system = SystemFn::new(test_system_fn, "test_system");
        assert_eq!(system.name(), "test_system", "System should have correct name");
        assert!(system.dependencies().is_empty(), "New system should have no dependencies");
    }

    #[test]
    fn test_system_with_dependency() {
        let system = SystemFn::new(test_system_fn, "test_system")
            .with_dependency("other_system");
        
        assert_eq!(system.dependencies().len(), 1, "System should have one dependency");
        assert_eq!(system.dependencies()[0], "other_system", "System should have correct dependency");
    }

    #[test]
    fn test_system_with_multiple_dependencies() {
        let system = SystemFn::new(test_system_fn, "test_system")
            .with_dependencies(&["system1", "system2", "system3"]);
        
        assert_eq!(system.dependencies().len(), 3, "System should have three dependencies");
        assert_eq!(system.dependencies()[0], "system1", "First dependency should match");
        assert_eq!(system.dependencies()[1], "system2", "Second dependency should match");
        assert_eq!(system.dependencies()[2], "system3", "Third dependency should match");
    }

    #[test]
    fn test_system_add_dependency() {
        let mut system = SystemFn::new(test_system_fn, "test_system");
        system.add_dependency("new_dependency");
        
        assert_eq!(system.dependencies().len(), 1, "System should have one dependency");
        assert_eq!(system.dependencies()[0], "new_dependency", "System should have correct dependency");
    }

    #[test]
    fn test_system_execution() {
        let mut system = SystemFn::new(test_system_fn, "test_system");
        let mut world = World::new();
        
        // First execution should set counter to 1
        system.run(&mut world);
        let counter = world.get_resource::<i32>().unwrap();
        assert_eq!(*counter, 1, "Counter should be incremented to 1");
        
        // Second execution should increment to 2
        system.run(&mut world);
        let counter = world.get_resource::<i32>().unwrap();
        assert_eq!(*counter, 2, "Counter should be incremented to 2");
    }

    #[test]
    fn test_system_debug_output() {
        let system = SystemFn::new(test_system_fn, "test_system")
            .with_dependency("dependency");
        
        let debug_output = format!("{:?}", system);
        assert!(debug_output.contains("test_system"), "Debug output should contain system name");
        assert!(debug_output.contains("dependency"), "Debug output should contain dependency");
    }
}
