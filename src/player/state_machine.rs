use super::{input::InputAction, Player, PlayerStartupSet};
use bevy::prelude::*;

pub mod triggers;
use seldom_state::prelude::*;
use states::*;
use triggers::*;

pub(super) struct PlayerStateMachinePlugin;

impl Plugin for PlayerStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_set(PlayerStartupSet::StateMachine));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single()).insert((
        GroundedState::Idle,
        StateMachine::default()
            .trans::<JumpingState>(AlwaysTrigger, FallingState)
            .trans::<FallingState>(GroundedTrigger, GroundedState::Idle)
            .trans::<FallingState>(JumpTrigger, JumpingState)
            .trans::<GroundedState>(JumpTrigger, JumpingState)
            .trans::<GroundedState>(GroundedTrigger.not().and(FallingTrigger), FallingState)
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
