use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*, render::camera::ScalingMode};
use std::{f32::consts::PI, ops::Range};

pub struct CameraPlugin;

#[derive(Component)]
struct MyCameraMarker;

#[derive(Debug, Resource)]
struct CameraSettings {
    /// The height of the viewport in world units when the orthographic camera's scale is 1
    pub orthographic_viewport_height: f32,
    /// Clamp the orthographic camera's scale to this range
    pub orthographic_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the orthographic camera
    pub orthographic_zoom_speed: f32,
    /// Clamp perspective camera's field of view to this range
    pub perspective_zoom_range: Range<f32>,
    /// Multiply mouse wheel inputs by this factor when using the perspective camera
    pub perspective_zoom_speed: f32,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera_ortographic)
            .add_systems(Update, zoom)
            .insert_resource(CameraSettings {
                orthographic_viewport_height: 5.,
                // In orthographic projections, we specify camera scale relative to a default value of 1,
                // in which one unit in world space corresponds to one pixel.
                orthographic_zoom_range: 1.0..10.0,
                // This value was hand-tuned to ensure that zooming in and out feels smooth but not slow.
                orthographic_zoom_speed: 0.01,
                // Perspective projections use field of view, expressed in radians. We would
                // normally not set it to more than π, which represents a 180° FOV.
                perspective_zoom_range: (PI / 5.)..(PI - 0.2),
                // Changes in FOV are much more noticeable due to its limited range in radians
                perspective_zoom_speed: 0.05,
            });
    }
}

fn spawn_camera_ortographic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        MyCameraMarker,
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::from(OrthographicProjection {
            // 6 world units per pixel of window height.
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 6.0,
            },
            ..OrthographicProjection::default_3d()
        })
    ));
}

fn zoom(
    camera: Single<&mut Projection, With<Camera>>,
    camera_settings: Res<CameraSettings>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    // Usually, you won't need to handle both types of projection,
    // but doing so makes for a more complete example.
    match *camera.into_inner() {
        Projection::Orthographic(ref mut orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.orthographic_zoom_speed;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(
                camera_settings.orthographic_zoom_range.start,
                camera_settings.orthographic_zoom_range.end,
            );
        }
        Projection::Perspective(ref mut perspective) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * camera_settings.perspective_zoom_speed;

            // Adjust the field of view, but keep it within our stated range.
            perspective.fov = (perspective.fov + delta_zoom).clamp(
                camera_settings.perspective_zoom_range.start,
                camera_settings.perspective_zoom_range.end,
            );
        }
        _ => (),
    }
}
