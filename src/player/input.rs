use super::Player;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, _app: &mut App) {

    }
}

impl Player {
    pub fn init_input(cmd: &mut Commands, player: Entity) {
        cmd.entity(player)
            .insert(InputManagerBundle::<InputAction> {
                input_map: InputMap::new([(KeyCode::Up, InputAction::Jump)]),
                ..Default::default()
            }
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    Move,
    Jump,
}
