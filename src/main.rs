use bevy::prelude::*;

struct Player;

struct Velocity(Vec2);


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
            transform: Transform::from_translation(Vec3::new(
                0.0,
                0.0,
                0.0,
            )),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(Velocity(Vec2::default()))
        .with(Player);
}

const GRAVITY_ACCELERATION: f32 = -1.0;

fn gravity_system(mut query: Query<(&Player, &mut Velocity)>) {
    for (player, mut velocity) in query.iter_mut() {
        *velocity.0.y_mut() += GRAVITY_ACCELERATION;
    }
}

fn position_system(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += velocity.0.x();
        *transform.translation.y_mut() += velocity.0.y();
    }
}