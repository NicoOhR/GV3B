use bevy::{ecs::query, prelude::*};

#[derive(Component)]
struct Body;

#[derive(Resource)]
struct Velocity(f32);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        },
        Body,
    ));
}

fn move_ball(time: Res<Time>, mut query: Query<(&Body, &mut Transform)>, speed: Res<Velocity>) {
    for (_, mut transform) in &mut query {
        transform.translation.x += speed.0 * time.delta_seconds();

        if transform.translation.x > 500.0 {
            transform.translation.x = -500.0;
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Velocity(40.0))
        .add_systems(Startup, setup)
        .add_systems(Update, move_ball)
        .run();
}
