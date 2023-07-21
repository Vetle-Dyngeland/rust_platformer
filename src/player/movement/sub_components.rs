use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_rapier2d::prelude::*;
use std::{collections::HashMap, hash::Hash};

use crate::{
    level::Ground,
    player::{movement::CharacterController, Player, PlayerSet, PlayerStartupSet},
};

const DEBUG: bool = true;

const fn debug() -> bool {
    DEBUG
}

pub(super) struct MovementSubComponentsPlugin;

impl Plugin for MovementSubComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostStartup,
            spawn_grounded_checkers.after(PlayerStartupSet::Movement),
        )
        .add_systems(
            Last,
            (
                surface_checker.in_set(PlayerSet::PrePlayer),
                debug_surface_checker
                    .in_set(PlayerSet::Visuals)
                    .run_if(debug),
            ),
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
    let size_div = 1.4f32;

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

fn update_last_delay_message(
    last_delay_message: &mut Local<HashMap<Surface, f32>>,
    surfaces: Vec<Surface>,
    delta_seconds: f32,
    mut events: EventReader<ActivateGroundedDelay>,
) {
    if last_delay_message.is_empty() {
        **last_delay_message = HashMap::new();
        for surface in surfaces.iter() {
            last_delay_message.insert(*surface, 0f32);
        }
    }
    for sur in last_delay_message.clone().keys() {
        let val = *last_delay_message.get(sur).unwrap() + delta_seconds;
        last_delay_message.insert(*sur, val);
    }

    for ev in events.iter() {
        last_delay_message.insert(ev.0, 0f32);
    }
}

fn surface_checker(
    mut last_delay_message: Local<HashMap<Surface, f32>>,
    mut player_query: Query<(Entity, &mut CharacterController), With<Player>>,
    checker_query: Query<(Entity, &Collider, &GlobalTransform, &SurfaceChecker)>,
    ground_query: Query<Entity, (With<Ground>, With<Collider>)>,
    ctx: Res<RapierContext>,
    time: Res<Time>,
    grounded_delay: EventReader<ActivateGroundedDelay>,
) {
    let (player, mut controller) = player_query.single_mut();

    update_last_delay_message(
        &mut last_delay_message,
        controller
            .surface_checker
            .touching_surfaces
            .keys()
            .map(|k| *k)
            .collect::<Vec<Surface>>(),
        time.delta_seconds(),
        grounded_delay,
    );

    let ground_query_predicate = |e| ground_query.contains(e);

    let filter = QueryFilter::new()
        .exclude_sensors()
        .exclude_rigid_body(player)
        .predicate(&ground_query_predicate);

    for (col, pos, surface) in checker_query
        .iter()
        .map(|(_, c, t, s)| (c, t.translation().truncate(), s.0))
        .collect::<Vec<(&Collider, Vec2, Surface)>>()
        .iter()
    {
        let is_collision_valid = *last_delay_message
            .get(surface)
            .expect("last delay message local does not include {surface:?}")
            > controller.grounded_delay;

        let colliding = ctx
            .intersection_with_shape(*pos, 0f32, col, filter)
            .is_some();

        controller
            .surface_checker
            .set_surface(surface, colliding && is_collision_valid)
    }
}

#[derive(Event, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ActivateGroundedDelay(pub Surface);
