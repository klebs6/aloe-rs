crate::ix!();

pub struct FftEngine {

    /**
      used so that faster engines have
      priority over slower ones
      */
    engine_priority: i32,
}

impl FftEngine {

    pub fn new(priority_to_use: i32) -> Self {
    
        todo!();
        /*
        : engine_priority(priorityToUse),

            auto& list = getEngines();
                list.add (this);
                std::sort (list.begin(), list.end(), [] (FftEngine* a, FftEngine* b) { return b->enginePriority < a->enginePriority; });
        */
    }
    
    pub fn create_best_engine_for_platform(order: i32) -> *mut dyn FftInstance {
        
        todo!();
        /*
            for (auto* engine : getEngines())
                    if (auto* instance = engine->create (order))
                        return instance;

                jassertfalse;  // This should never happen as the fallback engine should always work!
                return nullptr;
        */
    }
    
    pub fn get_engines<'a>() -> &'a mut Vec<*mut FftEngine> {
        
        todo!();
        /*
            static Vec<FftEngine*> engines;
                return engines;
        */
    }
}
