use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::{collections::HashMap, hash::Hash};

use crate::{
    level::Ground,
    player::{movement::CharacterController, Player, PlayerSet, PlayerStartupSet},
};

const DEBUG_SURFACE_CHECKER_ENABLED: bool = true;

pub(super) struct MovementSubComponentsPlugin;

impl Plugin for MovementSubComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            add_debug_sprites
                .in_set(PlayerStartupSet::PostPlayer)
                .run_if(debug_surface_checker_enabled),
        )
        .add_systems(
            Last,
            (
                debug_surface_checker
                    .in_set(PlayerSet::Visuals)
                    .run_if(debug_surface_checker_enabled),
                surface_checker.in_set(PlayerSet::PrePlayer),
            ),
        )
        .add_event::<ActivateGroundedDelay>()
        .register_type::<Surface>()
        .register_type::<SurfaceGroundedChecker>();
    }
}

const fn debug_surface_checker_enabled() -> bool {
    DEBUG_SURFACE_CHECKER_ENABLED
}

fn add_debug_sprites(
    mut cmd: Commands,
    player_query: Query<(Entity, &CharacterController), With<Player>>,
) {
    let (entity, controller) = match player_query.get_single() {
        Ok((e, c)) => (e, c),
        Err(err) => {
            println!(
                "\n\n\nCould not get player entity. Error provided: {}",
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
                        Surface::Top | Surface::Bottom => {
                            Vec2::new(controller.size.x / 1.1f32, 2.5f32)
                        }
                        Surface::Left | Surface::Right => {
                            Vec2::new(2.5f32, controller.size.y / 1.1f32)
                        }
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
                    } + Vec3::Z * 5f32,
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
        generate_child(Surface::Right),
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
                println!("Surface checker did not contain surface: {:?}", surface.0);
                return;
            }
        };

        sprite.color = color;
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
    mut grounded_checker_query: Query<(Entity, &mut CharacterController, &Transform), With<Player>>,
    ground_query: Query<Entity, (With<Ground>, With<Collider>)>,
    mut last_delay_message: Local<HashMap<Surface, f32>>,
    ctx: Res<RapierContext>,
    time: Res<Time>,
    grounded_delay: EventReader<ActivateGroundedDelay>,
) {
    let (player, mut controller, transform) = grounded_checker_query.single_mut();
    let mut checker = controller.surface_checker.clone(); // Shorthand

    update_last_delay_message(
        &mut last_delay_message,
        checker
            .touching_surfaces
            .keys()
            .map(|k| *k)
            .collect::<Vec<Surface>>(),
        time.delta_seconds(),
        grounded_delay,
    );

    let pos = transform.translation.truncate();
    let rot = 0f32;
    let toi = 1f32;
    let filter = QueryFilter::default().exclude_collider(player);

    [
        (Vec2::Y, Surface::Top),
        (Vec2::NEG_Y, Surface::Bottom),
        (Vec2::X, Surface::Right),
        (Vec2::NEG_X, Surface::Left),
    ]
    .iter()
    .for_each(|(offset, surface)| {
        if *last_delay_message.get(surface).unwrap() < controller.grounded_delay {
            checker.set_surface(surface, false);
            return;
        }

        let size = controller.size / 2f32 - Vec2::new(offset.y, offset.x).abs() * 5f32;

        let pos = pos + size * *offset;
        let col = Collider::cuboid(size.x, size.y);
        checker.set_surface(
            surface,
            ctx.cast_shape(pos, rot, *offset, &col, toi, filter)
                .is_some_and(|e| ground_query.contains(e.0)),
        );
    });

    controller.surface_checker = checker;
}

#[derive(Event, Clone, Copy, PartialEq, Eq, Reflect)]
pub struct ActivateGroundedDelay(pub Surface);
