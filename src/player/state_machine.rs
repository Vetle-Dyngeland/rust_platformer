use super::Player;
use bevy::prelude::*;

pub(super) struct PlayerStateMachinePlugin;

impl Plugin for PlayerStateMachinePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::Startup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {

}
