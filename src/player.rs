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
            .add_startup_system(init.in_set(PlayerStartupSet::Main));
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

#[derive(SystemSet, Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum PlayerStartupSet {
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
                PlayerSet::Camera,
                PlayerSet::Visuals,
                PlayerSet::Movement,
                PlayerSet::PostPlayer,
            )
                .chain(),
        )
        .add_systems((
            apply_system_buffers.after(PlayerSet::PrePlayer).before(PlayerSet::Main),
            apply_system_buffers.after(PlayerSet::Main).before(PlayerSet::Input),
            apply_system_buffers.after(PlayerSet::Input).before(PlayerSet::StateMachine),
            apply_system_buffers.after(PlayerSet::StateMachine).before(PlayerSet::Camera),
            apply_system_buffers.after(PlayerSet::Camera).before(PlayerSet::Visuals),
            apply_system_buffers.after(PlayerSet::Visuals).before(PlayerSet::Movement),
            apply_system_buffers.after(PlayerSet::Movement).before(PlayerSet::PostPlayer),
            apply_system_buffers.after(PlayerSet::PostPlayer),
        ));

        let startup = match app.get_schedule_mut(CoreSchedule::Startup) {
            Some(schedule) => schedule,
            None => panic!("Error gettings startup schedule!"),
        };

        startup
            .configure_sets(
                (
                    PlayerStartupSet::PrePlayer,
                    PlayerStartupSet::Main,
                    PlayerStartupSet::Input,
                    PlayerStartupSet::StateMachine,
                    PlayerStartupSet::Camera,
                    PlayerStartupSet::Visuals,
                    PlayerStartupSet::Movement,
                    PlayerStartupSet::PostPlayer,
                )
                    .chain(),
            )
        .add_systems((
            apply_system_buffers.after(PlayerStartupSet::PrePlayer).before(PlayerStartupSet::Main),
            apply_system_buffers.after(PlayerStartupSet::Main).before(PlayerStartupSet::Input),
            apply_system_buffers.after(PlayerStartupSet::Input).before(PlayerStartupSet::StateMachine),
            apply_system_buffers.after(PlayerStartupSet::StateMachine).before(PlayerStartupSet::Camera),
            apply_system_buffers.after(PlayerStartupSet::Camera).before(PlayerStartupSet::Visuals),
            apply_system_buffers.after(PlayerStartupSet::Visuals).before(PlayerStartupSet::Movement),
            apply_system_buffers.after(PlayerStartupSet::Movement).before(PlayerStartupSet::PostPlayer),
            apply_system_buffers.after(PlayerStartupSet::PostPlayer),
        ));
    }
}
