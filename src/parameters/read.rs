use crate::{mode::Normal, Ebyte, Parameters};
use core::marker::PhantomData;
use embedded_hal_mock::{
    common::Generic,
    delay,
    pin::{
        Mock as Pin,
        State::{High, Low},
        Transaction as PinTransaction,
    },
    serial::{Mock as Serial, Transaction as SerialTransaction},
};

struct ReadParameterExpectations {
    serial: Serial<u8>,
    aux: Generic<PinTransaction>,
    m0: Generic<PinTransaction>,
    m1: Generic<PinTransaction>,
}

fn read_parameters_expectations(parameters: &Parameters) -> ReadParameterExpectations {
    let bytes = parameters.to_bytes();
    let serial = Serial::<u8>::new(&[
        SerialTransaction::write(0xC1),
        SerialTransaction::write(0xC1),
        SerialTransaction::write(0xC1),
        SerialTransaction::read(0xC0),
        SerialTransaction::read(bytes[0]),
        SerialTransaction::read(bytes[1]),
        SerialTransaction::read(bytes[2]),
        SerialTransaction::read(bytes[3]),
        SerialTransaction::read(bytes[4]),
    ]);
    let aux = Pin::new(&vec![
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(High),
        PinTransaction::get(Low),
        PinTransaction::get(High),
    ]);
    let m0 = Pin::new(&vec![PinTransaction::set(High), PinTransaction::set(Low)]);
    let m1 = Pin::new(&vec![PinTransaction::set(High), PinTransaction::set(Low)]);
    ReadParameterExpectations {
        serial,
        aux,
        m0,
        m1,
    }
}

use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 10000,
        .. ProptestConfig::default()
    })]

    #[test]
    fn reads_parameters(parameters in any::<Parameters>()) {
        let ReadParameterExpectations {
            serial,
            aux,
            m0,
            m1,
        } = read_parameters_expectations(&parameters);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };
        let response = ebyte.parameters().unwrap();

        assert_eq!(response, parameters);

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }
}
