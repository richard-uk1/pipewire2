mod builder;
pub use builder::PodBuf;

mod pod_type;
pub use pod_type::PodType;

mod decoder;
pub use decoder::{DecodePodError, PodDecoder};

mod props;
pub use props::Props;

mod traits;
pub use traits::{DecodePod, EncodePod};

fn pad_to_8(len: usize) -> usize {
    // final `% 8` is to turn `8` into `0`
    (8 - (len % 8)) % 8
}
