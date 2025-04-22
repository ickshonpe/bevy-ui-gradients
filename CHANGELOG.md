## Changelog

### 0.3.0
* Added a `start: f32` field to `ConicGradient`'. This is used to set the starting angle for the conic gradient.
* Added builder methods `with_start` and `with_position` to `ConicGradient`
* Removed the `position` parameter from `ConicGradient::new`. Use `with_position` to set a position.
* Angles for `AngularColorStop`s are clamped between `0.` and `2. * PI`. 
* Zero-width color-stops should no longer be sent to the shader which sometimes resulted in visual artifacts bfore.
* The fill flag is no longer set for an initial stops with distance `0.`. Trying to fill before `0.` could result in visual artifacts with conic gradients.