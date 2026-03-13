use axum::Json;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
pub struct WifiResponse {
    pub result:String
}

pub async fn scan() -> Json<WifiResponse> {

    #[cfg(target_os="linux")]
    {
        let out = Command::new("iw")
            .args(["dev","wlan0","scan"])
            .output()
            .unwrap();

        let result = String::from_utf8_lossy(&out.stdout).to_string();

        Json(WifiResponse{result})
    }

    #[cfg(not(target_os="linux"))]
    {
        Json(WifiResponse{result:"SIMULATED WIFI".into()})
    }
}