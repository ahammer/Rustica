//! # Resource System
//! 
//! This module defines the Resource trait, which is a marker trait
//! for types that can be stored in the application.

/// The Resource trait is a marker trait for types that can be stored
/// in the application as shared resources.
///
/// Resources are global data that can be accessed by systems.
/// They are stored in the App and can be retrieved using the
/// get_resource and get_resource_mut methods.
///
/// There's generally no need to implement this trait directly,
/// as it is automatically implemented for all types that can be
/// safely shared between threads.
///
/// # Examples
///
/// ```
/// use rustica_common::Resource;
///
/// #[derive(Debug)]
/// struct WindowConfig {
///     width: u32,
///     height: u32,
/// }
///
/// // WindowConfig is automatically a Resource
/// ```
pub trait Resource: 'static {}

// Automatically implement Resource for any type that is 'static
impl<T: 'static> Resource for T {}
