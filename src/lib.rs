#![cfg_attr(not(any(test, feature = "value_enum")), no_std)]
#![doc = include_str!("../README.md")]

use core::marker::PhantomData;
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{InputPin, OutputPin},
    serial::{Read, Write},
};
pub use error::Error;
use mode::{Mode, Normal, Program};
use model_data::ModelData;
use nb::block;
use parameters::{Parameters, Persistence};

mod error;
pub mod mode;
mod model_data;
pub mod parameters;
#[cfg(test)]
mod test;

pub struct Ebyte<S, Aux, M0, M1, D, M>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    serial: S,
    aux: Aux,
    m0: M0,
    m1: M1,
    delay: D,
    mode: PhantomData<M>,
}

impl<S, Aux, M0, M1, D> Ebyte<S, Aux, M0, M1, D, Normal>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    pub fn new(
        serial: S,
        mut aux: Aux,
        mut m0: M0,
        mut m1: M1,
        mut delay: D,
    ) -> Result<Self, Error> {
        Normal::set_pins(&mut aux, &mut m0, &mut m1, &mut delay);

        let ebyte = Self {
            serial,
            aux,
            m0,
            m1,
            delay,
            mode: PhantomData::<Normal>,
        };

        Ok(ebyte)
    }

    /// Write entire buffer to serial port
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), crate::Error> {
        for ch in buffer {
            block!(self.serial.write(*ch)).map_err(|_| Error::SerialWrite)?;
        }
        Ok(())
    }

    /// Read entire buffer from serial port
    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        for byte in buffer {
            *byte = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        }
        Ok(())
    }

    pub fn model_data(&mut self) -> Result<ModelData, Error> {
        Program::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        let result = self.read_model_data();
        Normal::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        result
    }

    fn read_model_data(&mut self) -> Result<ModelData, Error> {
        block!(self.serial.write(0xC3)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC3)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC3)).map_err(|_| Error::SerialWrite)?;

        let save = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        let model = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        let version = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        let features = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;

        if save == 0xC3 {
            Ok(ModelData {
                model,
                version,
                features,
            })
        } else {
            Err(Error::ReadModelData)
        }
    }

    pub fn parameters(&mut self) -> Result<Parameters, Error> {
        Program::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        let result = self.read_parameters();
        Normal::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        result
    }

    fn read_parameters(&mut self) -> Result<Parameters, Error> {
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;
        let save = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        let mut bytes = [0u8; 5];
        for byte in &mut bytes {
            *byte = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        }
        if save != 0xC0 {
            return Err(Error::ReadParameters);
        }
        let params = Parameters::from_bytes(&bytes).map_err(|e| Error::Parameter { source: e })?;
        Ok(params)
    }

    pub fn set_parameters(&mut self, params: &Parameters, mode: Persistence) -> Result<(), Error> {
        Program::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        let result = self.write_parameters(params, mode);
        Normal::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        result
    }

    fn write_parameters(&mut self, params: &Parameters, mode: Persistence) -> Result<(), Error> {
        let persistence = u8::from(mode);
        block!(self.serial.write(persistence)).map_err(|_| Error::SerialWrite)?;

        let bytes: [u8; 5] = params.to_bytes();
        block!(self.serial.write(bytes[0])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[1])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[2])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[3])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[4])).map_err(|_| Error::SerialWrite)?;

        let response_prefix = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        let mut response = [0u8; 5];
        for byte in &mut response {
            *byte = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        }
        if response_prefix != 0xC0 {
            return Err(Error::SetParameters);
        }

        if response != bytes {
            return Err(Error::SetParameters);
        }

        self.delay.delay_ms(40);

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        Program::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        let result = self.perform_reset();
        Normal::set_pins(&mut self.aux, &mut self.m0, &mut self.m1, &mut self.delay);
        result
    }

    fn perform_reset(&mut self) -> Result<(), Error> {
        block!(self.serial.write(0xC4)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC4)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC4)).map_err(|_| Error::SerialWrite)?;

        Ok(())
    }

    pub fn release(self) -> (S, Aux, M0, M1, D) {
        (self.serial, self.aux, self.m0, self.m1, self.delay)
    }
}

impl<S, Aux, M0, M1, D> Read<u8> for Ebyte<S, Aux, M0, M1, D, Normal>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    type Error = <S as Read<u8>>::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.serial.read()
    }
}

/// Implement Write for Ebyte in normal mode.
/// This just defers to the underlying serial implementation.
impl<S, Aux, M0, M1, D> Write<u8> for Ebyte<S, Aux, M0, M1, D, Normal>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    type Error = <S as Write<u8>>::Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.serial.write(word)
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.serial.flush()
    }
}
