use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct I2CDevice {
    pub address:u16
}

pub async fn scan() -> Json<Vec<I2CDevice>> {

    #[cfg(target_os="linux")]
    {
        use rppal::i2c::I2c;

        let mut devices=Vec::new();

        for addr in 0..128 {

            if let Ok(mut i2c)=I2c::new() {

                if i2c.set_slave_address(addr).is_ok() {
                    devices.push(I2CDevice{address:addr});
                }

            }

        }

        Json(devices)
    }

    #[cfg(not(target_os="linux"))]
    {
        Json(vec![I2CDevice{address:0x40}])
    }
}