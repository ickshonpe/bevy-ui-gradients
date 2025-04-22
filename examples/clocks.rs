//! Simple example demonstrating conic gradients

use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy_ui_gradients::*;
use std::f32::consts::TAU;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiGradientsPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update_clocks)
        .run();
}

#[derive(Component)]
struct ClockMode(u32);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.),
            ..Default::default()
        })
        .with_children(|commands| {
            for i in 0..7 {
                commands.spawn((
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(150.),
                        ..Default::default()
                    },
                    BorderRadius::MAX,
                    BackgroundGradient::default(),
                    ClockMode(i),
                    Outline {
                        width: Val::Px(2.),
                        color: Color::WHITE,
                        ..Default::default()
                    },
                ));
            }
        });
}

fn update_clocks(time: Res<Time>, mut query: Query<(&ClockMode, &mut BackgroundGradient)>) {
    let t = time.elapsed_secs();
    for (clock_mode, mut gradient) in query.iter_mut() {
        let angle = match clock_mode.0 {
            0 => t,
            1 => TAU - t,
            2 => t.rem_euclid(TAU),
            3 => TAU - (time.elapsed_secs() % (2.0 * TAU) - TAU).abs(),
            4 => time.elapsed_secs().rem_euclid(12.).floor() * TAU / 12.,
            5 => 0.,
            _ => TAU,
        };
        *gradient = BackgroundGradient::from(ConicGradient {
            start: 0.75 * TAU,
            position: Default::default(),
            stops: vec![
                AngularColorStop::new(RED, 0.0),
                AngularColorStop::new(RED, angle),
                AngularColorStop::new(BLUE, angle),
            ],
        });
    }
}
