To create a sprite animation in Bevy, you can use the bevy_sprite and bevy_animation crates. Here's an example of how you can create a simple sprite animation:

```rust
use bevy::{
    prelude::*,
    sprite::{ColorMaterial, Sprite, SpriteSheet, TextureAtlas},
    animation::{
        Animation, AnimationCommand, AnimationControl, AnimationId, AnimationSet, AnimationSetBuilder,
        AnimationTrack,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(animate_sprite.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load sprite sheet
    let texture_handle = load_texture(&mut commands, "path/to/sprite_sheet.png");
    let sprite_sheet = load_sprite_sheet(&mut commands, &texture_atlases, texture_handle);

    // Create a sprite entity
    let sprite_entity = commands
        .spawn()
        .insert(Sprite::new(sprite_sheet.texture_atlas.clone()))
        .insert(sprite_sheet)
        .with_children(|parent| {
            parent.spawn().insert(Sprite::from(sprite_sheet.get_sprite("frame_1").unwrap()));
        })
        .id();

    // Create animation set
    let animation_set = AnimationSetBuilder::new()
        .with_animation(
            "idle",
            Animation {
                tracks: vec![AnimationTrack {
                    keyframes: vec![(0.0, 0), (1.0, 1)],
                    duration: 1.0,
                    loop_mode: bevy::animation::LoopMode::Repeat,
                }],
            },
        )
        .build();

    // Add animation control component to the sprite entity
    commands.insert_resource(AnimationSet { set: animation_set });
    commands.insert(sprite_entity, AnimationControl {
        animation_id: AnimationId::new("idle"),
        time: 0.0,
    });
}

fn animate_sprite(
    time: Res<Time>,
    mut animation_controls: Query<(&AnimationControl, &AnimationSet)>,
    mut sprites: Query<&mut Sprite>,
) {
    for (animation_control, animation_set) in animation_controls.iter() {
        let animation = &animation_set.set.get(&animation_control.animation_id).unwrap();

        animation_control.time += time.delta_seconds();

        let frame_index = animation.tracks[0].keyframes.iter().position(|(t, _)| *t > animation_control.time).unwrap_or(animation.tracks[0].keyframes.len() - 1);
        let frame = animation.tracks[0].keyframes[frame_index].1;

        if let Ok(sprite) = sprites.get_mut(animation_control.entity) {
            sprite.texture_atlas = Some(animation_set.set.texture_atlases.clone());
            sprite.texture_id = Some(animation.tracks[0].keyframes[frame].1);
        }
    }
}

fn load_texture(commands: &mut Commands, path: &str) -> Handle<Texture> {
    commands.load(path)
}

fn load_sprite_sheet(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    texture_handle: Handle<Texture>,
) -> SpriteSheet {
    texture_atlases.add(texture_handle).unwrap()
}
```

In this example, we load a sprite sheet texture and create a sprite entity with a child sprite. We then create an animation set with an "idle" animation and add an AnimationControl component to the sprite entity. In the animate_sprite system, we update the animation time and set the sprite's texture to the appropriate frame based on the animation.

Please note that you need to have the bevy_sprite and bevy_animation crates in your project's dependencies in the Cargo.toml file:

```toml
[dependencies]
bevy = "0.8"
bevy_sprite = "0.8"
bevy_animation = "0.8"
```

Also, make sure to replace "path/to/sprite_sheet.png" with the actual path to your sprite sheet texture.
