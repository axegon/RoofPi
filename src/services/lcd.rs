use crate::utils::format_string;
use rppal::i2c::I2c;
use std::{thread, time};

pub const LCD_WIDTH: usize = 16;

pub const LCD_CHR: u8 = 1;
pub const LCD_CMD: u8 = 0;

pub const LCD_LINE_1: u8 = 0x80;
pub const LCD_LINE_2: u8 = 0xC0;

const LCD_BACKLIGHT: u8 = 0x08;
const ENABLE: u8 = 0b00000100;
const SLAVE_ADDR: u16 = 0x27;

const E_PULSE: u64 = 500;
const E_DELAY: u64 = 500;

pub(crate) struct LcdService {
    lcd: I2c,
    started: bool,
}

impl LcdService {
    pub fn new() -> LcdService {
        let mut i2c = match I2c::new() {
            Ok(i2c) => i2c,
            Err(e) => {
                println!("Error: {}", e);
                panic!("Can't create I2C interface");
            }
        };
        match i2c.set_slave_address(SLAVE_ADDR) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                panic!("Can't set I2C slave address");
            }
        }
        LcdService {
            lcd: i2c,
            started: false,
        }
    }

    fn lcd_byte(&mut self, bits: u8, mode: u8) {
        let bits_high = mode | (bits & 0xF0) | LCD_BACKLIGHT;
        let bits_low = mode | ((bits << 4) & 0xF0) | LCD_BACKLIGHT;

        self.lcd.write(&[bits_high]).unwrap();
        self.lcd_toggle_enable(bits_high);

        self.lcd.write(&[bits_low]).unwrap();
        self.lcd_toggle_enable(bits_low);
    }

    fn lcd_init(&mut self) {
        if !self.started {
            self.lcd_byte(0x33, LCD_CMD);
            self.lcd_byte(0x32, LCD_CMD);
            self.lcd_byte(0x06, LCD_CMD);
            self.lcd_byte(0x0C, LCD_CMD);
            self.lcd_byte(0x28, LCD_CMD);
            self.lcd_byte(0x01, LCD_CMD);
            thread::sleep(time::Duration::from_micros(E_DELAY));
            self.started = true;
        }
    }

    fn lcd_toggle_enable(&mut self, bits: u8) {
        thread::sleep(time::Duration::from_micros(E_DELAY));
        self.lcd.write(&[bits | ENABLE]).unwrap();
        thread::sleep(time::Duration::from_micros(E_PULSE));
        self.lcd.write(&[bits & !ENABLE]).unwrap();
        thread::sleep(time::Duration::from_micros(E_DELAY));
    }

    pub fn lcd_string(&mut self, message: &str, line: u8) {
        let message = format_string(message, LCD_WIDTH);

        self.lcd_byte(line, LCD_CMD);

        for byte in message.bytes() {
            self.lcd_byte(byte, LCD_CHR);
        }
    }

    pub(crate) fn write(&mut self, message: &str, line: u8) {
        self.lcd_init();
        self.lcd_string(message, line);
    }
}
