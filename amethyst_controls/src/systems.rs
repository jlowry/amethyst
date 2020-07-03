use derive_new::new;
use winit::{DeviceEvent, Event, Window, WindowEvent};

#[cfg(feature = "profiler")]
use thread_profiler::profile_scope;

use amethyst_core::{
    transform::{Translation, Rotation, LocalToWorld},
    ecs::prelude::*,
    dispatcher::{DispatcherBuilder, Stage, SystemBundle},
    math::{convert, Unit, Vector3, Translation3, UnitQuaternion},
    shrev::{EventChannel, ReaderId},
    timing::Time,
};

use amethyst_derive::SystemDesc;
use amethyst_input::{get_input_axis_simple, get_action_simple, BindingTypes, InputHandler};

use crate::{
    components::{ArcBallControl, FlyControl},
    resources::{HideCursor, WindowFocus},
};

/// The system that manages the fly movement.
///
/// # Type parameters
///
/// * `T`: This are the keys the `InputHandler` is using for axes and actions. Often, this is a `StringBindings`.
pub fn build_fly_movement_system<T: BindingTypes>(
    speed: f32, 
    horizontal_axis: Option<T::Axis>, 
    vertical_axis: Option<T::Axis>, 
    longitudinal_axis: Option<T::Axis>,
) -> Box<dyn FnOnce(&mut World, &mut Resources) -> Box<dyn Schedulable>> {
    Box::new(move |_world, _resources| {
        SystemBuilder::<()>::new("FreeMovementSystem")
            .read_resource::<Time>()
            .read_resource::<InputHandler<T>>()
            .with_query(<(Read<FlyControl>, Write<Translation>)>::query())
            .build(move |commands, world, (time, input), controls| {
                #[cfg(feature = "profiler")]
                profile_scope!("free_movement_system");

                let x = get_input_axis_simple(&horizontal_axis, &input);
                let y = get_input_axis_simple(&vertical_axis, &input);
                let z = get_input_axis_simple(&longitudinal_axis, &input);


                if let Some(dir) = Unit::try_new(Vector3::new(x, y, z), convert(1.0e-6)) {
                    for (_, mut trans) in controls.iter_mut(world) {
                        let delta_sec = time.delta_seconds();
                        trans.0 *= Translation3::from(dir.as_ref() * delta_sec * speed); 
                    }
                }
            })
    })
}

/// The system that manages the arc ball movement;
/// In essence, the system will align the camera with its target while keeping the distance to it
/// and while keeping the orientation of the camera.
///
/// To modify the orientation of the camera in accordance with the mouse input, please use the
/// `FreeRotationSystem`.
pub fn build_arc_ball_rotation_system(_world: &mut World, _res: &mut Resources) -> Box<dyn Schedulable> {
    SystemBuilder::<()>::new("ArcBallRotationSystem")
        .with_query(<(Read<ArcBallControl>, Read<Rotation>, Write<Translation>)>::query())
        .build(move |commands, world, (), controls| {
            #[cfg(feature = "profiler")]
            profile_scope!("arc_ball_rotation_system");  

            let (mut query_world, mut world) = world.split_for_query(&controls);

            for (control, rot, mut trans) in controls.iter_mut(&mut query_world) {
                let pos_vec = rot.0 * (-Vector3::z() * control.distance);
                if let Some(target_trans) = world.get_component::<Translation>(control.target) {
                    let new_pos = (target_trans.vector - pos_vec);
                    *trans = Translation(Translation3::from(new_pos));
                }
            }
        })
}

/// The system that manages the view rotation.
///
/// Controlled by the mouse.
/// Goes into an inactive state if the window is not focused (`WindowFocus` resource).
///
/// Can be manually disabled by making the mouse visible using the `HideCursor` resource:
/// `HideCursor.hide = false`
pub fn build_free_rotation_system(
    sensitivity_x: f32, 
    sensitivity_y: f32, 
) -> Box<dyn FnOnce(&mut World, &mut Resources) -> Box<dyn Schedulable>> {
    Box::new(move |_world, resources| {
        let mut reader = resources
            .get_mut::<EventChannel<Event>>()
            .unwrap()
            .register_reader();

        SystemBuilder::<()>::new("FreeRotationSystem")
            .read_resource::<EventChannel<Event>>()
            .read_resource::<WindowFocus>()
            .read_resource::<HideCursor>()
            .with_query(<(Read<FlyControl>, Write<Rotation>)>::query())
            .build(move |commands, world, (events, focus, hide), controls| {
                #[cfg(feature = "profiler")]
                profile_scope!("free_rotation_system");

                let focused = focus.is_focused;
                for event in events.read(&mut reader) {
                    if focused && hide.hide {
                        if let Event::DeviceEvent { ref event, .. } = *event {
                            if let DeviceEvent::MouseMotion { delta: (x, y) } = *event {
                                for (_, mut rotation) in controls.iter_mut(world) {
                                    rotation.0 *= UnitQuaternion::from_euler_angles(
                                        (-(y as f32) * sensitivity_y).to_radians(),
                                        (-(x as f32) * sensitivity_x).to_radians(),
                                        0.0, 
                                    );
                                }
                            }
                        }
                    }
                }
            })
        })
}

pub fn build_mouse_focus_update_system(_world: &mut World, resources: &mut Resources) -> Box<dyn Schedulable> {
    resources.insert(WindowFocus::new());

    let mut reader = resources
        .get_mut::<EventChannel<Event>>()
        .unwrap()
        .register_reader();

    SystemBuilder::<()>::new("MouseFocusUpdateSystem")
        .read_resource::<EventChannel<Event>>()
        .write_resource::<WindowFocus>()
        .build(move |commands, world, (events, focus), ()| {
            #[cfg(feature = "profiler")]
            profile_scope!("mouse_focus_update_system");

            for event in events.read(&mut reader) {
                if let Event::WindowEvent { ref event, .. } = *event {
                    if let WindowEvent::Focused(focused) = *event {
                        focus.is_focused = focused;
                    }
                }
            }
        })
}


/// System which hides the cursor when the window is focused.
/// Requires the usage MouseFocusUpdateSystem at the same time.
pub fn build_cursor_hide_system(
    _world: &mut World, 
    resources: &mut Resources,
) -> Box<dyn Schedulable> {
    let mut is_hidden = true;

    resources.insert(HideCursor::default());

    SystemBuilder::<()>::new("CursorHideSystem")
        .read_resource::<HideCursor>()
        .read_resource::<WindowFocus>()
        .read_resource::<Window>()
        .build(move |commands, world, (hide, focus, window), ()| {
            #[cfg(feature = "profiler")]
            profile_scope!("cursor_hide_system");

            let should_be_hidden = focus.is_focused && hide.hide;
            if !is_hidden && should_be_hidden {
                if let Err(err) = window.grab_cursor(true) {
                    log::error!("Unable to grab the cursor. Error: {:?}", err);
                }
                window.hide_cursor(true);
                is_hidden = true;
            } else if is_hidden && !should_be_hidden {
                if let Err(err) = window.grab_cursor(false) {
                    log::error!("Unable to release the cursor. Error: {:?}", err);
                }
                window.hide_cursor(false);
                is_hidden = false;
            }
        })
}