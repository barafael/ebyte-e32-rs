use crate::{mode::Normal, Ebyte};
use embedded_hal::serial::{Read, Write};
use embedded_hal_mock::{
    delay,
    pin::{
        Mock as Pin,
        State::{High, Low},
        Transaction as PinTransaction,
    },
    serial::Mock as Serial,
};
use std::marker::PhantomData;

mod read_model;

#[test]
fn acquire_release() {
    let aux = Pin::new(&vec![
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(Low),
        PinTransaction::get(High),
    ]);
    let m0 = Pin::new(&vec![PinTransaction::set(Low)]);
    let m1 = Pin::new(&vec![PinTransaction::set(Low)]);
    let serial = Serial::new(&[]);
    let delay = delay::MockNoop;

    let mock = Ebyte::new(serial, aux, m0, m1, delay).unwrap();
    let (mut s, mut aux, mut m0, mut m1, _delay) = mock.release();
    s.done();
    aux.done();
    m0.done();
    m1.done();
}

use embedded_hal_mock::serial::Transaction as SerialTransaction;
use proptest::prelude::*;

#[test]
fn flushes() {
    let serial = Serial::new(&[SerialTransaction::flush()]);
    let aux = Pin::new(&vec![]);
    let m0 = Pin::new(&vec![]);
    let m1 = Pin::new(&vec![]);

    let mut ebyte = Ebyte {
        serial,
        aux,
        m0,
        m1,
        delay: delay::MockNoop,
        mode: PhantomData::<Normal>,
    };

    ebyte.flush().unwrap();

    let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
    s.done();
    aux.done();
    m0.done();
    m1.done();
}

#[test]
fn resets() {
    let serial = Serial::new(&[
        SerialTransaction::write(0xC4),
        SerialTransaction::write(0xC4),
        SerialTransaction::write(0xC4),
    ]);
    let aux = Pin::new(&vec![
        PinTransaction::get(Low),
        PinTransaction::get(High),
        PinTransaction::get(Low),
        PinTransaction::get(High),
    ]);
    let m0 = Pin::new(&vec![PinTransaction::set(High), PinTransaction::set(Low)]);
    let m1 = Pin::new(&vec![PinTransaction::set(High), PinTransaction::set(Low)]);

    let mut ebyte = Ebyte {
        serial,
        aux,
        m0,
        m1,
        delay: delay::MockNoop,
        mode: PhantomData::<Normal>,
    };

    ebyte.reset().unwrap();

    let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
    s.done();
    aux.done();
    m0.done();
    m1.done();
}

proptest! {

    #[test]
    fn reads_byte(byte in any::<u8>()) {
        let serial = Serial::new(
            &[SerialTransaction::read(byte)]
        );
        let aux = Pin::new(&vec![]);
        let m0 = Pin::new(&vec![]);
        let m1 = Pin::new(&vec![]);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };

        let result = ebyte.read().unwrap();
        assert_eq!(result, byte);

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }

    #[test]
    fn writes_byte(byte in any::<u8>()) {
        let serial = Serial::new(
            &[SerialTransaction::write(byte)]
        );
        let aux = Pin::new(&vec![]);
        let m0 = Pin::new(&vec![]);
        let m1 = Pin::new(&vec![]);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };

        ebyte.write(byte).unwrap();

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }

    #[test]
    fn reads_buffer(buffer in any::<[u8; 16]>()) {
        let serial = Serial::new(
            &buffer.iter().copied().map(SerialTransaction::read).collect::<Vec<_>>()[..]
        );
        let aux = Pin::new(&vec![]);
        let m0 = Pin::new(&vec![]);
        let m1 = Pin::new(&vec![]);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };

        let mut result = [0u8; 16];
        ebyte.read_buffer(&mut result).unwrap();
        assert_eq!(result, buffer);

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }

    #[test]
    fn writes_buffer(buffer in any::<[u8; 16]>()) {
        let serial = Serial::new(
            &buffer.iter().copied().map(SerialTransaction::write).collect::<Vec<_>>()[..]
        );
        let aux = Pin::new(&vec![]);
        let m0 = Pin::new(&vec![]);
        let m1 = Pin::new(&vec![]);

        let mut ebyte = Ebyte {
            serial,
            aux,
            m0,
            m1,
            delay: delay::MockNoop,
            mode: PhantomData::<Normal>,
        };

        ebyte.write_buffer(&buffer).unwrap();

        let (mut s, mut aux, mut m0, mut m1, _delay) = ebyte.release();
        s.done();
        aux.done();
        m0.done();
        m1.done();
    }
}
