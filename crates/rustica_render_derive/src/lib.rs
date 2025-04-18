use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields, Data, Meta, Lit, Expr, ExprLit, Path, Attribute, Field, Type};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use std::collections::HashSet;

// --- Helper Structs for Parsed Attributes ---

#[derive(Debug)]
enum ShaderSource {
    File(String),
    Inline(String),
}

#[derive(Debug, Default)]
struct ShaderAttr {
    source: Option<ShaderSource>,
}

#[derive(Debug, Default)]
struct VertexAttrParams {
    location: Option<u32>,
    format: Option<String>,
    semantic: Option<Path>,
}

#[derive(Debug, Default)]
struct UniformAttrParams {
    binding: Option<u32>,
}

#[derive(Debug, Default)]
struct InstanceAttrParams {
    location: Option<u32>,
}

#[derive(Debug)]
enum FieldAttr {
    Vertex(VertexAttrParams),
    Uniform(UniformAttrParams),
    Instance(InstanceAttrParams),
}

// --- Attribute Parsing Functions ---

fn parse_shader_attribute(attrs: &[Attribute]) -> Result<ShaderAttr, syn::Error> {
    let mut shader_attr = ShaderAttr::default();

    for attr in attrs {
        if attr.path().is_ident("shader") {
            // Prevent multiple #[shader] attributes
            if shader_attr.source.is_some() {
                return Err(syn::Error::new_spanned(attr, "Duplicate #[shader] attribute"));
            }

            let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated)?;
            for meta in args {
                if let Meta::NameValue(nv) = meta {
                    if nv.path.is_ident("file") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = nv.value {
                            shader_attr.source = Some(ShaderSource::File(s.value()));
                        } else {
                            return Err(syn::Error::new_spanned(nv.value, "Expected string literal for 'file'"));
                        }
                    } else if nv.path.is_ident("inline") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = nv.value {
                            shader_attr.source = Some(ShaderSource::Inline(s.value()));
                        } else {
                            return Err(syn::Error::new_spanned(nv.value, "Expected string literal for 'inline'"));
                        }
                    } else {
                        return Err(syn::Error::new_spanned(nv.path, "Unknown parameter in #[shader] attribute"));
                    }
                } else {
                     return Err(syn::Error::new_spanned(meta, "Unsupported attribute format in #[shader]"));
                }
            }
             // Ensure either file or inline was provided if #[shader] is present
            if shader_attr.source.is_none() {
                 return Err(syn::Error::new_spanned(attr, "#[shader] attribute requires 'file' or 'inline' parameter"));
            }
        }
    }
    Ok(shader_attr)
}


fn parse_field_attribute(field: &Field) -> Result<Option<FieldAttr>, syn::Error> {
    let mut field_attr_opt: Option<FieldAttr> = None;

    for attr in &field.attrs {
        let attr_ident = if let Some(ident) = attr.path().get_ident() {
            ident.to_string()
        } else {
            continue; // Skip complex paths for now
        };

        let mut current_attr: Option<FieldAttr> = None;

        match attr_ident.as_str() {
            "vertex" => {
                let mut params = VertexAttrParams::default();
                let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).unwrap_or_default();
                for meta in args {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("location") {
                            if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                params.location = Some(int_lit.base10_parse::<u32>()?);
                            } else { return Err(syn::Error::new_spanned(nv.value, "Expected integer literal for 'location'")); }
                        } else if nv.path.is_ident("format") {
                            if let Expr::Lit(ExprLit { lit: Lit::Str(fmt), .. }) = &nv.value {
                                params.format = Some(fmt.value());
                            } else { return Err(syn::Error::new_spanned(nv.value, "Expected string literal for 'format'")); }
                        } else if nv.path.is_ident("semantic") {
                            if let Expr::Path(path_expr) = &nv.value {
                                params.semantic = Some(path_expr.path.clone());
                            } else { return Err(syn::Error::new_spanned(nv.value, "Expected path for 'semantic'")); }
                        } else { return Err(syn::Error::new_spanned(nv.path, "Unknown parameter in #[vertex] attribute")); }
                    } else { return Err(syn::Error::new_spanned(meta, "Unsupported attribute format in #[vertex]")); }
                }
                current_attr = Some(FieldAttr::Vertex(params));
            }
            "uniform" => {
                let mut params = UniformAttrParams::default();
                let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).unwrap_or_default();
                 for meta in args {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("binding") {
                             if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                params.binding = Some(int_lit.base10_parse::<u32>()?);
                            } else { return Err(syn::Error::new_spanned(nv.value, "Expected integer literal for 'binding'")); }
                        } else { return Err(syn::Error::new_spanned(nv.path, "Unknown parameter in #[uniform] attribute")); }
                    } else { return Err(syn::Error::new_spanned(meta, "Unsupported attribute format in #[uniform]")); }
                }
                current_attr = Some(FieldAttr::Uniform(params));
            }
            "instance" => {
                 let mut params = InstanceAttrParams::default();
                 let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).unwrap_or_default();
                 for meta in args {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("location") {
                             if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                params.location = Some(int_lit.base10_parse::<u32>()?);
                            } else { return Err(syn::Error::new_spanned(nv.value, "Expected integer literal for 'location'")); }
                        } else { return Err(syn::Error::new_spanned(nv.path, "Unknown parameter in #[instance] attribute")); }
                    } else { return Err(syn::Error::new_spanned(meta, "Unsupported attribute format in #[instance]")); }
                }
                current_attr = Some(FieldAttr::Instance(params));
            }
            _ => {} // Ignore other attributes
        }

        if let Some(parsed_attr) = current_attr {
            if field_attr_opt.is_some() {
                // Found another relevant attribute on the same field
                return Err(syn::Error::new_spanned(attr, "Field cannot have multiple #[vertex/uniform/instance] attributes"));
            }
            field_attr_opt = Some(parsed_attr);
        }
    }

    Ok(field_attr_opt)
}


/// `#[derive(ShaderProperties)]` macro splits one struct into multiple data blocks: Vertex, Uniforms, Instances
///
/// This macro automatically:
/// - Generates separate structs for vertex, instance, and uniform data
/// - Auto-assigns binding and location values if not explicitly provided
/// - Validates that there are no duplicate bindings or locations
/// - Creates helper methods to work with the shader
///
/// # Automatic Assignment
/// - For `#[vertex]` and `#[instance]` fields without a `location`, a unique location is auto-assigned
/// - For `#[uniform]` fields without a `binding`, a unique binding point is auto-assigned
/// - Auto-assignment skips over any explicitly assigned values to avoid conflicts
/// - Compile-time errors are generated if duplicate bindings or locations are detected
///
/// # Example
/// ```ignore
/// #[derive(ShaderProperties)]
/// #[shader(file = "shaders/my_shader.wgsl")]
/// struct MyShader {
///     // Location 0 automatically assigned
///     #[vertex(semantic = VertexSemantic::Position)]
///     position: [f32; 3],
///
///     // Explicitly set location 2 (skipping 1)
///     #[vertex(location = 2, semantic = VertexSemantic::Normal)]
///     normal: [f32; 3],
///
///     // Location 1 automatically assigned (filling the gap)
///     #[vertex]
///     color: [f32; 4],
///
///     // Auto-assigned binding 0
///     #[uniform]
///     model: [[f32; 4]; 4],
///
///     // Explicitly set binding 1
///     #[uniform(binding = 1)]
///     view_proj: [[f32; 4]; 4],
///
///     // Explicitly set location 3
///     #[instance(location = 3)]
///     model_matrix: [[f32; 4]; 4],
/// }
///
/// // Generates:
/// // - MyShaderVertex { position: [f32; 3], normal: [f32; 3], color: [f32; 4] }
/// // - MyShaderInstances { model_matrix: [[f32; 4]; 4] }
/// // - MyShaderUniforms { model: [[f32; 4]; 4], view_proj: [[f32; 4]; 4] }
/// // - MyShaderVertexFactory for creating vertices
/// ```
#[proc_macro_derive(ShaderProperties, attributes(shader, vertex, uniform, instance))]
pub fn derive_shader_properties(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return syn::Error::new_spanned(&input, "Expected named fields").to_compile_error().into(),
        },
        _ => return syn::Error::new_spanned(&input, "ShaderProperties can only be derived for structs").to_compile_error().into(),
    };

    // Parse the top-level #[shader] attribute
    let shader_attr = match parse_shader_attribute(&input.attrs) {
        Ok(attr) => attr,
        Err(e) => return e.to_compile_error().into(),
    };

    // Prepare lists to hold categorized field information
    let mut vertex_fields: Vec<(syn::Ident, Type, u32, Option<String>, Option<Path>)> = Vec::new();
    let mut uniform_fields: Vec<(syn::Ident, Type, u32)> = Vec::new();
    let mut instance_fields: Vec<(syn::Ident, Type, u32)> = Vec::new();

    // State for auto-assignment
    let mut binding_counter: u32 = 0;
    let mut location_counter: u32 = 0;
    let mut used_bindings: HashSet<u32> = HashSet::new();
    let mut used_locations: HashSet<u32> = HashSet::new();

    // Process each field
    for field in fields {
        let ident = field.ident.as_ref().expect("Expected named fields").clone();
        let ty = field.ty.clone();

        // Parse the field's attribute ([vertex], [uniform], or [instance])
        let field_attr = match parse_field_attribute(field) {
             Ok(Some(attr)) => attr,
             Ok(None) => continue, // Skip fields without relevant attributes
             Err(e) => return e.to_compile_error().into(),
        };

        // Handle based on the parsed attribute type
        match field_attr {
            FieldAttr::Vertex(params) => {
                let loc = if let Some(user_loc) = params.location {
                    if !used_locations.insert(user_loc) {
                        return syn::Error::new_spanned(
                            &field.ident,
                            format!("Duplicate location assignment: {}. Each vertex/instance attribute must have a unique location.", user_loc)
                        ).to_compile_error().into();
                    }
                    user_loc
                } else {
                    while used_locations.contains(&location_counter) {
                        location_counter += 1;
                    }
                    let assigned_loc = location_counter;
                    used_locations.insert(assigned_loc);
                    location_counter += 1; // Increment for the next potential auto-assignment
                    assigned_loc
                };
                vertex_fields.push((ident, ty, loc, params.format, params.semantic));
            }
            FieldAttr::Instance(params) => {
                 let loc = if let Some(user_loc) = params.location {
                    if !used_locations.insert(user_loc) {
                        return syn::Error::new_spanned(
                            &field.ident,
                            format!("Duplicate location assignment: {}. Each vertex/instance attribute must have a unique location.", user_loc)
                        ).to_compile_error().into();
                    }
                    user_loc
                } else {
                    while used_locations.contains(&location_counter) {
                        location_counter += 1;
                    }
                     let assigned_loc = location_counter;
                    used_locations.insert(assigned_loc);
                    location_counter += 1;
                    assigned_loc
                };
                instance_fields.push((ident, ty, loc));
            }
            FieldAttr::Uniform(params) => {
                let bind = if let Some(user_bind) = params.binding {
                    if !used_bindings.insert(user_bind) {
                        return syn::Error::new_spanned(
                            &field.ident,
                            format!("Duplicate binding assignment: {}. Each uniform must have a unique binding.", user_bind)
                        ).to_compile_error().into();
                    }
                    user_bind
                } else {
                    while used_bindings.contains(&binding_counter) {
                        binding_counter += 1;
                    }
                    let assigned_bind = binding_counter;
                    used_bindings.insert(assigned_bind);
                    binding_counter += 1; // Increment for the next potential auto-assignment
                    assigned_bind
                };
                uniform_fields.push((ident, ty, bind));
            }
        }
    }

    // --- Code Generation ---

    let vertex_name = format_ident!("{}Vertex", name);
    let uniform_name = format_ident!("{}Uniforms", name);
    let instance_name = format_ident!("{}Instances", name);

    // Create vertex attribute definitions for the layout implementation
    let vertex_attribute_defs = vertex_fields.iter().enumerate().map(|(idx, (field_ident, ty, loc, format_opt, _semantic_opt))| {
        // Determine format based on provided format string or infer from Rust type
        let format_expr = if let Some(fmt_str) = format_opt {
            // If format is explicitly provided as a string, parse it
            // We need to convert the string (e.g., "Float32x3") into a wgpu::VertexFormat variant
            // This requires parsing the string and creating the corresponding identifier.
            // For simplicity here, we'll assume the string directly matches a variant name.
            // A more robust solution might use a match statement or a library.
            let format_ident = format_ident!("{}", fmt_str);
            quote! { wgpu::VertexFormat::#format_ident }
        } else {
            // Otherwise infer from the Rust type (keep existing logic)
             match ty.to_token_stream().to_string().as_str() {
                 "[f32; 2]" => quote! { wgpu::VertexFormat::Float32x2 },
                 "[f32; 3]" => quote! { wgpu::VertexFormat::Float32x3 },
                 "[f32; 4]" => quote! { wgpu::VertexFormat::Float32x4 },
                 "f32" => quote! { wgpu::VertexFormat::Float32 },
                 // Add more mappings as needed
                 _ => {
                     // Emit a compile error if type inference fails for a required field
                     return syn::Error::new_spanned(
                         ty,
                         format!("Cannot infer VertexFormat for type '{}'. Please specify format explicitly using #[vertex(format = \"...\")]", ty.to_token_stream())
                     ).to_compile_error().into();
                 }
             }
        };

        // Calculate offset based on previous fields' sizes
         let offset_expr = if idx == 0 {
            quote! { 0 }
        } else {
            // Calculate offset based on the size of previous fields in the generated struct
            let prev_field_types = vertex_fields.iter().take(idx).map(|(_, ty, _, _, _)| ty);
            quote! {
                 {
                    let mut current_offset = 0;
                    #(
                        current_offset += std::mem::size_of::<#prev_field_types>();
                    )*
                    current_offset as wgpu::BufferAddress
                 }
            }
        };

        quote! {
            wgpu::VertexAttribute {
                format: #format_expr,
                offset: #offset_expr,
                 shader_location: #loc,
            }
        }
    }).collect::<Vec<_>>();

    // Create instance attribute definitions (similar logic, simpler as format/offset are often standard for matrices)
    // TODO: Add instance attribute generation if needed, following vertex pattern but with VertexStepMode::Instance

    // Now we need to properly store the semantic information from the attributes
    // Create a way to map our vertex fields to semantics for our custom VertexAttribute
    let custom_attributes = if !vertex_fields.is_empty() {
        // Generate code to create custom vertex attributes with semantics from our Vertex implementation
        let attribute_fields = vertex_fields.iter().enumerate().map(|(idx, (field_name, _, loc, _, semantic_opt))| {
            let field_str = field_name.to_string();
            let semantic = if let Some(sem_path) = semantic_opt {
                // Check if the path starts with "VertexSemantic"
                if let Some(first_segment) = sem_path.segments.first() {
                    if first_segment.ident == "VertexSemantic" {
                        // Prepend the full path from foundation
                        quote! { Some(rustica_foundation::geometry::#sem_path) }
                    } else {
                        // Use the path as provided by the user if it's not VertexSemantic
                        quote! { Some(#sem_path) }
                    }
                } else {
                     // Use the path as provided if it's simple (shouldn't happen for enums like this)
                     quote! { Some(#sem_path) }
                }
            } else {
                quote! { None }
            };
            
            quote! {
                rustica_foundation::geometry::VertexAttribute {
                    name: #field_str.to_string(),
                    location: #loc,
                    format: ATTRIBUTES[#idx].format,
                    offset: ATTRIBUTES[#idx].offset,
                    semantic: #semantic,
                }
            }
        });
        
        quote! {
            // Override VertexAttributeProvider implementation to include semantics
            impl rustica_foundation::geometry::VertexAttributeProvider for #vertex_name {
                fn attributes() -> Vec<rustica_foundation::geometry::VertexAttribute> {
                    let layout = Self::layout();
                    static ATTRIBUTES: &[wgpu::VertexAttribute] = &[ 
                        #(#vertex_attribute_defs,)* 
                    ];
                    
                    vec![
                        #(#attribute_fields),*
                    ]
                }
             }
        }
    } else {
        // If no vertex fields, provide a default empty implementation or handle as needed
        quote! {
            // Provide a default implementation for VertexAttributeProvider if no vertex fields
             impl rustica_foundation::geometry::VertexAttributeProvider for #vertex_name {
                 fn attributes() -> Vec<rustica_foundation::geometry::VertexAttribute> {
                     Vec::new() // No attributes
                 }
             }
        }
    };


    // --- Generate Struct Definitions ---

    // Create vertex struct definition
     let vertex_def = if !vertex_fields.is_empty() {
        let vertex_struct_fields = vertex_fields.iter().map(|(ident, ty, _, _, _)| quote! { pub #ident: #ty });
        quote! {
             #[repr(C)] // <-- ADDED THIS LINE
             #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #vertex_name {
                 #(#vertex_struct_fields,)*
             }

             // Implement Vertex trait for layout
             impl rustica_foundation::geometry::Vertex for #vertex_name {
                 fn layout() -> wgpu::VertexBufferLayout<'static> {
                     static ATTRIBUTES: &[wgpu::VertexAttribute] = &[
                         #(#vertex_attribute_defs,)*
                     ];

                     wgpu::VertexBufferLayout {
                         array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
                         step_mode: wgpu::VertexStepMode::Vertex,
                         attributes: ATTRIBUTES,
                     }
                 }
             }

             #custom_attributes // Add the VertexAttributeProvider impl
        }
    } else {
         // Define an empty struct if no vertex fields
         quote! {
             #[repr(C)] // <-- ADDED THIS LINE (though likely less critical for empty structs)
             #[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #vertex_name;

             // Implement Vertex trait with empty layout
             impl rustica_foundation::geometry::Vertex for #vertex_name {
                 fn layout() -> wgpu::VertexBufferLayout<'static> {
                     wgpu::VertexBufferLayout {
                         array_stride: 0,
                         step_mode: wgpu::VertexStepMode::Vertex,
                         attributes: &[],
                     }
                 }
             }
             #custom_attributes // Add the (empty) VertexAttributeProvider impl
         }
    };

     // Define the vertex factory struct and methods
     let factory_def = if !vertex_fields.is_empty() {
        let factory_name = format_ident!("{}VertexFactory", name);
        let factory_params = vertex_fields.iter().map(|(ident, ty, _, _, _)| quote! { #ident: #ty });
        let factory_assignments = vertex_fields.iter().map(|(ident, _, _, _, _)| quote! { #ident });

         quote! {
             /// Factory for creating shader-compatible vertices
             #[derive(Debug, Clone, Copy, Default)]
             pub struct #factory_name;

             impl #factory_name {
                 /// Creates a new vertex instance.
                 pub fn create_vertex(#(#factory_params),*) -> #vertex_name {
                     #vertex_name {
                         #(#factory_assignments),*
                     }
                 }
             }
         }
    } else {
         // If no vertex fields, provide a default factory
         let factory_name = format_ident!("{}VertexFactory", name);
         quote! {
             #[derive(Debug, Clone, Copy, Default)]
             pub struct #factory_name;
             impl #factory_name {
                 pub fn create_vertex() -> #vertex_name { #vertex_name::default() }
             }
         }
    };

     // Generate the vertex_factory() associated function for the original struct
     let vertex_factory_method = {
        let factory_name = format_ident!("{}VertexFactory", name);
         quote! {
             /// Get a vertex factory for this shader's vertex type.
             pub fn vertex_factory() -> #factory_name {
                 #factory_name::default()
             }
         }
    };


    // Create uniform struct definition
     let uniform_def = if !uniform_fields.is_empty() {
        let uniform_struct_fields = uniform_fields.iter().map(|(ident, ty, _)| quote! { pub #ident: #ty });
         quote! {
             #[repr(C)] // <-- ADDED THIS LINE
             #[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #uniform_name {
                 #(#uniform_struct_fields,)*
             }
         }
    } else {
         // Define an empty struct if no uniform fields
         quote! {
             #[repr(C)] // <-- ADDED THIS LINE
             #[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #uniform_name;
         }
    };

     // Create instance struct definition
     let instance_def = if !instance_fields.is_empty() {
        let instance_struct_fields = instance_fields.iter().map(|(ident, ty, _)| quote! { pub #ident: #ty });
         quote! {
             #[repr(C)] // <-- ADDED THIS LINE
             #[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #instance_name {
                 #(#instance_struct_fields,)*
             }
             // TODO: Add Instance trait implementation similar to Vertex if needed
         }
    } else {
         // Define an empty struct if no instance fields
         quote! {
             #[repr(C)] // <-- ADDED THIS LINE
             #[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
             pub struct #instance_name;
             // TODO: Add Instance trait implementation similar to Vertex if needed
         }
    };


    // --- Generate ShaderDescriptor and Associated Functions ---

    // Determine shader source code based on parsed attribute
    let shader_source_expr = match shader_attr.source {
        Some(ShaderSource::Inline(s)) => quote! { String::from(#s) },
        Some(ShaderSource::File(p)) => quote! { String::from(include_str!(#p)) },
        None => {
            // Default path if #[shader] attribute is missing or doesn't specify source
            // Consider making this an error if explicit source is desired.
            let default_path = format!("./src/shaders/{}.wgsl", name);
            quote! { String::from(include_str!(#default_path)) }
        }
    };

    // Generate uniform parameter descriptions
    let uniform_param_exprs = uniform_fields.iter().map(|(ident, ty, binding)| {
         quote! {
             rustica_render::UniformParameter {
                 name: stringify!(#ident).to_string(),
                 binding: #binding,
                 size: std::mem::size_of::<#ty>() as u64, // Consider alignment
             }
         }
    });

    // --- Final Assembly of Generated Code ---

    let expanded = quote! {
        // Generated structs
        #vertex_def
        #factory_def // Place factory definition after vertex struct
         #uniform_def
         #instance_def

         // Implementation block for the original struct
         impl #name {
             /// Returns the shader descriptor containing metadata.
             pub fn descriptor() -> rustica_render::ShaderDescriptor {
                 rustica_render::ShaderDescriptor {
                     name: stringify!(#name).to_string(),
                     shader_source: #shader_source_expr,
                     // Use the VertexAttributeProvider trait method to get attributes
                     vertex_attributes: <#vertex_name as rustica_foundation::geometry::VertexAttributeProvider>::attributes(),
                     uniforms: vec![ #(#uniform_param_exprs),* ],
                     // TODO: Add instance attributes if implemented
                 }
             }

             /// Creates a new geometry builder for this shader's vertex type.
             pub fn geometry_builder() -> rustica_foundation::geometry::GeometryBuilder<#vertex_name> {
                 // Ensure the vertex type implements Default if needed by GeometryBuilder::new()
                 rustica_foundation::geometry::GeometryBuilder::<#vertex_name>::new()
             }

             // Add the vertex_factory() associated function
             #vertex_factory_method
         }
     };

    TokenStream::from(expanded)
 }
