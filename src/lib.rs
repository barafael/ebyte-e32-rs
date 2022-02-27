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
use parameters::{
    air_baudrate::AirBaudRate,
    baudrate::BaudRate,
    option::{
        ForwardErrorCorrectionMode, IoDriveMode, TransmissionMode, TransmissionPower, WakeupTime,
    },
    uart_parity::Parity,
    Parameters,
};

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
        self.serial.write(0xC3).map_err(|_| Error::SerialWrite)?;
        self.serial.write(0xC3).map_err(|_| Error::SerialWrite)?;
        self.serial.write(0xC3).map_err(|_| Error::SerialWrite)?;

        let save = self.serial.read().map_err(|_| Error::SerialRead)?;
        let model = self.serial.read().map_err(|_| Error::SerialRead)?;
        let version = self.serial.read().map_err(|_| Error::SerialRead)?;
        let features = self.serial.read().map_err(|_| Error::SerialRead)?;
        let _ = self.serial.read().map_err(|_| Error::SerialRead)?;
        let _ = self.serial.read().map_err(|_| Error::SerialRead)?;

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
        let _ = self.serial.write(0xC1);
        let _ = self.serial.write(0xC1);
        let _ = self.serial.write(0xC1);

        let save = self.serial.read().map_err(|_| Error::SerialRead)?;
        let address_high = self.serial.read().map_err(|_| Error::SerialRead)?;
        let address_low = self.serial.read().map_err(|_| Error::SerialRead)?;
        let speed = self.serial.read().map_err(|_| Error::SerialRead)?;
        let channel = self.serial.read().map_err(|_| Error::SerialRead)?;
        let options = self.serial.read().map_err(|_| Error::SerialRead)?;

        if save != 0xC1 {
            return Err(Error::ReadParameters);
        }

        let address = (address_high as u16) << 8 | address_low as u16;
        let uart_parity = Parity::try_from((speed & 0xC0) >> 6)?;
        let uart_rate = BaudRate::try_from((speed & 0x38) >> 3)?;
        let air_rate = AirBaudRate::try_from(speed & 0x7)?;

        let transmission_mode = TransmissionMode::try_from((options & 0x80) >> 7)?;
        let io_drive_mode = IoDriveMode::try_from((options & 0x40) >> 6)?;
        let wakeup_time = WakeupTime::try_from((options & 0x38) >> 3)?;
        let fec = ForwardErrorCorrectionMode::try_from((options & 0x07) >> 2)?;
        let transmission_power = TransmissionPower::try_from(options & 0x3)?;

        Ok(Parameters {
            address,
            channel,
            uart_parity,
            uart_rate,
            air_rate,
            transmission_mode,
            io_drive_mode,
            wakeup_time,
            fec,
            transmission_power,
        })
    }

    pub fn reset(&mut self) {
        let _ = self.serial.write(0xC4);
        let _ = self.serial.write(0xC4);
        let _ = self.serial.write(0xC4);

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
