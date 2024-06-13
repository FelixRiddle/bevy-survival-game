use bevy::prelude::*;
// use bevy::sprite::MaterialMesh2dBundle;
// use bevy::ecs::system::Command;

// use super::planet::PlanetBundle;

pub struct SpawnPlanet {
    pub radius: f32,
    pub position: Vec2,
}

// Mesh handle and material handle don't return anything, so this doesn't works
// impl Command for SpawnPlanet {
//     fn apply(self, world: &mut World) {
//         // Resource scope works by removing the resource from the world
//         // and then running our closure. This lets us mutably borrow
//         // the world safely multiple times
//         let mesh_handle = world.resource_scope(|_world, mut meshes: Mut<Assets<Mesh>>| {
//             let shape = Circle::new(self.radius);
//             meshes.add(Mesh::from(shape));
//         });
        
//         let material_handle = world.resource_scope(|_world, mut materials: Mut<Assets<ColorMaterial>>| {
//             let material = ColorMaterial::from(Color::rgb(0.0, 0.0, 1.0));
//             materials.add(material);
//         });
        
//         world.spawn((
//             PlanetBundle::new(self.radius),
//             MaterialMesh2dBundle {
//                 mesh: mesh_handle,
//                 material: material_handle,
//                 transform: Transform::from_translation(self.position.extend(0.)),
//                 ..default()
//             }
//         ));
//     }
// }
