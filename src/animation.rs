use crate::from_component::{FromComponentLabel, FromComponentPlugin};
use std::{marker::PhantomData, ops::Range};

use bevy::prelude::*;

#[derive(SystemSet, Clone, PartialEq, Eq, Debug, Hash)]
pub struct AnimationLabel;

#[derive(Clone, Debug, Default, Component)]
pub struct SpriteSheetAnimation {
    pub indices: Range<usize>,
    pub frame_timer: Timer,
    pub repeat: bool,
}

pub fn sprite_sheet_animation(
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut SpriteSheetAnimation)>,
    time: Res<Time>,
    mut event_writer: EventWriter<AnimationEvent>,
) {
    for (entity, mut sprite, mut sprite_sheet_animation) in query.iter_mut() {
        sprite_sheet_animation.frame_timer.tick(time.delta());

        if sprite_sheet_animation.frame_timer.just_finished() {
            sprite.index += 1;
            if sprite.index >= sprite_sheet_animation.indices.end {
                // Animation finished
                if sprite_sheet_animation.repeat {
                    sprite.index = sprite_sheet_animation.indices.start;
                } else {
                    sprite.index = sprite_sheet_animation.indices.end - 1;
                    event_writer.send(AnimationEvent::Finished(entity));
                }
            }
        }
    }
}

pub fn set_initial_sprite_index(
    mut query: Query<
        (&mut TextureAtlasSprite, &SpriteSheetAnimation),
        Changed<SpriteSheetAnimation>,
    >,
) {
    for (mut sprite, sprite_sheet_animation) in query.iter_mut() {
        let indices = &sprite_sheet_animation.indices;
        if sprite.index < indices.start || sprite.index > indices.end {
            sprite.index = indices.start;
        }
    }
}

pub struct SpriteSheetAnimationPlugin;

impl Plugin for SpriteSheetAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationEvent>()
            .add_systems(
                Update,
                sprite_sheet_animation
                    .in_set(AnimationLabel)
                    .after(FromComponentLabel),
            )
            .add_systems(
                Update,
                set_initial_sprite_index
                    .in_set(AnimationLabel)
                    .after(FromComponentLabel),
            );
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Event)]
pub enum AnimationEvent {
    Finished(Entity),
}

pub struct FromComponentAnimator<F>
where
    F: Into<SpriteSheetAnimation> + Component + 'static + Send + Sync + Clone + Iterator<Item = F>,
{
    from_type: PhantomData<F>,
}

impl<F> Plugin for FromComponentAnimator<F>
where
    F: Into<SpriteSheetAnimation> + Component + 'static + Send + Sync + Clone + Iterator<Item = F>,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(FromComponentPlugin::<F, SpriteSheetAnimation>::new())
            .add_systems(Update, animation_finisher::<F>.before(AnimationLabel));
    }
}

fn animation_finisher<F>(
    mut query: Query<(&mut F, &mut TextureAtlasSprite, &mut SpriteSheetAnimation)>,
    mut event_reader: EventReader<AnimationEvent>,
) where
    F: Into<SpriteSheetAnimation> + Component + 'static + Send + Sync + Clone + Iterator<Item = F>,
{
    for event in event_reader.read() {
        match event {
            AnimationEvent::Finished(entity) => {
                if let Ok((mut from, mut sprite, mut sprite_sheet_animation)) =
                    query.get_mut(*entity)
                {
                    *from = from.next().unwrap();
                    *sprite_sheet_animation = from.clone().into();
                    sprite.index = sprite_sheet_animation.indices.start;
                }
            }
        }
    }
}
