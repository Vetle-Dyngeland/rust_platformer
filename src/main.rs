use bevy::{
    app::{AppExit, PluginGroupBuilder},
    prelude::*,
    window::WindowMode,
};
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;
use seldom_state::StateMachinePlugin;
use std::time::Duration;

pub mod level;
pub mod player;

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
            .add(EditorPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100f32))
            .add(ExitPlugin::default())
    }
}

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_plugins(GamePlugins)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0f32, -500f32),
            ..Default::default()
        })
        .add_plugins(OtherPlugins)
        .run();
}

struct ExitPlugin {
    keys: Vec<KeyCode>,
    reset_timer_duration: Duration,
    press_count: usize,
}

impl Default for ExitPlugin {
    fn default() -> Self {
        Self {
            keys: vec![KeyCode::LControl, KeyCode::L],
            reset_timer_duration: Duration::from_secs_f32(0.5f32),
            press_count: 1,
        }
    }
}

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ExitKeys {
            keys: self.keys.clone(),
            reset_timer: Timer::new(self.reset_timer_duration, TimerMode::Once),
            press_count: self.press_count,
        })
        .add_system(exit_system);
    }
}

#[derive(Resource)]
pub struct ExitKeys {
    pub keys: Vec<KeyCode>,
    pub reset_timer: Timer,
    pub press_count: usize,
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    mut keys: ResMut<ExitKeys>,
    mut count: Local<usize>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    keys.reset_timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if keys.reset_timer.just_finished() {
        keys.reset_timer.reset();
        *count = 0
    }

    for key in keys.keys.iter() {
        if !keyboard.pressed(*key) {
            return;
        }
    }

    keys.keys.iter().for_each(|key| {
        if keyboard.just_pressed(*key) {
            *count += 1;
            if *count >= keys.press_count {
                exit.send(AppExit);
            }
            return;
        }
    });
}
