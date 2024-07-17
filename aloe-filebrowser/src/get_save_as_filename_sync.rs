crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct GetSaveAsFilenameSync {
    p: &mut Pimpl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl GetSaveAsFilenameSync {

    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.getSaveAsFilenameSync (std::forward<Ts> (ts)...);
        */
    }
}

