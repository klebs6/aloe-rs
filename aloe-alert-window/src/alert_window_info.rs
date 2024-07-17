crate::ix!();

pub enum Async { 
    no, 
    yes 
}

#[no_copy]
#[leak_detector]
pub struct AlertWindowInfo<'a> {
    options:      MessageBoxOptions<'a>,
    callback:     Box<dyn ModalComponentManagerCallback>,
    async_:       Async,
    return_value: i32, // default = 0
}

impl<'a> AlertWindowInfo<'a> {

    pub fn new(
        opts:       &MessageBoxOptions,
        cb:         Box<dyn ModalComponentManagerCallback>,
        show_async: Async) -> Self {
    
        todo!();
        /*


            : options (opts),
              callback (std::move (cb)),
              async (showAsync)
        */
    }
    
    pub fn invoke(&self) -> i32 {
        
        todo!();
        /*
            MessageManager::getInstance()->callFunctionOnMessageThread (showCallback, (void*) this);
            return returnValue;
        */
    }
    
    pub fn show_callback(user_data: *mut c_void)  {
        
        todo!();
        /*
            static_cast<AlertWindowInfo*> (userData)->show();
            return nullptr;
        */
    }
    
    pub fn show(&mut self)  {
        
        todo!();
        /*
            auto* component = options.getAssociatedComponent();

            auto& lf = (component != nullptr ? component->getLookAndFeel()
                                             : LookAndFeel::getDefaultLookAndFeel());

            std::unique_ptr<AlertWindow> alertBox (lf.createAlertWindow (options.getTitle(), options.getMessage(),
                                                                         options.getButtonText (0), options.getButtonText (1), options.getButtonText (2),
                                                                         options.getIconType(), options.getNumButtons(), component));

            jassert (alertBox != nullptr); // you have to return one of these!

            alertBox->setAlwaysOnTop (aloe_areThereAnyAlwaysOnTopWindows());

           #if ALOE_MODAL_LOOPS_PERMITTED
            if (async == Async::no)
                returnValue = alertBox->runModalLoop();
            else
           #endif
            {
                ignoreUnused (async);

                alertBox->enterModalState (true, callback.release(), true);
                alertBox.release();
            }
        */
    }
}
