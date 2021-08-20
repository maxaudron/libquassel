#![feature(type_name_of_val)]

use std::sync::Arc;

use aliasmanager::AliasManagerWidget;
use druid::{
    widget::{Align, Either, Flex, Label, List, Split},
    AppDelegate, Command,
};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use libquassel::message::objects::AliasManager;
use tracing::debug;

use crate::server::{Message, ServerWidget};

const SPACING: f64 = 10.0;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const WINDOW_TITLE: LocalizedString<StateTracker> = LocalizedString::new("StateTracker");

mod command;
mod connect;
mod formatter;
mod server;

mod aliasmanager;

#[derive(Clone, Data, Lens)]
struct StateTracker {
    server: server::Server,
    messages: Arc<Vec<server::Message>>,
    alias_manager: Arc<AliasManager>,
    connected: bool,
    test: i32,
}

impl StateTracker {
    fn new() -> StateTracker {
        StateTracker {
            server: server::Server::default(),
            messages: Arc::new(Vec::new()),
            alias_manager: Arc::new(AliasManager {
                aliases: Vec::new(),
            }),
            connected: false,
            test: 0,
        }
    }

    fn widget() -> impl Widget<StateTracker> {
        let either = Either::new(
            |server, _env| server.connected,
            Split::columns(
                Flex::column()
                    .with_child(Label::new("AliasManager"))
                    .with_child(AliasManagerWidget::new().lens(StateTracker::alias_manager)),
                List::new(|| {
                    Label::new(|item: &Message, _env: &_| format!("{:#?}", item)).padding(10.0)
                })
                .scroll()
                .vertical()
                .lens(StateTracker::messages),
            )
            .expand(),
            ServerWidget::new()
                .fix_width(200.0)
                .lens(StateTracker::server),
        );

        let layout = Flex::column()
            .with_flex_child(either, 1.0)
            .with_spacer(VERTICAL_WIDGET_SPACING);

        Align::centered(layout)
    }
}

struct StateTrackerDelegate;
impl AppDelegate<StateTracker> for StateTrackerDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &Command,
        data: &mut StateTracker,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(_) = cmd.get(command::CONNECT) {
            debug!("got CONNECT command");

            data.connect(ctx.get_external_handle());
            data.connected = true;

            return druid::Handled::Yes;
        } else if let Some(msg) = cmd.get(command::ADD_MESSAGE) {
            debug!("got ADD_MESSAGE command");

            let list = Arc::make_mut(&mut data.messages);
            list.push(msg.clone());
        } else if let Some(alias) = cmd.get(command::ALIASMANAGER_ADD_ALIAS) {
            let mut alias_manager = Arc::make_mut(&mut data.alias_manager).clone();
            alias_manager.add_alias(alias.to_owned());
            data.alias_manager = Arc::new(alias_manager);
        } else if let Some(alias_manager) = cmd.get(command::ALIASMANAGER_INIT) {
            data.alias_manager = Arc::new(alias_manager.to_owned());
        }

        druid::Handled::No
    }
}

fn main() {
    // pretty_env_logger::init();

    // describe the main window
    let main_window = WindowDesc::new(StateTracker::widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = StateTracker::new();

    // let state = Arc::new(RwLock::new(initial_state));

    // start the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .delegate(StateTrackerDelegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
