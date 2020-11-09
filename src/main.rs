use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

struct Player;

struct Velocity(Vec2);

struct Platform;

struct Jumps(usize);

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(gravity_system.system())
        .add_system(jump_system.system())
        .add_system(position_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

const JUMP_COUNT: usize = 3;

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
        .with(Jumps(JUMP_COUNT))
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
    mut query: Query<(&Player, &Sprite, &Transform, &mut Velocity, &mut Jumps)>,
    collider_query: Query<(&Transform, &Sprite)>,
) {
    for (_player, player_sprite, player_transform, mut velocity, mut jumps) in query.iter_mut() {
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
                jumps.0 = JUMP_COUNT;
            }
        }

        if falling {
            *velocity.0.y_mut() += GRAVITY_ACCELERATION;
        }
    }
}

fn jump_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Jumps, &mut Velocity)>) {
        if !keyboard_input.just_pressed(KeyCode::Space) {
            return;
        }

        for (mut jump, mut velocity) in query.iter_mut() {
            if jump.0 > 0 {
                jump.0 -= 1;
                *velocity.0.y_mut() += 100.0;
            }
        }
}

fn position_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += time.delta_seconds * velocity.0.x();
        *transform.translation.y_mut() += time.delta_seconds * velocity.0.y();
    }
}
