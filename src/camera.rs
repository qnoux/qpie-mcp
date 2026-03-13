use axum::Json;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct CameraResponse {
    pub file:String
}

pub async fn photo() -> Json<CameraResponse> {

    #[cfg(target_os="linux")]
    {
        let file="photo.jpg";

        Command::new("libcamera-still")
            .args(["-o",file])
            .output()
            .unwrap();

        Json(CameraResponse{file:file.into()})
    }

    #[cfg(not(target_os="linux"))]
    {
        Json(CameraResponse{file:"SIMULATED".into()})
    }
}