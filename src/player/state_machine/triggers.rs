// Mostly copied from https://github.com/Seldom-SE/seldom_state/blob/main/src/trigger/input.rs since
// the module was private

use bevy::prelude::*;
use seldom_state::prelude::*;

use crate::player::{
    movement::{sub_components::Surface, CharacterController},
    Player,
};

#[derive(Debug)]
pub struct JumpTrigger;

impl BoolTrigger for JumpTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CharacterController, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        match param.get_single() {
            Ok(val) => !val.coyote_timer.finished() && !val.jump_buffer_timer.finished(),
            Err(message) => {
                println!(
                    "Could not get controller and velocity in jump trigger. Error message: {}",
                    message.to_string()
                );
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct GroundedTrigger;

impl BoolTrigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CharacterController, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        match param.get_single() {
            Ok(val) => val
                .surface_checker
                .surface_touching_ground(&Surface::Bottom),
            Err(message) => {
                println!("Could not get player character controller in grounded trigger. Error message: {}", message.to_string());
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct WallslidingTrigger;

impl BoolTrigger for WallslidingTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CharacterController, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        match param.get_single() {
            Ok(val) => {
                (val.surface_checker.surface_touching_ground(&Surface::Left)
                    || val.surface_checker.surface_touching_ground(&Surface::Right))
                    && !val
                        .surface_checker
                        .surface_touching_ground(&Surface::Bottom)
            }
            Err(message) => {
                println!("Could not get player character controller in grounded trigger. Error message: {}", message.to_string());
                false
            }
        }
    }
}

#[derive(Debug)]
pub struct FallingTrigger;

impl BoolTrigger for FallingTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CharacterController, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        match param.get_single() {
            Ok(val) => !val
                .surface_checker
                .surface_touching_ground(&Surface::Bottom),
            Err(message) => {
                println!("Could not get player character controller in falling trigger. Error message: {}", message.to_string());
                false
            }
        }
    }
}
