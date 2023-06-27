use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod camera;
pub mod input;
pub mod movement;
pub mod state_machine;
pub mod visuals;

struct PlayerPlugins;

impl PluginGroup for PlayerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(input::PlayerInputPlugin)
            .add(state_machine::PlayerStateMachinePlugin)
            .add(movement::PlayerMovementPlugin)
            .add(camera::PlayerCameraPlugin)
            .add(visuals::PlayerVisualsPlugin)
    }
}

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConfigurePlayerSetsPlugin)
            .add_plugins(PlayerPlugins)
            .add_startup_systems(
                (init, apply_system_buffers)
                    .chain()
                    .in_base_set(StartupSet::PreStartup),
            );
    }
}

#[derive(Component)]
pub struct Player;

pub fn init(mut cmd: Commands) {
    cmd.spawn((
        Player,
        Name::from("Player"),
        SpatialBundle::from_transform(Transform::from_xyz(0f32, 100f32, 0f32)),
    ));
}

#[derive(SystemSet, Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum PlayerSet {
    PrePlayer,
    Main,
    Input,
    StateMachine,
    Movement,
    Camera,
    Visuals,
    PostPlayer,
}

struct ConfigurePlayerSetsPlugin;

impl Plugin for ConfigurePlayerSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                PlayerSet::PrePlayer,
                PlayerSet::Main,
                PlayerSet::Input,
                PlayerSet::StateMachine,
                PlayerSet::Movement,
                PlayerSet::Camera,
                PlayerSet::Visuals,
                PlayerSet::PostPlayer,
            )
                .chain(),
        );
    }
}
