use bevy::{math::VectorSpace, prelude::*};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bodies::apply_gravity;
mod bodies;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed {
                dt: 1.0 / 10.0,
                substeps: 1,
            },
            gravity: Vec2::new(0.0, 0.0),
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10, // Set subdivision level for scaled shapes
            force_update_from_transform_changes: true, // Force updates based on transform changes
        })
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Startup, bodies::setup_vectors.after(setup_physics))
        .add_systems(Update, bodies::apply_gravity)
        .add_systems(Update, camera_update)
        .add_systems(Update, bodies::debug_vel_vector.after(apply_gravity))
        .run();
}
fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.5,
            ..default()
        },
        ..default()
    });
}

fn camera_update(
    q_bodies: Query<&Transform, With<Velocity>>,
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Velocity>)>,
) {
    let mut average: Vec3 = Vec3::ZERO;
    for transform in q_bodies.iter() {
        average = average + transform.translation;
    }
    average /= 3.0;
    for mut transform in camera.iter_mut() {
        average.z = transform.translation.z;
        transform.translation = average;
    }
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let mass1 = 60.0;
    let mass2 = 60.0;
    let mass3 = 60.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Restitution::coefficient(1.0))
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
        .insert(Restitution::coefficient(1.0))
        .insert(ColliderMassProperties::Mass(mass3))
        .insert(ExternalForce::default())
        .insert(Velocity {
            linvel: Vec2::new(0.0, 25.0),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-600.0, 0.0, 0.0)));
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(40.0))
        .insert(Restitution::coefficient(1.0))
        .insert(ColliderMassProperties::Mass(mass2))
        .insert(ExternalForce::default())
        .insert(Velocity {
            linvel: Vec2::new(0.0, -25.0),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
}
