use crate::{mode::Normal, model_data::ModelData, Ebyte};
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

struct ReadModelDataExpectations {
    serial: Serial<u8>,
    aux: Generic<PinTransaction>,
    m0: Generic<PinTransaction>,
    m1: Generic<PinTransaction>,
}

fn read_model_data_expectations(model_data: ModelData) -> ReadModelDataExpectations {
    let bytes = model_data.to_bytes();
    let serial = Serial::<u8>::new(&[
        SerialTransaction::write(0xC3),
        SerialTransaction::write(0xC3),
        SerialTransaction::write(0xC3),
        SerialTransaction::read(0xC3),
        SerialTransaction::read(bytes[0]),
        SerialTransaction::read(bytes[1]),
        SerialTransaction::read(bytes[2]),
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
    ReadModelDataExpectations {
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
    fn reads_model_data(model_data in any::<ModelData>()) {
        let ReadModelDataExpectations {
            serial,
            aux,
            m0,
            m1,
        } = read_model_data_expectations(model_data);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };
        let model = ebyte.model_data().unwrap();

        assert_eq!(model, model_data);

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }
}
