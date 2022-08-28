use bevy::prelude::*;

const MOV_SPEED: f32 = 100.;

#[derive(Component)]
struct Arrow;

#[derive(Component)]
struct Player;

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "Gajelas".to_owned(),
        ..Default::default()
    };

    App::new()
        .insert_resource(window_descriptor)
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(arrow)
        .add_system(spawn_arrow)
        .add_system(player_move)
        .add_system(player_rotate)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("player.png"),
        transform: Transform {
            // scale: Vec3::new(5., 5., 0.),
            translation: Vec3::new(-100., 0., 0.) ,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player);
}

fn spawn_arrow(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>
) {
    let player = player_q.single();
    let (_,_,rotation) = player.rotation.to_euler(EulerRot::XYZ);
    let rotation = rotation + 0.25;
    let x_component = rotation.cos() * 60.;
    let y_component = rotation.sin() * 60.;
    let spawn_pos_adjust = Vec3::new(x_component, y_component, 0.);
    if input.just_pressed(KeyCode::Space) {
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("arrow.png"),
            transform: Transform {
                translation: player.translation.clone() + spawn_pos_adjust,
                rotation: player.rotation.clone(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Arrow);
    }
}

fn arrow(
    mut arrow_q: Query<&mut Transform, With<Arrow>>,
    time: Res<Time>
) {
    for mut arrow in arrow_q.iter_mut() {
        let (_,_,r) = arrow.rotation.to_euler(EulerRot::XYZ);
        arrow.translation.x += r.cos() * 500. * time.delta_seconds();
        arrow.translation.y += r.sin() * 500. * time.delta_seconds();
    }
}

fn player_move(
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    for mut player in player_q.iter_mut() {
        if input.pressed(KeyCode::W) {
            player.translation.y += MOV_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::S) {
            player.translation.y -= MOV_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::A) {
            player.translation.x -= MOV_SPEED * time.delta_seconds();
        }
        if input.pressed(KeyCode::D) {
            player.translation.x += MOV_SPEED * time.delta_seconds();
        }
    }
}

fn player_rotate(
    mut player_q: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player = player_q.single_mut();
    player.rotate_z(2. * time.delta_seconds());
}