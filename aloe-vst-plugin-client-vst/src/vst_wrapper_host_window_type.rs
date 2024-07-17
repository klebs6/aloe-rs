crate::ix!();

#[cfg(any(target_os="linux",target_os="bsd"))]
pub type AloeVSTWrapperHostWindowType = Window;

#[cfg(target_os="windows")]
pub type AloeVSTWrapperHostWindowType = HWND;

#[cfg(not(all(any(target_os="linux",target_os="bsd"),target_os="windows")))]
pub type AloeVSTWrapperHostWindowType = *mut c_void;
