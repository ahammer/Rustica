use crate::stage::Stage;

#[cfg(test)]
mod stage_tests {
    use super::*;

    #[test]
    fn test_stage_ordering() {
        // Test that stages have the correct ordering
        assert!(Stage::Start < Stage::EarlyUpdate, "Start should come before EarlyUpdate");
        assert!(Stage::EarlyUpdate < Stage::Update, "EarlyUpdate should come before Update");
        assert!(Stage::Update < Stage::LateUpdate, "Update should come before LateUpdate");
        assert!(Stage::LateUpdate < Stage::PreRender, "LateUpdate should come before PreRender");
        assert!(Stage::PreRender < Stage::Render, "PreRender should come before Render");
        assert!(Stage::Render < Stage::PostRender, "Render should come before PostRender");
        assert!(Stage::PostRender < Stage::End, "PostRender should come before End");
    }

    #[test]
    fn test_stage_all() {
        // Test that all() returns all stages in the correct order
        let stages = Stage::all();
        
        assert_eq!(stages.len(), 8, "Should have 8 stages");
        assert_eq!(stages[0], Stage::Start, "First stage should be Start");
        assert_eq!(stages[1], Stage::EarlyUpdate, "Second stage should be EarlyUpdate");
        assert_eq!(stages[2], Stage::Update, "Third stage should be Update");
        assert_eq!(stages[3], Stage::LateUpdate, "Fourth stage should be LateUpdate");
        assert_eq!(stages[4], Stage::PreRender, "Fifth stage should be PreRender");
        assert_eq!(stages[5], Stage::Render, "Sixth stage should be Render");
        assert_eq!(stages[6], Stage::PostRender, "Seventh stage should be PostRender");
        assert_eq!(stages[7], Stage::End, "Eighth stage should be End");
    }

    #[test]
    fn test_stage_equality() {
        // Test stage equality comparisons
        assert_eq!(Stage::Start, Stage::Start, "Stages should be equal to themselves");
        assert_ne!(Stage::Start, Stage::Update, "Different stages should not be equal");
    }

    #[test]
    fn test_stage_hash() {
        // Test that stages can be used as hash keys
        use std::collections::HashMap;
        
        let mut stage_map = HashMap::new();
        stage_map.insert(Stage::Start, "Start");
        stage_map.insert(Stage::EarlyUpdate, "EarlyUpdate");
        stage_map.insert(Stage::Update, "Update");
        
        assert_eq!(stage_map.get(&Stage::Start), Some(&"Start"), "Should get correct value for Start");
        assert_eq!(stage_map.get(&Stage::EarlyUpdate), Some(&"EarlyUpdate"), "Should get correct value for EarlyUpdate");
        assert_eq!(stage_map.get(&Stage::Update), Some(&"Update"), "Should get correct value for Update");
        assert_eq!(stage_map.get(&Stage::LateUpdate), None, "Should get None for missing stage");
    }

    #[test]
    fn test_stage_debug() {
        // Test the Debug implementation
        let debug_str = format!("{:?}", Stage::Start);
        assert!(debug_str.contains("Start"), "Debug output should include stage name");
    }
    
    #[test]
    fn test_stage_ordinal() {
        // Test that ordinal values are correctly assigned
        assert_eq!(Stage::Start.ordinal(), 0);
        assert_eq!(Stage::EarlyUpdate.ordinal(), 1);
        assert_eq!(Stage::Update.ordinal(), 2);
        assert_eq!(Stage::LateUpdate.ordinal(), 3);
        assert_eq!(Stage::PreRender.ordinal(), 4);
        assert_eq!(Stage::Render.ordinal(), 5);
        assert_eq!(Stage::PostRender.ordinal(), 6);
        assert_eq!(Stage::End.ordinal(), 7);
    }
    
    #[test]
    fn test_stage_default() {
        // Test that the default stage is Update
        let default_stage = Stage::default();
        assert_eq!(default_stage, Stage::Update);
    }
}
