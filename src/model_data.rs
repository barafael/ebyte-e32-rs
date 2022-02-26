#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelData {
    pub save: u8,
    pub model: u8,
    pub version: u8,
    pub features: u8,
}
