use axum::Json;
use serde::Serialize;
use sysinfo::{System, Disks};

#[derive(Serialize)]
pub struct CpuResponse {
    pub usage: f32,
}

pub async fn cpu() -> Json<CpuResponse> {

    let mut sys = System::new_all();

    sys.refresh_cpu();

    let usage = sys.global_cpu_info().cpu_usage();

    Json(CpuResponse { usage })
}

#[derive(Serialize)]
pub struct MemoryResponse {

    pub total: u64,
    pub used: u64,
    pub free: u64,
}

pub async fn memory() -> Json<MemoryResponse> {

    let mut sys = System::new_all();

    sys.refresh_memory();

    Json(MemoryResponse {

        total: sys.total_memory(),
        used: sys.used_memory(),
        free: sys.free_memory(),

    })
}

#[derive(Serialize)]
pub struct DiskResponse {

    pub name: String,
    pub total: u64,
    pub available: u64,
}

pub async fn disk() -> Json<Vec<DiskResponse>> {

    let disks = Disks::new_with_refreshed_list();

    let list = disks.iter().map(|d| {

        DiskResponse {

            name: d.name().to_string_lossy().to_string(),
            total: d.total_space(),
            available: d.available_space(),

        }

    }).collect();

    Json(list)
}