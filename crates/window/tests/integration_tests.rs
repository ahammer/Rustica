use rustica_window::WindowApp;

#[test]
fn test_window_app_creation() {
    let app = WindowApp::new("Integration Test Window", 1024, 768);
    assert!(app.window().is_none());
}

// This test is conditionally compiled only when the test-display feature is enabled
// This allows us to skip display tests in environments where a display is not available
#[cfg(feature = "test-display")]
#[test]
#[ignore = "This test requires running on the main thread and would block the test runner"]
fn test_window_app_run() {
    // Note: This test will be ignored by default because it requires running on the main thread
    // and would block the test runner. To run it manually, use:
    // cargo test --features test-display -- --ignored
    
    // In a real application, you would typically run this on the main thread
    let app = WindowApp::new("Test Run Window", 800, 600);
    
    // We don't actually call run() here because it would block the test runner
    // and requires being on the main thread
    // Instead, we just verify that the app was created successfully
    // We can't directly access private fields, so we just check that it doesn't panic
    assert!(app.window().is_none());
}
