//! Enables building Pods into a buffer without allocations.

use bytes::{BufMut, BytesMut};

use crate::pod::{PodType, pad_to_8};

pub struct PodBuf<'buf> {
    buf: &'buf mut BytesMut,
}

impl<'buf> PodBuf<'buf> {
    pub fn new(buf: &'buf mut BytesMut) -> Self {
        Self { buf }
    }

    /// Use the `builder_fn` function to add the `struct`'s fields as Pods.
    pub fn put_struct(&mut self, builder_fn: impl FnOnce(&mut PodBuf)) {
        let mut builder = PodBuilder::new(self, PodType::Struct);
        builder_fn(&mut builder.inner);
    }

    pub fn put_int(&mut self, value: i32) {
        let builder = PodBuilder::new(self, PodType::Int);
        builder.inner.buf.put_i32_ne(value)
    }

    pub fn put_string(&mut self, value: impl AsRef<[u8]>) {
        let value = value.as_ref();
        assert!(
            !value.contains(&0),
            "TOOD make this an error - string must not contain interior null"
        );
        let builder = PodBuilder::new(self, PodType::String);
        builder.inner.buf.put(value);
        builder.inner.buf.put_u8(0);
    }
}

/// When you create this type it will store the offset to the size. When it is dropped,
/// it fills in the size with the correct value.
struct PodBuilder<'a, 'buf> {
    inner: &'a mut PodBuf<'buf>,
    start_offset: usize,
}

impl<'a, 'buf> PodBuilder<'a, 'buf> {
    fn new(inner: &'a mut PodBuf<'buf>, pod_type: PodType) -> Self {
        inner.buf.put_bytes(0, 4);
        inner.buf.put_u32_ne(pod_type.into());

        // this must come after we added the header.
        let start_offset = inner.buf.len();
        Self {
            start_offset,
            inner,
        }
    }
}

impl<'a, 'buf> Drop for PodBuilder<'a, 'buf> {
    fn drop(&mut self) {
        // Calculate padding, then calculate and set length
        let end_len = self.inner.buf.len();
        let content_len = end_len - self.start_offset;
        let padding = pad_to_8(content_len);
        self.inner.buf.put_bytes(0, padding);
        let content_len = u32::try_from(content_len).expect("int overflow");
        // size is stored at 8 bytes before the start of the pod content
        (&mut self.inner.buf[(self.start_offset - 8)..]).put_u32_ne(content_len);
    }
}
