use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    /// Invalid UART parity.
    #[snafu(display("Invalid UART parity {value}"))]
    InvalidUartParity { value: u8 },

    /// Invalid baud rate.
    #[snafu(display("Invalid baud rate {rate}"))]
    InvalidBaudrate { rate: u8 },

    /// Invalid transmission mode.
    #[snafu(display("Invalid transmission mode {value}"))]
    InvalidTransmissionMode { value: u8 },

    /// Invalid IO drive mode.
    #[snafu(display("Invalid IO drive mode {value}"))]
    InvalidIoDriveMode { value: u8 },

    /// Invalid wakeup time.
    #[snafu(display("Invalid wakeup time {value}"))]
    InvalidWakeupTime { value: u8 },

    /// Invalid Forward Error Correction mode.
    #[snafu(display("Invalid FEC mode {value}"))]
    InvalidFecMode { value: u8 },

    /// Invalid transmission power.
    #[snafu(display("Invalid transmission power {value}"))]
    InvalidTransmissionPower { value: u8 },
}
