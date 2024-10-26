use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use physical_constants::{self, NEWTONIAN_CONSTANT_OF_GRAVITATION};
use rapier2d::na::Vector2;

#[derive(Component)]
pub struct CenterOfMassLine;

pub fn gravitational_force(
    mass1: f32,
    mass2: f32,
    position1: Vector2<f32>,
    position2: Vector2<f32>,
) -> Vector2<f32> {
    let r = position2 - position1;
    let direction = r.norm();
    let f_mag = 1000.0 * ((mass1 * mass2) / direction.powi(2));
    r.normalize() * f_mag
}

pub fn apply_gravity(mut bodies: Query<(&ColliderMassProperties, &Transform, &mut ExternalForce)>) {
    let mut combinations = bodies.iter_combinations_mut::<2>();
    while let Some([body1, body2]) = combinations.fetch_next() {
        let (mass_properties_1, translation1, mut ex_force_1) = body1;
        let (mass_properties_2, translation2, mut ex_force_2) = body2;

        //now this is just awful
        let mass1 = match mass_properties_1 {
            ColliderMassProperties::Mass(mass) => Some(*mass),
            _ => Some(0.0),
        };
        let mass2 = match mass_properties_2 {
            ColliderMassProperties::Mass(mass) => Some(*mass),
            _ => Some(0.0),
        };
        let f_1_2 = gravitational_force(
            mass1.unwrap(),
            mass2.unwrap(),
            translation1.translation.truncate().into(),
            translation2.translation.truncate().into(),
        );
        let f_2_1 = -f_1_2;
        ex_force_1.force = f_1_2.into();
        ex_force_2.force = f_2_1.into();
    }
}

pub fn debug_vel_vector(
    mut query_line: Query<&mut Path, With<CenterOfMassLine>>,
    query_body: Query<(&Transform, &Velocity)>,
) {
    for mut path in query_line.iter_mut() {
        for (transform, velocity) in query_body.iter() {
            let center_of_mass = transform.translation.truncate();
            let vel = velocity.linvel * 3.0;
            let new_line = shapes::Line(center_of_mass, center_of_mass + vel);
            *path = ShapePath::build_as(&new_line);
        }
    }
}
