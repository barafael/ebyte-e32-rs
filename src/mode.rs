use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::PinState::{High, Low};
use embedded_hal::digital::v2::{InputPin, OutputPin};

/// Send and receive.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Normal;

/// Send preamble to wake receiver.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Wakeup;

/// Powerdown.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Monitor;

/// Programming.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Program;

pub(crate) trait Mode {
    fn id(&self) -> u8;

    fn set_pins<M0, M1, Aux, D>(&self, m0: &mut M0, m1: &mut M1, aux: &mut Aux, delay: &mut D)
    where
        M0: OutputPin,
        M1: OutputPin,
        Aux: InputPin,
        D: DelayMs<u32>;
}

macro_rules! impl_mode {
    ($($type: ty)*, $id: literal, $m0_state: path, $m1_state: path) => {
        $(
            impl Mode for $type {
                fn id(&self) -> u8 {
                    $id
                }

                fn set_pins<M0, M1, Aux, D>(&self, m0: &mut M0, m1: &mut M1, aux: &mut Aux, delay: &mut D)
                where
                    M0: OutputPin,
                    M1: OutputPin,
                    Aux: InputPin,
                    D: DelayMs<u32> {
                        // TODO check if delay times can be reduced.
                            delay.delay_ms(40);
                            // TODO handle errors here.
                            let _ = m0.set_state($m0_state);
                            let _ = m1.set_state($m1_state);
                            delay.delay_ms(40);

                            loop {
                                // TODO timeouts?
                                match aux.is_low() {
                                    Ok(true) => continue,
                                    Ok(false) => break,
                                    // TODO error handling.
                                    Err(_e) => panic!("failed to wait for aux pin"),
                                }
                            }
                    }
                }
        )*
    };
}

impl_mode!(Normal, 0, Low, Low);
impl_mode!(Wakeup, 1, High, Low);
impl_mode!(Monitor, 2, Low, High);
impl_mode!(Program, 3, High, High);

#[cfg(test)]
mod test {
    use super::*;
    use embedded_hal_mock::delay::MockNoop;
    use embedded_hal_mock::pin::Mock as Pin;
    use embedded_hal_mock::pin::{
        State::{High, Low},
        Transaction,
    };

    #[test]
    fn id() {
        let mode = Normal;
        assert_eq!(mode.id(), 0);

        let mode = Wakeup;
        assert_eq!(mode.id(), 1);

        let mode = Monitor;
        assert_eq!(mode.id(), 2);

        let mode = Program;
        assert_eq!(mode.id(), 3);
    }

    #[test]
    fn pins_normal() {
        let mode = Normal;
        let mut m0 = Pin::new(&vec![Transaction::set(Low)]);
        let mut m1 = Pin::new(&vec![Transaction::set(Low)]);
        let mut aux = Pin::new(&vec![
            Transaction::get(Low),
            Transaction::get(Low),
            Transaction::get(High),
        ]);
        mode.set_pins(&mut m0, &mut m1, &mut aux, &mut MockNoop);
        m0.done();
        m1.done();
        aux.done();
    }

    #[test]
    fn pins_wakeup() {
        let mode = Wakeup;
        let mut m0 = Pin::new(&vec![Transaction::set(High)]);
        let mut m1 = Pin::new(&vec![Transaction::set(Low)]);
        let mut aux = Pin::new(&vec![
            Transaction::get(Low),
            Transaction::get(Low),
            Transaction::get(High),
        ]);
        mode.set_pins(&mut m0, &mut m1, &mut aux, &mut MockNoop);
        m0.done();
        m1.done();
        aux.done();
    }

    #[test]
    fn pins_powerdown() {
        let mode = Monitor;
        let mut m0 = Pin::new(&vec![Transaction::set(Low)]);
        let mut m1 = Pin::new(&vec![Transaction::set(High)]);
        let mut aux = Pin::new(&vec![
            Transaction::get(Low),
            Transaction::get(Low),
            Transaction::get(High),
        ]);
        mode.set_pins(&mut m0, &mut m1, &mut aux, &mut MockNoop);
        m0.done();
        m1.done();
        aux.done();
    }

    #[test]
    fn pins_program() {
        let mode = Program;
        let mut m0 = Pin::new(&vec![Transaction::set(High)]);
        let mut m1 = Pin::new(&vec![Transaction::set(High)]);
        let mut aux = Pin::new(&vec![
            Transaction::get(Low),
            Transaction::get(Low),
            Transaction::get(High),
        ]);
        mode.set_pins(&mut m0, &mut m1, &mut aux, &mut MockNoop);
        m0.done();
        m1.done();
        aux.done();
    }
}
