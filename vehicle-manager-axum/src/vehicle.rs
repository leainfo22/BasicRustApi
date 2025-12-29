use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}

pub async fn vehicle_get() -> Json<Vehicle>{
    println!("vehicle get");
    Json::from(Vehicle{
        manufacturer: "Mazda".to_string(),
        model: "CX-3".to_string(),
        year: 2018,
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle>
{
    println!("vehicle post: {0} {1} {2}",v.manufacturer,v.model,v.year);
    v.id=Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}