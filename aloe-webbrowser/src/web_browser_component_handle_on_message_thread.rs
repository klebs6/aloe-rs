crate::ix!();

pub struct WebBrowserComponentHandleOnMessageThread<'a> {
    owner:          *mut WebBrowserComponentImpl<'a>, // default = nullptr
    cmd_to_send:    String,
    params_to_send: Var,
}

impl<'a> CallbackMessage for WebBrowserComponentHandleOnMessageThread<'a> {

    fn message_callback(&mut self)  {
        
        todo!();
        /*
            owner->handleCommandOnMessageThread (cmdToSend, paramsToSend);
        */
    }
}

impl<'a> WebBrowserComponentHandleOnMessageThread<'a> {

    pub fn new(
        impl_:      *mut WebBrowserComponentImpl,
        cmd_to_use: &String,
        params:     &Var

    ) -> Self {
    
        todo!();
        /*
        : owner(impl),
        : cmd_to_send(cmdToUse),
        : params_to_send(params),

        
        */
    }
}
