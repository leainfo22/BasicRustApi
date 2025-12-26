use axum::{debug_handler, routing::get, Json, Router};

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
#[derive(serde::Serialize)]
struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: String
}
#[debug_handler]
async fn vehicle_get() -> Json<Vehicle>{
    println!("vehicle get");
    Json::from(Vehicle{
        manufacturer: "Mazda".to_string(),
        model: "CX-3".to_string(),
        year: 2018,
        id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn vehicle_post(){}