use axum::{ routing::get,  Router};
mod vehicle;

use vehicle::{vehicle_get, vehicle_post};

#[tokio::main]
async fn main() {
    //create a axam router
    let router01 = Router::new()
        .route("/vehicle", get(vehicle_get).post(vehicle_post)); // path and handler

    // define ip and port listener (tcp)
    let add = "0.0.0.0:6579";
    //lunch the server
    let listener = tokio::net::TcpListener::bind(add).await.unwrap();

    // run our app with hyper, listening globally on port 6579
    axum::serve(listener, router01).await.unwrap(); //axum crate. is set by the listener configuration
}
