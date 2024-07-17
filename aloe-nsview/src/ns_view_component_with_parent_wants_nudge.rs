crate::ix!();

#[cfg(target_os="macos")]
pub enum NSViewComponentWithParentWantsNudge { 
    no, 
    yes 
}

#[cfg(target_os="macos")]
lazy_static!{
    /*
    NSViewComponentWithParentWantsNudge wantsNudge = NSViewComponentWithParentWantsNudge::no;
    */
}
