use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use physical_constants::{self, NEWTONIAN_CONSTANT_OF_GRAVITATION};
use rapier2d::na::Vector2;

#[derive(Component)]
struct CenterOfMassLine;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, apply_gravity)
        .add_systems(Update, debug_vel_vector)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn gravitational_force(
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

fn debug_vel_vector(
    mut query_line: Query<&mut Path, With<CenterOfMassLine>>,
    query_body: Query<(&Transform, &Velocity)>,
) {
    for mut path in query_line.iter_mut() {
        for (transform, velocity) in query_body.iter() {
            let center_of_mass = transform.translation.truncate();
            let vel = velocity.linvel.normalize();
            let new_line = shapes::Line(center_of_mass, vel);
            *path = ShapePath::build_as(&new_line);
        }
    }
}

fn apply_gravity(mut bodies: Query<(&Collider, &Transform, &mut ExternalForce)>) {
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

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let mass1 = 40.0;
    let mass2 = 200.0;

    let line = shapes::Line(Vec2::ZERO, Vec2::new(100.0, 0.0));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0), // White line with 2.0 thickness
        CenterOfMassLine,               // Mark this entity with a tag component
    ));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(mass1))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.0))
        .insert(ExternalForce::default())
        .insert(Velocity {
            linvel: Vec2::new(0.0, 25.0),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(mass2))
        .insert(Restitution::coefficient(0.9))
        .insert(GravityScale(0.0))
        .insert(ExternalForce::default())
        .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
