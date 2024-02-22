use bevy::{
    core_pipeline::prepass::DepthPrepass,
    pbr::{ExtendedMaterial, NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use rand::{thread_rng, Rng};
mod decal;
use decal::{DecalMaterial, DecalPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DecalPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, bob)
        .run();
}
#[derive(Component)]
struct Bob;

fn bob(mut query: Query<&mut Transform, With<Bob>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y = (time.elapsed_seconds() * 1.0).sin() / 4.0;
    }
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut decal_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, DecalMaterial>>>,
) {
    // circular base
    commands.spawn((PbrBundle {
        transform: Transform::default(),
        mesh: meshes.add(shape::Plane::from_size(10.0).into()),
        material: materials.add(Color::WHITE.into()),
        ..default()
    },));
    for _ in 0..50 {
        let x = thread_rng().gen_range(-1.2..1.2);
        let z = thread_rng().gen_range(-1.2..1.2);
        let rot_x = thread_rng().gen();
        let rot_y = thread_rng().gen();
        let rot_z = thread_rng().gen();
        let rot_w = thread_rng().gen();
        let rotation = Quat {
            x: rot_x,
            y: rot_y,
            z: rot_z,
            w: rot_w,
        };
        let transform = Transform {
            translation: Vec3::new(x, 0.0, z),
            rotation,
            ..default()
        };

        commands.spawn((PbrBundle {
            transform,
            mesh: meshes.add(shape::Box::new(0.1, 0.1, 0.1).into()),
            material: materials.add(Color::WHITE.into()),
            ..default()
        },));
    }

    commands.spawn((PbrBundle {
        transform: Transform::default(),
        mesh: meshes.add(shape::Box::new(0.2, 0.1, 0.1).into()),
        material: materials.add(Color::WHITE.into()),
        ..default()
    },));

    commands.spawn((PbrBundle {
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        mesh: meshes.add(shape::Cube::new(0.5).into()),
        material: materials.add(Color::WHITE.into()),
        ..default()
    },));

    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(1.5, 0.1, 0.0)),
            mesh: meshes.add(shape::Quad::new(Vec2::splat(1.0)).into()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    color: Color::BLUE,
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowReceiver,
        NotShadowCaster,
    ));

    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(2.0, 1.0, 0.0)),
            mesh: meshes.add(shape::Quad::new(Vec2::splat(1.0)).into()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    color: Color::BLUE,
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));

    commands.spawn((
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(-2.0, -1.0, 0.0)),
            mesh: meshes.add(shape::Quad::new(Vec2::splat(1.0)).into()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    color: Color::BLUE,
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
        NotShadowReceiver,
    ));

    commands.spawn((
        Bob,
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            )),
            mesh: meshes.add(shape::Quad::new(Vec2::splat(1.0)).into()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    color: Color::BLUE,
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowReceiver,
        NotShadowCaster,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-1.5, 4.5, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        DepthPrepass,
    ));
}
