use bevy::prelude::*;

#[derive(Debug, Default)]
struct Tile {
    x: f32,
    z: f32,
}

#[derive(Default)]
pub struct Config {
    pub tile_width: f32,
    pub tile_height: f32,
    pub trunk_ratio: f32,
}

pub(crate) struct TreeBuilderBuilder {
    config: Config,
}

impl TreeBuilderBuilder {
    pub fn new() -> Self {
        Self {
            config: Config {
                tile_width: 1.0,
                tile_height: 2.0,
                trunk_ratio: 0.2,
            },
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    pub fn build(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> TreeBuilder {
        TreeBuilder::new(meshes, materials, self.config)
    }
}

pub(crate) struct TreeBuilder {
    tree_data: Vec<(PbrBundle, PbrBundle)>,
    config: Config,
}

impl TreeBuilder {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        config: Config,
    ) -> Self {
        let leafe_mat = materials.add(StandardMaterial {
            base_color: Color::hex("3bb526").unwrap(),
            metallic: 0.2,
            roughness: 1.0,
            ..Default::default()
        });

        let trunk_mat = materials.add(StandardMaterial {
            base_color: Color::hex("362312").unwrap(),
            metallic: 0.2,
            roughness: 1.0,
            ..Default::default()
        });

        let tree_data = (0..8)
            .map(|size| {
                let trunk_height = config.tile_height * config.trunk_ratio;
                let leafe_space = (config.tile_height - trunk_height) * (size as f32 / 8.0);

                let treetop_center_y = trunk_height + (leafe_space / 2.0);
                let leafes_mesh = meshes.add(Mesh::from(shape::Box::new(
                    config.tile_width * 0.8,
                    leafe_space,
                    config.tile_width * 0.8,
                )));
                let leafes = PbrBundle {
                    mesh: leafes_mesh,
                    material: leafe_mat.clone(),
                    transform: Transform::from_xyz(0.0, treetop_center_y, 0.0),
                    ..Default::default()
                };
                let trunk_mesh = meshes.add(Mesh::from(shape::Box::new(
                    config.tile_width * 0.2,
                    trunk_height,
                    config.tile_width * 0.2,
                )));
                let trunk = PbrBundle {
                    mesh: trunk_mesh,
                    material: trunk_mat.clone(),
                    transform: Transform::from_xyz(0.0, trunk_height / 2.0, 0.0),
                    ..Default::default()
                };
                (leafes, trunk)
            })
            .collect::<Vec<(PbrBundle, PbrBundle)>>();

        Self { tree_data, config }
    }
    pub fn build_tree_at(&self, x: f32, z: f32, size: f32, commands: &mut Commands) {
        commands
            .spawn_bundle((
                Tile { x, z },
                GlobalTransform::from_xyz(x, 0.0, z),
                Transform::from_xyz(x, 0.0, z),
            ))
            .with_children(|parent| {
                parent.spawn_bundle(self.tree_data[size as usize].0.clone());
                parent.spawn_bundle(self.tree_data[size as usize].1.clone());
            });
    }
}
