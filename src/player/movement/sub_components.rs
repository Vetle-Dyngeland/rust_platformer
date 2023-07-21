use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;
use std::{collections::HashMap, hash::Hash};

use crate::{
    debug,
    level::Ground,
    player::{movement::CharacterController, Player, PlayerStartupSet},
    DEBUG,
};

pub(super) struct MovementSubComponentsPlugin;

impl Plugin for MovementSubComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            spawn_grounded_checkers.after(PlayerStartupSet::Movement),
        )
        .add_systems(
            PreUpdate,
            (surface_checker, debug_surface_checker.run_if(debug)).chain(),
        )
        .add_event::<ActivateGroundedDelay>()
        .register_type::<Surface>()
        .register_type::<SurfaceGroundedChecker>();
    }
}

fn spawn_grounded_checkers(
    mut cmd: Commands,
    player_query: Query<(Entity, &CharacterController), With<Player>>,
) {
    let (entity, controller) = player_query.single();
    let size_div = 2f32;

    let mut generate_child = |surface: Surface| -> Entity {
        let size = match surface {
            Surface::Top | Surface::Bottom => Vec2::new(controller.size.x / size_div, 1f32),
            Surface::Left | Surface::Right => Vec2::new(1f32, controller.size.y / size_div),
        };
        let child = cmd
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(
                        match surface {
                            Surface::Top => Vec3::Y * controller.size.y / 2f32,
                            Surface::Bottom => Vec3::NEG_Y * controller.size.y / 2f32,
                            Surface::Right => Vec3::X * controller.size.x / 2f32,
                            Surface::Left => Vec3::NEG_X * controller.size.x / 2f32,
                        } + Vec3::Z * 5f32,
                    ),
                    ..Default::default()
                },
                Collider::cuboid(size.x / 2f32, size.y / 2f32),
                Sensor,
                SurfaceChecker(surface),
                Name::from(format!("{surface:?} checker")),
            ))
            .id();

        if DEBUG {
            cmd.entity(child).insert((
                Sprite {
                    custom_size: Some(size * size_div * 0.9f32),
                    color: Color::WHITE,
                    ..Default::default()
                },
                DEFAULT_IMAGE_HANDLE.typed::<Image>(),
            ));
        }

        child
    };

    let children = [
        generate_child(Surface::Top),
        generate_child(Surface::Bottom),
        generate_child(Surface::Left),
        generate_child(Surface::Right),
    ];

    cmd.entity(entity).push_children(&children);
}

#[derive(Component)]
struct SurfaceChecker(Surface);

fn debug_surface_checker(
    controller_query: Query<&CharacterController, With<Player>>,
    mut debug_query: Query<(&mut Sprite, &SurfaceChecker)>,
) {
    let controller = controller_query.single();

    for (mut sprite, surface) in debug_query.iter_mut() {
        let touching = *controller
            .surface_checker
            .touching_surfaces
            .get(&surface.0)
            .expect(&format!(
                "Surface checker did not include surface {:?}",
                surface.0
            ));

        sprite.color = match touching {
            true => Color::LIME_GREEN,
            false => Color::RED,
        };
    }
}

#[derive(Clone, Debug, Reflect)]
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Reflect)]
pub enum Surface {
    Top,
    Bottom,
    Left,
    Right,
}

fn surface_checker(
    mut player_query: Query<(Entity, &mut CharacterController), With<Player>>,
    checker_query: Query<(&Collider, &GlobalTransform, &SurfaceChecker)>,
    ground_query: Query<Entity, (With<Ground>, With<Collider>)>,
    ctx: Res<RapierContext>,
) {
    let (player, mut controller) = player_query.single_mut();

    let ground_query_predicate = |e| ground_query.contains(e);

    let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(player)
        .predicate(&ground_query_predicate);

    for (col, pos, surface) in checker_query
        .iter()
        .map(|(c, t, s)| (c, t.translation().truncate(), s.0))
        .collect::<Vec<(&Collider, Vec2, Surface)>>()
        .iter()
    {
        controller.surface_checker.set_surface(
            surface,
            ctx.intersection_with_shape(*pos, 0f32, col, filter)
                .is_some(),
        );
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ActivateGroundedDelay(pub Surface);
