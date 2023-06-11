use bevy::prelude::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
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
