use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::MassProperties};
use rapier2d::na::Vector2;
mod bodies;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, bodies::apply_gravity)
        .add_systems(Update, bodies::debug_vel_vector)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let mass1 = 60.0;
    let mass2 = 600.0;

    let line = shapes::Line(Vec2::ZERO, Vec2::new(100.0, 0.0));
    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&line),
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0), // White line with 2.0 thickness
        bodies::CenterOfMassLine,       // Mark this entity with a tag component
    ));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.0))
        .insert(ColliderMassProperties::Mass(mass1))
        .insert(ExternalForce::default())
        .insert(Velocity {
            linvel: Vec2::new(0.0, 25.0),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(500.0, 0.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Restitution::coefficient(0.9))
        .insert(ColliderMassProperties::Mass(mass2))
        .insert(GravityScale(0.0))
        .insert(ExternalForce::default())
        .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
