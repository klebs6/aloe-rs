crate::ix!();

pub struct AloeVst3ComponentScopedInSetupProcessingSetter<'a> {
    controller: *mut AloeVst3EditController<'a>, // default = nullptr
}

impl<'a> Drop for AloeVst3ComponentScopedInSetupProcessingSetter<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            if (controller != nullptr)
                    controller->inSetupProcessing = false;
        */
    }
}

impl<'a> AloeVst3ComponentScopedInSetupProcessingSetter<'a> {

    pub fn new(c: *mut AloeVst3EditController) -> Self {
    
        todo!();
        /*
        : controller(c),

            if (controller != nullptr)
                    controller->inSetupProcessing = true;
        */
    }
}
