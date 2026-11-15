use bytes::{Buf, BytesMut};
use futures::channel::oneshot;
use spa::PodBuf;
use std::{collections::HashMap, fmt::Write, io};
use tokio_util::codec::{Decoder, Encoder};
use tracing::{debug, trace};

mod frame;
pub use frame::PwFrame;

mod header;
use header::{HEADER_LEN, Header};

use crate::PwMethod;

#[derive(Debug)]
pub struct PwCodec {
    next_seq: u32,
    /// Counter for sync seq numbers - separate from message seq
    next_sync_seq: u32,
    /// Senders to notify when sync call response is received
    ///
    /// TODO periodic job to look for closed channels or old seq numbers and remove senders.
    sync_senders: HashMap<u32, oneshot::Sender<()>>,
}

impl PwCodec {
    pub fn new() -> Self {
        Self {
            next_seq: 0,
            next_sync_seq: 0,
            sync_senders: HashMap::new(),
        }
    }

    fn next_seq(&mut self) -> u32 {
        let seq = self.next_seq;
        self.next_seq += 1;
        seq
    }

    fn next_sync_seq(&mut self) -> u32 {
        let seq = self.next_sync_seq;
        self.next_sync_seq += 1;
        seq
    }

    /// Create a header with placeholder length/fd count.
    fn header(&mut self, id: u32, op: u8) -> Header {
        Header::new_placeholder(id, op, self.next_seq())
    }
}

impl Decoder for PwCodec {
    type Item = PwFrame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        trace!(
            raw = human_readable_encoded_packet(src),
            "received packet from pw"
        );
        let Some(header) = Header::parse(&*src) else {
            return Ok(None);
        };

        let total_len = header.size_inc_header();
        if src.len() < total_len {
            // Reserve capacity in advance to avoid repeated reallocations on large frames
            src.reserve(total_len - src.len());
            return Ok(None);
        }

        // Consume the header and freeze the payload zero-copy
        src.advance(HEADER_LEN);
        let payload = src.split_to(header.size()).freeze();

        Ok(Some(PwFrame {
            id: header.id,
            opcode: header.opcode(),
            seq: header.seq,
            payload,
        }))
    }
}

impl<'a, T> Encoder<&'a T> for PwCodec
where
    T: PwMethod,
{
    type Error = io::Error;

    fn encode(&mut self, item: &'a T, dst: &mut BytesMut) -> Result<(), Self::Error> {
        debug_assert!(dst.is_empty());
        debug!(method = ?item, "sending message");
        let mut header = self.header(item.id(), item.opcode());
        header.encode_to(dst);
        {
            let mut pod_buf = PodBuf::new(dst);
            item.encode(&mut pod_buf);
        }

        let actual_size = dst.len() - HEADER_LEN;

        header.set_size(actual_size);
        header.update_op_size(&mut dst[4..8]);

        // TODO update n_fds before writing to dst
        header.update_n_fds(&mut dst[12..16]);

        trace!(
            raw = human_readable_encoded_packet(dst),
            "sending encoded packet"
        );
        Ok(())
    }
}

/// Prints out a buffer as groups of 4 bytes.
fn human_readable_encoded_packet(input: &BytesMut) -> String {
    let mut output = String::new();
    let (chunks, rem) = input.as_chunks::<4>();
    for line in chunks {
        write!(
            output,
            "{:02X} {:02X} {:02X} {:02X} | ",
            line[0], line[1], line[2], line[3]
        )
        .unwrap();
    }
    for byte in rem {
        write!(output, "{byte:02X} ").unwrap();
    }
    output
}
