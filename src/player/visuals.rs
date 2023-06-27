use super::Player;
use bevy::prelude::*;

pub(super) struct PlayerVisualsPlugin;

impl Plugin for PlayerVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::Startup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(match player_query.get_single() {
        Ok(entity) => entity,
        Err(err) => panic!("{}", err.to_string()),
    })
    .insert(Sprite {
        custom_size: Some((25f32, 25f32).into()),
        color: Color::rgb_u8(125, 205, 255),
        ..Default::default()
    });
}
