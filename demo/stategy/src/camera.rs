
use bevy::{
    prelude::*,
    input::mouse::{
        MouseButton,
        MouseMotion,
        MouseWheel,
    },
};

pub struct LookAtTarget {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub camera_entity: Entity,
}
impl LookAtTarget {
    pub fn new(camera_entity: Entity) -> Self {
        LookAtTarget {
            yaw: 0.0,
            distance: 40.0,
            pitch: 30.0f32.to_radians(),
            camera_entity,
        }
    }
}

impl Default for LookAtTarget {
    fn default() -> Self {
        LookAtTarget {
            yaw: 0.,
            distance: 20.,
            pitch: 30.0f32.to_radians(),
            camera_entity: Entity::new(),
        }
    }
}


#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

fn process_mouse_events(
    time: Res<Time>,
    mut state: ResMut<State>, 
    mouse_input_button: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query: Query<&mut LookAtTarget>,
) {
    let mut look = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        look = event.delta;
    }

    let mut zoom_delta = 0.;
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        zoom_delta = event.y;
    }

    let zoom_sense = 20.0;
    let look_sense = 1.0;

    for mut target in &mut query.iter() {
        if mouse_input_button.pressed(MouseButton::Middle) {
            target.yaw += look.x() * time.delta_seconds;
            target.pitch -= look.y() * time.delta_seconds * look_sense;
        }
        target.distance -= zoom_delta * time.delta_seconds * zoom_sense;
    }
}

fn update_target(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut target_query: Query<(&mut LookAtTarget, &mut Translation, &Transform, &mut Rotation)>,
    camera_query: Query<(&mut Translation, &mut Rotation)>,
) {
    let mut movement = Vec2::zero();
    if keyboard_input.pressed(KeyCode::W) { *movement.y_mut() += 1.; }
    if keyboard_input.pressed(KeyCode::S) { *movement.y_mut() -= 1.; }
    if keyboard_input.pressed(KeyCode::D) { *movement.x_mut() += 1.; }
    if keyboard_input.pressed(KeyCode::A) { *movement.x_mut() -= 1.; }

    if movement != Vec2::zero() { movement.normalize(); }

    let move_speed = 10.0;
    movement *= time.delta_seconds * move_speed;

    for (mut target, mut translation, transform, mut rotation) in &mut target_query.iter() {
        target.pitch = target.pitch.max(1f32.to_radians()).min(179f32.to_radians());
        target.distance = target.distance.max(5.).min(60.);

        let fwd = -transform.value.z_axis().truncate() * movement.y();
        let right = transform.value.x_axis().truncate() * movement.x();

        translation.0 += Vec3::from(fwd + right);
        rotation.0 = Quat::from_rotation_y(-target.yaw);

        let cam_pos = Vec3::new(0., target.pitch.cos(), target.pitch.sin()).normalize() * target.distance;
        if let Ok(mut cam_trans) = camera_query.get_mut::<Translation>(target.camera_entity) {
            cam_trans.0 = cam_pos;
        }

        if let Ok(mut camera_rotation) = camera_query.get_mut::<Rotation>(target.camera_entity) {
            let look = Mat4::face_toward(cam_pos, Vec3::zero(), Vec3::new(0.0, 1.0, 0.0));
            camera_rotation.0 = look.to_scale_rotation_translation().1;
        }
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<State>()
            .add_system(process_mouse_events.system())
            .add_system(update_target.system())
        ;
    }
}

