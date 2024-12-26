use crate::services::lcd::{LcdService, LCD_LINE_1, LCD_LINE_2};
use crate::services::network::Network;
use crate::services::system_resources::SystemResources;
use std::{thread, time};

pub struct RoofPiService {
    lcd: LcdService,
    cpu: SystemResources,
}

impl RoofPiService {
    pub fn new() -> RoofPiService {
        RoofPiService {
            lcd: LcdService::new(),
            cpu: SystemResources::new(),
        }
    }

    /// Get the IP address of the device.
    /// # Returns
    /// A string representation of the IP address.
    fn get_ip_address() -> String {
        Network::get_ip_address()
    }

    pub(crate) fn run(&mut self) {
        loop {
            let ip = RoofPiService::get_ip_address();
            let cpu_load = self.cpu.get_line();
            self.lcd.write(&ip, LCD_LINE_1);
            self.lcd.write(&cpu_load, LCD_LINE_2);
            thread::sleep(time::Duration::from_secs(3));
        }
    }
}
