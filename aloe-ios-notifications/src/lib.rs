#[cfg(target_os="ios")]
#[macro_use] mod imports; 

#[cfg(target_os="ios")]
use imports::*;

#[cfg(target_os="ios")] x!{delegate}
#[cfg(target_os="ios")] x!{delegate_class}
#[cfg(target_os="ios")] x!{delegate_details}
#[cfg(target_os="ios")] x!{delegate_interface}
#[cfg(target_os="ios")] x!{notification_impl}
