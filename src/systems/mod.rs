mod despawn_with;
pub use self::despawn_with::*;

mod init_resource;
pub use self::init_resource::*;

mod remove_resource;
pub use self::remove_resource::*;

mod save_on_exit;

#[cfg(feature = "icon")]
mod set_window_icon;
#[cfg(feature = "icon")]
pub(crate) use self::set_window_icon::*;
