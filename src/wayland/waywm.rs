use crate::wayland::{
    data,
    element::{PointerElement, PointerRenderElement},
    input::Action,
    state,
    workspace::Workspaces,
};
use smithay::{
    backend::{
        input::{
            AbsolutePositionEvent, ButtonState, Event, InputEvent, KeyState, KeyboardKeyEvent,
            PointerButtonEvent,
        },
        renderer::{
            damage::OutputDamageTracker,
            element::AsRenderElements,
            gles::{GlesRenderer, GlesTexture},
        },
        winit::{self, WinitEvent},
    },
    desktop::{space::render_output, Space, Window, WindowSurfaceType},
    input::{
        keyboard::{keysyms, FilterResult},
        pointer::{ButtonEvent, CursorImageStatus, MotionEvent},
        Seat, SeatState,
    },
    output,
    reexports::{
        calloop::{
            generic::Generic,
            timer::{TimeoutAction, Timer},
            EventLoop, Interest, Mode, PostAction,
        },
        wayland_server::{protocol::wl_surface::WlSurface, Display},
    },
    utils::{Clock, Scale, Transform, SERIAL_COUNTER},
    wayland::{
        compositor::CompositorState, data_device::DataDeviceState, output::OutputManagerState,
        shell::xdg::XdgShellState, shm::ShmState, socket::ListeningSocketSource,
    },
};
use std::{convert::TryInto, os::unix::prelude::AsRawFd, sync::Arc, time::Duration};

pub fn wayrun() -> anyhow::Result<(), anyhow::Error> {
    let mut event_loop: EventLoop<data::Data> = EventLoop::try_new()?;

    let mut display: Display<state::State> = Display::new()?;

    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();

    event_loop
        .handle()
        .insert_source(socket, |stream, _, data| {
            data.display
                .handle()
                .insert_client(stream, Arc::new(data::ClientData::default()))
                .unwrap();
        })?;

    event_loop.handle().insert_source(
        Generic::new(
            display.backend().poll_fd().as_raw_fd(),
            Interest::READ,
            Mode::Level,
        ),
        |_, _, data| {
            data.display.dispatch_clients(&mut data.state).unwrap();

            Ok(PostAction::Continue)
        },
    )?;

    let dh = display.handle();

    let clock = Clock::new().unwrap();

    let compositor_state = CompositorState::new::<state::State>(&dh);
    let shm_state = ShmState::new::<state::State>(&dh, vec![]);
    let output_manager_state = OutputManagerState::new_with_xdg_output::<state::State>(&dh);
    let xdg_shell_state = XdgShellState::new::<state::State>(&dh);
    let mut seat_state = SeatState::<state::State>::new();
    let space = Space::<Window>::default();
    let data_device_state = DataDeviceState::new::<state::State>(&dh);

    let mut seat: Seat<state::State> = seat_state.new_wl_seat(&dh, "mwm_seat");
    seat.add_keyboard(Default::default(), 500, 500)?;
    seat.add_pointer();

    let state = state::State {
        clock,
        compositor_state,
        data_device_state,
        seat_state,
        cursor_status: CursorImageStatus::Default,
        pointer_location: (0.0, 0.0).into(),
        shm_state,
        space,
        output_manager_state,
        xdg_shell_state,
        workspaces: Workspaces::new(),
    };

    let mut data = data::Data { state, display };

    let (mut backend, mut winit) = winit::init::<GlesRenderer>().unwrap();

    let size = backend.window_size().physical_size;

    let mode = output::Mode {
        size,
        refresh: 60_000,
    };

    let physical_properties = output::PhysicalProperties {
        size: (0, 0).into(),
        subpixel: output::Subpixel::Unknown,
        make: "sswm".into(),
        model: "Winit".into(),
    };

    let output = output::Output::new("winit".to_string(), physical_properties);
    output.create_global::<state::State>(&data.display.handle());
    output.change_current_state(
        Some(mode),
        Some(Transform::Flipped180),
        None,
        Some((0, 0).into()),
    );
    output.set_preferred(mode);
    data.state.space.map_output(&output, (0, 0));

    std::env::set_var("WAYLAND_DISPLAY", socket_name);

    let start_time = std::time::Instant::now();
    let timer = Timer::immediate();

    let mut output_damage_tracker = OutputDamageTracker::from_output(&output);

    let mut pointer_element = PointerElement::<GlesTexture>::new(backend.renderer());

    event_loop
        .handle()
        .insert_source(timer, move |_, _, data| {
            let display = &mut data.display;
            let state = &mut data.state;

            winit
                .dispatch_new_events(|event| {
                    if let WinitEvent::Input(event) = event {
                        if let InputEvent::Keyboard { event } = event {
                            let serial = SERIAL_COUNTER.next_serial();
                            let time = Event::time_msec(&event);
                            let press_state = event.state();
                            let action = seat.get_keyboard().unwrap().input::<Action, _>(
                                state,
                                event.key_code(),
                                press_state,
                                serial,
                                time,
                                |_, modifiers, handle| {
                                    let keysym = handle.modified_sym();

                                    if press_state == KeyState::Pressed
                                        && keysym == keysyms::KEY_t | keysyms::KEY_T
                                    {
                                        return FilterResult::Intercept(Action::Spawn(
                                            "firefox-dev".to_string(),
                                        ));
                                    }

                                    if press_state == KeyState::Pressed
                                        && (modifiers.alt || modifiers.logo)
                                        && (keysyms::KEY_1..=keysyms::KEY_9).contains(&keysym)
                                    {
                                        return if modifiers.logo {
                                            FilterResult::Intercept(Action::WindowSetWorkspace(
                                                (keysym - keysyms::KEY_1).try_into().unwrap(),
                                            ))
                                        } else {
                                            FilterResult::Intercept(Action::WorkspaceSetActive(
                                                (keysym - keysyms::KEY_1).try_into().unwrap(),
                                            ))
                                        };
                                    }

                                    FilterResult::Forward
                                },
                            );

                            if let Some(action) = action {
                                match action {
                                    Action::WorkspaceSetActive(workspace) => {
                                        state.workspaces.set_active(workspace, &mut state.space);
                                    }
                                    Action::WindowSetWorkspace(workspace) => {
                                        state.workspaces.move_window(workspace, &mut state.space);
                                    }
                                    Action::Spawn(program) => {
                                        std::process::Command::new(program).spawn().unwrap();
                                    }
                                }
                            }
                        }

                        if let InputEvent::PointerButton { event, .. } = event {
                            let pointer = seat.get_pointer().unwrap();
                            let keyboard = seat.get_keyboard().unwrap();

                            let serial = SERIAL_COUNTER.next_serial();

                            let button = event.button_code();

                            let button_state = event.state();

                            if ButtonState::Pressed == button_state {
                                if let Some((window, _loc)) = state
                                    .space
                                    .element_under(pointer.current_location())
                                    .map(|(w, l)| (w.clone(), l))
                                {
                                    state.space.raise_element(&window, true);
                                    keyboard.set_focus(
                                        state,
                                        Some(window.toplevel().wl_surface().clone()),
                                        serial,
                                    );
                                    state.space.elements().for_each(|window| {
                                        window.toplevel().send_pending_configure();
                                    });
                                } else {
                                    state.space.elements().for_each(|window| {
                                        window.set_activated(false);
                                        window.toplevel().send_pending_configure();
                                    });
                                    keyboard.set_focus(state, Option::<WlSurface>::None, serial);
                                }
                            };

                            pointer.button(
                                state,
                                &ButtonEvent {
                                    button,
                                    state: button_state,
                                    serial,
                                    time: event.time_msec(),
                                },
                            );
                        }

                        if let InputEvent::PointerMotionAbsolute { event, .. } = event {
                            let output = state.space.outputs().next().unwrap();
                            let output_geo = state.space.output_geometry(output).unwrap();
                            let pointer_location = event.position_transformed(output_geo.size);

                            state.pointer_location = pointer_location;

                            let pointer = seat.get_pointer().unwrap();

                            let element_with_location = state.space.element_under(pointer_location);

                            if let Some((window, location)) = element_with_location {
                                state.workspaces.set_active_window(window.clone());

                                let surface_under_pointer = window
                                    .surface_under(
                                        pointer_location - location.to_f64(),
                                        WindowSurfaceType::ALL,
                                    )
                                    .map(|(s, p)| (s, p + location));
                                let serial = SERIAL_COUNTER.next_serial();
                                pointer.motion(
                                    state,
                                    surface_under_pointer,
                                    &MotionEvent {
                                        location: pointer_location,
                                        serial,
                                        time: event.time_msec(),
                                    },
                                );
                            }
                        }
                    }
                })
                .unwrap();

            backend.bind().unwrap();

            pointer_element.set_current_delay(&state.clock);
            pointer_element.set_status(state.cursor_status.clone());

            let scale = Scale::from(output.current_scale().fractional_scale());
            let cursor_pos = state.pointer_location;
            let cursor_pos_scaled = cursor_pos.to_physical(scale).to_i32_round();

            let elements = pointer_element.render_elements::<PointerRenderElement<GlesRenderer>>(
                backend.renderer(),
                cursor_pos_scaled,
                scale,
                1.0,
            );

            let age = backend.buffer_age().unwrap_or(0);

            render_output::<_, PointerRenderElement<GlesRenderer>, _, _>(
                &output,
                backend.renderer(),
                1.0,
                age,
                [&state.space],
                elements.as_slice(),
                &mut output_damage_tracker,
                [0.1, 0.1, 0.1, 1.0],
            )
            .unwrap();

            backend.submit(None).unwrap();

            state.space.elements().for_each(|window| {
                window.send_frame(
                    &output,
                    start_time.elapsed(),
                    Some(Duration::ZERO),
                    |_, _| Some(output.clone()),
                )
            });

            state.space.refresh();

            display.flush_clients().unwrap();

            TimeoutAction::ToDuration(Duration::from_millis(16))
        })
        .unwrap();

    event_loop.run(None, &mut data, |_| {})?;

    Ok(())
}
