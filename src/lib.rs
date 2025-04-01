mod render;

use bevy::app::{App, Plugin};
use bevy::color::{Color, Srgba};
use bevy::ecs::component::Component;
use bevy::math::Vec2;
use bevy::prelude::ReflectDefault;
use bevy::utils::default;
use bevy::{reflect::Reflect, ui::Val};
use core::{f32, f32::consts::TAU};
use render::{build_gradients_renderer, finish_gradients_renderer};

fn scale_val(val: Val, scale_factor: f32) -> Val {
    match val {
        Val::Px(px) => Val::Px(px * scale_factor),
        _ => val,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Default, Debug, PartialEq)]
/// Responsive position relative to a UI node.
pub struct Position {
    /// Normalized anchor point
    pub anchor: Vec2,
    /// Responsive horizontal position relative to the anchor point
    pub x: Val,
    /// Responsive vertical position relative to the anchor point
    pub y: Val,
}

impl Default for Position {
    fn default() -> Self {
        Self::CENTER
    }
}

impl Position {
    /// Position at the given normalized anchor point
    pub const fn anchor(anchor: Vec2) -> Self {
        Self {
            anchor,
            x: Val::ZERO,
            y: Val::ZERO,
        }
    }

    /// Position at the top-left corner
    pub const TOP_LEFT: Self = Self::anchor(Vec2::new(-0.5, -0.5));

    /// Position at the center of the left edge
    pub const LEFT: Self = Self::anchor(Vec2::new(-0.5, 0.0));

    /// Position at the bottom-left corner
    pub const BOTTOM_LEFT: Self = Self::anchor(Vec2::new(-0.5, 0.5));

    /// Position at the center of the top edge
    pub const TOP: Self = Self::anchor(Vec2::new(0.0, -0.5));

    /// Position at the center of the element
    pub const CENTER: Self = Self::anchor(Vec2::new(0.0, 0.0));

    /// Position at the center of the bottom edge
    pub const BOTTOM: Self = Self::anchor(Vec2::new(0.0, 0.5));

    /// Position at the top-right corner
    pub const TOP_RIGHT: Self = Self::anchor(Vec2::new(0.5, -0.5));

    /// Position at the center of the right edge
    pub const RIGHT: Self = Self::anchor(Vec2::new(0.5, 0.0));

    /// Position at the bottom-right corner
    pub const BOTTOM_RIGHT: Self = Self::anchor(Vec2::new(0.5, 0.5));

    /// Create a new position
    pub const fn new(anchor: Vec2, x: Val, y: Val) -> Self {
        Self { anchor, x, y }
    }

    /// Creates a position from self with the given `x` and `y` coordinates
    pub const fn at(self, x: Val, y: Val) -> Self {
        Self { x, y, ..self }
    }

    /// Creates a position from self with the given `x` coordinate
    pub const fn at_x(self, x: Val) -> Self {
        Self { x, ..self }
    }

    /// Creates a position from self with the given `y` coordinate
    pub const fn at_y(self, y: Val) -> Self {
        Self { y, ..self }
    }

    /// Creates a position in logical pixels from self with the given `x` and `y` coordinates
    pub const fn at_px(self, x: f32, y: f32) -> Self {
        self.at(Val::Px(x), Val::Px(y))
    }

    /// Creates a percentage position from self with the given `x` and `y` coordinates
    pub const fn at_percent(self, x: f32, y: f32) -> Self {
        self.at(Val::Percent(x), Val::Percent(y))
    }

    /// Creates a position from self with the given `anchor` point
    pub const fn with_anchor(self, anchor: Vec2) -> Self {
        Self { anchor, ..self }
    }

    /// Position relative to the top-left corner
    pub const fn top_left(x: Val, y: Val) -> Self {
        Self::TOP_LEFT.at(x, y)
    }

    /// Position relative to the left edge
    pub const fn left(x: Val, y: Val) -> Self {
        Self::LEFT.at(x, y)
    }

    /// Position relative to the bottom-left corner
    pub const fn bottom_left(x: Val, y: Val) -> Self {
        Self::BOTTOM_LEFT.at(x, y)
    }

    /// Position relative to the top edge
    pub const fn top(x: Val, y: Val) -> Self {
        Self::TOP.at(x, y)
    }

    /// Position relative to the center
    pub const fn center(x: Val, y: Val) -> Self {
        Self::CENTER.at(x, y)
    }

    /// Position relative to the bottom edge
    pub const fn bottom(x: Val, y: Val) -> Self {
        Self::BOTTOM.at(x, y)
    }

    /// Position relative to the top-right corner
    pub const fn top_right(x: Val, y: Val) -> Self {
        Self::TOP_RIGHT.at(x, y)
    }

    /// Position relative to the right edge
    pub const fn right(x: Val, y: Val) -> Self {
        Self::RIGHT.at(x, y)
    }

    /// Position relative to the bottom-right corner
    pub const fn bottom_right(x: Val, y: Val) -> Self {
        Self::BOTTOM_RIGHT.at(x, y)
    }

    /// Resolves the `Position` into physical coordinates.
    pub fn resolve(
        self,
        scale_factor: f32,
        physical_size: Vec2,
        physical_target_size: Vec2,
    ) -> Vec2 {
        let d = self.anchor.map(|p| if 0. < p { -1. } else { 1. });

        physical_size * self.anchor
            + d * Vec2::new(
                scale_val(self.x, scale_factor)
                    .resolve(physical_size.x, physical_target_size)
                    .unwrap_or(0.),
                scale_val(self.y, scale_factor)
                    .resolve(physical_size.y, physical_target_size)
                    .unwrap_or(0.),
            )
    }
}

impl From<Val> for Position {
    fn from(x: Val) -> Self {
        Self { x, ..default() }
    }
}

impl From<(Val, Val)> for Position {
    fn from((x, y): (Val, Val)) -> Self {
        Self { x, y, ..default() }
    }
}

/// A color stop for a gradient
#[derive(Debug, Copy, Clone, PartialEq, Reflect)]
#[reflect(Default, PartialEq, Debug)]
pub struct ColorStop {
    /// Color
    pub color: Color,
    /// Logical position along the gradient line.
    /// Stop positions are relative to the start of the gradient and not other stops.
    pub point: Val,
    /// Normalized position between this and the following stop of the interpolation midpoint.
    pub hint: f32,
}

impl ColorStop {
    /// Create a new color stop
    pub fn new(color: impl Into<Color>, point: Val) -> Self {
        Self {
            color: color.into(),
            point,
            hint: 0.5,
        }
    }

    /// An automatic color stop.
    /// The positions of automatic stops are interpolated evenly between explicit stops.
    pub fn auto(color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
            point: Val::Auto,
            hint: 0.5,
        }
    }

    // Set the interpolation midpoint between this and and the following stop
    pub fn with_hint(mut self, hint: f32) -> Self {
        self.hint = hint;
        self
    }
}

impl From<(Color, Val)> for ColorStop {
    fn from((color, stop): (Color, Val)) -> Self {
        Self {
            color,
            point: stop,
            hint: 0.5,
        }
    }
}

impl From<Color> for ColorStop {
    fn from(color: Color) -> Self {
        Self {
            color,
            point: Val::Auto,
            hint: 0.5,
        }
    }
}

impl From<Srgba> for ColorStop {
    fn from(color: Srgba) -> Self {
        Self {
            color: color.into(),
            point: Val::Auto,
            hint: 0.5,
        }
    }
}

impl Default for ColorStop {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            point: Val::Auto,
            hint: 0.5,
        }
    }
}

/// An angular color stop for a conic gradient
#[derive(Default, Debug, Copy, Clone, PartialEq, Reflect)]
#[reflect(Default, PartialEq, Debug)]
pub struct AngularColorStop {
    /// Color of the stop
    pub color: Color,
    /// The angle of the stop.
    /// Angles are relative to the start of the gradient and not other stops.
    /// If set to `None` the angle of the stop will be interpolated between the explicit stops or 0 and 2 PI degrees if there no explicit stops.
    pub angle: Option<f32>,
    /// Normalized angle between this and the following stop of the interpolation midpoint.
    pub hint: f32,
}

impl AngularColorStop {
    // Create a new color stop
    pub fn new(color: impl Into<Color>, angle: f32) -> Self {
        Self {
            color: color.into(),
            angle: Some(angle),
            hint: 0.5,
        }
    }

    /// An angular stop without an explicit angle. The angles of automatic stops
    /// are interpolated evenly between explicit stops.
    pub fn auto(color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
            angle: None,
            hint: 0.5,
        }
    }

    // Set the interpolation midpoint between this and and the following stop
    pub fn with_hint(mut self, hint: f32) -> Self {
        self.hint = hint;
        self
    }
}

/// A linear gradient
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient>
#[derive(Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct LinearGradient {
    /// The direction of the gradient.
    /// An angle of `0.` points upward, angles increasing clockwise.
    pub angle: f32,
    /// The list of color stops
    pub stops: Vec<ColorStop>,
}

impl LinearGradient {
    /// Angle of a linear gradient transitioning from bottom to top
    pub const TO_TOP: f32 = 0.;
    /// Angle of a linear gradient transitioning from bottom-left to top-right
    pub const TO_TOP_RIGHT: f32 = TAU / 8.;
    /// Angle of a linear gradient transitioning from left to right
    pub const TO_RIGHT: f32 = 2. * Self::TO_TOP_RIGHT;
    /// Angle of a linear gradient transitioning from top-left to bottom-right
    pub const TO_BOTTOM_RIGHT: f32 = 3. * Self::TO_TOP_RIGHT;
    /// Angle of a linear gradient transitioning from top to bottom
    pub const TO_BOTTOM: f32 = 4. * Self::TO_TOP_RIGHT;
    /// Angle of a linear gradient transitioning from top-right to bottom-left
    pub const TO_BOTTOM_LEFT: f32 = 5. * Self::TO_TOP_RIGHT;
    /// Angle of a linear gradient transitioning from right to left
    pub const TO_LEFT: f32 = 6. * Self::TO_TOP_RIGHT;
    /// Angle of a linear gradient transitioning from bottom-right to top-left
    pub const TO_TOP_LEFT: f32 = 7. * Self::TO_TOP_RIGHT;

    /// Create a new linear gradient
    pub fn new(angle: f32, stops: Vec<ColorStop>) -> Self {
        Self { angle, stops }
    }

    /// A linear gradient transitioning from bottom to top
    pub fn to_top(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_TOP,
            stops,
        }
    }

    /// A linear gradient transitioning from bottom-left to top-right
    pub fn to_top_right(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_TOP_RIGHT,
            stops,
        }
    }

    /// A linear gradient transitioning from left to right
    pub fn to_right(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_RIGHT,
            stops,
        }
    }

    /// A linear gradient transitioning from top-left to bottom-right
    pub fn to_bottom_right(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_BOTTOM_RIGHT,
            stops,
        }
    }

    /// A linear gradient transitioning from top to bottom
    pub fn to_bottom(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_BOTTOM,
            stops,
        }
    }

    /// A linear gradient transitioning from top-right to bottom-left
    pub fn to_bottom_left(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_BOTTOM_LEFT,
            stops,
        }
    }

    /// A linear gradient transitioning from right to left
    pub fn to_left(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_LEFT,
            stops,
        }
    }

    /// A linear gradient transitioning from bottom-right to top-left
    pub fn to_top_left(stops: Vec<ColorStop>) -> Self {
        Self {
            angle: Self::TO_TOP_LEFT,
            stops,
        }
    }

    /// A linear gradient with the given angle in degrees
    pub fn degrees(degrees: f32, stops: Vec<ColorStop>) -> Self {
        Self {
            angle: degrees.to_radians(),
            stops,
        }
    }
}

/// A radial gradient
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/radial-gradient>
#[derive(Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct RadialGradient {
    /// The center of the radial gradient
    pub position: Position,
    /// Defines the end shape of the radial gradient
    pub shape: RadialGradientShape,
    /// The list of color stops
    pub stops: Vec<ColorStop>,
}

impl RadialGradient {
    /// Create a new radial gradient
    pub fn new(position: Position, shape: RadialGradientShape, stops: Vec<ColorStop>) -> Self {
        Self {
            position,
            shape,
            stops,
        }
    }
}

impl Default for RadialGradient {
    fn default() -> Self {
        Self {
            position: Position::CENTER,
            shape: RadialGradientShape::ClosestCorner,
            stops: Vec::new(),
        }
    }
}

/// A conic gradient
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/conic-gradient>
#[derive(Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub struct ConicGradient {
    /// The center of the conic gradient
    pub position: Position,
    /// The list of color stops
    pub stops: Vec<AngularColorStop>,
}

impl ConicGradient {
    /// create a new conic gradient
    pub fn new(position: Position, stops: Vec<AngularColorStop>) -> Self {
        Self { position, stops }
    }
}

#[derive(Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
pub enum Gradient {
    /// A linear gradient
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient>
    Linear(LinearGradient),
    /// A radial gradient
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient>
    Radial(RadialGradient),
    /// A conic gradient
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/radial-gradient>
    Conic(ConicGradient),
}

impl Gradient {
    /// Returns true if the gradient has no stops.
    pub fn is_empty(&self) -> bool {
        match self {
            Gradient::Linear(gradient) => gradient.stops.is_empty(),
            Gradient::Radial(gradient) => gradient.stops.is_empty(),
            Gradient::Conic(gradient) => gradient.stops.is_empty(),
        }
    }

    /// If the gradient has only a single color stop `get_single` returns its color.
    pub fn get_single(&self) -> Option<Color> {
        match self {
            Gradient::Linear(gradient) => gradient
                .stops
                .first()
                .and_then(|stop| (gradient.stops.len() == 1).then_some(stop.color)),
            Gradient::Radial(gradient) => gradient
                .stops
                .first()
                .and_then(|stop| (gradient.stops.len() == 1).then_some(stop.color)),
            Gradient::Conic(gradient) => gradient
                .stops
                .first()
                .and_then(|stop| (gradient.stops.len() == 1).then_some(stop.color)),
        }
    }
}

impl From<LinearGradient> for Gradient {
    fn from(value: LinearGradient) -> Self {
        Self::Linear(value)
    }
}

impl From<RadialGradient> for Gradient {
    fn from(value: RadialGradient) -> Self {
        Self::Radial(value)
    }
}

impl From<ConicGradient> for Gradient {
    fn from(value: ConicGradient) -> Self {
        Self::Conic(value)
    }
}

#[derive(Component, Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
/// A UI node that displays a gradient
pub struct BackgroundGradient(pub Vec<Gradient>);

impl<T: Into<Gradient>> From<T> for BackgroundGradient {
    fn from(value: T) -> Self {
        Self(vec![value.into()])
    }
}

#[derive(Component, Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq)]
/// A UI node border that displays a gradient
pub struct BorderGradient(pub Vec<Gradient>);

impl<T: Into<Gradient>> From<T> for BorderGradient {
    fn from(value: T) -> Self {
        Self(vec![value.into()])
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug, Reflect)]
#[reflect(PartialEq, Default)]
pub enum RadialGradientShape {
    /// A circle with radius equal to the distance from its center to the closest side
    ClosestSide,
    /// A circle with radius equal to the distance from its center to the farthest side
    FarthestSide,
    /// An ellipse with extents equal to the distance from its center to the nearest corner
    #[default]
    ClosestCorner,
    /// An ellipse with extents equal to the distance from its center to the farthest corner
    FarthestCorner,
    /// A circle
    Circle(Val),
    /// An ellipse
    Ellipse(Val, Val),
}

fn close_side(p: f32, h: f32) -> f32 {
    (-h - p).abs().min((h - p).abs())
}

fn far_side(p: f32, h: f32) -> f32 {
    (-h - p).abs().max((h - p).abs())
}

fn close_side2(p: Vec2, h: Vec2) -> f32 {
    close_side(p.x, h.x).min(close_side(p.y, h.y))
}

fn far_side2(p: Vec2, h: Vec2) -> f32 {
    far_side(p.x, h.x).max(far_side(p.y, h.y))
}

impl RadialGradientShape {
    /// Resolve the physical dimensions of the end shape of the radial gradient
    pub fn resolve(
        self,
        position: Vec2,
        scale_factor: f32,
        physical_size: Vec2,
        physical_target_size: Vec2,
    ) -> Vec2 {
        let half_size = 0.5 * physical_size;
        match self {
            RadialGradientShape::ClosestSide => Vec2::splat(close_side2(position, half_size)),
            RadialGradientShape::FarthestSide => Vec2::splat(far_side2(position, half_size)),
            RadialGradientShape::ClosestCorner => Vec2::new(
                close_side(position.x, half_size.x),
                close_side(position.y, half_size.y),
            ),
            RadialGradientShape::FarthestCorner => Vec2::new(
                far_side(position.x, half_size.x),
                far_side(position.y, half_size.y),
            ),
            RadialGradientShape::Circle(radius) => Vec2::splat(
                scale_val(radius, scale_factor)
                    .resolve(physical_size.x, physical_target_size)
                    .unwrap_or(0.),
            ),
            RadialGradientShape::Ellipse(x, y) => Vec2::new(
                scale_val(x, scale_factor)
                    .resolve(physical_size.x, physical_target_size)
                    .unwrap_or(0.),
                scale_val(y, scale_factor)
                    .resolve(physical_size.y, physical_target_size)
                    .unwrap_or(0.),
            ),
        }
    }
}

pub struct UiGradientsPlugin;

impl Plugin for UiGradientsPlugin {
    fn build(&self, app: &mut App) {
        build_gradients_renderer(app);
    }

    fn finish(&self, app: &mut App) {
        finish_gradients_renderer(app);
    }
}
