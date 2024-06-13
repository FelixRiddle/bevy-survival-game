use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {
    pub radius: f32,
}

#[derive(Component)]
pub struct PlanetBundle {
    pub planet: Planet,
    pub transform: Transform,
}

impl PlanetBundle {
    pub fn new(radius: f32) -> Self {
        Self {
            planet: Planet {
                radius,
            },
            transform: Transform::default(),
        }
    }
}



// /// Spawns a blue planet in the middle of the screen
// /// 
// /// 
// pub fn spawn_planet_manually(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let shape = Circle::new(100.);
//     let color_material = ColorMaterial::from(Color::rgb(0.0, 0.0, 1.0));
    
//     commands.spawn((
//         PlanetBundle::new(100.0),
//         ColorMesh2dBundle {
//             // This part ain't working
//             mesh: meshes.add(Mesh::from(shape).into()),
//             material: materials.add(color_material),
//             ..Default::default()
//         }
//     ));
// }
