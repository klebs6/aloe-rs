crate::ix!();

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
pub struct AskToOverwriteFileSync {
    p: &mut Impl,
}

#[cfg(ALOE_MODAL_LOOPS_PERMITTED)]
impl AskToOverwriteFileSync {

    pub fn invoke<Ts>(&self, ts: Ts)  {
    
        todo!();
        /*
            p.askToOverwriteFileSync (std::forward<Ts> (ts)...);
        */
    }
}

