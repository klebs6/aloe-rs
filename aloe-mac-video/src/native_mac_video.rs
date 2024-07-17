crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_video/native/aloe_mac_Video.h]

#[cfg(target_os="macos")]
pub type Base<'a> = NSViewComponent<'a>;

#[cfg(not(target_os="macos"))]
pub type Base = UIViewComponent;
