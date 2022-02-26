#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Parameters {
    save: u8,
    address: u16,
    parity: bool,
    uart_rate: u8,
    air_rate: u8,
    trans: u8,
    pullup: u8,
    wakeup: u8,
    fec: u8,
    power: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Parity {
    EightNoneOne,
    EightOddOne,
    EightEvenOne,
}
