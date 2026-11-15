#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PodType {
    Int,
    Long,
    String,
    Struct,
    Unknown(u32),
}

impl From<u32> for PodType {
    fn from(value: u32) -> Self {
        match value {
            4 => PodType::Int,
            5 => PodType::Long,
            8 => PodType::String,
            14 => PodType::Struct,
            other => PodType::Unknown(other),
        }
    }
}

impl From<PodType> for u32 {
    fn from(pod_type: PodType) -> u32 {
        match pod_type {
            PodType::Int => 4,
            PodType::Long => 5,
            PodType::String => 8,
            PodType::Struct => 14,
            PodType::Unknown(v) => v,
        }
    }
}
