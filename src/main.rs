use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

struct Player;

#[derive(Debug)]
struct Velocity(Vec2);

struct Platform;

struct Jumps(usize);

#[derive(Debug)]
struct WalkLoop {
    walk_state: WalkState,
    timer: Timer,
}

#[derive(Debug)]
enum WalkState {
    Standing,
    Walking(u32),
}

impl WalkLoop {
    fn default() -> WalkLoop {
        WalkLoop {
            walk_state: WalkState::Standing,
            timer: Timer::from_seconds(0.1, true),
        }
    }

    fn stop(&mut self) {
        self.walk_state = WalkState::Standing;
    }

    fn increment(&mut self, time: &Res<Time>) {
        match self.walk_state {
            WalkState::Standing => {
                self.walk_state = WalkState::Walking(0);
            }
            WalkState::Walking(frame) => {
                self.timer.tick(time.delta_seconds());
                if self.timer.finished() {
                    self.walk_state = WalkState::Walking((frame + 1) % 8);
                }
            }
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(gravity_system.system())
        .add_system(horizontal_movement.system())
        .add_system(jump_system.system())
        .add_system(position_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(animate_sprite_system.system())
        .run();
}

const JUMP_COUNT: usize = 3;

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let walk_handle = asset_server.load("textures/sam.png");
    let walk_atlas = TextureAtlas::from_grid(walk_handle, Vec2::new(27.0, 52.0), 9, 1);
    let walk_handle = texture_atlases.add(walk_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: walk_handle,
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
            ..Default::default()
        })
        .with(Velocity(Vec2::default()))
        .with(Jumps(JUMP_COUNT))
        .with(WalkLoop::default())
        .with(Player);
}

fn animate_sprite_system(mut query: Query<(&mut TextureAtlasSprite, &WalkLoop)>) {
    for (mut sprite, walk_loop) in query.iter_mut() {
        match walk_loop.walk_state {
            WalkState::Standing => {
                sprite.index = 8;
            }
            WalkState::Walking(index) => {
                sprite.index = index;
            }
        }
    }
}

const GRAVITY_ACCELERATION: f32 = -20.0;

fn gravity_system(
    time: Res<Time>,
    mut query: Query<(&Player, &Sprite, &mut Transform, &mut Velocity, &mut Jumps)>,
    platform_query: Query<(&Platform, &Transform, &Sprite)>,
) {
    for (_player, player_sprite, mut player_transform, mut velocity, mut jumps) in query.iter_mut()
    {
        let mut falling = true;

        for (_platform, platform_transform, platform_sprite) in platform_query.iter() {
            let collision = collide(
                player_transform.translation,
                player_sprite.size,
                platform_transform.translation,
                platform_sprite.size,
            );

            if let Some(collision) = collision {
                if let Collision::Top = collision {
                    velocity.0.y = 0.0;
                    let delta = (platform_transform.translation.y + platform_sprite.size.y / 2.0)
                        - (player_transform.translation.y - player_sprite.size.y / 2.0);
                    player_transform.translation.y += delta;
                    falling = false;
                    jumps.0 = JUMP_COUNT;
                }
            }
        }

        if falling {
            velocity.0.y += time.delta_seconds() * GRAVITY_ACCELERATION;
        }
    }
}

const MAX_HORIZONTAL_VELOCITY: f32 = 50.0;
const HORIZONTAL_ACCELERATION: f32 = 10.0;

fn horizontal_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,

    mut query: Query<(&Player, &mut Transform, &mut Velocity, &mut WalkLoop)>,
) {
    for (_player, mut _player_transform, mut velocity, mut walk_loop) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            velocity.0.x -= time.delta_seconds() * HORIZONTAL_ACCELERATION;
        } else if keyboard_input.pressed(KeyCode::D) {
            velocity.0.x += time.delta_seconds() * HORIZONTAL_ACCELERATION;
            walk_loop.increment(&time);
        } else {
            velocity.0.x = 0.0;
            walk_loop.stop();
        }

        velocity.0.x = velocity
            .0
            .x
            .min(MAX_HORIZONTAL_VELOCITY)
            .max(-1.0 * MAX_HORIZONTAL_VELOCITY);
    }
}

fn jump_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Jumps, &mut Velocity)>) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    for (mut jump, mut velocity) in query.iter_mut() {
        if jump.0 > 0 {
            jump.0 -= 1;

            velocity.0.y += 40.0;
        }
    }
}

fn position_system(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += time.delta_seconds() * velocity.0.x;
        transform.translation.y += time.delta_seconds() * velocity.0.y;
    }
}
