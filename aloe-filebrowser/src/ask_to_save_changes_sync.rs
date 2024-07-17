crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct AskToSaveChangesSync {
    p: &mut Pimpl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl AskToSaveChangesSync {
    
    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.askToSaveChangesSync (std::forward<Ts> (ts)...);
        */
    }
}

