crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct SaveSync {
    p: &mut Pimpl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl SaveSync {

    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.saveSync (std::forward<Ts> (ts)...);
        */
    }
}

