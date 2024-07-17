crate::ix!();

pub enum LoopMode
{
    none,
    forward,
    pingpong
}

pub trait VariantConverter {

    fn from_var(v: &Var) -> LoopMode;

    fn to_var(loop_mode: LoopMode) -> Var;
}

impl VariantConverter for LoopMode {
    
    fn from_var(v: &Var) -> LoopMode {
        
        todo!();
        /*
            return static_cast<LoopMode> (int (v));
        */
    }
    
    fn to_var(loop_mode: LoopMode) -> Var {
        
        todo!();
        /*
            return static_cast<int> (loopMode);
        */
    }
}
