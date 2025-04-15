use anyhow::Result;
use rustica_build::generate_shader_bindings;

fn main() -> Result<()> {
    let _ = generate_shader_bindings();
    Ok(())
}
