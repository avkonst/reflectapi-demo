use std::error::Error;

use axum::{response::Html, Json};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // prepare the app router and schema
    let builder = reflectapi_demo::builder();
    let (schema, routers) = builder.build()?;

    // capture the spec for online documentation
    let openapi_spec = reflectapi::codegen::openapi::Spec::from(&schema);

    // generate typescript code
    let ts_code: String = reflectapi::codegen::typescript::generate(
        schema,
        &reflectapi::codegen::typescript::Config::default().format(true),
    )?;
    save_generated_ts_if_changed(&ts_code).await?;

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

async fn save_generated_ts_if_changed(ts_code: &str) -> Result<(), Box<dyn Error>> {
    let existing_ts_code = tokio::fs::read_to_string(format!(
        "{}/client/{}",
        env!("CARGO_MANIFEST_DIR"),
        "generated.ts"
    ))
    .await?;
    if existing_ts_code != ts_code {
        tokio::fs::write(
            format!("{}/client/{}", env!("CARGO_MANIFEST_DIR"), "generated.ts"),
            ts_code,
        )
        .await?;
    }
    Ok(())
}
