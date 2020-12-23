#![no_std]

mod register;

#[cfg(feature = "rttdebug")]
use panic_rtt_core::rtprintln;

use embedded_hal as hal;
// use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{WriteRead,Write};

use register::Register;

/// Operating mode for the LSM6DS33, determining which of the Gyroscope and/or Accelerometer are active
#[derive(Debug)]
pub enum OperatingMode {
    AccelerometerOnly,
    GyroscopeOnly,
    Combination,
}

/// The Gyroscope operating mode which determines power consumption, speed, etc...
#[derive(Debug)]
pub enum GyroscopePowerMode {
    LowPower,
    NormalPower,
    HighPerformance,
}

/// The Accelerometer operating mode which determines power consumption, speed, etc... 
#[derive(Debug)]
pub enum AccelerometerPowerMode {
    LowPower,
    NormalPower,
    HighPerformance,
}

#[derive(Debug)]
pub enum Error {
    CommunicationError,
    PinError,
    UnknownChipId,
}

fn i2c_error<E>(_: E) -> Error {
    Error::CommunicationError
}


#[allow(dead_code)]
pub struct LSM6DS33<I2C> {
    i2c: I2C,
    address: u8,
}

#[allow(dead_code)]
impl<I2C,E> LSM6DS33<I2C> where I2C: WriteRead<Error=E> + Write<Error=E>,
{
    /// Create a new driver instance with a specified I2C port
    pub fn new (i2c: I2C) -> Result<Self,Error> {
        let lsm6ds33 = LSM6DS33 {
            i2c,
            address: 0x6a
        };
        Ok(lsm6ds33)
    }

    pub fn who_am_i(&mut self) -> Result<u8,Error> {
        self.register_read(Register::WHO_AM_I)
    }

    fn register_read(&mut self, register: Register) -> Result<u8, Error> {
        let mut buffer = [0];
        self.i2c.write_read(self.address, &[register.address()], &mut buffer).map_err(i2c_error)?;
        Ok(buffer[0])
    }
}
