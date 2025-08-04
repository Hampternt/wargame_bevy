use bevy::prelude::*;

// Components
#[derive(Component, Debug, Clone, Copy, PartialEq)]
struct HexPosition {
    q: i32,  // Hex coordinate Q
    r: i32,  // Hex coordinate R
}

impl HexPosition {
    fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }
    
    // Convert hex coordinates to world position
    fn to_world_pos(&self, hex_size: f32) -> Vec3 {
        let x = hex_size * (3.0_f32.sqrt() * self.q as f32 + 3.0_f32.sqrt() / 2.0 * self.r as f32);
        let y = hex_size * (3.0 / 2.0 * self.r as f32);
        Vec3::new(x, y, 0.0)
    }
}

// Constants
const HEX_SIZE: f32 = 30.0;
const GRID_SIZE: i32 = 8;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hex Wargame".to_string(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_camera, setup_hex_grid))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_hex_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create hex mesh
    let hex_mesh = create_hex_mesh();
    let mesh_handle = meshes.add(hex_mesh);
    
    // Create material
    let material = materials.add(ColorMaterial::from(Color::srgb(0.8, 0.8, 0.8)));
    
    // Generate hex grid
    for q in -GRID_SIZE..=GRID_SIZE {
        for r in -GRID_SIZE..=GRID_SIZE {
            if (q + r).abs() <= GRID_SIZE {
                let hex_pos = HexPosition::new(q, r);
                let world_pos = hex_pos.to_world_pos(HEX_SIZE);
                
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: mesh_handle.clone().into(),
                        material: material.clone(),
                        transform: Transform::from_translation(world_pos),
                        ..default()
                    },
                    hex_pos,
                ));
            }
        }
    }
}

fn create_hex_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    
    // Create hexagon vertices
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Center vertex
    vertices.push([0.0, 0.0, 0.0]);
    
    // Outer vertices (6 points of hexagon)
    for i in 0..6 {
        let angle = i as f32 * std::f32::consts::PI / 3.0;
        let x = HEX_SIZE * angle.cos();
        let y = HEX_SIZE * angle.sin();
        vertices.push([x, y, 0.0]);
    }
    
    // Create triangles from center to each edge
    for i in 0..6 {
        let next = (i + 1) % 6;
        indices.extend_from_slice(&[0, (i + 1) as u32, (next + 1) as u32]);
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}
