use tower_http::cors::CorsLayer;

pub fn cors_layer() -> CorsLayer {
    use tower_http::cors::CorsLayer;
    use axum::http::HeaderValue;

    let allowed_origins: Vec<&str> = vec![
        "http://localhost:5173",
        "http://localhost:3000",
        "https://5173-b7492d738b4a6566.monkeycode-ai.online",
        "https://3000-b7492d738b4a6566.monkeycode-ai.online",
    ];

    let origins: Vec<HeaderValue> = allowed_origins
        .iter()
        .map(|o| o.parse().unwrap())
        .collect();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
        .expose_headers(tower_http::cors::Any)
        .max_age(std::time::Duration::from_secs(86400))
}
