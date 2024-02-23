use bevy::{
    core_pipeline::prepass::DepthPrepass,
    math::primitives,
    pbr::{ExtendedMaterial, NotShadowCaster},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui,
};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use rand::{thread_rng, Rng};
mod decal;
use decal::{DecalMaterial, DecalPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
        .add_plugins(DecalPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, (setup,))
        .add_systems(Update, (bob, inspector_ui))
        .run();
}
#[derive(Component)]
struct Bob(Vec3, Vec3);

fn bob(mut query: Query<(&mut Transform, &Bob)>, time: Res<Time>) {
    for (mut transform, bob) in &mut query {
        transform.translation = bob.0 * (time.elapsed_seconds() * 1.0).sin() / 4.0 + bob.1;
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
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        mesh: meshes.add(primitives::Rectangle::new(10.0, 10.0)),
        material: materials.add(Color::WHITE),
        ..default()
    },));

    for _ in 0..150 {
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
            mesh: meshes.add(primitives::Cuboid::new(0.1, 0.1, 0.1)),
            material: materials.add(Color::WHITE),
            ..default()
        },));
    }

    commands.spawn((PbrBundle {
        transform: Transform::default(),
        mesh: meshes.add(primitives::Cuboid::from_size(Vec3::splat(0.5))),
        material: materials.add(Color::WHITE),
        ..default()
    },));

    commands.spawn((PbrBundle {
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        mesh: meshes.add(primitives::Cuboid::from_size(Vec3::splat(0.5))),
        material: materials.add(Color::WHITE),
        ..default()
    },));

    let mut mesh: Mesh = primitives::Rectangle::new(1.0, 1.0).into();
    mesh = mesh.with_generated_tangents().unwrap();
    commands.spawn((
        Bob(Vec3::Y, Vec3::ZERO),
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)),
            mesh: meshes.add(mesh.clone()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn((
        Bob(Vec3::X, Vec3::X),
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(-1.0, 0.0, 0.0)),
            mesh: meshes.add(mesh.clone()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::PURPLE,
                    base_color_texture: Some(asset_server.load("boys.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn((
        Bob(Vec3::Z, Vec3::Z),
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(1.0, 0.0, 0.0)),
            mesh: meshes.add(mesh.clone()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::RED,
                    base_color_texture: Some(asset_server.load("blast.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    commands.spawn((
        Name::new("Moveme"),
        MaterialMeshBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(
                -std::f32::consts::FRAC_PI_2,
            ))
            .with_translation(Vec3::new(1.0, 0.0, 0.0)),
            mesh: meshes.add(mesh.clone()),
            material: decal_materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::RED,
                    base_color_texture: Some(asset_server.load("UVCheckerMap01-512.png")),
                    alpha_mode: AlphaMode::Blend,
                    ..default()
                },
                extension: DecalMaterial {
                    center_pos: Vec3::default(),
                },
            }),
            ..default()
        },
        NotShadowCaster,
    ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
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
        PanOrbitCamera::default(),
        DepthPrepass,
    ));
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    let decal_query: Vec<Entity> = world
        .query::<(
            Entity,
            &Handle<ExtendedMaterial<StandardMaterial, DecalMaterial>>,
        )>()
        .iter(world)
        .map(|f| f.0)
        .collect();

    egui::Window::new("Decals").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            //
            egui::CollapsingHeader::new("Decals").show(ui, |ui| {
                for entity in decal_query {
                    bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                }
            });
        });
    });

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            //
            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });
            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);

            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
        });
    });
}
