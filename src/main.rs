mod tools;
mod system;
mod gpio;
mod temperature;
mod wifi;
mod camera;
mod process;
mod i2c;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/tools", get(tools::list_tools))
        .route("/call", post(tools::call_tool))

        // system
        .route("/cpu", get(system::cpu))
        .route("/memory", get(system::memory))
        .route("/disk", get(system::disk))

        // hardware
        .route("/gpio/:pin", get(gpio::read_pin))
        .route("/gpio/write", post(gpio::write_pin))

        .route("/temperature", get(temperature::temperature))
        .route("/wifi/scan", get(wifi::scan))

        .route("/camera/photo", get(camera::photo))

        .route("/process/list", get(process::list))

        .route("/i2c/scan", get(i2c::scan));

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("QPie MCP running on 8080");

    axum::serve(listener, app).await.unwrap();
}