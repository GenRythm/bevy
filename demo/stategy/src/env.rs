
use bevy::prelude::*;

use super::{
    camera::{
        LookAtTarget,
        CameraPlugin,
    },
    pickup::*,
};

pub struct EnvironmentPlugin;
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(CameraPlugin)
            .add_startup_system(start_up_env.system())
            .add_startup_system(set_highlight_params.system())
            // .add_system(get_picks.system())
        ;
    }
}

fn start_up_env(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let camera_entity = Entity::new();

    let target_entity = commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(0.1, 0.1)))),
            material: materials.add(Color::default().into()),
            draw: Draw {
                is_visible: false,
                is_transparent: true,
                render_commands: Default::default(),
            },
            ..Default::default()
        })
        .with(LookAtTarget::new(camera_entity))
        .current_entity()
        .unwrap();

    let ground_mesh = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.9, 1.9))));
    let ground_material = materials.add(Color::rgb(0.9, 0.9, 0.9).into());
    const AMOUNT: i32 = 12;
    for x in -(AMOUNT / 2)..(AMOUNT / 2) {
        for y in -(AMOUNT / 2)..(AMOUNT / 2) {
            commands
                .spawn(PbrComponents {
                  mesh: ground_mesh,
                  material: materials.add(Color::rgba(0.9, 0.9, 0.9, 0.05).into()),
                  translation: Translation::new((x * 2) as f32, 0.1, (y * 2) as f32),
                  rotation: Rotation::from_rotation_x(-90f32.to_radians()),
                  draw: Draw {
                      is_transparent: true,
                      ..Default::default()
                  },
                  ..Default::default()
                })
                .with(PickableMesh::new(camera_entity))
                .with(HighlightablePickMesh::new())
                .with(SelectablePickMesh::new())
            ;
        }
    }
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 30.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(6.0, 1.0, -2.0),
            ..Default::default()
        })
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 1.,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            translation: Translation::new(-8.0, 1., -8.),
            ..Default::default()
        })
        //quad
        // .spawn(PbrComponents {
        //     mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
        //     material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
        //     translation: Translation::new(0.5, 3.5, 0.5),
        //     ..Default::default()
        // })
        // tree1
        .spawn(PbrComponents {
            // load a mesh from glTF
            mesh: asset_server
                .load("assets/models/tree1.gltf")
                .unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            ..Default::default()
        })
        // tree3
        .spawn(PbrComponents {
            // load a mesh from glTF
            mesh: asset_server
                .load("assets/models/tree3.glb")
                .unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(4.0, 0.0, 4.0),
            ..Default::default()
        })
        // monkey
        .spawn(PbrComponents {
            // load a mesh from binary glTF
            mesh: asset_server
                .load("assets/models/monkey/Monkey.glb")
                .unwrap(),
            // create a material for the mesh
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            translation: Translation::new(-6., 1., 4.),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        })
        // camera
        .spawn_as_entity(
            camera_entity,
            Camera3dComponents::default(),
        )
        .push_children(target_entity, &[camera_entity])
        ;
}

fn set_highlight_params(mut highlight_params: ResMut<PickHighlightParams>) {
    highlight_params.set_hover_color(Color::rgb(0.8, 1.0, 1.0));
    highlight_params.set_selection_color(Color::rgb(1.0, 0.0, 1.0));
}

fn get_picks(pick_state: ResMut<PickState>) {
    println!("All entities:\n{:?}", pick_state.list());
    println!("Top entity:\n{:?}", pick_state.top());
}
