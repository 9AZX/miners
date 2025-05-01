use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map)
            .insert_resource(ClearColor(Color::srgb_u8(135, 206, 235)));
    }
}

fn spawn_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Plane"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            // Turning off culling keeps the plane visible when viewed from beneath.
            cull_mode: None,
            ..default()
        })),
    ));


    commands.spawn((
        Name::new("Light"),
        PointLight::default(),
        Transform::from_xyz(3.0, 8.0, 5.0),
    ));

    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 8.0, 4.0),
    // ));
}