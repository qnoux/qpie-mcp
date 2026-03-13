use axum::{
    extract::Path,
    Json
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct GpioResponse {

    pub pin: u8,
    pub value: String,
}

#[derive(Deserialize)]
pub struct WriteRequest {

    pub pin: u8,
    pub value: String,
}

#[derive(Serialize)]
pub struct WriteResponse {

    pub pin: u8,
    pub status: String,
}

#[cfg(target_os = "linux")]
pub async fn read_pin(Path(pin): Path<u8>) -> Json<GpioResponse> {

    use rppal::gpio::Gpio;

    let gpio = Gpio::new().unwrap();

    let pin = gpio.get(pin).unwrap().into_input();

    let value = if pin.is_high() { "HIGH" } else { "LOW" };

    Json(GpioResponse {

        pin,
        value: value.to_string(),

    })
}

#[cfg(not(target_os = "linux"))]
pub async fn read_pin(Path(pin): Path<u8>) -> Json<GpioResponse> {

    Json(GpioResponse {

        pin,
        value: "SIMULATED".into(),

    })
}

#[cfg(target_os = "linux")]
pub async fn write_pin(Json(req): Json<WriteRequest>) -> Json<WriteResponse> {

    use rppal::gpio::Gpio;

    let gpio = Gpio::new().unwrap();

    let mut pin = gpio.get(req.pin).unwrap().into_output();

    match req.value.as_str() {

        "HIGH" => pin.set_high(),

        "LOW" => pin.set_low(),

        _ => {}

    }

    Json(WriteResponse {

        pin: req.pin,
        status: "ok".into(),

    })
}

#[cfg(not(target_os = "linux"))]
pub async fn write_pin(Json(req): Json<WriteRequest>) -> Json<WriteResponse> {

    Json(WriteResponse {

        pin: req.pin,
        status: "SIMULATED".into(),

    })
}