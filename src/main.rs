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
    let triangle = make_hexagon();
    let mesh_handle = meshes.add(triangle);
    let material = materials.add(ColorMaterial::from(Color::WHITE));
    
    // First hexagon at (0, 0)
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    ));
    
    // Second hexagon to the right
    commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(870.0, 0.0, 0.0)), // Magic number!
    ));
}

//fn make_triangle() -> Mesh {
//    let mut mesh = Mesh::new(
//        bevy::render::render_resource::PrimitiveTopology::TriangleList,
//        bevy::render::render_asset::RenderAssetUsages::default(),
//    );
//
//    // 3 dots that make a triangle
//    let vertices = vec![
//        [0.0, 50.0, 0.0],    // Top point
//        [-50.0, -50.0, 0.0], // Bottom left
//        [50.0, -50.0, 0.0],  // Bottom right
//    ];
//
//    // Tell Bevy to connect dots 0, 1, 2 into a triangle
//    let indices = vec![0, 1, 2];
//
//    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
//    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
//    mesh
//}


fn make_hexagon() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    
    // POINTY-TOP hexagon (rotated 30 degrees)
    let vertices = vec![
        [0.0, 0.0, 0.0],        // Center
        [0.0, 50.0, 0.0],       // Top point
        [43.3, 25.0, 0.0],      // Top-right
        [43.3, -25.0, 0.0],     // Bottom-right  
        [0.0, -50.0, 0.0],      // Bottom point
        [-43.3, -25.0, 0.0],    // Bottom-left
        [-43.3, 25.0, 0.0],     // Top-left
    ];
    
    let indices = vec![
        0, 1, 2,  0, 2, 3,  0, 3, 4,  
        0, 4, 5,  0, 5, 6,  0, 6, 1,
    ];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    mesh
}
