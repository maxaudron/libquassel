use std::thread;

use druid::ExtEventSink;
use futures::StreamExt;
use libquassel::frame::QuasselCodec;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_util::codec::Framed;
use tracing::debug;

use crate::{
    server::{ClientState, Server},
    StateTracker,
};

impl StateTracker {
    pub fn connect(&mut self, ctx: ExtEventSink) {
        debug!("starting connect");

        let server = self.server.clone();

        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();

            rt.block_on(async move {
                let mut s_server = TcpStream::connect(format!(
                    "{}:{}",
                    server.settings.host, server.settings.port
                ))
                .await
                .unwrap();

                let _connack = server.init(&mut s_server).await.unwrap();

                let codec = QuasselCodec::builder().compression(false).new_codec();
                let framed = Framed::new(s_server, codec);
                let (s_sink, s_stream) = framed.split();

                let listener = TcpListener::bind((server.listen_host, server.listen_port))
                    .await
                    .unwrap();
                let (mut client, _) = listener.accept().await.unwrap();

                //
                // Setup Listener
                //

                {
                    let (mut c_stream, mut c_sink) = client.split();

                    let mut init = [0; 12];
                    let n = c_stream.peek(&mut init).await.unwrap();
                    c_stream.read(&mut init[..n]).await.unwrap();
                    let init = libquassel::message::Init::parse(&init);
                    debug!("send init bytes: {:?}", init);

                    c_sink.write(&[0x0, 0x0, 0x0, 0x2]).await.unwrap();
                }

                let codec = QuasselCodec::builder().compression(false).new_codec();
                let framed = Framed::new(client, codec);
                let (c_sink, c_stream) = framed.split();

                // Start Processing

                let s_state = ClientState::Handshake;
                let c_state = ClientState::Handshake;

                tokio::join!(
                    Server::run(s_stream, c_sink, s_state, ctx.clone(), "server -> client"),
                    Server::run(c_stream, s_sink, c_state, ctx.clone(), "client -> server")
                );
            });
        });
    }
}
