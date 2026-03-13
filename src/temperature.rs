use axum::Json;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct TempResponse {
    pub temperature:String
}

pub async fn temperature() -> Json<TempResponse> {

    #[cfg(target_os="linux")]
    {
        let out = Command::new("vcgencmd")
            .arg("measure_temp")
            .output()
            .unwrap();

        let temp = String::from_utf8_lossy(&out.stdout).to_string();

        Json(TempResponse{temperature:temp})
    }

    #[cfg(not(target_os="linux"))]
    {
        Json(TempResponse{temperature:"SIMULATED".into()})
    }
}