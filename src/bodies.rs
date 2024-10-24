use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
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
    let f_mag = 1000000.0 * ((mass1 * mass2) / direction.powi(2));
    r.normalize() * f_mag
}

pub fn apply_gravity(mut bodies: Query<(&Collider, &Transform, &mut ExternalForce)>) {
    let mut combinations = bodies.iter_combinations_mut::<2>();
    while let Some([body1, body2]) = combinations.fetch_next() {
        let (mass1, translation1, mut ex_force) = body1;
        let (mass2, translation2, _) = body2;
        let f_1_2 = gravitational_force(
            mass1.as_ball().unwrap().radius(),
            mass2.as_ball().unwrap().radius(),
            translation1.translation.truncate().into(),
            translation2.translation.truncate().into(),
        );
        ex_force.force = f_1_2.into();
    }
}

pub fn debug_vel_vector(
    mut query_line: Query<&mut Path, With<CenterOfMassLine>>,
    query_body: Query<(&Transform, &Velocity)>,
) {
    for mut path in query_line.iter_mut() {
        for (transform, velocity) in query_body.iter() {
            let center_of_mass = transform.translation.truncate();
            let vel = velocity.linvel;
            let new_line = shapes::Line(center_of_mass, center_of_mass + vel);
            *path = ShapePath::build_as(&new_line);
        }
    }
}
