use anyhow::{bail, Error};

use std::convert::TryFrom;

use druid::{
    widget::{
        Align, Button, Checkbox, Container, Controller, ControllerHost, Flex, Label, TextBox,
    },
    Command, Data, ExtEventSink, Lens, SingleUse, Target, Widget, WidgetExt,
};

use libquassel::{
    deserialize::Deserialize,
    frame::QuasselCodec,
    message::{self, objects, ConnAck, HandshakeMessage, Init},
    primitive::VariantMap,
};

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
};
use tokio_util::codec::Framed;

use tracing::{debug, info, trace};

use crate::{command, formatter};

#[derive(Clone, Debug, Data, Lens)]
pub struct ServerSettings {
    pub tls: bool,
    pub compression: bool,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Default for ServerSettings {
    fn default() -> Self {
        ServerSettings {
            tls: false,
            compression: false,
            host: String::from("localhost"),
            port: 4242,
            username: String::default(),
            password: String::default(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct Server {
    pub server_name: String,
    pub listen_port: u16,
    pub listen_host: String,
    pub settings: ServerSettings,
}

#[derive(Debug)]
pub enum ClientState {
    Handshake,
    Connected,
}

#[derive(Clone, Debug)]
pub enum Message {
    Handshake(HandshakeMessage),
    SignalProxy(message::Message),
}

impl Data for Message {
    fn same(&self, other: &Self) -> bool {
        if let Self::Handshake(_) = self {
            if let Self::Handshake(_) = other {
                return true;
            } else {
                return false;
            }
        } else if let Self::SignalProxy(_) = self {
            if let Self::SignalProxy(_) = other {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

impl Server {
    pub async fn init(
        &self,
        stream: &mut (impl AsyncRead + AsyncWrite + Unpin),
    ) -> Result<ConnAck, Error> {
        let init = Init::new()
            .tls(self.settings.tls)
            .compression(self.settings.compression);

        stream.write(&init.serialize()).await?;

        let mut buf = [0; 4];
        stream.read(&mut buf).await?;

        let (_, connack) = ConnAck::parse(&buf).unwrap();
        Ok(connack)
    }

    pub async fn run(
        mut stream: SplitStream<Framed<TcpStream, QuasselCodec>>,
        mut sink: SplitSink<Framed<TcpStream, QuasselCodec>, Vec<u8>>,
        mut state: ClientState,
        ctx: ExtEventSink,
        direction: &str,
    ) {
        // Start event loop
        while let Some(msg) = stream.next().await {
            let msg = msg.unwrap();
            sink.send(msg.to_vec()).await.unwrap();
            let msg = match state {
                ClientState::Handshake => {
                    Server::handle_login_message(&msg, &mut state, direction, ctx.clone())
                        .await
                        .unwrap()
                }
                ClientState::Connected => Server::handle_message(&msg, direction, ctx.clone())
                    .await
                    .unwrap(),
            };

            ctx.submit_command(command::ADD_MESSAGE, SingleUse::new(msg), Target::Global)
                .unwrap();
        }
    }

    async fn handle_login_message(
        buf: &[u8],
        state: &mut ClientState,
        direction: &str,
        _ctx: ExtEventSink,
    ) -> Result<Message, Error> {
        use libquassel::HandshakeDeserialize;

        trace!(target: "handshakemessage", "Received bytes: {:x?}", buf);
        match HandshakeMessage::parse(buf) {
            Ok((_size, res)) => {
                info!("{}: {:#?}", direction, res);

                match res {
                    HandshakeMessage::SessionInit(_) => *state = ClientState::Connected,
                    HandshakeMessage::ClientLogin(_) => *state = ClientState::Connected,
                    _ => {}
                }

                return Ok(Message::Handshake(res));
            }
            Err(e) => bail!("failed to parse handshake message {}", e),
        }
    }

    async fn handle_message(
        buf: &[u8],
        _direction: &str,
        ctx: ExtEventSink,
    ) -> Result<Message, Error> {
        use libquassel::deserialize::*;
        trace!(target: "message", "Received bytes: {:x?}", buf);

        match message::Message::parse(buf) {
            Ok((_size, res)) => {
                let re = res.clone();

                #[allow(unused_variables)]
                match res {
                    message::Message::SyncMessage(msg) => match msg.class_name.as_str() {
                        "AliasManager" => match msg.slot_name.as_str() {
                            "update" => ctx
                                .submit_command(
                                    command::ALIASMANAGER_UPDATE,
                                    SingleUse::new(msg),
                                    Target::Global,
                                )
                                .unwrap(),
                            _ => (),
                        },
                        _ => (),
                    },
                    message::Message::RpcCall(msg) => (),
                    message::Message::InitRequest(msg) => (),
                    message::Message::InitData(msg) => match msg.init_data {
                        objects::Types::AliasManager(alias_manager) => ctx
                            .submit_command(
                                command::ALIASMANAGER_INIT,
                                SingleUse::new(alias_manager),
                                Target::Global,
                            )
                            .unwrap(),
                        _ => (),
                    },
                    message::Message::HeartBeat(msg) => (),
                    message::Message::HeartBeatReply(msg) => (),
                }

                return Ok(Message::SignalProxy(re));
            }
            Err(e) => {
                bail!("failed to parse message {}", e);
            }
        }
    }
}

impl Default for Server {
    fn default() -> Self {
        Server {
            server_name: String::default(),
            listen_port: 4243,
            listen_host: String::from("localhost"),
            settings: ServerSettings::default(),
        }
    }
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Server");
        fmt.field("settings", &self.settings);
        fmt.field("name", &self.server_name).finish()
    }
}

pub struct ServerWidget {}

impl ServerWidget {
    pub fn new() -> ControllerHost<impl Widget<Server>, ServerWidget> {
        let layout = Flex::column()
            .with_child(Label::new("Connect").align_left())
            .with_spacer(crate::SPACING)
            .with_child(
                TextBox::new()
                    .with_placeholder("Server Name")
                    .expand_width()
                    .lens(Server::server_name),
            )
            .with_spacer(crate::SPACING)
            .with_child(
                Container::new(
                    Flex::column()
                        .with_child(
                            Flex::row()
                                .with_flex_child(
                                    TextBox::new()
                                        .with_placeholder("Host")
                                        .expand_width()
                                        .lens(ServerSettings::host),
                                    2.0,
                                )
                                .with_spacer(crate::SPACING)
                                .with_flex_child(
                                    TextBox::new()
                                        .with_formatter(formatter::U16Formatter)
                                        .lens(ServerSettings::port),
                                    1.0,
                                )
                                .expand_width(),
                        )
                        .with_spacer(crate::SPACING)
                        .with_child(
                            Flex::row()
                                .with_child(Checkbox::new("TLS").lens(ServerSettings::tls))
                                .with_flex_spacer(1.0)
                                .with_child(
                                    Checkbox::new("Compression").lens(ServerSettings::compression),
                                ),
                        )
                        .with_spacer(crate::SPACING * 2.0)
                        .with_child(Label::new("Login").align_left())
                        .with_spacer(crate::SPACING)
                        .with_child(
                            TextBox::new()
                                .with_placeholder("Username")
                                .expand_width()
                                .lens(ServerSettings::username),
                        )
                        .with_spacer(crate::SPACING)
                        .with_child(
                            TextBox::new()
                                .with_placeholder("Password")
                                .expand_width()
                                .lens(ServerSettings::password),
                        ),
                )
                .lens(Server::settings),
            )
            .with_spacer(crate::SPACING)
            .with_child(Label::new("Listen").align_left())
            .with_spacer(crate::SPACING)
            .with_child(
                Flex::row()
                    .with_flex_child(
                        TextBox::new()
                            .with_placeholder("Address")
                            .expand_width()
                            .lens(Server::listen_host),
                        2.0,
                    )
                    .with_spacer(crate::SPACING)
                    .with_flex_child(
                        TextBox::new()
                            .with_placeholder("Port")
                            .with_formatter(formatter::U16Formatter)
                            .lens(Server::listen_port),
                        1.0,
                    )
                    .expand_width(),
            )
            .with_spacer(crate::SPACING)
            .with_child(
                Button::new("Connect")
                    .on_click(move |ctx, _, _| {
                        debug!("connect button pressed, sending command");
                        ctx.submit_command(Command::new(
                            command::CONNECT,
                            (),
                            druid::Target::Global,
                        ))
                    })
                    .align_right(),
            );

        let widget = Align::centered(layout);

        ControllerHost::new(widget, ServerWidget {})
    }
}

impl<T, W: Widget<T>> Controller<T, W> for ServerWidget {}
