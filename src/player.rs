use crate::animation::SpriteSheetAnimation;
use crate::from_component::FromComponentPlugin;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::control::{KinematicCharacterController, KinematicCharacterControllerOutput};
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::geometry::Collider;

// const PLAYER_VELOCITY_X: f32 = 400.0;
const PLAYER_VELOCITY_X: f32 = 260.0;
const PLAYER_VELOCITY_Y: f32 = 280.0;

// const MAX_JUMP_HEIGHT: f32 = 250.0;
const MAX_JUMP_HEIGHT: f32 = 120.0;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Component, Default)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Running,
    Falling,
    Jump,
    Attack,
}

impl From<PlayerAnimationState> for SpriteSheetAnimation {
    fn from(animation_state: PlayerAnimationState) -> Self {
        match animation_state {
            PlayerAnimationState::Idle => SpriteSheetAnimation {
                indices: 5..20,
                frame_timer: Timer::from_seconds(1. / 12., TimerMode::Repeating),
                repeat: true,
            },
            PlayerAnimationState::Running => SpriteSheetAnimation {
                indices: 28..34,
                frame_timer: Timer::from_seconds(1. / 12., TimerMode::Repeating),
                repeat: true,
            },
            PlayerAnimationState::Falling => SpriteSheetAnimation {
                indices: 20..22,
                frame_timer: Timer::from_seconds(1. / 2., TimerMode::Repeating),
                repeat: false,
            },
            PlayerAnimationState::Jump => SpriteSheetAnimation {
                indices: 26..28,
                frame_timer: Timer::from_seconds(1. / 2., TimerMode::Repeating),
                repeat: false,
            },
            PlayerAnimationState::Attack => SpriteSheetAnimation {
                indices: 0..5,
                frame_timer: Timer::from_seconds(1. / 8., TimerMode::Repeating),
                repeat: false,
            },
        }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FromComponentPlugin::<
            PlayerAnimationState,
            SpriteSheetAnimation,
        >::new())
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    movement,
                    start_attack,
                    swing_sword,
                    rise,
                    jump,
                    fall,
                    apply_movement_animation,
                    update_direction,
                    update_sprite_direction,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: textures.april.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_xyz(512., 0., 1.),
            // transform: Transform {
            //     translation: Vec3::new(1200., 400.0, 1.0),
            //     // scale: Vec3::new(
            //     //     SPRITE_RENDER_WIDTH / SPRITE_TILE_WIDTH,
            //     //     SPRITE_RENDER_HEIGHT / SPRITE_TILE_HEIGHT,
            //     //     1.0,
            //     // ),
            //     ..Default::default()
            // },
            ..Default::default()
        })
        .insert(Player)
        .insert(RigidBody::KinematicPositionBased)
        /*.insert(Collider::cuboid(
            SPRITE_TILE_WIDTH / 2.0,
            SPRITE_TILE_HEIGHT / 2.0,
        ))*/
        // .insert(Collider::capsule_y(16., 16.))
        // .insert(Collider::capsule_y(104., 32.))
        .insert(Collider::capsule_y(18., 16.))
        //.insert(Collider::cuboid(32. / 2.0, 64. / 2.0))
        .insert(KinematicCharacterController::default())
        .insert(Direction::Right)
        // .insert(Animation::new(SPRITE_IDX_STAND, CYCLE_DELAY))
        .insert(PlayerAnimationState::Idle)
        .insert(JumpBuffer(0.1))
        .insert(AirBuffer(0.1))
        .insert(ActorStatus {
            attack_timer: 0.0,
            attacking: false,
        });
}

fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
) {
    let mut player = query.single_mut();

    let mut movement = 0.0;

    if input.pressed(KeyCode::Right) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X;
    }

    if input.pressed(KeyCode::Left) {
        movement += time.delta_seconds() * PLAYER_VELOCITY_X * -1.0;
    }

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(movement, vec.y)),
        None => player.translation = Some(Vec2::new(movement, 0.0)),
    }
}

#[derive(Component)]
struct Jump(f32);

#[derive(Component)]
struct Swing(f32);

#[derive(Component)]
pub struct WeaponSwingAttackComponent {
    pub attack_duration: Timer,
}

#[derive(Component)]
pub struct ActorStatus {
    pub attacking: bool,
    pub attack_timer: f32,
}
#[derive(Component)]
struct AirBuffer(f32);

#[derive(Component)]
struct JumpBuffer(f32);

#[derive(Component)]
pub enum Direction {
    Right,
    Left,
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &KinematicCharacterControllerOutput),
        (
            With<KinematicCharacterController>,
            Without<Jump>,
            With<Player>,
        ),
    >,
) {
    for (player, output) in &mut query {
        // info!("output {:?}", output);
        if input.pressed(KeyCode::Up) && output.grounded {
            commands.entity(player).insert(Jump(0.0));
        }
    }
    // if query.is_empty() {
    //     return;
    // }
    //
    // let (player, output) = query.single();
    // info!("output {:?}", output);
    //
    // if input.pressed(KeyCode::Up) && output.grounded {
    //     commands.entity(player).insert(Jump(0.0));
    // }
}

fn start_attack(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &KinematicCharacterControllerOutput,
            &mut ActorStatus,
        ),
        (
            With<KinematicCharacterController>,
            Without<Swing>,
            With<Player>,
        ),
    >,
) {
    for (player, output, mut status) in &mut query {
        // info!("output {:?}", output);
        if input.pressed(KeyCode::X) && output.grounded {
            status.attacking = true;
            commands.entity(player).insert(Swing(0.0));
        }
    }
}

fn swing_sword(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Swing, &mut ActorStatus), With<Player>>,
) {
    if query.is_empty() {
        return;
    }
    for (entity, mut swing, mut actor_status) in query.iter_mut() {
        let attack = time.delta().as_secs_f32();
        // 1. / 30.
        // 0.65
        if attack + swing.0 >= 0.5 {
            info!("DONE ATTACKING");
            actor_status.attacking = false;
            commands.entity(entity).remove::<Swing>();
        }
        swing.0 += attack;
    }

    // let (entity, mut player, mut swing, mut animation_state) = query.single_mut();
    //
    // let mut attack = time.delta().as_secs_f32();
    // if attack + swing.0 >= 0.2 {
    //     commands.entity(entity).remove::<Swing>();
    // }
    // swing.0 += attack;
    // if *animation_state != PlayerAnimationState::Attack {
    //     info!("ATTACK!");
    //     *animation_state = PlayerAnimationState::Attack;
    // }
    // // if swing.0 >= 0.5 {
    // //     commands.entity(entity).remove::<Swing>();
    // // } else {
    // //     if *animation_state != PlayerAnimationState::Attack {
    // //         info!("ATTACK!");
    // //         *animation_state = PlayerAnimationState::Attack;
    // //     }
    // // }
}

fn rise(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut KinematicCharacterController, &mut Jump), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    let (entity, mut player, mut jump) = query.single_mut();

    let mut movement = time.delta().as_secs_f32() * 650.;

    if movement + jump.0 >= MAX_JUMP_HEIGHT {
        movement = MAX_JUMP_HEIGHT - jump.0;
        commands.entity(entity).remove::<Jump>();
    }

    jump.0 += movement;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn fall(
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController, (Without<Jump>, With<Player>)>,
) {
    if query.is_empty() {
        return;
    }

    let mut player = query.single_mut();
    let movement = time.delta().as_secs_f32() * (PLAYER_VELOCITY_Y / 0.95) * -1.0;

    match player.translation {
        Some(vec) => player.translation = Some(Vec2::new(vec.x, movement)),
        None => player.translation = Some(Vec2::new(0.0, movement)),
    }
}

fn apply_movement_animation(
    mut query: Query<
        (
            &mut PlayerAnimationState,
            &KinematicCharacterControllerOutput,
            &ActorStatus,
        ),
        With<Player>,
    >,
) {
    if query.is_empty() {
        return;
    }

    for (mut animation_state, output, status) in query.iter_mut() {
        // info!("{:?}", output);
        // commands.entity(player).remove::<Animation>();
        if status.attacking {
            if *animation_state != PlayerAnimationState::Attack {
                *animation_state = PlayerAnimationState::Attack;
            }
        } else if output.desired_translation.x != 0.0 && output.grounded {
            // *animation = Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY);
            if *animation_state != PlayerAnimationState::Running {
                *animation_state = PlayerAnimationState::Running;
            }

            // animation.sprites = SPRITE_IDX_WALKING;
            // commands
            //     .entity(player)
            //     .insert(Animation::new(SPRITE_IDX_WALKING, CYCLE_DELAY));
        } else if output.desired_translation.x == 0.0 && output.grounded {
            //*animation = Animation::new(SPRITE_IDX_STAND, CYCLE_DELAY);
            if *animation_state != PlayerAnimationState::Idle {
                *animation_state = PlayerAnimationState::Idle;
            }
            // animation.sprites = SPRITE_IDX_STAND;
            // commands
            //     .entity(player)
            //     .insert(Animation::new(SPRITE_IDX_STAND, CYCLE_DELAY));
        } else if output.desired_translation.y > 0.0 && !output.grounded {
            if *animation_state != PlayerAnimationState::Jump {
                *animation_state = PlayerAnimationState::Jump;
            }
            //*animation = Animation::new(&[16, 21], CYCLE_DELAY);

            // animation.sprites = &[16, 21];
            //animation.timer = Timer::new(Duration::from_millis(500), TimerMode::Repeating)
        } else if output.desired_translation.y < 0.0 && !output.grounded {
            //*animation = Animation::new(&[22, 23], CYCLE_DELAY);

            // animation.sprites = &[22, 23];
            if *animation_state != PlayerAnimationState::Falling {
                *animation_state = PlayerAnimationState::Falling;
            }
        }
    }
}

fn update_direction(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), With<Player>>,
) {
    if query.is_empty() {
        return;
    }

    for (player, output) in query.iter() {
        if output.desired_translation.x > 0.0 {
            commands.entity(player).insert(Direction::Left);
        } else if output.desired_translation.x < 0.0 {
            commands.entity(player).insert(Direction::Right);
        }
    }
}
fn update_sprite_direction(mut query: Query<(&mut TextureAtlasSprite, &Direction), With<Player>>) {
    if query.is_empty() {
        return;
    }

    for (mut sprite, direction) in &mut query {
        match direction {
            Direction::Right => sprite.flip_x = true,
            Direction::Left => sprite.flip_x = false,
        }
    }
}
