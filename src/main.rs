use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

struct Player;

struct Velocity(Vec2);

struct Platform;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(gravity_system.system())
        .add_system(position_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Velocity(Vec2::default()))
        .with(Player);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            sprite: Sprite::new(Vec2::new(1000.0, 10.0)),
            ..Default::default()
        })
        .with(Platform);
}

const GRAVITY_ACCELERATION: f32 = -1.0;

fn gravity_system(
    mut query: Query<(&Player, &Sprite, &Transform, &mut Velocity)>,
    collider_query: Query<(&Transform, &Sprite)>,
) {
    for (_player, player_sprite, player_transform, mut velocity) in query.iter_mut() {
        let mut falling = true;

        for (transform, sprite) in collider_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                transform.translation,
                sprite.size,
            );

            if let Some(collision) = collision {
                println!("Player: {:?}", player_transform);
                *velocity.0.y_mut() = 0.0;
                falling = false;
            }
        }

        if falling {
            *velocity.0.y_mut() += GRAVITY_ACCELERATION;
        }
    }
}

fn position_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += time.delta_seconds * velocity.0.x();
        *transform.translation.y_mut() += time.delta_seconds * velocity.0.y();
    }
}
