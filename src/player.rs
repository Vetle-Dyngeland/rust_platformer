use bevy::{prelude::*, app::PluginGroupBuilder};
use bevy_rapier2d::prelude::*;

pub mod input;
pub mod state_machine;

struct PlayerModulePlugins;

impl PluginGroup for PlayerModulePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(input::PlayerInputPlugin)
            .add(state_machine::PlayerStateMachinePlugin)
    }
}

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (
                PlayerStartupSet::Main,
                PlayerStartupSet::Input,
                PlayerStartupSet::StateMachine,
                PlayerStartupSet::Movement,
            )
                .chain()
                .in_base_set(StartupSet::Startup),
        ).configure_sets(
            (
                PlayerSet::PrePlayer,
                PlayerSet::Main,
                PlayerSet::Input,
                PlayerSet::StateMachine,
                PlayerSet::Movement,
                PlayerSet::Visuals,
                PlayerSet::PostPlayer,
            )
                .chain()
        ).add_system(init.in_set(PlayerStartupSet::Main));
    }
}

#[derive(SystemSet, Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum PlayerStartupSet {
    Main,
    Input,
    StateMachine,
    Movement,
}

#[derive(SystemSet, Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum PlayerSet {
    PrePlayer,
    Main,
    Input,
    StateMachine,
    Movement,
    Visuals,
    PostPlayer,
}

#[derive(Component)]
pub struct Player;

fn init(mut cmd: Commands) {
    let friction = Friction {
        combine_rule: CoefficientCombineRule::Min,
        coefficient: 0f32,
    };

    let restitution = Restitution {
        coefficient: 0f32,
        combine_rule: CoefficientCombineRule::Min,
    };

    let player = cmd.spawn((
        // VISUALS: MOVE TO VISUALS FILE LATER LATER
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(25, 78, 255),
                custom_size: Some(Vec2::new(50f32, 50f32)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0f32, 100f32, 0f32),
            ..Default::default()
        },
        // PHYSICS: MOVE TO MOVEMENT FILE LATER LATER
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::cuboid(25f32, 25f32),
        LockedAxes::ROTATION_LOCKED,
        ColliderMassProperties::Density(2f32),
        friction,
        restitution,
        // PLAYER
        Player,
        Name::from("Player")
    )).id();

    Player::init_input(&mut cmd, player);
    Player::init_state_machine(&mut cmd, player);
}
