use wayland_protocols_wlr::screencopy::v1::server::{zwlr_screencopy_frame_v1, zwlr_screencopy_manager_v1};
use wayland_server::{backend::GlobalId, Dispatch, GlobalDispatch};

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

/// Macro to delegate implementation of screencopy to [`ScreencopyState`](self::ScreencopyState)
///
/// [`ScreencopyHandler`](self::ScreencopyHandler) must be implemented to use this
#[macro_export]
macro_rules! delegate_screencopy {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt)* )? ),+>)? $ty: ty) => {};
}
