crate::ix!();

#[no_copy]
pub struct ApplicationCommandTargetCommandMessage<'a> {
    owner: WeakReference<ApplicationCommandTarget>,
    info:  ApplicationCommandTargetInvocationInfo<'a>,
}

impl<'a> MessageBaseInterface for ApplicationCommandTargetCommandMessage<'a> {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            if (ApplicationCommandTarget* const target = owner)
                    target->tryToInvoke (info, false);
        */
    }
}

impl<'a> ApplicationCommandTargetCommandMessage<'a> {

    pub fn new(
        target: *mut ApplicationCommandTarget,
        inf:    &ApplicationCommandTargetInvocationInfo) -> Self {
    
        todo!();
        /*
        : owner(target),
        : info(inf),

        
        */
    }
}
