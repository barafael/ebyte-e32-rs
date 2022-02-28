use self::{
    air_baudrate::AirBaudRate,
    baudrate::BaudRate,
    option::{
        fec_mode::ForwardErrorCorrectionMode, io_drive_mode::IoDriveMode,
        transmission_power::TransmissionPower, wakeup_time::WakeupTime, TransmissionMode,
    },
    uart_parity::Parity,
};
use crate::error::Error;
use smart_default::SmartDefault;
pub use typed_builder::TypedBuilder;

pub mod air_baudrate;
pub mod baudrate;
pub mod option;
pub mod uart_parity;

#[derive(Copy, Clone, Debug, PartialEq, Eq, SmartDefault)]
pub enum Persistence {
    #[default]
    Temporary,
    Permanent,
}

impl From<Persistence> for u8 {
    fn from(mode: Persistence) -> Self {
        match mode {
            Persistence::Temporary => 0xC2,
            Persistence::Permanent => 0xC0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, TypedBuilder)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Parameters {
    pub address: u16,
    pub channel: u8,
    #[builder(default)]
    pub uart_parity: Parity,
    #[builder(default)]
    pub uart_rate: BaudRate,
    #[builder(default)]
    pub air_rate: AirBaudRate,
    #[builder(default)]
    pub transmission_mode: TransmissionMode,
    #[builder(default)]
    pub io_drive_mode: IoDriveMode,
    #[builder(default)]
    pub wakeup_time: WakeupTime,
    #[builder(default)]
    pub fec: ForwardErrorCorrectionMode,
    #[builder(default)]
    pub transmission_power: TransmissionPower,
}

impl Parameters {
    pub fn to_bytes(&self) -> [u8; 5] {
        let mut bytes = [0u8; 5];
        bytes[0] = ((0xFF00 & self.address) >> 8) as u8;
        bytes[1] = (0x00FF & self.address) as u8;

        let parity = u8::from(self.uart_parity);
        let uart_rate = u8::from(self.uart_rate);
        let air_rate = u8::from(self.air_rate);
        let speed_byte = (parity << 6) | (uart_rate << 3) | air_rate;
        bytes[2] = speed_byte;

        bytes[3] = self.channel;

        let transmission_mode = u8::from(self.transmission_mode);
        let io_drive_mode = u8::from(self.io_drive_mode);
        let wakeup_time = u8::from(self.wakeup_time);
        let fec = u8::from(self.fec);
        let transmission_power = u8::from(self.transmission_power);
        let options = (transmission_mode << 7)
            | (io_drive_mode << 6)
            | (wakeup_time << 3)
            | (fec << 2)
            | transmission_power;
        bytes[4] = options;

        bytes
    }

    pub fn from_bytes(bytes: &[u8; 5]) -> Result<Self, Error> {
        let address_high = bytes[0];
        let address_low = bytes[1];
        let speed = bytes[2];
        let channel = bytes[3];
        let options = bytes[4];
        let address = (address_high as u16) << 8 | address_low as u16;
        let uart_parity = Parity::try_from((speed & 0xC0) >> 6)?;
        let uart_rate = BaudRate::try_from((speed & 0x38) >> 3)?;
        let air_rate = AirBaudRate::try_from(speed & 0x7)?;

        let transmission_mode = TransmissionMode::try_from((options & 0x80) >> 7)?;
        let io_drive_mode = IoDriveMode::try_from((options & 0x40) >> 6)?;
        let wakeup_time = WakeupTime::try_from((options & 0x38) >> 3)?;
        let fec = ForwardErrorCorrectionMode::try_from((options & 0x07) >> 2)?;
        let transmission_power = TransmissionPower::try_from(options & 0x3)?;

        Ok(Self {
            address,
            channel,
            uart_parity,
            uart_rate,
            air_rate,
            transmission_mode,
            io_drive_mode,
            wakeup_time,
            fec,
            transmission_power,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 10000,
            .. ProptestConfig::default()
        })]

        #[test]
        fn from_to_bytes_roundtrip(params in any::<Parameters>()) {
            let bytes = params.to_bytes();
            let decoded = Parameters::from_bytes(&bytes).unwrap();
            assert_eq!(decoded, params);
        }
    }
}
