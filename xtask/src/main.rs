use std::process::Command;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: cargo xtask <command>");
        eprintln!("Commands:");
        eprintln!("  build-with-clients    Build app and generate clients");
        return Ok(());
    }

    match args[1].as_str() {
        "build-with-clients" => build_with_clients()?,
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}

fn build_with_clients() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Building reflectapi-demo...");
    
    // Build the main application  
    let project_dir = std::env::current_dir()?;
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&project_dir)
        .status()?;
    
    if !status.success() {
        eprintln!("❌ Build failed");
        return Ok(());
    }
    
    println!("✅ Build complete");
    println!("🚀 Running app to generate schema...");
    
    // Run the app briefly to generate reflectapi.json
    let binary_path = project_dir.join("target/release/reflectapi-demo");
    let mut child = Command::new(&binary_path)
        .current_dir(&project_dir)
        .spawn()?;
    
    // Give it a moment to start up and generate the schema
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Kill the process
    let _ = child.kill();
    let _ = child.wait();
    
    // Check if schema was generated
    let schema_path = project_dir.join("reflectapi.json");
    if !schema_path.exists() {
        eprintln!("❌ Schema file not generated");
        return Ok(());
    }
    
    println!("✅ Schema generated");
    println!("📦 Generating clients...");
    
    // Generate TypeScript client
    let reflectapi_path = std::env::var("REFLECTAPI_PATH")
        .unwrap_or_else(|_| "/mnt/fast-dev/dev/reflectapi".to_string());
    
    let status = Command::new("cargo")
        .args([
            "run", "--bin", "reflectapi", "--",
            "codegen", 
            "--language", "typescript",
            "--schema", &schema_path.to_string_lossy(),
            "--output", &project_dir.join("client").to_string_lossy(),
        ])
        .current_dir(&reflectapi_path)
        .status()?;
    
    if status.success() {
        println!("✅ TypeScript client generated");
    } else {
        eprintln!("❌ Failed to generate TypeScript client");
    }
    
    println!("🎉 All done! Run your app with: cargo run --release");
    
    Ok(())
}