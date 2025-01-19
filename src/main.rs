mod login;

// use axum::{
//     routing::{get, post},
//     Router
// };
// use axum::handler::Handler;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::login::{get_login_cookie, get_session_cookie};

#[tokio::main]
async fn main() {
    let cookies = get_session_cookie().await;
    println!("{}", cookies.0);
    println!("{}", cookies.1);
    println!("{}", cookies.2);
    let auth = get_login_cookie(cookies, "", "");
    println!("{}", auth.await);
    // tracing_subscriber::registry()
    //     .with(tracing_subscriber::EnvFilter::new(
    //         std::env::var("RUST_LOG").unwrap_or_else(|_| "hacapi=debug".into())
    //     ))
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();
    // let app = Router::new()
    //     .route("/", get(|| async { "hello, world" }));
    // let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    // tracing::debug!("Listening on {}" ,addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Failed to start server")
}
