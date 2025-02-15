crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct SaveAsSyncImpl {
    p: &mut Impl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl SaveAsSyncImpl {

    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.saveAsSyncImpl (std::forward<Ts> (ts)...);
        */
    }
}

