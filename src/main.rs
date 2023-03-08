#![allow(non_snake_case)]
//! penrose :: EWMH support
//!
//! It is possible to add EWMH support to penrose via an extension. This provides
//! information to external utilities such as panels and statusbars so that they
//! are able to interact with the window manager.
//!
//! `penrose::extensions::hooks::add_ewmh_hooks` can be used to compose the required
//! hooks into your existing Config before starting the window manager. If you want
//! to modify the support, each of the individual hooks can be found in
//! `penrose::extensions::hooks::ewmh`.
use penrose::{
    builtin::layout::{MainAndStack,transformers::{Gaps, ReflectHorizontal, ReserveTop}},
    core::{
        bindings::parse_keybindings_with_xmodmap,
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::hooks::{add_ewmh_hooks, SpawnOnStartup}, stack,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};
mod keybindgs;
mod layouts;



fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("trace")
        .finish()
        .init();

    // This is all that is needed to add the required hooks to your config.
    // SpawnOnStartup is being used here to start polybar so that the EWMH support
    // can be demonstrated.
    let config = add_ewmh_hooks(Config {
        default_layouts: layouts::get(),
        startup_hook: Some(SpawnOnStartup::boxed("polybar")),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(keybindgs::get())?;
    let wm = WindowManager::new(config,
                                key_bindings,
                                HashMap::new(),
                                conn)?;

    wm.run()
}
