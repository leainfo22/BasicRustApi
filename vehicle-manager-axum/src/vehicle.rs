use axum::Json;
use axum::extract::Query;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}
#[derive(Deserialize)]
pub struct Customer {
    first_name: String,
    last_name: String,
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

pub async fn vehicle_post(
    Query(mut v): Query<Vehicle>,
    Query(c): Query<Customer>
) -> Json<Vehicle>
{
    println!("customer post: {0} {1}",c.first_name,c.last_name);
    println!("vehicle post: {0} {1} {2}",v.manufacturer,v.model,v.year);
    v.id=Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}