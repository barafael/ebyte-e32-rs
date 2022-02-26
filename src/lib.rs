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
use parameters::Parameters;

mod error;
mod mode;
mod model_data;
mod parameters;

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

    pub fn release(self) -> (S, M0, M1, Aux, D) {
        (self.serial, self.m0, self.m1, self.aux, self.delay)
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
        let _ = self.serial.write(0xC3);
        let _ = self.serial.write(0xC3);
        let _ = self.serial.write(0xC3);

        let save = self.serial.read().map_err(|_| Error::SerialRead)?;
        let model = self.serial.read().map_err(|_| Error::SerialRead)?;
        let version = self.serial.read().map_err(|_| Error::SerialRead)?;
        let features = self.serial.read().map_err(|_| Error::SerialRead)?;
        let _ = self.serial.read().map_err(|_| Error::SerialRead)?;
        let _ = self.serial.read().map_err(|_| Error::SerialRead)?;

        Ok(ModelData {
            save,
            model,
            version,
            features,
        })
    }

    pub fn read_parameters(&mut self) -> Result<Parameters, Error> {
        let _ = self.serial.write(0xC1);
        let _ = self.serial.write(0xC1);
        let _ = self.serial.write(0xC1);

        let save = self.serial.read().map_err(|_| Error::SerialRead)?;
        let address_high = self.serial.read().map_err(|_| Error::SerialRead)?;
        let address_low = self.serial.read().map_err(|_| Error::SerialRead)?;
        let speed = self.serial.read().map_err(|_| Error::SerialRead)?;
        let channel = self.serial.read().map_err(|_| Error::SerialRead)?;
        let options = self.serial.read().map_err(|_| Error::SerialRead)?;

        let address = (address_high as u16) << 8 | address_low as u16;
        let parity = ((speed & 0xC0) >> 6) == 1;

        Ok(Parameters::default())
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
        let (_s, _aux, _m0, _m1, _delay) = mock.release();
    }
}
