#![cfg_attr(not(test), no_std)]

use core::marker::PhantomData;
use embedded_hal::{
    blocking::delay::DelayMs,
    digital::v2::{InputPin, OutputPin},
    serial::{Read, Write},
};
use error::Error;
use mode::{Mode, Normal, Program};
use model_data::ModelData;
use nb::block;
use parameters::{Parameters, Persistence};

mod error;
mod mode;
mod model_data;
pub mod parameters;

pub struct Ebyte<S, Aux, M0, M1, D, M>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    serial: S,
    m0: M0,
    m1: M1,
    aux: Aux,
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
        mut m0: M0,
        mut m1: M1,
        mut aux: Aux,
        mut delay: D,
    ) -> Result<Self, Error> {
        // // Let pins settle.
        //delay.delay_ms(10);

        let mode = Normal;
        mode.set_pins(&mut m0, &mut m1, &mut aux, &mut delay);

        let ebyte = Self {
            serial,
            m0,
            m1,
            aux,
            delay,
            mode: PhantomData::<Normal>,
        };

        let mut ebyte = ebyte.into_program_mode();

        // Supposedly these have to be queried on startup.
        // TODO is this true?
        let _model_data = ebyte.read_model_data()?;

        let ebyte = ebyte.into_normal_mode();
        let mut ebyte = ebyte.into_program_mode();

        let _parameters = ebyte.read_parameters()?;
        let ebyte = ebyte.into_normal_mode();

        Ok(ebyte)
    }

    pub fn into_program_mode(mut self) -> Ebyte<S, Aux, M0, M1, D, Program> {
        let mode = Program;
        mode.set_pins(&mut self.m0, &mut self.m1, &mut self.aux, &mut self.delay);
        Ebyte {
            serial: self.serial,
            m0: self.m0,
            m1: self.m1,
            aux: self.aux,
            delay: self.delay,
            mode: PhantomData::<Program>,
        }
    }

    /// Write entire buffer to serial port
    pub fn write_buffer(&mut self, buffer: &[u8]) -> Result<(), crate::Error> {
        for ch in buffer {
            let _ = block!(self.serial.write(*ch));
        }
        Ok(())
    }

    /// Read entire buffer from serial port
    pub fn read_buffer(&mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let mut n = 0;
        while n < buffer.len() {
            if let Ok(ch) = block!(self.serial.read()) {
                buffer[n] = ch;
                n += 1;
            }
        }
        Ok(())
    }

    pub fn release(self) -> (S, M0, M1, Aux, D) {
        (self.serial, self.m0, self.m1, self.aux, self.delay)
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

/// Implement Write for Hc12 in normal mode.
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

impl<S, Aux, M0, M1, D> Ebyte<S, Aux, M0, M1, D, Program>
where
    S: Read<u8> + Write<u8>,
    Aux: InputPin,
    M0: OutputPin,
    M1: OutputPin,
    D: DelayMs<u32>,
{
    pub fn read_model_data(&mut self) -> Result<ModelData, Error> {
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

    pub fn read_parameters(&mut self) -> Result<Parameters, Error> {
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(0xC1)).map_err(|_| Error::SerialWrite)?;

        let save = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;

        let bytes = [0u8; 5];
        for _ in 0..5 {
            block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        }

        if save != 0xC0 {
            return Err(Error::ReadParameters);
        }

        let params = Parameters::from_bytes(&bytes)?;

        Ok(params)
    }

    pub fn set_parameters(&mut self, params: &Parameters, mode: Persistence) -> Result<(), Error> {
        let persistence = u8::from(mode);
        block!(self.serial.write(persistence)).map_err(|_| Error::SerialWrite)?;

        let bytes: [u8; 5] = params.to_bytes();
        block!(self.serial.write(bytes[0])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[1])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[2])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[3])).map_err(|_| Error::SerialWrite)?;
        block!(self.serial.write(bytes[4])).map_err(|_| Error::SerialWrite)?;

        for _ in 0..6 {
            let _ = block!(self.serial.read()).map_err(|_| Error::SerialRead)?;
        }

        self.delay.delay_ms(40);

        self.until_aux();

        Ok(())
    }

    pub fn reset(&mut self) {
        let _ = block!(self.serial.write(0xC4));
        let _ = block!(self.serial.write(0xC4));
        let _ = block!(self.serial.write(0xC4));

        self.until_aux();
    }

    fn until_aux(&mut self) {
        loop {
            // TODO timeouts?
            match self.aux.is_low() {
                Ok(true) => continue,
                Ok(false) => break,
                // TODO error handling.
                Err(_e) => panic!("failed to wait for aux pin"),
            }
        }
    }

    pub fn into_normal_mode(mut self) -> Ebyte<S, Aux, M0, M1, D, Normal> {
        let mode = Normal;
        mode.set_pins(&mut self.m0, &mut self.m1, &mut self.aux, &mut self.delay);
        Ebyte {
            serial: self.serial,
            m0: self.m0,
            m1: self.m1,
            aux: self.aux,
            delay: self.delay,
            mode: PhantomData::<Normal>,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Ebyte;
    use embedded_hal_mock::{
        delay,
        pin::{
            Mock as Pin,
            State::{High, Low},
            Transaction as PinTransaction,
        },
        serial::Mock as Serial,
    };

    #[test]
    #[ignore = "I/O performed is not implemented in unit test"]
    fn acquire_release() {
        let aux = Pin::new(&vec![
            PinTransaction::get(Low),
            PinTransaction::get(Low),
            PinTransaction::get(High),
        ]);
        let m0 = Pin::new(&vec![PinTransaction::set(Low)]);
        let m1 = Pin::new(&vec![PinTransaction::set(Low)]);
        let serial = Serial::<u8>::new(&[]);
        let delay = delay::MockNoop;

        let mock = Ebyte::new(serial, m0, m1, aux, delay).unwrap();
        let (mut s, mut aux, mut m0, mut m1, _delay) = mock.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }
}
