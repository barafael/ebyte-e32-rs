use crate::{
    mode::Program,
    parameters::{Parameters, Persistence},
    Ebyte,
};
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
use std::marker::PhantomData;

struct SetParameterExpectations {
    serial: Serial<u8>,
    aux: Generic<PinTransaction>,
    m0: Generic<PinTransaction>,
    m1: Generic<PinTransaction>,
}

fn set_parameters_expectations(
    parameters: &Parameters,
    persistence: Persistence,
) -> SetParameterExpectations {
    let persistence = u8::from(persistence);
    let bytes = parameters.to_bytes();
    let serial = Serial::<u8>::new(&[
        SerialTransaction::write(persistence),
        SerialTransaction::write(bytes[0]),
        SerialTransaction::write(bytes[1]),
        SerialTransaction::write(bytes[2]),
        SerialTransaction::write(bytes[3]),
        SerialTransaction::write(bytes[4]),
        SerialTransaction::read(0xC0),
        SerialTransaction::read(bytes[0]),
        SerialTransaction::read(bytes[1]),
        SerialTransaction::read(bytes[2]),
        SerialTransaction::read(bytes[3]),
        SerialTransaction::read(bytes[4]),
    ]);
    let aux = Pin::new(&vec![
        // for programming ready check, await high level on AUX.
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(High),
        // for return to normal mode, await high level on AUX.
        PinTransaction::get(Low),
        PinTransaction::get(High),
    ]);
    let m0 = Pin::new(&vec![PinTransaction::set(Low)]);
    let m1 = Pin::new(&vec![PinTransaction::set(Low)]);
    SetParameterExpectations {
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
    fn sets_parameters(parameters in any::<Parameters>(), persistence in any::<Persistence>()) {
        let SetParameterExpectations {
            serial,
            aux,
            m0,
            m1,
        } = set_parameters_expectations(&parameters, persistence);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Program>,
        };
        ebyte.set_parameters(&parameters, persistence).unwrap();
        let ebyte = ebyte.into_normal_mode();

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }
}
