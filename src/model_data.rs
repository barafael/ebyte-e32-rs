#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct ModelData {
    pub model: u8,
    pub version: u8,
    pub features: u8,
}

impl ModelData {
    pub const fn to_bytes(self) -> [u8; 3] {
        [self.model, self.version, self.features]
    }

    pub const fn from_bytes(bytes: [u8; 3]) -> Self {
        Self {
            model: bytes[0],
            version: bytes[1],
            features: bytes[2],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model_data::ModelData;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn model_data_from_to_bytes_roundtrip(model_data in any::<ModelData>()) {
            let bytes = model_data.to_bytes();
            let decoded = ModelData::from_bytes(bytes);
            assert_eq!(model_data, decoded);
        }
    }
}
