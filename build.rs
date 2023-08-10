use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/zkp_auth.proto"], &["proto"])?;

    // Get the OUT_DIR environment variable
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");

    // Define the destination path for the generated files
    let dest_path = Path::new("generated");
    if !dest_path.exists() {
        fs::create_dir_all(&dest_path)?;
    }

    // Copy the generated files
    for entry in fs::read_dir(Path::new(&out_dir))? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            fs::copy(
                entry.path(),
                dest_path.join(entry.path().file_name().unwrap()),
            )?;
        }
    }

    Ok(())
}
