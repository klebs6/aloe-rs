crate::ix!();

pub struct Vst3HostContextMessageMapComparator { }

impl Vst3HostContextMessageMapComparator {

    pub fn invoke(
        &self, 
        a: *const u8,
        b: *const u8

    ) -> bool {
        
        todo!();
        /*
            return std::strcmp (a, b) < 0;
        */
    }
}

//----------------------------------------
pub struct Vst3HostContextMessageMap {

    /**
      | Steinberg's docs say: >  Please note
      | that messages from the processor to the
      | controller must not be sent during the
      | process call, as this would not be fast
      | enough and would break the real time
      | processing. Such tasks should be
      | handled in a separate timer thread.
      | Using a lock here is fine (plugins
      | should be aware that sending messages
      | is not realtime-safe), and protects the
      | data structure in case the processor
      | sends messages from a background thread
      | rather than from the message thread.
      */
    storage: HashMap<*const u8,VstComSmartPtr<Vst3HostContextMessage>,Vst3HostContextMessageMapComparator>,

    mutex:   CriticalSection,
}

impl Vst3HostContextMessageMap {

    pub fn get_binary(&mut self, 
        id:   *const u8,
        data: &mut *const c_void,
        size: &mut u32) -> tresult {
        
        todo!();
        /*
            jassert (id != nullptr);

                const ScopedLock lock (mutex);

                const auto it = storage.find (id);

                if (it != storage.cend())
                {
                    if (auto* binaryData = it->second->value.getBinaryData())
                    {
                        data = binaryData->getData();
                        size = (uint32) binaryData->getSize();
                        return kResultTrue;
                    }
                }

                return kResultFalse;
        */
    }
    
    
    pub fn add_message_to_queue<Type>(
        &mut self, 
        id:    *const u8,
        list:  *mut dyn IAttributeList<AttrID = *const u8>,
        value: &Type

    ) {
    
        todo!();
        /*
            jassert (id != nullptr);

                const ScopedLock lock (mutex);

                const auto it = storage.find (id);

                if (it != storage.cend())
                    it->second->value = value;
                else
                    storage.emplace (id, new Vst3HostContextMessage (list, id, value));
        */
    }
    
    
    pub fn find_message_on_queue_withid<Type>(&mut self, 
        id:    *const u8,
        value: &mut Type) -> bool {
    
        todo!();
        /*
            jassert (id != nullptr);

                const ScopedLock lock (mutex);

                const auto it = storage.find (id);

                if (it == storage.cend())
                    return false;

                value = it->second->value;
                return true;
        */
    }
    
    pub fn add(&mut self, message: VstComSmartPtr<Vst3HostContextMessage>)  {
        
        todo!();
        /*
            const ScopedLock lock (mutex);

                const auto* id = message->getMessageID();
                storage.erase (id);
                storage.emplace (id, std::move (message));
        */
    }
}
