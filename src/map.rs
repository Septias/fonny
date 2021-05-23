//! Everything related to map generation
use bevy::prelude::*;

/// A tile in the grid of tiles
#[derive(Debug, Default)]
struct Tile {
    x: f32,
    z: f32,
}

/// Config for the treebuilder
pub struct TreeBuilderConfig {
    pub tile_width: f32,
    pub tile_height: f32,
    pub trunk_ratio: f32,
    pub leaf_padding_perc: f32,
}

impl Default for TreeBuilderConfig {
    fn default() -> Self {
        TreeBuilderConfig {
            tile_width: 1.0,
            tile_height: 2.0,
            trunk_ratio: 0.2,
            leaf_padding_perc: 0.8,
        }
    }
}
/// Tree-builder builder
pub(crate) struct TreeBuilderBuilder {
    config: TreeBuilderConfig,
    /// number of different tree levels
    levels: u32,
}

#[allow(dead_code)]
impl TreeBuilderBuilder {
    pub fn new() -> Self {
        Self {
            config: TreeBuilderConfig::default(),
            levels: 8,
        }
    }

    pub fn with_config(mut self, config: TreeBuilderConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_levels(mut self, levels: u32) -> Self {
        self.levels = levels;
        self
    }

    pub fn build(
        self,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> TreeBuilder {
        TreeBuilder::new(meshes, materials, self.config, self.levels)
    }
}

/// Treebuilder which creates tree-entities
pub(crate) struct TreeBuilder {
    tree_data: Vec<(PbrBundle, PbrBundle)>,
    levels: u32,
}

impl TreeBuilder {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        config: TreeBuilderConfig,
        levels: u32,
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

        let trunk_height = 1.0;
        let leaf_width = config.tile_width * config.leaf_padding_perc;
        let leaf_base = leaf_width;
        let tree_data = (0..=levels)
            .map(|size| {
                let leafe_space = leaf_base
                    + (config.tile_height - trunk_height - leaf_base)
                        * (size as f32 / levels as f32);
                let leafes_mesh = meshes.add(Mesh::from(shape::Box::new(
                    leaf_width,
                    leafe_space,
                    leaf_width,
                )));
                let leafes = PbrBundle {
                    mesh: leafes_mesh,
                    material: leafe_mat.clone(),
                    transform: Transform::from_xyz(0.0, trunk_height + (leafe_space / 2.0), 0.0),
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

        Self { tree_data, levels }
    }

    /// create a tree at given coordinates
    ///# Arguments
    /// * `size` - size of the tree that must be less then [`TreeBuilder::levels`]
    pub fn build_tree_at(&self, x: f32, z: f32, size: f32, commands: &mut Commands) {
        if size as u32 > self.levels {
            panic!("size of tree must be betwen 0 and `[TreeBuilder::levels]`, is: {} ", size)
        }
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

#[allow(dead_code)]
pub mod generators {
    //! Map Generators
    /// Generates a crate-like structure
    pub struct CraterGenerator {
         /// width of map
         width: i32,
         /// depth of map
         depth: i32,
         /// Number of tree-levels
         levels: i32,
    }

    impl CraterGenerator {
        pub fn compute(&self, x: i32, z: i32) -> f32 {
            let max = ((self.width.pow(2) + self.depth.pow(2)) as f32).sqrt();
            let val = ((x.pow(2) + z.pow(2)) as f32).sqrt();
            val / max * self.levels as f32
        }
        pub fn new(width: i32, depth: i32, levels: i32) -> Self {
            Self {
                width,
                depth,
                levels,
            }
        }
    }

    /// Generates a donut-like structure
    pub struct DonutGenerator {
        /// width of map
        width: i32,
        /// depth of map
        depth: i32,
        /// Number of tree-levels
        levels: i32,
        /// radius at which the donut has it's peak
        radius: f32,
    }

    impl DonutGenerator {
        pub fn compute(&self, x: i32, z: i32) -> f32 {
            let val = ((x.pow(2) + z.pow(2)) as f32).sqrt();
            self.levels as f32 / ((val - self.radius).abs() / 3.0).max(1.0)
        }
        pub fn new(width: i32, depth: i32, levels: i32, radius: f32) -> Self {
            Self {
                width,
                depth,
                levels,
                radius,
            }
        }
    }
}
