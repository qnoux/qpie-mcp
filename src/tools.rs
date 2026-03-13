use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
}

pub async fn list_tools() -> Json<Vec<Tool>> {

    Json(vec![

        Tool{ name:"cpu_usage".into(), description:"Get CPU usage".into() },

        Tool{ name:"memory_usage".into(), description:"Get memory usage".into() },

        Tool{ name:"disk_usage".into(), description:"Get disk usage".into() },

        Tool{ name:"gpio_read".into(), description:"Read GPIO pin".into() },

        Tool{ name:"gpio_write".into(), description:"Write GPIO pin".into() },

        Tool{ name:"temperature".into(), description:"Raspberry Pi temperature".into() },

        Tool{ name:"wifi_scan".into(), description:"Scan WiFi networks".into() },

        Tool{ name:"camera_photo".into(), description:"Capture camera image".into() },

        Tool{ name:"process_list".into(), description:"List running processes".into() },

        Tool{ name:"i2c_scan".into(), description:"Scan I2C devices".into() },

    ])
}

#[derive(Deserialize)]
pub struct ToolRequest {
    pub name:String
}

pub async fn call_tool() -> Json<serde_json::Value> {

    Json(serde_json::json!({"message":"call tool using REST endpoints"}))
}