use bevy_rapier2d::prelude::*;
use bevy::prelude::*;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init);
    }
}

fn init(mut cmd: Commands) {
    cmd.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(255, 55, 150),
                custom_size: Some(Vec2::new(500f32, 10f32)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0f32, -50f32, 0f32),
            ..Default::default()
        },
        Collider::cuboid(250f32, 5f32),
        Ground,
        Name::from("Platform"),
    ));
}

#[derive(Component)]
pub struct Ground;
