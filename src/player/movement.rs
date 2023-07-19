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
        app.add_systems(Startup, init.in_set(PlayerStartupSet::Movement))
            .add_systems(
                Update,
                (controller_jump_variables, jump, fall, horizontal_movement)
                    // NOTE: put horizontal_movement last
                    .in_set(PlayerSet::Movement)
                    .chain(),
            )
            .add_plugins(sub_components::MovementSubComponentsPlugin)
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
        CharacterControllerBuilder {
            size,
            jump_force: 450f32,
            coyote_time: 0.175f32,
            jump_buffer_time: 0.2f32,
            jump_release_multi: 0.3f32,

            max_move_speed: 250f32,
            acceleration_force: 2500f32,
            decceleration_force: 1500f32,
            turnaround_multi: 1.5f32,

            air_control: 0.4f32,
        }
        .build(),
    ));
}

pub struct CharacterControllerBuilder {
    pub size: Vec2,

    pub jump_force: f32,
    pub coyote_time: f32,
    pub jump_buffer_time: f32,
    pub jump_release_multi: f32,

    pub max_move_speed: f32,
    pub acceleration_force: f32,
    pub decceleration_force: f32,
    pub turnaround_multi: f32,

    pub air_control: f32,
}

impl CharacterControllerBuilder {
    pub fn build(self) -> CharacterController {
        CharacterController {
            size: self.size,

            jump_force: self.jump_force,
            coyote_timer: Timer::new(Duration::from_secs_f32(self.coyote_time), TimerMode::Once),
            jump_buffer_timer: Timer::new(
                Duration::from_secs_f32(self.jump_buffer_time),
                TimerMode::Once,
            ),
            has_released_jump: true,
            jump_release_multi: self.jump_release_multi,

            max_move_speed: self.max_move_speed,
            acceleration_force: self.acceleration_force,
            decceleration_force: self.decceleration_force,
            turnaround_multi: self.turnaround_multi,

            air_control: self.air_control,

            surface_checker: SurfaceGroundedChecker::default(),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct CharacterController {
    pub jump_force: f32,
    pub coyote_timer: Timer,
    pub jump_buffer_timer: Timer,
    pub has_released_jump: bool,
    pub jump_release_multi: f32,

    pub max_move_speed: f32,
    pub acceleration_force: f32,
    pub decceleration_force: f32,
    pub turnaround_multi: f32,

    pub air_control: f32,

    pub surface_checker: SurfaceGroundedChecker,
    pub size: Vec2,
}

fn horizontal_movement(
    mut player_query: Query<
        (
            &CharacterController,
            &mut Velocity,
            &ActionState<InputAction>,
        ),
        With<Player>,
    >,
    time: Res<Time>,
) {
    let (controller, mut vel, input) = player_query.single_mut();

    let move_val = input.value(InputAction::Run);

    let grounded = controller
        .surface_checker
        .surface_touching_ground(&Surface::Bottom);

    let air_control_multi = if !grounded {
        controller.air_control
    } else {
        1f32
    };
    let turnaround_multi = if move_val != vel.linvel.x.signum() {
        controller.turnaround_multi
    } else {
        1f32
    };

    let add_val = controller.acceleration_force
        * time.delta_seconds()
        * move_val
        * turnaround_multi
        * air_control_multi;

    if (vel.linvel.x + add_val).abs() > controller.max_move_speed {
        vel.linvel.x = controller.max_move_speed * add_val.signum();
    } else {
        vel.linvel.x += add_val;
    }

    if move_val.abs() > 0f32 && vel.linvel.x > 0f32 || !grounded {
        return;
    }

    // Deccelerate
    let sub_val = controller.decceleration_force
        * time.delta_seconds()
        * vel.linvel.x.signum();

    if (vel.linvel.x - sub_val).signum() != vel.linvel.x.signum() {
        vel.linvel.x = 0f32;
    } else {
        vel.linvel.x -= sub_val
    }
}

fn controller_jump_variables(
    mut player_query: Query<(&mut CharacterController, &ActionState<InputAction>), With<Player>>,
    time: Res<Time>,
) {
    let (mut controller, input) = player_query.single_mut();

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

fn fall(
    mut player_query: Query<
        (
            Option<&FallingState>,
            &mut Velocity,
            &mut CharacterController,
            &ActionState<InputAction>,
        ),
        With<Player>,
    >,
) {
    let (mut vel, mut controller, input) = match player_query.single_mut() {
        (Some(_), v, c, i) => (v, c, i),
        _ => return,
    };

    controller.has_released_jump = controller.has_released_jump || vel.linvel.y < 0f32;
    if controller.has_released_jump {
        return;
    }

    if input.released(InputAction::Jump) {
        controller.has_released_jump = true;
        vel.linvel.y *= controller.jump_release_multi;
    }
}

fn jump(
    mut player_query: Query<
        (
            Option<&JumpingState>,
            &mut Velocity,
            &mut CharacterController,
        ),
        With<Player>,
    >,
) {
    let (force_multi, mut vel, mut controller) = match player_query.single_mut() {
        (Some(state), v, c) => (state.0, v, c),
        _ => return,
    };

    controller.has_released_jump = false;
    controller
        .jump_buffer_timer
        .tick(Duration::from_secs_f32(1000f32));
    controller
        .coyote_timer
        .tick(Duration::from_secs_f32(1000f32));

    vel.linvel.y = controller.jump_force * force_multi;
    if vel.linvel.x.abs() > 0f32 {
        vel.linvel.x *= 1.1f32
    }
}
