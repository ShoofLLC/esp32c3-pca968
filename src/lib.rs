use esp_idf_svc::hal::i2c::I2cDriver;
use esp_idf_svc::sys::TickType_t;

pub struct Driver<'b> {
    m_i2c_driver: I2cDriver<'b>,
    device_address: u8,
    default_timeout: TickType_t,
}

pub struct Motor {
    port: u8,
    angle: f32,
}

impl Motor {
    pub const MAX_ANGLE:f32 = 120.0;
    pub const MIN_VALUE:f32 = 0x0074 as f32;
    pub const MAX_VALUE:f32 = 0x01A8 as f32;
    pub fn set_angle(&mut self, mut angle: f32, driver: &mut Driver) {
        if angle > Self::MAX_ANGLE {
            log::warn!("Input to set_angle exceeded maximum angle on port {}", self.port.clone());
            angle = Self::MAX_ANGLE;
        }
        let off_value = (angle*((Self::MAX_VALUE-Self::MIN_VALUE)/Self::MAX_ANGLE) + Self::MIN_VALUE) as u16;
        driver.write_led_on_register(self.port, 0);
        driver.write_led_off_register(self.port, off_value);
        log::info!("set angle to {}", angle.clone());
        self.angle = angle;
    }

    pub fn new(port: u8, angle: f32) -> Motor {
        return Motor{port, angle};
    }
}

impl<'b> Driver<'b> {
    pub fn write_register(&mut self, register: u8, value: u8, timeout: Option<TickType_t>) -> () {
        let _write_result = match self.m_i2c_driver.write(self.device_address, &[register, value], timeout.unwrap_or(self.default_timeout)){
            Ok(_write_result) => {
                log::info!("successfully wrote register {}", register);
                return _write_result
            },
            Err(err) => {
                log::info!("didn't work {}", err);
                return ()
            },
        };
    }

    pub fn write_led_on_register(&mut self, port: u8, value: u16){
        self.write_register((port*4)+0x06, value.clone() as u8, None);
        log::info!("writing value {} to register {}", value.clone() as u8, port+0x06);
        self.write_register((port*4)+0x07, ((value>>8) & 0x0F) as u8, None);
        log::info!("writing value {} to register {}", ((value>>8) & 0x0F) as u8, port+0x07);
    }
    
    pub fn write_led_off_register(&mut self, port: u8, value: u16){
        self.write_register((port*4)+0x08, value.clone() as u8, None);
        log::info!("writing value {} to register {}", value.clone() as u8, port+0x08);
        self.write_register((port*4)+0x09, ((value>>8) & 0x0F) as u8, None);
        log::info!("writing value {} to register {}", ((value>>8) & 0x0F) as u8, port+0x09);
    }

    pub fn write_prescale_value(&mut self, value: u8){
        let _ = self.write_register(0x00, 0x11, None);
        // write prescale value
        let _ = self.write_register(0xFE, value, None);
        // switch to normal mode 
        let _ = self.write_register(0x00, 0x01, None);
    }

    pub fn new(m_i2c_driver: I2cDriver<'b>, device_address: u8, default_timeout: TickType_t) -> Driver<'b> {
        return Driver{m_i2c_driver, device_address, default_timeout}
    }

}
    

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
