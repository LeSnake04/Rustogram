use std::sync::atomic::Ordering;
use std::thread::sleep;
use std::time::Duration;
use bevy::app::Events;
use bevy::asset::{Assets, AssetServer};
use bevy::DefaultPlugins;
use bevy::math::{Vec2};
use bevy::prelude::{App, Color, Commands, IntoExclusiveSystem, OrthographicCameraBundle, Res, ResMut, SpriteBundle, Time, Transform, UiCameraBundle, Windows, IntoSystem};
use bevy::sprite::{ColorMaterial, Sprite};
use bevy::window::{WindowDescriptor, WindowResized};
use atomic_float::AtomicF32;
use bevy::log::debug;

static WINDOW_WIDTH: AtomicF32 = AtomicF32::new(600.0);
static WINDOW_HEIGHT: AtomicF32 = AtomicF32::new(600.0);

#[macro_export]
macro_rules! get_window_width {
    () => { WINDOW_WIDTH.load(Ordering::Relaxed)};
}
macro_rules! get_window_height {
    () => { WINDOW_HEIGHT.load(Ordering::Relaxed)};
}

fn main() {
	println!("Hello, world!");
	App::build()
		.insert_resource( WindowDescriptor{
			title: "Rustogram".to_string(),
			width: get_window_width!(),
			height: WINDOW_HEIGHT.load(Ordering::Relaxed),
			vsync: true,
			..Default::default()
		} )
		.add_plugins(DefaultPlugins)
		.add_system(detect_resize.system())
		//.add_startup_system(setup)
		.run();
}

/*fn setup(
	mut commands: Commands,
	mut materials: ResMut<Assets<ColorMaterial>>,
	asset_server: Res<AssetServer>
){
	//cameras
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands.spawn_bundle(UiCameraBundle::default());

	//Background
	commands.spawn_bundle(SpriteBundle{
		sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT),
		material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
		transform: Transform::from_xyz(WINDOW_WIDTH,WINDOW_HEIGHT,0.0),
		..Default::default()
	});
}*/

/// This system detects window changes
fn detect_resize(
	resize_event: Res<Events<WindowResized>>
) {
	let mut reader = resize_event.get_reader();
	let mut updated: bool = false;
	debug!(updated);
	for e in reader.iter(&resize_event) {
		if e.width != get_window_width!() {
			WINDOW_WIDTH.store(e.width, Ordering::Relaxed);
			updated=true;
		};
		if e.height != get_window_height!() {
			WINDOW_HEIGHT.store(e.height, Ordering::Relaxed);
			updated=true
		};
		sleep(Duration::from_millis(15));
	}
	if updated {println!("Resized to x:{} y:{}", get_window_width!(), get_window_height!())}
}

/// This system will then change the title during execution
fn timer_title(time: Res<Time>, mut windows: ResMut<Windows>) {
	let window = windows.get_primary_mut().unwrap();
	window.set_title(format!(
		"Rustogram Game: {}",
		time.seconds_since_startup().round()
	));
}