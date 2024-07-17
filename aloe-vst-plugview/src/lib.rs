#![feature(associated_type_defaults)]
#[macro_use] mod imports; use imports::*;

x!{common_pluginview}
x!{iplug_view}
x!{iplugframe}
x!{iplugviewcontentscalesupport}
x!{ivstplugview}

#[cfg(SMTG_OS_LINUX)] 
x!{linux_irunloop}

#[cfg(SMTG_OS_LINUX)] 
x!{linux_itimer_handler}

x!{platform_ui}
x!{view_rect}
