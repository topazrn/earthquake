use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(rotate)
        .run();
}

#[derive(Component)]
struct Earth;

#[derive(Component)]
struct Earthquake {
    latitude: f32,
    longitude: f32,
    depth: f32,
    magnitude: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("textures/earth.png");

    let earth_material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    });

    let mesh_handle = meshes.add(
        shape::UVSphere {
            sectors: 128,
            ..Default::default()
        }
        .into(),
    );

    let earthquake0 = Earthquake {
        latitude: 90.,
        longitude: 0.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquake1 = Earthquake {
        latitude: -90.,
        longitude: 0.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquake2 = Earthquake {
        latitude: 0.,
        longitude: -90.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquake3 = Earthquake {
        latitude: 0.,
        longitude: 180.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquake4 = Earthquake {
        latitude: 0.,
        longitude: 90.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquake5 = Earthquake {
        latitude: 0.,
        longitude: 0.,
        depth: 122.2,
        magnitude: 1.5,
    };

    let earthquakes = vec![
        earthquake0,
        earthquake1,
        earthquake2,
        earthquake3,
        earthquake4,
        earthquake5,
    ];

    commands
        .spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: earth_material_handle,
                ..default()
            },
            Earth,
        ))
        .with_children(|parent| {
            earthquakes.into_iter().for_each(|earthquake| {
                parent.spawn((
                    PbrBundle {
                        mesh: meshes.add(
                            shape::Icosphere {
                                radius: earthquake.magnitude / 20.,
                                ..Default::default()
                            }
                            .try_into()
                            .unwrap(),
                        ),
                        material: materials.add(StandardMaterial {
                            emissive: Color::rgb_linear(13.99, 5.32, 2.0),
                            ..default()
                        }),
                        transform: Transform::from_xyz(
                            earthquake.latitude.to_radians().cos()
                                * earthquake.longitude.to_radians().sin(),
                            earthquake.latitude.to_radians().sin(),
                            earthquake.latitude.to_radians().cos()
                                * earthquake.longitude.to_radians().cos(),
                        ),
                        ..default()
                    },
                    earthquake,
                ));
            });
        });

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0., 0., 3.).looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
            ..default()
        },
        BloomSettings {
            composite_mode: BloomCompositeMode::Additive,
            ..default()
        },
    ));
}

fn rotate(mut query: Query<&mut Transform, With<Earth>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}
