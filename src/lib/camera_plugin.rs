use bevy::{
    app::{Plugin, Startup, Update},
    camera::{Camera, Camera2d, Projection},
    ecs::{
        message::MessageReader,
        query::With,
        schedule::{IntoScheduleConfigs, common_conditions::on_message},
        system::Single,
    },
    input::mouse::MouseWheel,
    scene::{Scene, SpawnSystem, bsn},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, scene.spawn());

        app.add_systems(Update, zoom_camera.run_if(on_message::<MouseWheel>));
    }
}

fn scene() -> impl Scene {
    bsn! {
        Camera2d
    }
}

fn zoom_camera(
    mut mouse_wheel_reader: MessageReader<MouseWheel>,
    mut projection: Single<&mut Projection, With<Camera>>,
) {
    let total: f32 = mouse_wheel_reader.read().map(|event| -event.y).sum();

    match **projection {
        Projection::Orthographic(ref mut orthographic_projection) => {
            let clamped = (orthographic_projection.scale + (total / 10.0)).clamp(0.1, 3.0);

            orthographic_projection.scale = clamped;
        }
        _ => {}
    };
}
