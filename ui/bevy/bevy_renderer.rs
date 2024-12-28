use transmission::renderer::Renderer;
use crate::tool::{Tool, update_tool};
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct BevyRenderer {
    app: App
}

impl BevyRenderer {
    pub fn new() -> Box<dyn Renderer> {
        Box::new(BevyRenderer {
            app: App::new()
        })
    }
}

impl Renderer for BevyRenderer {
    fn initialize(&mut self) {
        self.app
            .add_plugins(DefaultPlugins)
            .add_plugins(PanCamPlugin)
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_tool);
    }

    fn start(&mut self) {
        self.app.run();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //tool
    commands.insert_resource(Tool::default());
    // cubes
    for x in -10..10 {
        for y in -10..10 {
            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.5, 1.0, 0.5))),
                MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
                Transform::from_xyz(x as f32, y as f32, 0.5),
            ));
        }
    }
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // camera
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        PanCam::default(),
    ));
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}