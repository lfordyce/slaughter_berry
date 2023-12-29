use crate::camera::camera_fit_inside_current_level;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_parallax::ParallaxSystems;
use bevy_rapier2d::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_level)
            .add_systems(
                Update,
                camera_fit_inside_current_level
                    .before(ParallaxSystems)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Component)]
pub struct Level;

struct Coordinates {
    from: Vec2,
    to: Vec2,
}

fn setup_level(mut cmd: Commands, textures: Res<TextureAssets>) {
    cmd.spawn(SpriteBundle {
        // transform: Transform::from_translation(Vec3::new(512. / 2., 0., 0.)),
        sprite: Sprite {
            ..Default::default()
        },
        texture: textures.ground.clone(),
        ..Default::default()
    })
    .insert(Level)
    .with_children(|p| {
        // // 2234
        // // 228
        let coords = vec![
            Coordinates {
                from: Vec2::new(2234. - 1208., -144.),
                to: Vec2::new(2416. - 1208., -84.),
            },
            Coordinates {
                from: Vec2::new(1633. - 1208., -144.),
                to: Vec2::new(2100. - 1208., -84.),
            },
            Coordinates {
                from: Vec2::new(1162. - 1208., -144.),
                to: Vec2::new(1539. - 1208., -84.),
            },
            Coordinates {
                from: Vec2::new(-1208., -144.),
                to: Vec2::new(1080. - 1208., -84.),
            },
        ];

        for cord in &coords {
            let from = cord.from;
            let to = cord.to;
            let size = from.max(to) - from.min(to);
            let translation = (from + to) * 0.5;
            p.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::NONE,
                        custom_size: Some(size),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        translation.x,
                        translation.y,
                        0.0,
                    )),
                    ..Default::default()
                },
                RigidBody::Fixed,
                Collider::cuboid(size.x * 0.5, size.y * 0.5),
            ));
        }

        // p.spawn((
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::NONE,
        //             custom_size: Some(Vec2::new(1080., 60.)),
        //             ..Default::default()
        //         },
        //         transform: {
        //             Transform::from_xyz(
        //                 -((2416. / 2.) - (1080. / 2.)),
        //                 -((288. - (60. / 2.)) - (288. / 2.)),
        //                 0.0,
        //             )
        //         },
        //         ..Default::default()
        //     },
        //     RigidBody::Fixed,
        //     Collider::cuboid(0.5 * 1080., 0.5 * 60.),
        // ));

        // 1162
        // 1539
        // 142.5,
        // p.spawn((
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::NONE,
        //             custom_size: Some(Vec2::new(377., 60.)),
        //             ..Default::default()
        //         },
        //         transform: {
        //             Transform::from_xyz(
        //                 ((377. / 2.) + (1162.)) - (2416. / 2.),
        //                 -((288. - (60. / 2.)) - (288. / 2.)),
        //                 0.0,
        //             )
        //         },
        //         ..Default::default()
        //     },
        //     RigidBody::Fixed,
        //     Collider::cuboid(0.5 * 377., 0.5 * 60.),
        // ));

        // 1633
        // 2100
        // p.spawn((
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::NONE,
        //             custom_size: Some(Vec2::new((2100. - 1633.), 60.)),
        //             ..Default::default()
        //         },
        //         transform: {
        //             Transform::from_xyz(
        //                 (((2100. - 1633.) / 2.) + (1633.)) - (2416. / 2.),
        //                 -((288. - (60. / 2.)) - (288. / 2.)),
        //                 0.0,
        //             )
        //         },
        //         ..Default::default()
        //     },
        //     RigidBody::Fixed,
        //     Collider::cuboid(0.5 * (2100. - 1633.), 0.5 * 60.),
        // ));

        // 2234
        // 228
        // p.spawn((
        //     SpriteBundle {
        //         sprite: Sprite {
        //             color: Color::NONE,
        //             custom_size: Some(Vec2::new((2416. - 2234.), 60.)),
        //             ..Default::default()
        //         },
        //         transform: {
        //             Transform::from_xyz(
        //                 (((2416. - 2234.) / 2.) + (2234.)) - (2416. / 2.),
        //                 -((288. - (60. / 2.)) - (288. / 2.)),
        //                 0.0,
        //             )
        //         },
        //         ..Default::default()
        //     },
        //     RigidBody::Fixed,
        //     Collider::cuboid(0.5 * (2416. - 2234.), 0.5 * 60.),
        // ));
    });
}
