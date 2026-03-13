use axum::Json;
use serde::Serialize;
use sysinfo::System;

#[derive(Serialize)]
pub struct Proc {
    pub name:String
}

pub async fn list() -> Json<Vec<Proc>> {

    let sys = System::new_all();

    let procs = sys.processes()
        .values()
        .map(|p| Proc{name:p.name().into()})
        .collect();

    Json(procs)
}