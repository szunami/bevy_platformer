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
        .add_system(horizontal_movement.system())
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
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Velocity(Vec2::default()))
        .with(Jumps(JUMP_COUNT))
        .with(Player);


    commands
    .spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
        sprite: Sprite::new(Vec2::new(100.0, 10.0)),
        ..Default::default()
    })
    .with(Platform);

    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
            sprite: Sprite::new(Vec2::new(1000.0, 200.0)),
            ..Default::default()
        })
        .with(Platform);
}

const GRAVITY_ACCELERATION: f32 = -20.0;

fn gravity_system(
    time: Res<Time>,
    mut query: Query<(&Player, &Sprite, &mut Transform, &mut Velocity, &mut Jumps)>,
    platform_query: Query<(&Platform, &Transform, &Sprite)>,
) {
    for (_player, player_sprite, mut player_transform, mut velocity, mut jumps) in query.iter_mut() {
        let mut falling = true;

        for (_platform, platform_transform, platform_sprite) in platform_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                platform_transform.translation,
                platform_sprite.size,
            );

            if let Some(collision) = collision {
                if let collision = Collision::Top {
                    *velocity.0.y_mut() = 0.0;
                    let delta = (platform_transform.translation.y() + platform_sprite.size.y() / 2.0) - (player_transform.translation.y() - player_sprite.size.y() / 2.0);
                    *player_transform.translation.y_mut() += delta;
                    falling = false;
                    jumps.0 = JUMP_COUNT;
                }
            }
        }

        if falling {
            *velocity.0.y_mut() += time.delta_seconds * GRAVITY_ACCELERATION;
        }
    }
}

const MAX_HORIZONTAL_VELOCITY: f32 = 50.0;
const HORIZONTAL_ACCELERATION: f32 = 10.0;

fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,

    mut query: Query<(&Player,  &mut Velocity)>,
) {
    for (_player, mut velocity) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            *velocity.0.x_mut() -= time.delta_seconds * HORIZONTAL_ACCELERATION;
        } else if keyboard_input.pressed(KeyCode::D) {
            *velocity.0.x_mut() += time.delta_seconds * HORIZONTAL_ACCELERATION;
        } else {
            *velocity.0.x_mut() = 0.0;
        }

        *velocity.0.x_mut() = velocity.0.x().min( MAX_HORIZONTAL_VELOCITY).max(-1.0 * MAX_HORIZONTAL_VELOCITY);
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
                *velocity.0.y_mut() += 40.0;
            }
        }
}

fn position_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += time.delta_seconds * velocity.0.x();
        *transform.translation.y_mut() += time.delta_seconds * velocity.0.y();
    }
}
