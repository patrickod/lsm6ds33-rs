use crate::{Error, Register};
use embedded_hal::blocking::i2c;

/// Generic interface implemented over I2C/SPI protocol
pub trait Device {
    type Error;
    fn read(&mut self, reg: Register) -> Result<u8, Self::Error> {
        let mut buffer = [0u8];
        self.read_many(reg, &mut buffer)?;
        Ok(buffer[0])
    }
    fn read_many(&mut self, reg: Register, buffer: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, reg: Register, value: u8) -> Result<(), Self::Error>;
    fn write_many(&mut self, reg: Register, buffer: &[u8]) -> Result<(), Self::Error>;
    fn mutate<F>(&mut self, reg: Register, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(u8) -> u8,
    {
        let current = self.read(reg)?;
        self.write(reg, f(current))
    }
}

pub struct I2CDevice<I2C> {
    i2c: I2C,
    address: u8,
}

fn i2c_error<E>(_: E) -> Error<E> {
    Error::CommunicationError
}

impl<E, I2C> I2CDevice<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Read<Error = E> + i2c::WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        I2CDevice { i2c, address }
    }
}

impl<E, I2C> Device for I2CDevice<I2C>
where
    I2C: i2c::Read<Error = E> + i2c::Write<Error = E> + i2c::WriteRead<Error = E>,
{
    type Error = Error<E>;

    fn read_many(&mut self, reg: Register, buffer: &mut [u8]) -> Result<(), Error<E>> {
        self.i2c
            .write_read(self.address, &[reg.address()], buffer)
            .map_err(i2c_error)?;
        Ok(())
    }

    fn write(&mut self, reg: Register, value: u8) -> Result<(), Error<E>> {
        let buffer: [u8; 2] = [reg.address(), value];
        self.i2c.write(self.address, &buffer).map_err(i2c_error)?;
        Ok(())
    }

    fn write_many(&mut self, reg: Register, buffer: &[u8]) -> Result<(), Error<E>> {
        if buffer.len() > 16 {
            return Err(Error::CommunicationError);
        }
        let mut message: [u8; 17] = [0; 17];
        let message = &mut message[0..buffer.len() + 1];
        message[0] = reg.address();
        message[1..].copy_from_slice(&buffer[0..]);
        self.i2c.write(self.address, message).map_err(i2c_error)?;
        Ok(())
    }
}
