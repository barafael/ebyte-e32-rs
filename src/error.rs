use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    /// Failed to read model data.
    #[snafu(display("Failed to read model data"))]
    ReadModelData,

    /// Failed to read parameters.
    #[snafu(display("Failed to read parameters"))]
    ReadParameters,

    /// Failed to set parameters.
    #[snafu(display("Failed to set parameters"))]
    SetParameters,

    /// Failed to read serial port.
    #[snafu(display("Failed to read serial port"))]
    SerialRead,

    /// Failed to write serial port.
    #[snafu(display("Failed to write serial port"))]
    SerialWrite,

    /// Failed to wait for AUX pin.
    #[snafu(display("Failed to wait for AUX pin"))]
    AuxPin,

    /// Failed to process parameters.
    #[snafu(display("Parameter error {source}"))]
    Parameter {
        source: crate::parameters::error::Error,
    },
}
