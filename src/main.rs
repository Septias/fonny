use bevy::{pbr::AmbientLight, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use map::{TreeBuilderBuilder, TreeBuilderConfig};
mod map;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tree_builder = TreeBuilderBuilder::new()
        .with_config(TreeBuilderConfig {
            tile_height: 10.0,
            ..Default::default()
        })
        .with_levels(20)
        .build(&mut meshes, &mut materials);

    let mapgen = map::generators::DonutGenerator::new(100, 100, 20, 15.0);

    for x in -50..50 {
        for z in -50..50 {
            if x % 2 == 0 && z % 2 == 0 {
                let size = mapgen.compute(x, z);
                tree_builder.build_tree_at(x as f32, z as f32, size, &mut commands);
            }
        }
    }
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        light: Light {
            intensity: 800.0,
            range: 120.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
