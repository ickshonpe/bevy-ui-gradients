//! Test, none of the gradient nodes should show any red.
use bevy::color::palettes::css::RED;
use bevy::color::palettes::css::WHITE;
use bevy::prelude::*;
use bevy_ui_gradients::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiGradientsPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Node::default()).with_children(|commands| {
        for _ in 0..10 {
            commands.spawn((
                Node {
                    aspect_ratio: Some(1.),
                    height: Val::Px(103.),
                    border: UiRect::all(Val::Px(10.)),
                    margin: UiRect::left(Val::Px(30.)),
                    ..default()
                },
                BorderRadius::all(Val::Px(20.)),
                BackgroundGradient::from(ConicGradient {
                    stops: vec![
                        AngularColorStop::new(RED, 0.),
                        AngularColorStop::new(RED, 0.),
                        AngularColorStop::new(Color::NONE, 0.),
                    ],
                    position: Position::CENTER,
                }),
                BorderColor(WHITE.into()),
            ));
        }
    });
}
