use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    
    // Make a triangle from 3 dots
    let triangle = make_triangle();
    let mesh_handle = meshes.add(triangle);
    let material = materials.add(ColorMaterial::from(Color::WHITE));
    
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(material),
    ));
}

fn make_triangle() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    
    // 3 dots that make a triangle
    let vertices = vec![
        [0.0, 50.0, 0.0],    // Top point
        [-50.0, -50.0, 0.0], // Bottom left
        [50.0, -50.0, 0.0],  // Bottom right
    ];
    
    // Tell Bevy to connect dots 0, 1, 2 into a triangle
    let indices = vec![0, 1, 2];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}
