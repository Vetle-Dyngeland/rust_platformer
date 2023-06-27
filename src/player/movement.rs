use std::collections::HashMap;

use crate::level::Ground;

use super::{Player, PlayerSet};
use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::prelude::*;
use super::state_machine::states::*;

pub(super) struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init.in_base_set(StartupSet::PostStartup))
            .add_system(surface_checker.in_set(PlayerSet::PrePlayer))
            .add_system(character_controller.in_set(PlayerSet::Movement));
    }
}

pub fn init(mut cmd: Commands, player_query: Query<(Entity, &Sprite), With<Player>>) {
    let (entity, sprite) = player_query.single();
    let size = sprite.custom_size.unwrap_or(Vec2::new(50f32, 50f32)) / 2f32;

    cmd.entity(entity).insert((
        Friction {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0f32,
            combine_rule: CoefficientCombineRule::Min,
        },
        RigidBody::Dynamic,
        ColliderMassProperties::Density(2f32),
        Velocity::default(),
        Collider::cuboid(size.x, size.y),
        Ccd::enabled(),
        LockedAxes::ROTATION_LOCKED,
        SurfaceGroundedChecker::new(Vec2::ONE * 50f32),
        CharacterController::default()
    ));
}

#[derive(Component)]
pub struct CharacterController {
    jump_force: f32,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            jump_force: 100f32
        }
    }
}

fn character_controller(
    mut player_query: Query<(&mut Velocity, &CharacterController, &SurfaceGroundedChecker), With<Player>>,
    state_query: Query<(Option<&JumpingState>, Option<&FallingState>, Option<&GroundedState>), With<Player>>,
    time: Res<Time>,
) { 
    let surface_grounded_checker = player_query.single().2;
    println!("{}", surface_grounded_checker.surface_touching_ground(&Surface::Bottom));

    match state_query.single() {
        (Some(_), None, None) => jumping_movement(&mut player_query, &time),
        (None, Some(_), None) => {},
        (None, None, Some(_)) => {},
        _ => panic!("\nPlayer was in more states at the same time!\n")
    }
}

fn jumping_movement(player_query: &mut Query<(&mut Velocity, &CharacterController, &SurfaceGroundedChecker), With<Player>>, time: &Res<Time>) {
    let (mut vel, controller, surface_checker) = player_query.single_mut();

    if surface_checker.surface_touching_ground(&Surface::Bottom) {
        vel.linvel.y += controller.jump_force * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct SurfaceGroundedChecker {
    pub size: Vec2,
    touching_surfaces: HashMap<Surface, bool>,
}

impl SurfaceGroundedChecker {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    fn set_surface(&mut self, surface: &Surface, value: bool) {
        self.touching_surfaces.insert(*surface, value);
    }

    pub fn surface_touching_ground(&self, surface: &Surface) -> bool {
        *self.touching_surfaces.get(surface).unwrap()
    }
}

impl Default for SurfaceGroundedChecker {
    fn default() -> Self {
        let mut touching_surfaces = HashMap::new();
        touching_surfaces.insert(Surface::Top, false);
        touching_surfaces.insert(Surface::Bottom, false);
        touching_surfaces.insert(Surface::Left, false);
        touching_surfaces.insert(Surface::Right, false);

        Self {
            size: Vec2::ZERO,
            touching_surfaces
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Surface {
    Top,
    Bottom,
    Left,
    Right,
}

fn surface_checker(
    mut grounded_checker_query: Query<(&mut SurfaceGroundedChecker, &Transform)>,
    ground_query: Query<Entity, (With<Ground>, With<Collider>)>,
    ctx: Res<RapierContext>,
) {
    for (mut checker, transform) in grounded_checker_query.iter_mut() {
        let (pos, size) = (transform.translation, checker.size);

        let shapes: Vec<(Aabb, Surface)> = [
            (Vec2::new(0f32, -1f32), Surface::Bottom),
            (Vec2::new(0f32, 1f32), Surface::Top),
            (Vec2::new(-1f32, 0f32), Surface::Left),
            (Vec2::new(1f32, 0f32), Surface::Right),
        ]
        .iter()
        .map(|(offset, surface)| {
            let v = Vec3::new(
                if offset.x == 0f32 { 0.1f32 } else { offset.x },
                if offset.y == 0f32 { 0.1f32 } else { offset.y },
                0f32,
            );
            let size = Vec3::new(size.x, size.y, 0f32);

            let pos = size / 1.9f32 * v + pos;
            let size = size * v.abs();

            (
                Aabb::from_min_max(pos - size / 2f32, pos + size / 2f32),
                *surface,
            )
        })
        .collect();

        for (shape, surface) in shapes.iter() {
            let mut colliding = false;
            ctx.colliders_with_aabb_intersecting_aabb(*shape, |entity| {
                if ground_query.contains(entity) {
                    colliding = true;
                    return false;
                }
                true
            });

            checker.set_surface(surface, colliding)
        }
    }
}
