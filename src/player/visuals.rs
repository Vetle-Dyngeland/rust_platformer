use super::{Player, PlayerStartupSet};
use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};

pub(super) struct PlayerVisualsPlugin;

impl Plugin for PlayerVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerStartupSet::Visuals));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<Entity, With<Player>>) {
    cmd.entity(player_query.single()).insert((
        Sprite {
            custom_size: Some((25f32, 25f32).into()),
            color: Color::rgb_u8(125, 205, 255),
            ..Default::default()
        },
        Handle::<Image>::from(DEFAULT_IMAGE_HANDLE.typed()),
    ));
}
