use crate::error::Error;

use self::{
    air_baudrate::AirBaudRate,
    baudrate::BaudRate,
    option::{
        ForwardErrorCorrectionMode, IoDriveMode, TransmissionMode, TransmissionPower, WakeupTime,
    },
    uart_parity::Parity,
};

pub mod air_baudrate;
pub mod baudrate;
pub mod option;
pub mod uart_parity;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameters {
    pub address: u16,
    pub channel: u8,
    pub uart_parity: Parity,
    pub uart_rate: BaudRate,
    pub air_rate: AirBaudRate,
    pub transmission_mode: TransmissionMode,
    pub io_drive_mode: IoDriveMode,
    pub wakeup_time: WakeupTime,
    pub fec: ForwardErrorCorrectionMode,
    pub transmission_power: TransmissionPower,
}
