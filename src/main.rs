mod level;
mod block;

use std::collections::HashMap;
use bevy::app::{App, AppLabel};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::DefaultPlugins;
use bevy::input::{Input};
use bevy::math::Vec3;
use bevy::prelude::{Camera2dBundle, Commands, Image, Res, default, Bundle, Component, KeyCode, Query, Visibility, Transform, With, Entity, Vec2, ResMut, Deref, DerefMut, Window, ClearColor, Color};
use bevy::render::render_resource::AsBindGroupShaderType;
use bevy::render::texture::ImageSettings;
use bevy::sprite::{Rect, SpriteBundle, SpriteSheetBundle, TextureAtlas, TextureAtlasBuilder, TextureAtlasSprite};
use bevy::time::{Time, Timer};
use bevy::window::{MonitorSelection, WindowDescriptor, WindowMode, WindowPosition, Windows};
use crate::block::{BlockAtlas, BlockTexture, BlockTexturesIndex};
use crate::Direction::LEFT;

#[derive(Component)]
struct MarioSize {
    big: bool,
}

#[derive(Component)]
struct Player;

#[derive(Component, PartialEq)]
enum Direction {
    LEFT,
    RIGHT
}

#[derive(Bundle)]
struct Mario {
    size: MarioSize,
    direction: Direction
    // textures: MarioTextures,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct MarioTexturesIndex {
    standing: usize,
    moving_one: usize,
    moving_two: usize,
    moving_three: usize,
}

/*#[derive(Component)]
struct MarioTextures {
    #[bundle]
    standing: SpriteBundle,
    #[bundle]
    moving: SpriteBundle
}*/

struct MarioStanding(SpriteBundle);

struct MarioMoving(Handle<Image>);

/*#[derive(Component)]
struct MarioStanding(SpriteBundle);*/

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Super mario bros".to_string(),
            position: WindowPosition::Centered(MonitorSelection::Current),
            width: 1524.,
            height: 412.,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        // .insert_resource(ClearColor(Color::rgb(31.6, 58., 98.8)))
        .insert_resource(ClearColor(Color::rgb(0.316, 0.58, 0.988)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(load_textures)
        .add_startup_system(init)
        .add_system(bevy::window::close_on_esc)
        .add_system(mario_moving)
        .add_system(test)
        .run();
}

fn test() {}

fn mario_moving(keys: Res<Input<KeyCode>>,
                time: Res<Time>,
                mut query: Query<(&mut Transform, &mut TextureAtlasSprite, &mut AnimationTimer, &mut Direction), With<Player>>,
                mario_textures_index: Res<MarioTexturesIndex>
) {
    for (mut transform, mut atlas_sprite, mut anim_timer, mut direction) in query.iter_mut() {
        if keys.pressed(KeyCode::Right) {
            *direction = Direction::RIGHT;
            transform.translation.x += 10.0;
            atlas_sprite.flip_x = false;
            anim_timer.tick(time.delta());
            if anim_timer.just_finished() {
                if atlas_sprite.index == mario_textures_index.moving_one {
                    atlas_sprite.index = mario_textures_index.moving_two;
                } else if atlas_sprite.index == mario_textures_index.moving_two {
                    atlas_sprite.index = mario_textures_index.moving_three;
                } else {
                    atlas_sprite.index = mario_textures_index.moving_one;
                }
            }
        } else if keys.pressed(KeyCode::Left) {
            *direction = LEFT;
            transform.translation.x -= 10.0;
            atlas_sprite.flip_x = true;
            anim_timer.tick(time.delta());
            if anim_timer.just_finished() {
                if atlas_sprite.index == mario_textures_index.moving_one {
                    atlas_sprite.index = mario_textures_index.moving_two;
                } else if atlas_sprite.index == mario_textures_index.moving_two {
                    atlas_sprite.index = mario_textures_index.moving_three;
                } else {
                    atlas_sprite.index = mario_textures_index.moving_one;
                }
            }
        } else {
            atlas_sprite.flip_x = *direction == LEFT;
            atlas_sprite.index = mario_textures_index.standing;
        }
    }
}

fn init(mut commands: Commands/*, block_atlas: Res<BlockAtlas>, block_texture: Res<BlockTexture>*/) {
    commands.spawn_bundle(Camera2dBundle::default());
    /*commands.spawn_bundle(SpriteBundle {
        texture: mario_standing.0,
        ..default()
    });*/
}

fn load_textures(mut commands: Commands, server: Res<AssetServer>,
                 mut texture_atlases: ResMut<Assets<TextureAtlas>>,
                 windows: Res<Windows>) {
    // let handle_stand: Handle<Image> = server.load("images/mario/standing.png");
    // let handle_move: Handle<Image> = server.load("images/mario/moving.png");
    // commands.insert_resource(MarioMoving(handle));
    // commands.insert_resource(MarioStanding(handle));
    // let mario_handle: Handle<Image> = server.load("images/mario/mario.png");
    // let mario_atlas = TextureAtlas::new_empty(mario_handle, Vec2 { x: 584.0, y: 436.0 });
    let handle_backgrounds: Handle<Image> = server.load("images/background_overworld.png");
    commands.spawn_bundle(SpriteBundle {
        texture: handle_backgrounds,
        transform: Transform {
            translation: Vec3::new(0., 31., 0.),
            scale: Vec3 {x: 2., y: 2., z: 1.},
            ..default()
        },
        ..default()
    });

    let handle_mario: Handle<Image> = server.load("images/mario/mario.png");
    let mut atlas = TextureAtlas::new_empty(handle_mario, Vec2 { x: 584., y: 436. });
    let mario_standing_index = atlas.add_texture(Rect { min: Vec2 { x: 0., y: 8. }, max: Vec2 { x: 15., y: 24. } });
    let mario_moving_one_index = atlas.add_texture(Rect { min: Vec2 { x: 20., y: 8. }, max: Vec2 { x: 35., y: 24. } });
    let mario_moving_two_index = atlas.add_texture(Rect { min: Vec2 { x: 38., y: 8. }, max: Vec2 { x: 53., y: 24. } });
    let mario_moving_three_index = atlas.add_texture(Rect { min: Vec2 { x: 56., y: 8. }, max: Vec2 { x: 71., y: 24. } });
    let handle_mario_atlas = texture_atlases.add(atlas);
    commands.insert_resource(MarioTexturesIndex {
        standing: mario_standing_index,
        moving_one: mario_moving_one_index,
        moving_two: mario_moving_two_index,
        moving_three: mario_moving_three_index,
    });

    let handle_blocks: Handle<Image> = server.load("images/items_bricks.png");
    let mut atlas = TextureAtlas::new_empty(handle_blocks, Vec2 { x: 448., y: 256. });
    let brick_index = atlas.add_texture(Rect { min: Vec2 { x: 272., y: 112. }, max: Vec2 { x: 288., y: 128. } });
    let handle_blocks_atlas = texture_atlases.add(atlas);

    commands.insert_resource(BlockTexturesIndex {
        brick: brick_index
    });
    let mut blocks_textures = BlockTexture {textures: HashMap::new()};
    blocks_textures.textures.insert(0, brick_index);

    // commands.insert_resource(atlas);
    commands.spawn_bundle(Mario {
        size: MarioSize { big: false },
        direction: Direction::RIGHT
        /*textures: MarioTextures {
            standing: SpriteBundle {
                texture: handle_stand,
                ..default()
            },
            moving: SpriteBundle {
                texture: handle_move,
                visibility: Visibility { is_visible: false },
                ..default()
            }
        }*/
    }).insert_bundle(SpriteSheetBundle {
        transform: Transform {
            // scale: Vec3::new(5., 5., 0.),
            ..default()
        },
        sprite: TextureAtlasSprite::new(mario_standing_index),
        texture_atlas: handle_mario_atlas,
        ..default()
    })
        /*.insert_bundle(
        SpriteBundle {
            texture: handle_stand,
            ..default()
        }
    )*/
        /*.insert_bundle(SpriteBundle {
        texture: handle_stand,
        ..default()
    }).insert_bundle(SpriteBundle {
        texture: handle_move,
        visibility: Visibility { is_visible: false },
        ..default()
    })*/.insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

    let window = match windows.get_primary() {
        None => {panic!("no primary")}
        Some(w) => w
    };
    let x_base = -1 * (window.width() as i32 / 2);
    let y_base = -1 * (window.height() as i32 / 2);

    let level_one = level::loader::load("assets/levels/level1.txt").unwrap();
    for block in level_one.blocks {
        let index = blocks_textures.textures.get(&block.id);
        let index = match index {
            Some(i) => i,
            None => continue
        };
        commands.spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new((x_base + block.x as i32 * 32 + 16) as f32, (y_base + block.y as i32 * 32 + 13)  as f32, 0.),
                scale: Vec3::new(2., 2., 1.),
                ..default()
            },
            sprite: TextureAtlasSprite::new(*index),
            texture_atlas: handle_blocks_atlas.clone_weak(),
            ..default()
        });
    }

    commands.insert_resource(BlockAtlas {handle: handle_blocks_atlas});
    commands.insert_resource(blocks_textures);
}
