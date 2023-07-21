use bevy::{app::PluginGroupBuilder, prelude::*, window::WindowMode};
use bevy_rapier2d::prelude::*;
use seldom_state::StateMachinePlugin;

pub mod exit;
pub mod level;
pub mod player;

pub const DEBUG: bool = true;

pub const fn debug() -> bool {
    DEBUG
}

struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(level::LevelPlugin)
            .add(player::PlayerPlugin)
    }
}

struct OtherPlugins;

impl PluginGroup for OtherPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StateMachinePlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100f32))
            .add(exit::ExitPlugin::default())
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Platformer".to_string(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            GamePlugins,
            OtherPlugins,
        ))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0f32, -1100f32),
            ..Default::default()
        })
        .run();
}
