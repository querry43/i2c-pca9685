//! PWM 16 channel controller i2cbus
//! Data sheet https://cdn-shop.adafruit.com/datasheets/PCA9685.pdf
//!

extern crate i2cdev;

use i2cdev::core::I2CDevice;
use std::{thread, time};

// const DEFAULT_PCA9685_ADDRESS: u8 = 0x40;
const MODE1: u8 = 0x00;
const MODE2: u8 = 0x01;
// const SUBADR1: u8 = 0x02;
// const SUBADR2: u8 = 0x03;
// const SUBADR3: u8 = 0x04;
const PRESCALE: u8 = 0xFE;
const LED0_ON_L: u8 = 0x06;
const LED0_ON_H: u8 = 0x07;
const LED0_OFF_L: u8 = 0x08;
const LED0_OFF_H: u8 = 0x09;
const ALL_LED_ON_L: u8 = 0xFA;
const ALL_LED_ON_H: u8 = 0xFB;
const ALL_LED_OFF_L: u8 = 0xFC;
const ALL_LED_OFF_H: u8 = 0xFD;
const RESTART: u16 = 0x80;
const SLEEP: u8 = 0x10;
const ALLCALL: u16 = 0x01;
// const INVRT: u8 = 0x10;
const OUTDRV: u16 = 0x04;
// const SWRESET: u8 = 0x06;
// const ALLDEV: u8 = 0x00;

fn sleep_5ms() {
    let five_millis = time::Duration::from_millis(10);
    thread::sleep(five_millis);
}

pub struct PCA9685<T: I2CDevice + Sized> {
    i2cdev: T,
}

impl<T> PCA9685<T>
    where T: I2CDevice + Sized
{
    #[allow(unused_must_use)]
    pub fn new(mut i2cdev: T) -> Result<PCA9685<T>, T::Error> {
        // self.set_all_pwm(0, 0)
        i2cdev.smbus_write_word_data(MODE2, OUTDRV)?;
        i2cdev.smbus_write_word_data(MODE1, ALLCALL)?;
        sleep_5ms();  // wait for oscillator
        let mut mode1 = i2cdev.smbus_read_byte_data(MODE1)?;
        mode1 = mode1 & SLEEP;  // wake up
        i2cdev.smbus_write_word_data(MODE1, mode1 as u16)?;
        sleep_5ms(); // wait for oscillator
        Ok(PCA9685 { i2cdev: i2cdev })
    }
    // 60 is a decent value for servos
    #[allow(unused_must_use)]
    pub fn set_pwm_freq(&mut self, freq: f32) -> Result<(), T::Error> {
        // Set the PWM frequency to the provided value in hertz.
        let mut prescaleval = 25000000.0; // 25MHz
        prescaleval /= 4096.0; // 12-bit
        prescaleval /= freq;
        prescaleval -= 1.0;
        let prescale: u16 = (prescaleval + 0.5).floor() as u16;
        let oldmode = self.i2cdev.smbus_read_word_data(MODE1)?;
        let newmode = (oldmode & 0x7F) | SLEEP as u16;    // sleep
        self.i2cdev.smbus_write_word_data(MODE1, newmode)?;  // go to sleep
        self.i2cdev.smbus_write_word_data(PRESCALE, prescale)?;
        self.i2cdev.smbus_write_word_data(MODE1, oldmode)?;
        sleep_5ms();
        self.i2cdev.smbus_write_word_data(MODE1, oldmode | RESTART)?;
        Ok(())
    }

    #[allow(unused_must_use)]
    pub fn set_pwm(&mut self, channel: u8, on: u16, off: u16) -> Result<(), T::Error> {
        // Sets a single PWM channel.
        self.i2cdev.smbus_write_word_data(LED0_ON_L + 4 * channel, on & 0xFF)?;
        self.i2cdev.smbus_write_word_data(LED0_ON_H + 4 * channel, on >> 8)?;
        self.i2cdev.smbus_write_word_data(LED0_OFF_L + 4 * channel, off & 0xFF)?;
        self.i2cdev.smbus_write_word_data(LED0_OFF_H + 4 * channel, off >> 8)?;
        Ok(())
    }

    #[allow(unused_must_use)]
    pub fn set_all_pwm(&mut self, on: u16, off: u16) -> Result<(), T::Error> {
        // Sets all PWM channels.
        self.i2cdev.smbus_write_word_data(ALL_LED_ON_L, on & 0xFF)?;
        self.i2cdev.smbus_write_word_data(ALL_LED_ON_H, on >> 8)?;
        self.i2cdev.smbus_write_word_data(ALL_LED_OFF_L, off & 0xFF)?;
        self.i2cdev.smbus_write_word_data(ALL_LED_OFF_H, off >> 8)?;
        Ok(())
    }

    #[allow(unused_must_use)]
    pub fn reset_all_servos(&mut self) -> Result<(), T::Error> {
        self.i2cdev.smbus_write_byte(0x00)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}