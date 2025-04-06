use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Fields, Data, Meta, Lit, Expr, ExprLit, Path};
use syn::punctuated::Punctuated;
use syn::token::Comma;

/// `#[derive(ShaderProperties)]` macro splits one struct into multiple data blocks: Vertex, Uniforms, Instances
///
/// # Example
/// ```ignore
/// #[derive(ShaderProperties)]
/// #[shader(file = "shaders/my_shader.wgsl")]
/// struct MyShader {
///     #[vertex(location = 0, semantic = VertexSemantic::Position)]
///     position: [f32; 3],
///
///     #[instance(location = 1)]
///     model_matrix: [[f32; 4]; 4],
///
///     #[uniform(binding = 0)]
///     color: [f32; 4],
/// }
///
/// // Generates:
/// // - MyShaderVertex { position: [f32; 3] }
/// // - MyShaderInstances { model_matrix: [[f32; 4]; 4] }
/// // - MyShaderUniforms { color: [f32; 4] }
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
            Some("vertex") => {
                let loc = location.unwrap_or_else(|| { let l = location_counter; location_counter += 1; l });
                vertex_fields.push((ident, ty, loc, format, semantic));
            },
            Some("uniform") => {
                let bind = binding.unwrap_or_else(|| { let b = binding_counter; binding_counter += 1; b });
                uniform_fields.push((ident, ty, bind));
            },
            Some("instance") => {
                let loc = location.unwrap_or_else(|| { let l = location_counter; location_counter += 1; l });
                instance_fields.push((ident, ty, loc));
            },
            _ => {}
        }
    }

    let vertex_name = format_ident!("{}Vertex", name);
    let uniform_name = format_ident!("{}Uniforms", name);
    let instance_name = format_ident!("{}Instances", name);

    // Create vertex attribute definitions for the layout implementation
    let vertex_attribute_defs = vertex_fields.iter().enumerate().map(|(idx, (field_ident, ty, loc, format_opt, semantic_opt))| {
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
        
        let semantic = if let Some(sem) = semantic_opt {
            quote! { Some(#sem) }
        } else {
            quote! { None }
        };
        
        quote! {
            wgpu::VertexAttribute {
                format: #format,
                offset: #offset,
                shader_location: #loc,
                semantic: #semantic,
            }
        }
    }).collect::<Vec<_>>();

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
            impl rustica_render::Vertex for #vertex_name {
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
            
            // We don't need to explicitly implement VertexAttributeProvider since it's 
            // automatically implemented for any type that implements Vertex
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
                /// Create a new vertex factory
                pub fn new() -> Self {
                    Self
                }
                
                /// Create a vertex with the given attributes
                pub fn create_vertex(&self, #(#factory_params),*) -> #vertex_name {
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
                #factory_name::new()
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
        quote! { std::fs::read_to_string(#p).expect("Failed to load shader file") }
    } else {
        quote! {
            std::fs::read_to_string(concat!("./src/shaders/", stringify!(#name), ".wgsl"))
                .or_else(|_| std::fs::read_to_string(concat!("./shaders/", stringify!(#name), ".wgsl")))
                .expect("Could not locate shader file")
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_shader_macro_parses_struct() {
        let input: DeriveInput = parse_quote! {
            #[derive(ShaderProperties)]
            #[shader(inline = "shader code")]
            struct ExampleShader {
                #[vertex(location = 0)]
                pos: [f32; 3],
                #[uniform(binding = 1)]
                color: [f32; 4],
                #[instance(location = 2)]
                transform: [[f32; 4]; 4],
            }
        };

        let _ = derive_shader_properties(TokenStream::from(quote! { #input }));
    }
    
    #[test]
    fn test_vertex_format_respected() {
        // Define a test struct with different vertex format attributes
        let input: DeriveInput = parse_quote! {
            #[derive(ShaderProperties)]
            #[shader(inline = "test shader")]
            struct TestFormatShader {
                #[vertex(location = 0, format = "Float32x2")]
                position_2d: [f32; 2],
                
                #[vertex(location = 1, format = "Float32x3")]
                normal: [f32; 3],
                
                #[vertex(location = 2, format = "Float32x4")]
                color: [f32; 4],
            }
        };

        // Get the generated code
        let result = derive_shader_properties(TokenStream::from(quote! { #input }));
        let result_str = result.to_string();
        
        // Check that the vertex format is respected
        assert!(result_str.contains("format : wgpu :: VertexFormat :: Float32x2"), 
                "Float32x2 format should be used for position_2d");
        assert!(result_str.contains("format : wgpu :: VertexFormat :: Float32x3"), 
                "Float32x3 format should be used for normal");
        assert!(result_str.contains("format : wgpu :: VertexFormat :: Float32x4"), 
                "Float32x4 format should be used for color");
        
        // Check offset calculation is correct based on actual sizes
        // Find offset references in generated code
        assert!(!result_str.contains("offset : (1u64 * std :: mem :: size_of :: < [f32 ; 3] > ())"), 
                "Offset should not be based on hardcoded [f32; 3] size");
                
        // Verify appropriate offset calculations
        assert!(result_str.contains("offset : 0u64"), "First vertex attribute should have 0 offset");
        assert!(result_str.contains("offset : 8u64") || 
                result_str.contains("offset : (std :: mem :: size_of :: < [f32 ; 2] > ())"),
                "Second vertex attribute should have offset after [f32; 2]");
    }
}
