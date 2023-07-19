use bevy::prelude::*;

use super::PlayerStartupSet;

pub(super) struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(PlayerStartupSet::Camera));
    }
}

fn init(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        Name::from("Camera"),
    ));
}
