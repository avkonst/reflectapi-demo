use std::env;
use std::error::Error;

use axum::{response::Html, Json};
use reflectapi::{serde_json, Schema};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // prepare the app router and schema
    let builder = reflectapi_demo::builder();
    let (schema, routers) = builder.build()?;

    // If this is a codegen run, exit after saving the schema
    if env::args().any(|arg| arg == "--codegen") {
        save_schema_if_changed(&schema).await?;
        return Ok(());
    }

    // capture the spec for online documentation
    let openapi_spec = reflectapi::codegen::openapi::Spec::from(&schema);

    // start the server based on axum web framework
    let app_state = Default::default();
    let axum_app = reflectapi::axum::into_router(app_state, routers, |_name, r| r)
        // serve two custom extra routes for rendering online documentation
        .route(
            "/openapi.json",
            axum::routing::get(|| async { Json(openapi_spec) }),
        )
        .route(
            "/doc",
            axum::routing::get(|| async { Html(include_str!("./redoc.html")) }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    eprintln!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, axum_app).await?;

    Ok(())
}

async fn save_schema_if_changed(schema: &Schema) -> Result<(), Box<dyn Error>> {
    let file_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "reflectapi.json");

    // Try to read existing content, handle case where file doesn't exist
    let existing_schema = match tokio::fs::read_to_string(&file_path).await {
        Ok(content) => content,
        Err(_) => String::new(), // File doesn't exist, treat as empty
    };

    let schema = serde_json::to_string_pretty(schema)?;
    if existing_schema != schema {
        tokio::fs::write(&file_path, schema).await?;
        println!("Reflectapi schema written to {}", file_path);
    } else {
        println!("Reflectapi schema is up to date");
    }
    Ok(())
}
