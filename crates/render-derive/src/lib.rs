use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type, Meta, Lit, Expr, ExprLit};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::spanned::Spanned;

/// Derive macro for implementing the Vertex trait
#[proc_macro_derive(Vertex, attributes(vertex))]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the struct
    let name = &input.ident;
    
    // Check if the input is a struct
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => {
                    return syn::Error::new(
                        input.span(),
                        "Vertex can only be derived for structs with named fields",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        _ => {
            return syn::Error::new(
                input.span(),
                "Vertex can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Collect field information
    let mut vertex_attributes = Vec::new();
    let mut next_location = 0;
    
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        
        // Default values
        let mut location = None;
        let mut format = None;
        let mut skip = false;
        
        // Parse vertex attributes
        for attr in &field.attrs {
            if attr.path().is_ident("vertex") {
                let attr_parser = attr.parse_args_with(
                    Punctuated::<Meta, Comma>::parse_terminated
                ).ok();
                
                if let Some(attr_args) = attr_parser {
                    for meta in attr_args {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("location") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = &nv.value {
                                    location = Some(lit.base10_parse::<u32>().unwrap());
                                }
                            } else if nv.path.is_ident("format") {
                                if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = &nv.value {
                                    format = Some(lit.value());
                                }
                            }
                        } else if let Meta::Path(path) = meta {
                            if path.is_ident("skip") {
                                skip = true;
                            }
                        }
                    }
                }
            }
        }
        
        // Skip this field if marked with skip
        if skip {
            continue;
        }
        
        // Auto-assign location if not specified
        if location.is_none() {
            location = Some(next_location);
            next_location += 1;
        } else {
            // Update next_location if this one is higher
            next_location = location.unwrap() + 1;
        }
        
        // Add to the list (we'll determine format later if not specified)
        vertex_attributes.push((field_name, field_type, location, format));
    }
    
    // Sort by location if specified
    vertex_attributes.sort_by_key(|(_, _, location, _)| *location);
    
    // Generate the vertex buffer layout implementation
    let mut attributes = Vec::new();
    let mut offset = 0;
    
    for (_field_name, field_type, location, format_str) in &vertex_attributes {
        let location = location.unwrap_or_else(|| attributes.len() as u32);
        
        // Determine format based on field type or explicit format
        let format = if let Some(format) = format_str {
            quote! { wgpu::VertexFormat::#format }
        } else {
            // Default format based on field type
            match field_type {
                Type::Array(array) => {
                    if let syn::Expr::Lit(syn::ExprLit { lit: Lit::Int(size), .. }) = &array.len {
                        let size_str = size.base10_digits();
                        let size: usize = size_str.parse().unwrap_or(0);
                        
                        if let Type::Path(path) = &*array.elem {
                            let type_str = quote! { #path }.to_string();
                            if type_str.contains("f32") {
                                match size {
                                    2 => quote! { wgpu::VertexFormat::Float32x2 },
                                    3 => quote! { wgpu::VertexFormat::Float32x3 },
                                    4 => quote! { wgpu::VertexFormat::Float32x4 },
                                    _ => quote! { wgpu::VertexFormat::Float32 },
                                }
                            } else if type_str.contains("u32") {
                                match size {
                                    2 => quote! { wgpu::VertexFormat::Uint32x2 },
                                    3 => quote! { wgpu::VertexFormat::Uint32x3 },
                                    4 => quote! { wgpu::VertexFormat::Uint32x4 },
                                    _ => quote! { wgpu::VertexFormat::Uint32 },
                                }
                            } else {
                                quote! { wgpu::VertexFormat::Float32 }
                            }
                        } else {
                            quote! { wgpu::VertexFormat::Float32 }
                        }
                    } else {
                        quote! { wgpu::VertexFormat::Float32 }
                    }
                },
                _ => quote! { wgpu::VertexFormat::Float32 },
            }
        };
        
        // Add attribute
        attributes.push(quote! {
            wgpu::VertexAttribute {
                offset: #offset,
                shader_location: #location,
                format: #format,
            }
        });
        
        // Update offset based on format size
        let format_size = match format.to_string().as_str() {
            "wgpu :: VertexFormat :: Float32" => 4u64,
            "wgpu :: VertexFormat :: Float32x2" => 8u64,
            "wgpu :: VertexFormat :: Float32x3" => 12u64,
            "wgpu :: VertexFormat :: Float32x4" => 16u64,
            "wgpu :: VertexFormat :: Uint32" => 4u64,
            "wgpu :: VertexFormat :: Uint32x2" => 8u64,
            "wgpu :: VertexFormat :: Uint32x3" => 12u64,
            "wgpu :: VertexFormat :: Uint32x4" => 16u64,
            _ => 4u64, // Default to 4 bytes
        };
        
        offset += format_size;
    }
    
    // Generate the implementation
    let expanded = quote! {
        impl rustica_render::Vertex for #name {
            fn layout() -> wgpu::VertexBufferLayout<'static> {
                use std::mem;
                
                static ATTRIBUTES: &[wgpu::VertexAttribute] = &[
                    #(#attributes),*
                ];
                
                wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: ATTRIBUTES,
                }
            }
        }
        
        // Automatically implement Pod and Zeroable
        unsafe impl bytemuck::Pod for #name {}
        unsafe impl bytemuck::Zeroable for #name {}
    };
    
    // Return the generated implementation
    TokenStream::from(expanded)
}

/// Derive macro for creating a ShaderDescriptor
#[proc_macro_derive(ShaderDescriptor, attributes(shader, vertex_type, uniform))]
pub fn derive_shader_descriptor(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    
    // Get the name of the struct
    let name = &input.ident;
    
    // Check if the input is a struct
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => &fields.named,
                _ => {
                    return syn::Error::new(
                        input.span(),
                        "ShaderDescriptor can only be derived for structs with named fields",
                    )
                    .to_compile_error()
                    .into();
                }
            }
        }
        _ => {
            return syn::Error::new(
                input.span(),
                "ShaderDescriptor can only be derived for structs",
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Extract shader source path from attributes
    let mut shader_source_path = None;
    let mut shader_source_inline = None;
    
    for attr in &input.attrs {
        if attr.path().is_ident("shader") {
            let attr_parser = attr.parse_args_with(
                Punctuated::<Meta, Comma>::parse_terminated
            ).ok();
            
            if let Some(attr_args) = attr_parser {
                for meta in attr_args {
                    if let Meta::NameValue(nv) = meta {
                        if nv.path.is_ident("source") {
                            if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = &nv.value {
                                shader_source_path = Some(lit.value());
                            }
                        } else if nv.path.is_ident("inline") {
                            if let Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) = &nv.value {
                                shader_source_inline = Some(lit.value());
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Find vertex type field
    let mut vertex_type_field = None;
    let mut uniform_fields = Vec::new();
    
    for field in fields {
        let has_vertex_type_attr = field.attrs.iter().any(|attr| attr.path().is_ident("vertex_type"));
        let has_uniform_attr = field.attrs.iter().any(|attr| attr.path().is_ident("uniform"));
        
        if has_vertex_type_attr {
            if vertex_type_field.is_some() {
                return syn::Error::new(
                    field.span(),
                    "Only one field can be marked with #[vertex_type]",
                )
                .to_compile_error()
                .into();
            }
            vertex_type_field = Some(field);
        } else if has_uniform_attr {
            uniform_fields.push(field);
        }
    }
    
    // Ensure we have a vertex type field
    let vertex_type_field = match vertex_type_field {
        Some(field) => field,
        None => {
            return syn::Error::new(
                input.span(),
                "ShaderDescriptor requires a field marked with #[vertex_type]",
            )
            .to_compile_error()
            .into();
        }
    };
    
    // Extract vertex type
    let vertex_type = &vertex_type_field.ty;
    
    // Process uniform fields
    let mut uniform_params = Vec::new();
    
    for field in &uniform_fields {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let field_type = &field.ty;
        
        // Default binding
        let mut binding = None;
        
        // Parse uniform attributes
        for attr in &field.attrs {
            if attr.path().is_ident("uniform") {
                let attr_parser = attr.parse_args_with(
                    Punctuated::<Meta, Comma>::parse_terminated
                ).ok();
                
                if let Some(attr_args) = attr_parser {
                    for meta in attr_args {
                        if let Meta::NameValue(nv) = meta {
                            if nv.path.is_ident("binding") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(lit), .. }) = &nv.value {
                                    binding = Some(lit.base10_parse::<u32>().unwrap());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Auto-assign binding if not specified
        let binding = binding.unwrap_or_else(|| uniform_params.len() as u32);
        
        // Determine size based on field type
        let size_expr = quote! { std::mem::size_of::<#field_type>() as u64 };
        
        uniform_params.push((field_name, binding, size_expr));
    }
    
    // Generate code for loading shader source
    let shader_source_code = if let Some(path) = shader_source_path {
        quote! {
            std::fs::read_to_string(#path).expect(&format!("Failed to read shader source from {}", #path))
        }
    } else if let Some(source) = shader_source_inline {
        quote! {
            String::from(#source)
        }
    } else {
        // Default to looking for a shader with the same name as the struct
        let shader_name = format!("{}.wgsl", name.to_string().to_lowercase());
        quote! {
            std::fs::read_to_string(&format!("./src/shaders/{}", #shader_name))
                .or_else(|_| std::fs::read_to_string(&format!("./shaders/{}", #shader_name)))
                .expect(&format!("Failed to find shader file {}", #shader_name))
        }
    };
    
    // Generate the implementation
    let _descriptor_name = format_ident!("{}Descriptor", name);
    
    let uniform_param_exprs = uniform_params.iter().map(|(name, binding, size)| {
        quote! {
            rustica_render::UniformParameter {
                name: #name.to_string(),
                binding: #binding,
                size: #size,
            }
        }
    });
    
    // Generate field names for dummy usage - adding underscore to mark as intentionally unused
    let _field_names = fields.iter().filter_map(|f| f.ident.as_ref()).collect::<Vec<_>>();
    
    let expanded = quote! {
        #[allow(dead_code)]
        impl #name {
            /// Create a ShaderDescriptor for this shader
            pub fn descriptor() -> rustica_render::ShaderDescriptor {
                // No need for a dummy method, we'll just add #[allow(dead_code)] to the struct
                
                // Get vertex attributes from the vertex type
                let vertex_attributes = <#vertex_type as rustica_render::VertexAttributeProvider>::attributes();
                
                // Create the shader descriptor
                rustica_render::ShaderDescriptor {
                    name: stringify!(#name).to_string(),
                    shader_source: #shader_source_code,
                    vertex_attributes,
                    uniforms: vec![
                        #(#uniform_param_exprs),*
                    ],
                }
            }
        }
    };
    
    // Return the generated implementation
    TokenStream::from(expanded)
}
