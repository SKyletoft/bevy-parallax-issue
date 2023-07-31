use bevy::prelude::*;

#[derive(Component)]
pub struct Ground;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, setup)
		.add_systems(Update, update)
		.run();
}

pub fn update(mut query: Query<&mut Transform, With<Ground>>, d_time: Res<Time>) {
	for mut t in query.iter_mut() {
		t.translation.z = d_time.elapsed_seconds().sin();
		t.rotation.y = (d_time.elapsed_seconds() / 1.2).sin() * 0.4;
	}
}

pub fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let mesh = asset_server.load("SimpleGround.gltf#Mesh0/Primitive0");
	let depth = asset_server.load("EasyGroundDepth.png");
	let albedo = asset_server.load("EasyGroundAlbedo.png");
	let normal = asset_server.load("EasyGroundNormal.png");

	commands.spawn((
		PbrBundle {
			mesh,
			material: materials.add(StandardMaterial {
				base_color_texture: Some(albedo),
				normal_map_texture: Some(normal),
				depth_map: Some(depth),
				perceptual_roughness: 0.4,
				parallax_depth_scale: 0.09,
				parallax_mapping_method: ParallaxMappingMethod::Relief { max_steps: 4 },
				max_parallax_layer_count: 25.0,
				..Default::default()
			}),
			..Default::default()
		},
		Ground,
	));

	commands.spawn(DirectionalLightBundle {
		directional_light: DirectionalLight {
			shadows_enabled: true,
			illuminance: 25_000.0,
			..Default::default()
		},
		transform: Transform::from_xyz(20.0, 14.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
		..Default::default()
	});

	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(-30.0, 25.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
		..default()
	});
}
