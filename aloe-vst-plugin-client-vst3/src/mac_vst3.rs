crate::ix!();

#[cfg(target_os="macos")]
#[cfg(not(ALOE_64BIT))]
lazy_static!{
    /*
    extern void updateEditorCompBoundsVST (Component*);
    */
}

#[cfg(target_os="macos")]
lazy_static!{
    /*
    extern void initialiseMacVST();
        extern  void* attachComponentToWindowRefVST (Component*, void* parentWindowOrView, bool isNSView);
        extern  void detachComponentFromWindowRefVST (Component*, void* nsWindow, bool isNSView);
    */
}
