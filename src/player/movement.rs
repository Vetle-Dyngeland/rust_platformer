use super::state_machine::states::*;
use super::{Player, PlayerSet, PlayerStartupSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod sub_components;
use sub_components::*;

pub(super) struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_set(PlayerStartupSet::Movement))
            .add_system(jumping_movement.in_set(PlayerSet::Movement))
            .add_plugin(sub_components::MovementSubComponentsPlugin)
            .register_type::<Surface>()
            .register_type::<SurfaceGroundedChecker>();
    }
}

fn init(mut cmd: Commands, player_query: Query<(Entity, &Sprite), With<Player>>) {
    let (entity, sprite) = player_query.single();
    let size = sprite.custom_size.unwrap_or(Vec2::ONE * 25f32);

    cmd.entity(entity).insert((
        Friction {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min,
        },
        RigidBody::Dynamic,
        ColliderMassProperties::Density(2f32),
        Velocity::default(),
        Collider::cuboid(size.x / 2f32, size.y / 2f32),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        CharacterController::new(size),
    ));
}

#[derive(Component)]
pub struct CharacterController {
    pub jump_force: f32,

    pub surface_checker: SurfaceGroundedChecker,
    size: Vec2,
}

impl CharacterController {
    pub fn new(size: Vec2) -> Self {
        Self {
            surface_checker: SurfaceGroundedChecker::default(),
            size,
            ..Default::default()
        }
    }
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            jump_force: 15000f32,
            surface_checker: SurfaceGroundedChecker::default(),
            size: Vec2::ONE,
        }
    }
}

fn jumping_movement(
    mut player_query: Query<
        (Option<&JumpingState>, &mut Velocity, &CharacterController),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (state, mut vel, controller) = player_query.single_mut();

    if state.is_none() {
        return;
    }

    if controller
        .surface_checker
        .surface_touching_ground(&Surface::Bottom)
    {
        vel.linvel.y += controller.jump_force * time.delta_seconds();
    }
}
