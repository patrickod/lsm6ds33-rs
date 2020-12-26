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
    ODR25Hz,
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

        #[cfg(feature = "defmt-default")]
        defmt::info!("instantiating LSM6DS33 & checking WHO_AM_I");

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
            .write(Register::CTRL3_C, CTRL3_CBits::SW_RESET.bits())?;

        loop {
            let ctrl = self.dev.read(Register::CTRL3_C)?;
            let flags = CTRL3_CBits::from_bits_truncate(ctrl);
            if !flags.contains(CTRL3_CBits::SW_RESET) {
                break;
            }
        }
        Ok(())
    }

    fn initialize(&mut self) -> Result<(), E> {
        // Reset device to initial configuration
        self.soft_reset()?;

        // Enable Block Data Update operation (vs FIFO modes)
        self.dev.write(Register::CTRL3_C, CTRL3_CBits::BDU.bits())?;

        // Enable 2G accelerometer scale @ 26Hz
        self.dev
            .mutate(Register::CTRL1_XL, |r| r & CTRL1_XLBits::SCALE2G.bits())?;
        self.dev
            .mutate(Register::CTRL1_XL, |r| r & CTRL1_XLBits::ODR_26HZ.bits())?;

        // enable gyroscope 2000dps @ 26Hz
        self.dev
            .mutate(Register::CTRL2_G, |r| r & CTRL2_GBits::FS2000DPS.bits())?;
        self.dev
            .mutate(Register::CTRL2_G, |r| r & CTRL2_GBits::ODR_26HZ.bits())?;

        Ok(())
    }
}
