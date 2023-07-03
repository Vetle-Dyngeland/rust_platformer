use bevy::{prelude::*, render::primitives::Aabb};
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

use crate::{
    level::Ground,
    player::{movement::CharacterController, Player, PlayerSet},
};

const DEBUG_SURFACE_CHECKER_ENABLED: bool = true;

pub(super) struct MovementSubComponentsPlugin;

impl Plugin for MovementSubComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            add_debug_sprites
                .in_set(PlayerSet::PostPlayer)
                .run_if(debug_surface_checker_enabled),
        )
        .add_system(
            debug_surface_checker
                .in_set(PlayerSet::Visuals)
                .run_if(debug_surface_checker_enabled),
        )
        .add_system(surface_checker.in_set(PlayerSet::PrePlayer));
    }
}

const fn debug_surface_checker_enabled() -> bool {
    DEBUG_SURFACE_CHECKER_ENABLED
}

fn add_debug_sprites(
    mut cmd: Commands,
    player: Query<(Entity, &CharacterController), With<Player>>,
) {
    let (entity, controller) = match player.get_single() {
        Ok((e, c)) => (e, c),
        Err(err) => {
            println!(
                "\n\n\nCould not get player entity. Error provided: {}",/-
                err.to_string()
            );
            return;
        }
    };

    let mut generate_child = |surface: Surface| -> Entity {
        cmd.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(match surface {
                        Surface::Top | Surface::Bottom => Vec2::new(controller.size.x, 10f32),
                        Surface::Left | Surface::Right => Vec2::new(10f32, controller.size.y),
                    }),
                    color: Color::WHITE,
                    ..Default::default()
                },
                transform: Transform {
                    translation: match surface {
                        Surface::Top => Vec3::Y * controller.size.y / 2f32,
                        Surface::Bottom => Vec3::NEG_Y * controller.size.y / 2f32,
                        Surface::Right => Vec3::X * controller.size.x / 2f32,
                        Surface::Left => Vec3::NEG_X * controller.size.x / 2f32,
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            DebugSurfaceChecker(surface),
            Name::from(format!("{surface:?}")),
        ))
        .id()
    };

    let children = [
        generate_child(Surface::Top),
        generate_child(Surface::Bottom),
        generate_child(Surface::Left),
        generate_child(Surface::Top),
    ];

    cmd.entity(entity).push_children(&children);
}

#[derive(Component)]
struct DebugSurfaceChecker(Surface);

fn debug_surface_checker(
    controller_query: Query<&CharacterController, With<Player>>,
    mut debug_query: Query<(&mut Sprite, &DebugSurfaceChecker)>,
) {
    let surface_checker = match controller_query.get_single() {
        Ok(controller) => &controller.surface_checker,
        Err(err) => {
            println!(
                "Could not get CharacterController, message: {}",
                err.to_string()
            );
            return;
        }
    };

    for (mut sprite, surface) in debug_query.iter_mut() {
        let color = match surface_checker.touching_surfaces.get(&surface.0) {
            Some(s) => match s {
                &true => Color::LIME_GREEN,
                _ => Color::RED,
            },
            None => {
                panic!("Surface checker did not contain surface: {:?}", surface.0);
            }
        };

        sprite.color = color;
    }
}

#[derive(Clone, Debug, Reflect, FromReflect)]
pub struct SurfaceGroundedChecker {
    touching_surfaces: HashMap<Surface, bool>,
}

impl SurfaceGroundedChecker {
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

        Self { touching_surfaces }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Reflect, FromReflect)]
pub enum Surface {
    Top,
    Bottom,
    Left,
    Right,
}

fn surface_checker(
    mut grounded_checker_query: Query<(&mut CharacterController, &Transform)>,
    ground_query: Query<Entity, (With<Ground>, With<Collider>)>,
    ctx: Res<RapierContext>,
) {
    for (mut controller, transform) in grounded_checker_query.iter_mut() {
        let (pos, size) = (transform.translation, controller.size);

        let shapes: Vec<(Aabb, Surface)> = [
            (Vec2::NEG_Y, Surface::Bottom),
            (Vec2::Y, Surface::Top),
            (Vec2::NEG_X, Surface::Left),
            (Vec2::X, Surface::Right),
        ]
        .iter()
        .map(|(offset, surface)| {
            let offset = Vec3::new(offset.x, offset.y, 0f32);
            let size = Vec3::new(size.x, size.y, 0f32);
            let pos = pos + offset;

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

            controller.surface_checker.set_surface(surface, colliding)
        }
    }
}
