#[cfg(target_os="macos")]
#[macro_use] mod imports; 

#[cfg(target_os="macos")]
use imports::*;

#[cfg(target_os="macos")] x!{delegate}
#[cfg(target_os="macos")] x!{delegate_class}
#[cfg(target_os="macos")] x!{delegate_details}
#[cfg(target_os="macos")] x!{delegate_interface}
#[cfg(target_os="macos")] x!{push_notification_impl}
