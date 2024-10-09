#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod camera;
pub mod character;
pub mod debug_ui;
pub mod input;
pub mod player;
pub mod weapon;

pub mod prelude {
    pub use bevy::ecs as bevy_ecs;
    pub use bevy::prelude::*;
    pub use derive_more::derive::Constructor;
}
use debug_ui::DebugUiPlugin;
use player::PlayerPlugin;
use prelude::*;

use camera::FirstPersonCameraPlugin;
use clap::{ArgAction, Parser};
use input::GameInputPlugin;

pub fn create_app(info: GameInfo) -> App {
    let mut app = App::new();

    app.insert_resource(info);
    app.add_plugins((DefaultPlugins.set(bevy::window::WindowPlugin {
        primary_window: Some(Window {
            title: info.name.to_string(),
            ..default()
        }),
        ..default()
    }),));
    app.add_plugins((PlayerPlugin, GameInputPlugin, FirstPersonCameraPlugin));

    let args = EngineArgs::parse();
    if args.show_game_info_overlay {
        app.add_systems(Startup, spawn_info_overlay);
    }
    if args.enable_debug_ui {
        app.add_plugins(DebugUiPlugin);
    }

    app
}

#[derive(Resource, Clone, Copy)]
pub struct GameInfo {
    pub name: &'static str,
    pub version: Option<&'static str>,
}

fn spawn_info_overlay(mut commands: Commands, info: Res<GameInfo>) {
    let game_info = info
        .version
        .map_or(info.name.to_string(), |v| format!("{} {}", info.name, v));
    let engine_info = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    commands.spawn(
        TextBundle::from_section(
            format!("{} ({})", game_info, engine_info),
            TextStyle::default(),
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(4.0),
            right: Val::Px(4.0),
            ..default()
        }),
    );
}

#[derive(Parser)]
#[command(version)]
struct EngineArgs {
    #[arg(
        short = 'i',
        long = "info-overlay",
        help = "Show game info overlay",
        action = ArgAction::Set,
        default_value_t = true,
    )]
    pub show_game_info_overlay: bool,
    #[arg(short = 'u', long = "debug-ui", help = "Enable debug UI")]
    pub enable_debug_ui: bool,
}
