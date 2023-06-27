use super::{input::InputAction, Player};
use bevy::prelude::*;

pub mod triggers;
use seldom_state::prelude::*;
use triggers::*;
use states::*;

pub(super) struct PlayerStateMachinePlugin;

impl Plugin for PlayerStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::PostStartup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single()).insert((
        GroundedState::Idle,
        StateMachine::default()
            .trans::<FallingState>(GroundedTrigger, GroundedState::Idle)
            .trans::<FallingState>(FallingTrigger.not(), JumpingState)
            .trans::<JumpingState>(GroundedTrigger, GroundedState::Idle)
            .trans::<JumpingState>(FallingTrigger, FallingState)
            .trans::<GroundedState>(JustPressedTrigger(InputAction::Jump), JumpingState)
            .trans::<GroundedState>(GroundedTrigger.not().and(FallingTrigger), FallingState)
            .trans::<GroundedState>(
                GroundedTrigger.not().and(FallingTrigger.not()),
                JumpingState,
            )
            .trans_builder(
                ValueTrigger::unbounded(InputAction::Run),
                |_: &GroundedState, value| {
                    Some(match value {
                        value if value > 0.5f32 => GroundedState::WalkingRight,
                        value if value < 0.5f32 => GroundedState::WalkingLeft,
                        _ => GroundedState::Idle,
                    })
                },
            ),
    ));
}

pub mod states {
    use bevy::prelude::*;

    #[derive(Clone, Copy, Component, Reflect)]
    #[component(storage = "SparseSet")]
    pub enum GroundedState {
        WalkingLeft = -1,
        Idle = 0,
        WalkingRight = 1,
    }

    #[derive(Clone, Copy, Component, Reflect)]
    #[component(storage = "SparseSet")]
    pub struct JumpingState;

    #[derive(Clone, Copy, Component, Reflect)]
    #[component(storage = "SparseSet")]
    pub struct FallingState;
}
