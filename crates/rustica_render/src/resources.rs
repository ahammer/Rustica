use std::path::{Path, PathBuf};
use std::env;

/// Resource manager for finding assets like shader files
pub struct ResourceManager {
    executable_dir: PathBuf,
}

impl ResourceManager {
    /// Create a new resource manager
    pub fn new() -> Self {
        let executable_path = env::current_exe().expect("Failed to get executable path");
        let executable_dir = executable_path.parent().expect("Failed to get executable directory").to_path_buf();
        
        Self { executable_dir }
    }
    
    /// Find a shader file by name (without extension)
    pub fn find_shader(&self, name: &str) -> Option<PathBuf> {
        // List of possible locations to check, in order of priority
        let locations = [
            // 1. Check in 'shaders' directory next to the executable (release mode)
            self.executable_dir.join("shaders").join(format!("{}.wgsl", name)),
            
            // 2. Check in the workspace target directory (debug mode)
            self.executable_dir.join("..").join("shaders").join(format!("{}.wgsl", name)),
            
            // 3. Check in source (during development)
            PathBuf::from("src").join("shaders").join(format!("{}.wgsl", name)),
        ];
        
        // Try each location and return the first one that exists
        for location in &locations {
            if location.exists() {
                return Some(location.clone());
            }
        }
        
        None
    }
    
    /// Load a shader as a string by name
    pub fn load_shader(&self, name: &str) -> Result<String, String> {
        match self.find_shader(name) {
            Some(path) => std::fs::read_to_string(&path)
                .map_err(|e| format!("Failed to read shader '{}' from {:?}: {}", name, path, e)),
            None => Err(format!("Shader file '{}' not found", name)),
        }
    }
}

/// Get the default resource manager (singleton)
pub fn get_resource_manager() -> &'static ResourceManager {
    static mut INSTANCE: Option<ResourceManager> = None;
    static INIT: std::sync::Once = std::sync::Once::new();
    
    unsafe {
        INIT.call_once(|| {
            INSTANCE = Some(ResourceManager::new());
        });
        
        INSTANCE.as_ref().unwrap()
    }
}