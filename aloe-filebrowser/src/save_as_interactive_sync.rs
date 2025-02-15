crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct SaveAsInteractiveSyncImpl {
    p: &mut Impl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl SaveAsInteractiveSyncImpl {
    
    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.saveAsInteractiveSyncImpl (std::forward<Ts> (ts)...);
        */
    }
}

