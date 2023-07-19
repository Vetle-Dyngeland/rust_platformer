use super::{Player, PlayerStartupSet};
use bevy::{prelude::*, reflect::TypePath};
use leafwing_input_manager::{prelude::*, axislike::VirtualAxis};

pub(super) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<InputAction>::default())
            .add_systems(Startup, init.in_set(PlayerStartupSet::Input));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single())
        .insert(InputManagerBundle {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(VirtualAxis::horizontal_arrow_keys(), InputAction::Jump)
                .insert(KeyCode::C, InputAction::Jump)
                .build()
        });
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, TypePath)]
pub enum InputAction {
    Run,
    Jump,
}
