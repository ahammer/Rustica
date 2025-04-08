use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields, Data, Meta, Lit, Expr, ExprLit, Path};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use std::collections::HashSet;

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

    let mut vertex_fields = Vec::new();
    let mut uniform_fields = Vec::new();
    let mut instance_fields = Vec::new();

    let mut shader_path = None;
    let mut shader_inline = None;

    for attr in &input.attrs {
        if attr.path().is_ident("shader") {
            let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).unwrap();
            for meta in args {
                if let Meta::NameValue(nv) = meta {
                    if nv.path.is_ident("file") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = nv.value {
                            shader_path = Some(s.value());
                        }
                    } else if nv.path.is_ident("inline") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = nv.value {
                            shader_inline = Some(s.value());
                        }
                    }
                }
            }
        }
    }

    let mut binding_counter = 0;
    let mut location_counter = 0;
    let mut used_bindings = HashSet::new();
    let mut used_locations = HashSet::new();

    for field in fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        let mut field_category = None;
        let mut location = None;
        let mut binding = None;
        let mut format = None;
        let mut semantic = None;

        for attr in &field.attrs {
            if attr.path().is_ident("vertex") || attr.path().is_ident("uniform") || attr.path().is_ident("instance") {
                let args = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated).unwrap_or_default();

                if attr.path().is_ident("vertex") {
                    field_category = Some("vertex");
                    for meta in args {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("location") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                    location = Some(int_lit.base10_parse::<u32>().unwrap());
                                }
                            } else if nv.path.is_ident("format") {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(fmt), .. }) = &nv.value {
                                    format = Some(fmt.value());
                                }
                            } else if nv.path.is_ident("semantic") {
                                if let Expr::Path(path_expr) = &nv.value {
                                    semantic = Some(path_expr.path.clone());
                                }
                            }
                        }
                    }
                }
                else if attr.path().is_ident("uniform") {
                    field_category = Some("uniform");
                    for meta in args {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("binding") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                    binding = Some(int_lit.base10_parse::<u32>().unwrap());
                                }
                            }
                        }
                    }
                }
                else if attr.path().is_ident("instance") {
                    field_category = Some("instance");
                    for meta in args {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("location") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(int_lit), .. }) = &nv.value {
                                    location = Some(int_lit.base10_parse::<u32>().unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }

        match field_category {
            Some("vertex") | Some("instance") => {
                // Get location (user-defined or auto-assigned)
                let loc = if let Some(user_loc) = location {
                    // Check for duplicate location
                    if !used_locations.insert(user_loc) {
                        return syn::Error::new_spanned(
                            ident, 
                            format!("Duplicate location assignment: {}. Each vertex/instance attribute must have a unique location.", user_loc)
                        ).to_compile_error().into();
                    }
                    user_loc
                } else {
                    // Auto-assign: find the next available location
                    while used_locations.contains(&location_counter) {
                        location_counter += 1;
                    }
                    let assigned = location_counter;
                    used_locations.insert(assigned);
                    location_counter += 1;
                    assigned
                };

                if field_category == Some("vertex") {
                    vertex_fields.push((ident.clone(), ty.clone(), loc, format, semantic));
                } else {
                    instance_fields.push((ident.clone(), ty.clone(), loc));
                }
            },
            Some("uniform") => {
                // Get binding (user-defined or auto-assigned)
                let bind = if let Some(user_bind) = binding {
                    // Check for duplicate binding
                    if !used_bindings.insert(user_bind) {
                        return syn::Error::new_spanned(
                            ident, 
                            format!("Duplicate binding assignment: {}. Each uniform must have a unique binding.", user_bind)
                        ).to_compile_error().into();
                    }
                    user_bind
                } else {
                    // Auto-assign: find the next available binding
                    while used_bindings.contains(&binding_counter) {
                        binding_counter += 1;
                    }
                    let assigned = binding_counter;
                    used_bindings.insert(assigned);
                    binding_counter += 1;
                    assigned
                };
                
                uniform_fields.push((ident.clone(), ty.clone(), bind));
            },
            _ => {}
        }
    }

    let vertex_name = format_ident!("{}Vertex", name);
    let uniform_name = format_ident!("{}Uniforms", name);
    let instance_name = format_ident!("{}Instances", name);

    // Create vertex attribute definitions for the layout implementation
    let vertex_attribute_defs = vertex_fields.iter().enumerate().map(|(idx, (field_ident, ty, loc, format_opt, _semantic_opt))| {
        // Determine format based on provided format or Rust type
        let format = if let Some(fmt) = format_opt {
            // If format is explicitly provided, use it
            quote! { wgpu::VertexFormat::#fmt }
        } else {
            // Otherwise infer from the Rust type
            match ty.to_token_stream().to_string().as_str() {
                "[f32; 2]" => quote! { wgpu::VertexFormat::Float32x2 },
                "[f32; 3]" => quote! { wgpu::VertexFormat::Float32x3 },
                "[f32; 4]" => quote! { wgpu::VertexFormat::Float32x4 },
                "f32" => quote! { wgpu::VertexFormat::Float32 },
                // Add more mappings as needed
                _ => quote! { wgpu::VertexFormat::Float32x3 }, // Default fallback
            }
        };
        
        // Calculate offset based on actual field sizes
        let offset = if idx == 0 {
            quote! { 0u64 }
        } else {
            // Get previous field types for offset calculation
            let previous_fields = vertex_fields.iter().take(idx).map(|(_, ty, _, _, _)| ty);
            
            // Calculate cumulative offset based on actual sizes of previous fields
            quote! {
                {
                    let mut offset = 0;
                    #(
                        // Get size of each previous field type
                        offset += std::mem::size_of::<#previous_fields>();
                    )*
                    offset as u64
                }
            }
        };
        
        quote! {
            wgpu::VertexAttribute {
                format: #format,
                offset: #offset,
                shader_location: #loc,
            }
        }
    }).collect::<Vec<_>>();

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
        quote! {}
    };

    // Create vertex struct with Vertex trait implementation
    let vertex_def = if !vertex_fields.is_empty() {
        let fields = vertex_fields.iter().map(|(i, t, _, _, _)| quote! { pub #i: #t });
        
        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
            pub struct #vertex_name {
                #(#fields,)* 
            }

            // Implement Vertex trait
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
            
            #custom_attributes
        }
    } else { 
        quote! {} 
    };

    // Define the vertex factory if there are vertex fields
    let factory_def = if !vertex_fields.is_empty() {
        let factory_name = format_ident!("{}VertexFactory", name);
        
        // Generate constructor parameters for create_vertex
        let factory_params = vertex_fields.iter().map(|(i, t, _, _, _)| {
            quote! { #i: #t }
        });
        
        // Generate field assignments for create_vertex
        let factory_field_assignments = vertex_fields.iter().map(|(i, _, _, _, _)| {
            quote! { #i }
        });
        
        quote! {
            /// Factory for creating shader-compatible vertices
            #[derive(Debug, Clone, Copy)]
            pub struct #factory_name;
            
            impl #factory_name {
                pub fn create_vertex(#(#factory_params),*) -> #vertex_name {
                    #vertex_name {
                        #(#factory_field_assignments),*
                    }
                }
            }
        }
    } else {
        quote! {}
    };
    
    // Generate vertex_factory method if there are vertex fields
    let vertex_factory_method = if !vertex_fields.is_empty() {
        let factory_name = format_ident!("{}VertexFactory", name);
        
        quote! {
            /// Get a vertex factory for this shader
            pub fn vertex_factory() -> #factory_name {
                #factory_name
            }
        }
    } else {
        quote! {}
    };

    let uniform_def = if !uniform_fields.is_empty() {
        let fields = uniform_fields.iter().map(|(i, t, _)| quote! { pub #i: #t });
        
        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
            pub struct #uniform_name {
                #(#fields,)* 
            }
        }
    } else { 
        quote! {} 
    };

    let instance_def = if !instance_fields.is_empty() {
        let fields = instance_fields.iter().map(|(i, t, _)| quote! { pub #i: #t });
        
        quote! {
            #[repr(C)]
            #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
            pub struct #instance_name {
                #(#fields,)* 
            }
        }
    } else { 
        quote! {} 
    };

    let shader_source = if let Some(s) = shader_inline {
        quote! { String::from(#s) }
    } else if let Some(p) = shader_path {
        // Use include_str! to embed the shader file at compile time
        quote! { String::from(include_str!(#p)) }
    } else {
        // Fallback to a default path pattern using include_str!
        quote! {
            String::from(include_str!(concat!("./src/shaders/", stringify!(#name), ".wgsl")))
        }
    };

    let uniform_param_exprs = uniform_fields.iter().map(|(ident, ty, binding)| {
        quote! {
            rustica_render::UniformParameter {
                name: stringify!(#ident).to_string(),
                binding: #binding,
                size: std::mem::size_of::<#ty>() as u64,
            }
        }
    });

    let expanded = quote! {
        #vertex_def
        #factory_def
        #uniform_def
        #instance_def

        impl #name {
            pub fn descriptor() -> rustica_render::ShaderDescriptor {
                rustica_render::ShaderDescriptor {
                    name: stringify!(#name).to_string(),
                    shader_source: #shader_source,
                    vertex_attributes: <#vertex_name as rustica_render::VertexAttributeProvider>::attributes(),
                    uniforms: vec![ #(#uniform_param_exprs),* ]
                }
            }
            
            /// Create a new geometry builder for this shader's vertex type
            pub fn geometry_builder() -> rustica_foundation::geometry::GeometryBuilder<#vertex_name> {
                rustica_foundation::geometry::GeometryBuilder::new()
            }
            
            #vertex_factory_method
        }
    };

    TokenStream::from(expanded)
}
