use std::i32;

// Heavily based on https://github.com/BowlBird/bbsl/
use wayland_client::{
    Connection, Dispatch, EventQueue, QueueHandle,
    protocol::{
        wl_buffer::WlBuffer,
        wl_callback::{self, WlCallback},
        wl_compositor::WlCompositor,
        wl_registry::{self, WlRegistry},
        wl_shm::WlShm,
        wl_shm_pool::WlShmPool,
        wl_surface::{self, WlSurface},
    },
};

struct Rect {
    width: i32,
    height: i32,
}

struct AppState {
    compositor: Option<WlCompositor>,
    shm: Option<WlShm>,
    wl_surface: Option<WlSurface>,
    dimension: Option<Rect>,
    exit: bool,
}

impl Dispatch<WlCompositor, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlCompositor,
        _event: <WlCompositor as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlRegistry, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlRegistry,
        event: <WlRegistry as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            println!("[{}] {} (v{})", name, interface, version);
        }
    }
}

impl Dispatch<WlSurface, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlSurface,
        _event: <WlSurface as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlBuffer, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlBuffer,
        _event: <WlBuffer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlCallback, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlCallback,
        _event: <WlCallback as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        if let wl_callback::Event::Done { callback_data } = _event {
            let wl_surface = _state
                .wl_surface
                .as_ref()
                .expect("Cannot Connect to Surface");
            let _ = wl_surface.frame(_qhandle, ());

            //let buffer = draw_frame(_state, _qhandle).expect("Cannot Draw Frame");

            //wl_surface.attach(Some(&buffer), 0, 0);
            //wl_surface.damage_buffer(0, 0, i32::MAX, i32::MAX);
            //wl_surface.commit();
        }
    }
}

impl Dispatch<WlShm, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlShm,
        _event: <WlShm as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlShmPool, ()> for AppState {
    fn event(
        _state: &mut Self,
        _proxy: &WlShmPool,
        _event: <WlShmPool as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
    }
}

// fn draw_frame(state: AppState, qhandle: &QueueHandle<AppState>) -> Result<WlBuffer, ()> {}

fn main() {
    let mut state = AppState {
        compositor: None,
        shm: None,
        wl_surface: None,
        dimension: None,
        exit: false,
    };
    let conn = Connection::connect_to_env().unwrap();
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());

    println!("Globals:");

    event_queue.roundtrip(&mut state).unwrap();
}
