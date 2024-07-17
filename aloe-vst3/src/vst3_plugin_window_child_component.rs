crate::ix!();

#[cfg(target_os="macos")]
pub type Vst3PluginWindowHandleFormat = *mut NSView;

#[cfg(any(target_os="linux",target_os="bsd"))]
pub type Vst3PluginWindowHandleFormat = Window;

#[cfg(not(any(target_os="windows",target_os="macos",target_os="linux",target_os="bsd")))]
pub type Vst3PluginWindowHandleFormat = *mut c_void;

///------------------------
#[cfg(target_os="windows")]
pub type Vst3PluginWindowHandleFormat = HWND;

///------------------------
#[cfg(target_os="windows")]
#[no_copy]
#[leak_detector]
pub struct Vst3PluginWindowChildComponent {
    base: Component,
}

#[cfg(target_os="windows")]
impl Default for Vst3PluginWindowChildComponent {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);
        */
    }
}

#[cfg(target_os="windows")]
impl Vst3PluginWindowChildComponent {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::cornflowerblue);
        */
    }
}
