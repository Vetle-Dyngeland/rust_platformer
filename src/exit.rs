use bevy::{app::AppExit, prelude::*};
use std::time::Duration;

pub struct ExitPlugin {
    pub keys: Vec<KeyCode>,
    pub reset_timer_duration: Duration,
    pub press_count: usize,
}

impl Default for ExitPlugin {
    fn default() -> Self {
        Self {
            keys: vec![KeyCode::ControlLeft, KeyCode::L],
            reset_timer_duration: Duration::from_secs_f32(0.5f32),
            press_count: 1,
        }
    }
}

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ExitKeys {
            keys: self.keys.clone(),
            reset_timer: Timer::new(self.reset_timer_duration, TimerMode::Once),
            press_count: self.press_count,
        })
        .add_systems(Update, exit_system);
    }
}

#[derive(Resource)]
pub struct ExitKeys {
    pub keys: Vec<KeyCode>,
    pub reset_timer: Timer,
    pub press_count: usize,
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    mut keys: ResMut<ExitKeys>,
    mut count: Local<usize>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    keys.reset_timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if keys.reset_timer.just_finished() {
        keys.reset_timer.reset();
        *count = 0
    }

    for key in keys.keys.iter() {
        if !keyboard.pressed(*key) {
            return;
        }
    }

    keys.keys.iter().for_each(|key| {
        if keyboard.just_pressed(*key) {
            *count += 1;
            if *count >= keys.press_count {
                exit.send(AppExit);
            }
            return;
        }
    });
}
