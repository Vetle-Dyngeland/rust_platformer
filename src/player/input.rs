use super::Player;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<InputAction>::default())
            .add_startup_system(init.in_base_set(StartupSet::Startup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single())
        .insert(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::new([(KeyCode::C, InputAction::Jump)]),
        });
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum InputAction {
    Run,
    Jump,
}
