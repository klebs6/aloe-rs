crate::ix!();

pub trait ClearRenderingSequence {

    fn clear_rendering_sequence(&mut self);
}

pub trait BuildRenderingSequence {
    
    fn build_rendering_sequence(&mut self);
}
