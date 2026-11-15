use std::{
    collections::BTreeMap,
    env, fmt, io,
    path::PathBuf,
    sync::atomic::{AtomicU32, Ordering},
};

use anyhow::{Context, Result, anyhow};
use bytes::BytesMut;
use futures::{SinkExt, StreamExt, channel::oneshot};
use spa::{EncodePod, PodBuf};
use tokio::net::UnixStream;
use tokio_util::codec::Framed;
use tracing::{debug, info};

use crate::{
    codec::{PwCodec, PwFrame},
    messages::{
        client::UpdateProperties,
        core::{Hello, Sync},
    },
};

mod codec;

mod messages;

#[cfg(target_pointer_width = "16")]
compile_error!("expect at least 32-bit width pointers");

const DEFAULT_SOCKET_NAME: &str = "pipewire-0";

#[derive(Debug)]
pub struct Pipewire {
    conn: Framed<UnixStream, PwCodec>,
    /// Seq number of next message
    seq: AtomicU32,
}

impl Pipewire {
    fn new(conn: Framed<UnixStream, PwCodec>) -> Self {
        Self {
            conn,
            seq: AtomicU32::new(0),
        }
    }
    pub async fn connect() -> Result<Self> {
        let socket = connect_to_pipewire_socket(DEFAULT_SOCKET_NAME).await?;
        let conn = Framed::new(socket, PwCodec::new());
        let mut this = Self::new(conn);

        let _ = this.send_msg(&Hello::default()).await?;

        let mut props = BTreeMap::new();
        props.insert("application.name".into(), "my-first-pipewire".into());
        this.send_msg(&UpdateProperties { properties: props })
            .await?;

        this.sync().await?;

        Ok(this)
    }

    /// Awaiting this future doesn't return the response - it returns an object that can be awaited
    /// to get the response.
    pub async fn send_msg<M>(&mut self, msg: &M) -> io::Result<()>
    where
        M: PwMethod,
    {
        self.conn.send(msg).await
    }

    pub async fn sync(&mut self) -> io::Result<()> {
        let seq = self.next_seq();
        self.conn
            .send(&Sync {
                id: 0,
                seq: seq as i32,
            })
            .await?;

        // TODO create an object to route responses
        while let Some(msg) = self.conn.next().await {
            let msg = msg?;
            if let Some(core_event) = msg.try_decode_core().unwrap() {
                dbg!(core_event);
            }
        }
        Ok(())
    }

    fn next_seq(&self) -> u32 {
        // TODO It's probably OK for this to be less stringent
        self.seq.fetch_add(1, Ordering::SeqCst)
    }
}

async fn connect_to_pipewire_socket(socket_name: &str) -> Result<UnixStream> {
    for test_dir in ["PIPEWIRE_RUNTIME_DIR", "XDG_RUNTIME_DIR", "USERPROFILE"] {
        match try_socket_location(test_dir, socket_name).await {
            Ok(socket) => return Ok(socket),
            Err(e) => {
                info!("{e}, will try next possible location");
            }
        }
    }
    Err(anyhow!(
        "could not connect to pipewire socket after trying all socket locations"
    ))
}

async fn try_socket_location(directory_env: &str, socket_name: &str) -> Result<UnixStream> {
    let dir = env::var(directory_env)
        .with_context(|| format!("could not read environment variable \"{directory_env}\""))?;
    let mut path = PathBuf::from(dir);
    path.push(socket_name);
    debug!("trying to connect to socket at {}", path.display());
    let socket = UnixStream::connect(&path)
        .await
        .with_context(|| format!("could not connect to socket at {}", path.display()))?;
    info!("connected to pipewire socket at {}", path.display());
    Ok(socket)
}

/// A trait for objects that can be sent as pipewire messages
pub trait PwMethod: EncodePod + fmt::Debug {
    /// The opcode
    fn opcode(&self) -> u8;
    /// The target ID
    fn id(&self) -> u32;
}
