crate::ix!();

pub const PARAMETER_CHANGES_NOT_IN_VECTOR: i32 = -1;

pub struct ParameterChangesEntry {
    ptr:   VstComSmartPtr<ParamValueQueue>,
    index: i32, // default = notInVector
}

impl ParameterChangesEntry {

    pub fn new(queue: Box<ParamValueQueue>) -> Self {
    
        todo!();
        /*
        : ptr(queue.release()),

        
        */
    }
}

pub type ParameterChangesMap    = HashMap<ParamID,ParameterChangesEntry>;
pub type ParameterChangesQueues = Vec<*mut ParameterChangesEntry>;

/**
  | An implementation of IParameterChanges
  | with some important characteristics:
  | 
  | - Lookup by index is O(1)
  | 
  | - Lookup by paramID is also O(1)
  | 
  | - addParameterData never allocates,
  | as long you pass a paramID already passed
  | to initialise
  |
  */
#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
pub struct ParameterChanges {
    map:       ParameterChangesMap,
    queues:    ParameterChangesQueues,
    ref_count: Atomic<i32>,
}

impl IParameterChanges for ParameterChanges {

    #[PLUGIN_API]
    fn get_parameter_count(&mut self) -> i32 {
        
        todo!();
        /*
            return (i32) queues.size();
        */
    }
    
    #[PLUGIN_API]
    fn get_parameter_data(&mut self, index: i32) -> *mut dyn IParamValueQueue {
        
        todo!();
        /*
            if (isPositiveAndBelow (index, queues.size()))
            {
                auto& entry = queues[(size_t) index];
                // If this fails, our container has become internally inconsistent
                jassert (entry->index == index);
                return entry->ptr.get();
            }

            return nullptr;
        */
    }
    
    #[PLUGIN_API]
    fn add_parameter_data(
        &mut self, 
        id:    &ParamID,
        index: &mut i32

    ) -> *mut dyn IParamValueQueue {
        
        todo!();
        /*
            const auto it = map.find (id);

            if (it == map.end())
                return nullptr;

            auto& result = it->second;

            if (result.index == notInVector)
            {
                result.index = (i32) queues.size();
                queues.push_back (&result);
            }

            index = result.index;
            return result.ptr.get();
        */
    }
}

impl FUnknown for ParameterChanges {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl ParameterChanges {

    pub fn set(&mut self, 
        id:    ParamID,
        value: f32)  {
        
        todo!();
        /*
            i32 indexOut = notInVector;

            if (auto* queue = addParameterData (id, indexOut))
                queue->set (value);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            for (auto* item : queues)
                item->index = notInVector;

            queues.clear();
        */
    }
    
    pub fn initialise(&mut self, ids_in: &Vec<ParamID>)  {
        
        todo!();
        /*
            i32 index = 0;

            for (const auto& id : idsIn)
                map.emplace (id, ParameterChangesEntry { std::make_unique<ParamValueQueue> (id, i32 { index++ }) });

            queues.reserve (map.size());
            queues.clear();
        */
    }
    
    
    pub fn for_each<Callback>(&self, callback: Callback)  {
    
        todo!();
        /*
            for (const auto* item : queues)
            {
                auto* ptr = item->ptr.get();
                callback (ptr->getParameterIndex(), ptr->get());
            }
        */
    }
}
