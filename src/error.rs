use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read model data"))]
    ReadModelData,

    #[snafu(display("Failed to read serial port"))]
    SerialRead,
}
