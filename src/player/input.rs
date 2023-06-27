use super::Player;
use bevy::prelude::*;

pub(super) struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::Startup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {

}
