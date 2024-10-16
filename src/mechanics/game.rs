use bevy::prelude::*;
pub mod terraingen;

//Player Marker Component
#[derive(Component)]
pub struct PlayerMarker {
}

//Entity Movement Component
#[derive(Component)]
pub struct Movement {
    velocity: Vec3,
    verticalspeed: f32,
    horizontalspeed: f32,
    maxverticalspeed: f32,
    maxhorizontalspeed: f32
}

//Playercomposed of Entity Sprite -> Composition over Inheritance
#[derive(Bundle)]
pub struct PlayerSprite {
    pub spritebundle: SpriteBundle,
    pub playermarker: PlayerMarker,
    pub movement: Movement,

}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame
fn playermovement(mut playertransform: Query<(&mut Movement, &mut Transform), With<PlayerMarker>>, keyboardinput: Res<ButtonInput<KeyCode>>) {
    //moving sprite
    for (mut movement, mut playertransform) in playertransform.iter_mut() {
        // Horizontal movement
        if keyboardinput.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
            movement.velocity.x += movement.horizontalspeed;  // Move right
        }
        if keyboardinput.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
            movement.velocity.x -= movement.horizontalspeed;  // Move left
        }

        // Vertical movement
        if keyboardinput.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) {
            movement.velocity.y += movement.verticalspeed;  // Move up
        }
        if keyboardinput.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) {
            movement.velocity.y -= movement.verticalspeed;  // Move down
        }

        // Apply friction to the movement
        movement.velocity *= 0.7;
        movement.velocity.x = movement.velocity.x.clamp(-movement.maxhorizontalspeed, movement.maxhorizontalspeed);
        movement.velocity.y = movement.velocity.y.clamp(-movement.maxverticalspeed, movement.maxverticalspeed);
       
        // Update the transform translation based on velocity
        playertransform.translation += movement.velocity;


    }
}

fn cameramovement(mut playertransform: Query<(&mut Movement, &mut Transform), With<PlayerMarker>>, mut cameratransform: Query<&mut Transform, With<Camera2d>>) {
    //moving camera
    // Check if there's a player entity
    for (_movement, playertransform) in playertransform.iter_mut(){
        // Get the current camera transform
        for mut camtransform in cameratransform.iter_mut() {
            // Lerp factor for smooth movement
            let lerpfactor = 0.1; // Adjust this value for speed of the camera movement

            // Interpolate the camera's position towards the player's position
            camtransform.translation.x = camtransform.translation.x + (playertransform.translation.x - camtransform.translation.x) * lerpfactor;
            camtransform.translation.y = camtransform.translation.y + (playertransform.translation.y - camtransform.translation.y) * lerpfactor;
        }
    }
 }

    


//setup game mechanic wise
pub fn game_setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playertransform: Query<(&PlayerMarker, &mut Transform)>
) {

    //create player sprite
    commands.spawn(PlayerSprite {
                    spritebundle: SpriteBundle {
                        texture: asset_server.load("sprites/player/player.png"),
                        transform: Transform {
                            translation: Vec3::new(0.0,0.0,1.0),
                            scale: Vec3::splat(1.0),
                            ..default()
                        },
                        ..default()
                    },
                   playermarker: PlayerMarker {},
                   movement: Movement {
                    velocity: Vec3::new(0.0, 0.0, 0.0),
                    verticalspeed: 1.0,
                    horizontalspeed: 1.0,
                    maxhorizontalspeed: 3.0,
                    maxverticalspeed: 3.0,
                   }
                }
            );

    terraingen::generatechunk(commands, asset_server, playertransform);   
}

//Fix borrowing, ownership issues
pub fn game_core_plugin (mut playertransform: Query<(&mut Movement, &mut Transform),With<PlayerMarker>>, mut cameratransform: Query<&mut Transform, With<Camera2d>>, keyboardinput: Res<ButtonInput<KeyCode>>) {
    //In gameloop
    playermovement(playertransform, keyboardinput);
    cameramovement(playertransform, cameratransform);
}