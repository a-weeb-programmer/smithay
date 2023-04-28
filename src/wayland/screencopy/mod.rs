//! Utilities for handling the `screencopy` protocol
//!

use wayland_protocols_wlr::screencopy::v1::server::{
    zwlr_screencopy_frame_v1,
    zwlr_screencopy_manager_v1::{self, Request},
};
use wayland_server::{
    backend::GlobalId, protocol::wl_output::WlOutput, Dispatch, DisplayHandle, GlobalDispatch,
};

/// Screencopy global state
#[derive(Debug)]
pub struct ScreencopyState {
    id: GlobalId,
}

/// Screencopy handler type
///
/// Allows for handling of requests sent by the client utilizing screencopy
pub trait ScreencopyHandler {
    /// Gets the [`ScreencopyState`](crate::ScreencopyState) as a mutable reference
    fn screencopy_state(&mut self) -> &mut ScreencopyState;

    /// The function to be executed when a client wants to capture an output
    ///
    /// Called when a client requests to capture the given `output`
    /// with the `frame` that is ready for copying
    fn capture_output(
        &mut self,
        frame: &zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
        overlay_cursor: i32,
        output: &WlOutput,
    );
}

impl ScreencopyState {
    /// Creates a new [`ScreencopyState`](self::Self)
    ///
    /// Your `D` type must have [`ScreencopyHandler`](self::ScreencopyHandler) implemented for it
    /// and be delegated with [`delegate_screencopy`]
    pub fn new<D>(display: &DisplayHandle) -> ScreencopyState
    where
        D: GlobalDispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
            + Dispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
            + ScreencopyHandler
            + 'static,
    {
        let id = display.create_global::<D, zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, _>(3, ());
        ScreencopyState { id }
    }

    /// Gets the id of the [`ScreencopyManager`] global
    pub fn global(&self) -> GlobalId {
        self.id.clone()
    }
}

impl<D> GlobalDispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, (), D> for ScreencopyState
where
    D: GlobalDispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
        + Dispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
        + ScreencopyHandler
        + 'static,
{
    fn bind(
        _: &mut D,
        _: &wayland_server::DisplayHandle,
        _: &wayland_server::Client,
        resource: wayland_server::New<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1>,
        _: &(),
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D> Dispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, (), D> for ScreencopyState
where
    D: GlobalDispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
        + Dispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
        + Dispatch<zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1, ()>
        + ScreencopyHandler
        + 'static,
{
    fn request(
        state: &mut D,
        _client: &wayland_server::Client,
        _resource: &zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1,
        request: <zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1 as wayland_server::Resource>::Request,
        _: &(),
        _dhandle: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            Request::CaptureOutput {
                frame,
                overlay_cursor,
                output,
            } => {
                let frame = data_init.init::<zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1, ()>(frame, ());
                state.capture_output(&frame, overlay_cursor, &output);
            }
            _ => unreachable!(),
        }
    }

    fn destroyed(
        _state: &mut D,
        _client: wayland_server::backend::ClientId,
        _resource: wayland_server::backend::ObjectId,
        _data: &(),
    ) {
    }
}

impl<D> Dispatch<zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1, (), D> for ScreencopyState
where
    D: GlobalDispatch<zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1, ()>
        + Dispatch<zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1, ()>
        + ScreencopyHandler
        + 'static,
{
    fn request(
        state: &mut D,
        _client: &wayland_server::Client,
        resource: &zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1,
        request: <zwlr_screencopy_frame_v1::ZwlrScreencopyFrameV1 as wayland_server::Resource>::Request,
        _data: &(),
        _dhandle: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
    }

    fn destroyed(
        _state: &mut D,
        _client: wayland_server::backend::ClientId,
        _resource: wayland_server::backend::ObjectId,
        _data: &(),
    ) {
    }
}

/// Macro to delegate implementation of screencopy to [`ScreencopyState`](self::ScreencopyState)
///
/// [`ScreencopyHandler`](self::ScreencopyHandler) must be implemented to use this
#[macro_export]
macro_rules! delegate_screencopy {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt)* )? ),+>)? $ty: ty) => {};
}
