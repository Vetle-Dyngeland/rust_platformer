use std::time::Duration;

use super::input::InputAction;
use super::state_machine::states::*;
use super::{Player, PlayerSet, PlayerStartupSet};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod sub_components;
use leafwing_input_manager::prelude::ActionState;
use sub_components::*;

pub(super) struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_set(PlayerStartupSet::Movement))
            .add_systems(
                (controller_jump_timers, jump)
                    .in_set(PlayerSet::Movement)
                    .chain(),
            )
            .add_plugin(sub_components::MovementSubComponentsPlugin)
            .register_type::<CharacterController>();
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
        CharacterController::new(size, 0.175f32, 0.2f32),
    ));
}

#[derive(Component, Clone, Debug, Reflect, FromReflect)]
pub struct CharacterController {
    pub jump_force: f32,
    pub coyote_timer: Timer,
    pub jump_buffer_timer: Timer,

    pub surface_checker: SurfaceGroundedChecker,
    pub size: Vec2,
}

impl CharacterController {
    pub fn new(size: Vec2, coyote_time: f32, jump_buffer_time: f32) -> Self {
        let mut this = Self {
            surface_checker: SurfaceGroundedChecker::default(),
            coyote_timer: Timer::new(Duration::from_secs_f32(coyote_time), TimerMode::Once),
            jump_buffer_timer: Timer::new(
                Duration::from_secs_f32(jump_buffer_time),
                TimerMode::Once,
            ),
            size,
            ..Default::default()
        };

        this.coyote_timer.tick(Duration::MAX);
        this.jump_buffer_timer.tick(Duration::MAX);
        this
    }
}

impl Default for CharacterController {
    fn default() -> Self {
        let mut this = Self {
            jump_force: 15000f32,
            coyote_timer: Timer::new(Duration::from_secs_f32(0.175f32), TimerMode::Once),
            jump_buffer_timer: Timer::new(Duration::from_secs_f32(0.2f32), TimerMode::Once),

            surface_checker: SurfaceGroundedChecker::default(),
            size: Vec2::ONE,
        };

        this.coyote_timer.tick(Duration::MAX);
        this.jump_buffer_timer.tick(Duration::MAX);
        this
    }
}

fn controller_jump_timers(
    mut player_query: Query<(&mut CharacterController, &ActionState<InputAction>), With<Player>>,
    time: Res<Time>,
) {
    let (mut controller, input) = match player_query.get_single_mut() {
        Ok(val) => val,
        Err(message) => {
            println!(
                "Could not get player: Error message: {}",
                message.to_string()
            );
            return;
        }
    };

    if controller
        .surface_checker
        .surface_touching_ground(&Surface::Bottom)
    {
        controller.coyote_timer.unpause();
        controller.coyote_timer.reset();
    }
    controller
        .coyote_timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));
    if input.just_pressed(InputAction::Jump) {
        controller.jump_buffer_timer.unpause();
        controller.jump_buffer_timer.reset();
    }
    controller
        .jump_buffer_timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));
}

/// Happens once, the instant that the jumping requirements are met
fn jump(
    mut player_query: Query<
        (
            Option<&JumpingState>,
            &mut Velocity,
            &mut CharacterController,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (state, mut vel, mut controller) = match player_query.get_single_mut() {
        Ok(val) => val,
        Err(message) => {
            println!(
                "Could not get player in jumping movement. Error message: {}",
                message.to_string()
            );
            return;
        }
    };
    if state.is_none() {
        return;
    }
    controller
        .jump_buffer_timer
        .tick(Duration::from_secs_f32(1000f32));

    controller
        .coyote_timer
        .tick(Duration::from_secs_f32(1000f32));
    vel.linvel.y = controller.jump_force * time.delta_seconds();

    println!("Jumped!")
}
