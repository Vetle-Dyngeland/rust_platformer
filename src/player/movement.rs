use super::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub(super) struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::PostStartup));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<(Entity, &Sprite), With<Player>>) {
    let (entity, sprite) = player_query.single();
    let size = sprite.custom_size.unwrap_or(Vec2::new(50f32, 50f32)) / 2f32;

    cmd.entity(entity).insert((
        Friction {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min
        },
        Restitution {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min
        },
        RigidBody::Dynamic,
        ColliderMassProperties::Density(2f32),
        Velocity::default(),
        Collider::cuboid(size.x, size.y),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
    ));
}
