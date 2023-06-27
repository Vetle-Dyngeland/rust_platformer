use bevy::prelude::*;

pub(super) struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init); 
    }
}

fn init(mut cmd: Commands) {
    cmd.spawn((
        Camera2dBundle {
            ..Default::default()
        },
        Name::from("Camera")
    ));
}
