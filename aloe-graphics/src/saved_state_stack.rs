crate::ix!();

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct SavedStateStack<StateObjectType> {
    current_state: Box<StateObjectType>,
    stack:         Vec<StateObjectType>,
}

impl<StateObjectType> Deref for SavedStateStack<StateObjectType> {
    type Target = StateObjectType;

    
    fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return currentState.get();
        */
    }
}

impl<StateObjectType> SavedStateStack<StateObjectType> {

    pub fn new(initial_state: *mut StateObjectType) -> Self {
    
        todo!();
        /*
        : current_state(initialState),

        
        */
    }
    
    pub fn initialise(&mut self, state: *mut StateObjectType)  {
        
        todo!();
        /*
            currentState.reset (state);
        */
    }
    
    pub fn save(&mut self)  {
        
        todo!();
        /*
            stack.add (new StateObjectType (*currentState));
        */
    }
    
    pub fn restore(&mut self)  {
        
        todo!();
        /*
            if (auto* top = stack.getLast())
            {
                currentState.reset (top);
                stack.removeLast (1, false);
            }
            else
            {
                jassertfalse; // trying to pop with an empty stack!
            }
        */
    }
    
    pub fn begin_transparency_layer(&mut self, opacity: f32)  {
        
        todo!();
        /*
            save();
            currentState.reset (currentState->beginTransparencyLayer (opacity));
        */
    }
    
    pub fn end_transparency_layer(&mut self)  {
        
        todo!();
        /*
            std::unique_ptr<StateObjectType> finishedTransparencyLayer (currentState.release());
            restore();
            currentState->endTransparencyLayer (*finishedTransparencyLayer);
        */
    }
}
