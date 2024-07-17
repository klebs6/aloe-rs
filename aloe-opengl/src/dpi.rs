crate::ix!();

#[cfg(all(target_os="windows",ALOE_WIN_PER_MONITOR_DPI_AWARE))]
lazy_static!{
    /*
    extern  double getScaleFactorForWindow (HWND);
    */
}
