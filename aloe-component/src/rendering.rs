crate::ix!();
  
pub trait GetAvailableRenderingEngines {
    fn get_available_rendering_engines(&mut self) -> Vec<String>;
}

pub trait GetCurrentRenderingEngine {
    fn get_current_rendering_engine(&self) -> i32;
}

pub trait SetCurrentRenderingEngine {
    fn set_current_rendering_engine(&mut self, index: i32);
}
