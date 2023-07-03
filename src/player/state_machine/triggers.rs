// Mostly copied from https://github.com/Seldom-SE/seldom_state/blob/main/src/trigger/input.rs since
// the module was private

use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use leafwing_input_manager::prelude::*;
use seldom_state::prelude::*;
use std::any::type_name;

use crate::player::{
    movement::{sub_components::Surface, CharacterController},
    Player,
};

#[derive(Debug)]
pub struct GroundedTrigger;

impl BoolTrigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static CharacterController, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        param.single().surface_checker.surface_touching_ground(&Surface::Bottom)
    }
}

#[derive(Debug)]
pub struct FallingTrigger;

impl BoolTrigger for FallingTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static Velocity, With<Player>>;

    fn trigger(&self, _: Entity, param: Self::Param<'_, '_>) -> bool {
        param.single().linvel.y < 0f32
    }
}

#[derive(Debug)]
pub struct ValueTrigger<A: Actionlike> {
    /// The action
    pub action: A,
    /// The minimum value. If no minimum is necessary, use [`f32::NEG_INFINITY`], or similar
    pub min: f32,
    /// The maximum value. If no maximum is necessary, use [`f32::INFINITY`], or similar
    pub max: f32,
}

impl<A: Actionlike> Trigger for ValueTrigger<A> {
    type Param<'w, 's> = Query<'w, 's, &'static ActionState<A>>;
    type Ok = f32;
    type Err = f32;

    fn trigger(&self, entity: Entity, actors: Self::Param<'_, '_>) -> Result<f32, f32> {
        let value = actors
            .get(entity)
            .unwrap_or_else(|_| {
                panic!(
                    "entity {entity:?} with `ValueTrigger<{0}>` is missing `ActionState<{0}>`",
                    type_name::<A>()
                )
            })
            .value(self.action.clone());

        (value >= self.min && value <= self.max)
            .then_some(value)
            .ok_or(value)
    }
}

impl<A: Actionlike> ValueTrigger<A> {
    /// Unbounded trigger
    pub fn unbounded(action: A) -> Self {
        Self {
            action,
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }

    /// Trigger with a minimum bound
    pub fn min(action: A, min: f32) -> Self {
        Self {
            action,
            min,
            max: f32::INFINITY,
        }
    }

    /// Trigger with a maximum bound
    pub fn max(action: A, max: f32) -> Self {
        Self {
            action,
            min: f32::NEG_INFINITY,
            max,
        }
    }
}

/// Trigger that transitions upon pressing the given [`Actionlike`]
#[derive(Debug, Deref, DerefMut)]
pub struct JustPressedTrigger<A: Actionlike>(pub A);

impl<A: Actionlike> BoolTrigger for JustPressedTrigger<A> {
    type Param<'w, 's> = Query<'w, 's, &'static ActionState<A>>;

    fn trigger(&self, entity: Entity, actors: Self::Param<'_, '_>) -> bool {
        let Self(action) = self;
        actors
            .get(entity)
            .unwrap_or_else(|_| {
                panic!(
                    "entity {entity:?} with `JustPressedTrigger<{0}>` is missing `ActionState<{0}>`",
                    type_name::<A>()
                )
            })
            .just_pressed(action.clone())
    }
}

/// Trigger that transitions while pressing the given [`Actionlike`]
#[derive(Debug, Deref, DerefMut)]
pub struct PressedTrigger<A: Actionlike>(pub A);

impl<A: Actionlike> BoolTrigger for PressedTrigger<A> {
    type Param<'w, 's> = Query<'w, 's, &'static ActionState<A>>;

    fn trigger(&self, entity: Entity, actors: Self::Param<'_, '_>) -> bool {
        let Self(action) = self;
        actors
            .get(entity)
            .unwrap_or_else(|_| {
                panic!(
                    "entity {entity:?} with `PressedTrigger<{0}>` is missing `ActionState<{0}>`",
                    type_name::<A>()
                )
            })
            .pressed(action.clone())
    }
}

/// Trigger that transitions upon releasing the given [`Actionlike`]
#[derive(Debug, Deref, DerefMut)]
pub struct JustReleasedTrigger<A: Actionlike>(pub A);

#[cfg(feature = "leafwing_input")]
impl<A: Actionlike> BoolTrigger for JustReleasedTrigger<A> {
    type Param<'w, 's> = Query<'w, 's, &'static ActionState<A>>;

    fn trigger(&self, entity: Entity, actors: Self::Param<'_, '_>) -> bool {
        let Self(action) = self;
        actors
            .get(entity)
            .unwrap_or_else(|_| {
                panic!(
                    "entity {entity:?} with `JustReleasedTrigger<{0}>` is missing `ActionState<{0}>`",
                    type_name::<A>()
                )
            })
            .just_released(action.clone())
    }
}

/// Trigger that transitions while the given [`Actionlike`] is released
#[derive(Debug, Deref, DerefMut)]
pub struct ReleasedTrigger<A: Actionlike>(pub A);

impl<A: Actionlike> BoolTrigger for ReleasedTrigger<A> {
    type Param<'w, 's> = Query<'w, 's, &'static ActionState<A>>;

    fn trigger(&self, entity: Entity, actors: Self::Param<'_, '_>) -> bool {
        let Self(action) = self;
        actors
            .get(entity)
            .unwrap_or_else(|_| {
                panic!(
                    "entity {entity:?} with `ReleasedTrigger<{0}>` is missing `ActionState<{0}>`",
                    type_name::<A>()
                )
            })
            .just_pressed(action.clone())
    }
}
