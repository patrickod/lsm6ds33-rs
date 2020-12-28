#![no_std]

mod accelerometer;
mod device;
mod gyroscope;
mod register;

use core::convert::From;
use embedded_hal as hal;
use hal::blocking::i2c;

use accelerometer::*;
use device::{Device, I2CDevice};
use gyroscope::*;
use register::*;

pub enum DataRate {
    ODR12_5Hz,
    ODR26Hz,
    ODR52Hz,
    ODR104Hz,
    ODR208Hz,
    ODR416Hz,
    ODR833Hz,
    ODR1_66Khz,
    ODR3_33Khz,
    ODR6_66Khz,
}

#[allow(dead_code)]
pub struct LSM6DS33<DEV> {
    dev: DEV,
    accelerometer_power_mode: AccelerometerPowerMode,
    accelerometer_scale: AccelerometerScale,
    accelerometer_data_rate: DataRate,
    gyroscope_data_rate: DataRate,
    gyroscope_scale: GyroscopeScale,
}

pub const LSM6D33_CHIP_ID: u8 = 0x69;

#[derive(Debug)]
pub enum Error<E> {
    CommunicationError,
    PinError,
    UnknownChipId,
    BusError(E),
}

impl<E> From<E> for Error<E> {
    fn from(err: E) -> Self {
        Error::BusError(err)
    }
}

impl<E, I2C> LSM6DS33<I2CDevice<I2C>>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Result<Self, Error<Error<E>>> {
        let dev = I2CDevice::new(i2c, address);
        Self::new_imu(dev)
    }
}

impl<E, DEV> LSM6DS33<DEV>
where
    DEV: Device<Error = E>,
{
    pub fn new_imu(dev: DEV) -> Result<Self, Error<E>> {
        let mut lsm6ds33 = LSM6DS33 {
            dev,
            accelerometer_power_mode: AccelerometerPowerMode::NormalPower,
            accelerometer_data_rate: DataRate::ODR12_5Hz,
            accelerometer_scale: AccelerometerScale::Scale2g,
            gyroscope_data_rate: DataRate::ODR12_5Hz,
            gyroscope_scale: GyroscopeScale::Scale250Dps,
        };

        match lsm6ds33.who_am_i() {
            Ok(id) => {
                if id != LSM6D33_CHIP_ID {
                    return Err(Error::UnknownChipId);
                }
            }
            Err(_) => return Err(Error::CommunicationError),
        }

        lsm6ds33.initialize()?;
        Ok(lsm6ds33)
    }

    pub fn who_am_i(&mut self) -> Result<u8, E> {
        self.dev.read(Register::WHO_AM_I)
    }

    fn soft_reset(&mut self) -> Result<(), E> {
        self.dev
            .mutate(Register::CTRL3_C, |r| r | CTRL3_CBits::SW_RESET.bits())?;

        // Await boot signaled by SW_RESET returning to 0
        loop {
            let ctrl = self.dev.read(Register::CTRL3_C)?;
            let flags = CTRL3_CBits::from_bits_truncate(ctrl);

            if !flags.contains(CTRL3_CBits::SW_RESET) {
                return Ok(());
            }
        }
    }

    fn initialize(&mut self) -> Result<(), E> {
        // Reset device to initial configuration
        self.soft_reset()?;

        // Enable Block Data Update operation (vs FIFO modes)
        self.dev
            .mutate(Register::CTRL3_C, |r| r | CTRL3_CBits::BDU.bits())?;

        self.set_accelerometer_data_rate(DataRate::ODR12_5Hz)?;
        self.set_accelerometer_scale(AccelerometerScale::Scale2g)?;

        self.set_gyroscope_data_rate(DataRate::ODR12_5Hz)?;
        self.set_gyroscope_scale(GyroscopeScale::Scale125Dps)?;

        Ok(())
    }

    /// Specify the Ouput Data Rate of the Accelerometer
    pub fn set_accelerometer_data_rate(&mut self, rate: DataRate) -> Result<(), E> {
        let flag = match rate {
            DataRate::ODR12_5Hz => CTRL1_XLBits::ODR12_5_HZ,
            DataRate::ODR26Hz => CTRL1_XLBits::ODR26_HZ,
            DataRate::ODR52Hz => CTRL1_XLBits::ODR52_HZ,
            DataRate::ODR104Hz => CTRL1_XLBits::ODR104_HZ,
            DataRate::ODR208Hz => CTRL1_XLBits::ODR208_HZ,
            DataRate::ODR416Hz => CTRL1_XLBits::ODR416_HZ,
            DataRate::ODR833Hz => CTRL1_XLBits::ODR833_HZ,
            DataRate::ODR1_66Khz => CTRL1_XLBits::ODR1_66_KHZ,
            DataRate::ODR3_33Khz => CTRL1_XLBits::ODR3_33_KHZ,
            DataRate::ODR6_66Khz => CTRL1_XLBits::ODR6_66_KHZ,
        };
        self.dev
            .mutate(Register::CTRL1_XL, |r| (r & !0b1111_0000) | flag.bits())
    }

    /// Specify the Accelerometer measurement scale
    pub fn set_accelerometer_scale(&mut self, scale: AccelerometerScale) -> Result<(), E> {
        let flag = match scale {
            AccelerometerScale::Scale2g => CTRL1_XLBits::SCALE2_G,
            AccelerometerScale::Scale4g => CTRL1_XLBits::SCALE4_G,
            AccelerometerScale::Scale8g => CTRL1_XLBits::SCALE8_G,
            AccelerometerScale::Scale16g => CTRL1_XLBits::SCALE16_G,
        };
        self.dev
            .mutate(Register::CTRL1_XL, |r| (r & !0b0000_1100) | flag.bits())
    }

    /// Specify the Output Data Rate for the Gyroscope
    pub fn set_gyroscope_data_rate(&mut self, rate: DataRate) -> Result<(), E> {
        let flag = match rate {
            DataRate::ODR12_5Hz => CTRL2_GBits::ODR12_5_HZ,
            DataRate::ODR26Hz => CTRL2_GBits::ODR26_HZ,
            DataRate::ODR52Hz => CTRL2_GBits::ODR52_HZ,
            DataRate::ODR104Hz => CTRL2_GBits::ODR104_HZ,
            DataRate::ODR208Hz => CTRL2_GBits::ODR208_HZ,
            DataRate::ODR416Hz => CTRL2_GBits::ODR416_HZ,
            DataRate::ODR833Hz => CTRL2_GBits::ODR833_HZ,
            DataRate::ODR1_66Khz => CTRL2_GBits::ODR1_66_KHZ,
            DataRate::ODR3_33Khz => CTRL2_GBits::ODR3_33_KHZ,
            DataRate::ODR6_66Khz => CTRL2_GBits::ODR6_66_KHZ,
        };
        self.dev
            .mutate(Register::CTRL2_G, |r| (r & !0b1111_0000) | flag.bits())
    }

    /// Specify the full scale for Gyroscope measurements
    pub fn set_gyroscope_scale(&mut self, scale: GyroscopeScale) -> Result<(), E> {
        let flag = match scale {
            GyroscopeScale::Scale125Dps => CTRL2_GBits::FS125DPS,
            GyroscopeScale::Scale250Dps => CTRL2_GBits::FS250DPS,
            GyroscopeScale::Scale500Dps => CTRL2_GBits::FS500DPS,
            GyroscopeScale::Scale1000Dps => CTRL2_GBits::FS1000DPS,
            GyroscopeScale::Scale2000Dps => CTRL2_GBits::FS2000DPS,
        };
        self.dev
            .mutate(Register::CTRL2_G, |r| (r & !0b0000_1111) | flag.bits())
    }
}
