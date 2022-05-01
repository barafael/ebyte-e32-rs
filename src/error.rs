use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read model data"))]
    ReadModelData,

    #[snafu(display("Failed to read parameters"))]
    ReadParameters,

    #[snafu(display("Failed to set parameters"))]
    SetParameters,

    #[snafu(display("Failed to read serial port"))]
    SerialRead,

    #[snafu(display("Failed to write serial port"))]
    SerialWrite,

    #[snafu(display("Failed to wait for AUX pin"))]
    AuxPin,

    #[snafu(display("Parameter error {source}"))]
    Parameter {
        source: ebyte_e32_parameters::error::Error,
    },
}
