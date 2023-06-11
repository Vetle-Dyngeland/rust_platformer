use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;
use super::{Player, input::InputAction};

pub(super) struct PlayerStateMachinePlugin;

impl Plugin for PlayerStateMachinePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

impl Player {
    pub fn init_state_machine(cmd: &mut Commands, player: Entity) {

    }
}
