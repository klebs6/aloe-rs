crate::ix!();

/**
  | UpdateHandler implements IUpdateManager
  | and IUpdateHandler to handle dependencies
  | between objects to store and forward
  | messages to dependent objects.
  | 
  | This implementation is thread save,
  | so objects can send message, add or remove
  | dependents from different threads.
  | 
  | Do do so it uses mutex, so be aware of locking.
  |
  */
pub struct UpdateHandler {
    base:  FObject,
    lock:  FLock,
    table: *mut UpdateHandlerTable, // default = nullptr
}

impl FUnknown for UpdateHandler {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_3p::c_void) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { 
        todo!() 
    }

    fn release(&mut self) -> u32 { 
        todo!() 
    }
}

lazy_static!{
    /*
    static bool update_handler_lock_updates;
    */
}

obj_methods!{
    UpdateHandler, FObject
}

funknown_methods2!{
    IUpdateHandler, IUpdateManager, FObject
}

singleton!{
    UpdateHandler
}

impl IUpdateHandler for UpdateHandler {

    /**
      | register \param dependent to get messages
      | from \param object
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn add_dependent(&mut self, 
        object:    *mut dyn FUnknown,
        dependent: *mut dyn IDependent) -> tresult {
        
        todo!();
        /*
        
        */
    }

    /**
      | unregister \param dependent to get
      | no messages from \param object
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn remove_dependent(&mut self, 
        object:    *mut dyn FUnknown,
        dependent: *mut dyn IDependent) -> tresult {
        
        todo!();
        /*
        
        */
    }

    /**
      | send \param message to all dependents
      | of \param object immediately
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn trigger_updates(&mut self, 
        object:  *mut dyn FUnknown,
        message: i32) -> tresult {
        
        todo!();
        /*
        
        */
    }

    /**
      | send \param message to all dependents
      | of \param object when idle
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn defer_updates(&mut self, 
        object:  *mut dyn FUnknown,
        message: i32) -> tresult {
        
        todo!();
        /*
        
        */
    }
}

impl IUpdateManager for UpdateHandler {

    /**
      | cancel pending messages send by \param
      | object or by any if object is 0
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn cancel_updates(&mut self, object: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
        
        */
    }

    /**
      | send pending messages send by \param
      | object or by any if object is 0
      |
      */
    #[PLUGIN_API]
    #[SMTG_OVERRIDE]
    fn trigger_defered_updates(&mut self, object: *mut dyn FUnknown) -> tresult {

        todo!();
        /*
        
        */
    }
}

impl Drop for UpdateHandler {

    fn drop(&mut self) {
        todo!();
        /*
            if (FObject::getUpdateHandler () == this)
            FObject::setUpdateHandler (nullptr);
        delete table;
        table = nullptr;
        */
    }
}

impl Default for UpdateHandler {

    fn default() -> Self {
    
        todo!();
        /*
        table = NEW UpdateHandlerTable;

        if (FObject::getUpdateHandler () == nullptr)
            FObject::setUpdateHandler (this);
        */
    }
}

impl UpdateHandler {

    /**
      | obsolete functions kept for compatibility
      |
      */
    pub fn check_updates(&mut self, object: *mut FObject)  {

        todo!();
        /*
            triggerDeferedUpdates (object->unknownCast ());
        */
    }
    
    pub fn flush_updates(&mut self, object: *mut FObject)  {
        
        todo!();
        /*
            cancelUpdates (object->unknownCast ());
        */
    }
    
    pub fn defer_update(&mut self, 
        object:  *mut FObject,
        message: i32)  {
        
        todo!();
        /*
            deferUpdates (object->unknownCast (), message);
        */
    }
    
    pub fn signal_change(
        &mut self, 
        object:               *mut FObject,
        message:              i32,
        suppress_update_done: Option<bool>
    ) {

        let suppress_update_done: bool = suppress_update_done.unwrap_or(false);

        todo!();
        /*
            doTriggerUpdates (object->unknownCast (), message, suppressUpdateDone);
        */
    }

    #[cfg(DEVELOPMENT)]
    pub fn check_deferred(&mut self, object: *mut FUnknown) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(DEVELOPMENT)]
    pub fn has_dependencies(&mut self, object: *mut FUnknown) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(DEVELOPMENT)]
    pub fn print_for_object(&self, object: *mut FObject)  {
        
        todo!();
        /*
        
        */
    }
    
    #[PLUGIN_API]
    pub fn add_dependent(&mut self, 
        u:         *mut dyn FUnknown,
        dependent: *mut dyn IDependent) -> tresult {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (!unknown || !_dependent)
            return kResultFalse;

        FGuard guard (lock);

    #if CLASS_NAME_TRACKED
        Update::UpdateHandlerDependency dependent (unknown, _dependent);

        FObject* obj = FObject::unknownToObject (unknown);
        if (obj)
            dependent.objClass = obj->isA ();
        obj = FObject::unknownToObject (_dependent);
        if (obj)
            dependent.depClass = obj->isA ();
    #else
        IDependent* dependent = _dependent;
    #endif

        Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
        Update::DependentMapIter it = map.find (unknown);
        if (it == map.end ())
        {
            Update::DependentList list;
            list.push_back (dependent);
            map[unknown] = list;
        }
        else
        {
            (*it).second.push_back (dependent);
        }

        return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn remove_dependent(&mut self, 
        u:         *mut dyn FUnknown,
        dependent: *mut dyn IDependent) -> tresult {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (unknown == nullptr && dependent == nullptr)
            return kResultFalse;

        FGuard guard (lock);

        Update::UpdateDataListIterConst iter = table->updateData.begin ();
        while (iter != table->updateData.end ())
        {
            if ((*iter).obj == unknown || unknown == nullptr)
            {
                for (uint32 count = 0; count < (*iter).count; count++)
                {
                    if ((*iter).dependents[count] == dependent)
                        (*iter).dependents[count] = nullptr;
                }
            }
            ++iter;
        }
        // Remove all objects for the given dependent
        if (unknown == nullptr)
        {
            for (uint32 j = 0; j < Update::kHashSize; j++)
            {
                Update::DependentMap& map = table->depMap[j];
                Update::DependentMapIter iterMap = map.begin ();
                while (iterMap != map.end ())
                {
                    Update::DependentList& list = (*iterMap).second;
                    Update::DependentListIter iterList = list.begin ();
                    bool listIsEmpty = false;
                    
                    while (iterList != list.end ())
                    {
    #if CLASS_NAME_TRACKED
                        if ((*iterList).dep == dependent)
    #else
                        if ((*iterList) == dependent)
    #endif
                        {
                            if (list.size () == 1u)
                            {
                                listIsEmpty = true;
                                break;
                            }
                            else
                                iterList = list.erase (iterList);
                        }
                        else
                        {
                            ++iterList;
                        }
                    }
                    
                    if (listIsEmpty)
                        iterMap = map.erase (iterMap);
                    else
                        ++iterMap;
                }
            }
        }
        else
        {
            bool mustFlush = true;

            Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
            Update::DependentMapIter iterList = map.find (unknown);

    #if NON_EXIstatic inlineG_DEPENDENCY_CHECK
            SMTG_ASSERT (iterList != map.end () && "ERROR: Trying to remove a non existing dependency!")
    #endif
            if (iterList != map.end ())
            {
                if (dependent == nullptr) // Remove all dependents of object
                {
                    map.erase (iterList);
                }
                else // Remove one dependent
                {
                    int32 eraseCount = 0;
                    Update::DependentList& dependentlist = (*iterList).second;
                    Update::DependentListIter iterDependentlist = dependentlist.begin ();
                    while (iterDependentlist != dependentlist.end ())
                    {
    #if CLASS_NAME_TRACKED
                        if ((*iterDependentlist).dep == dependent)
    #else
                        if ((*iterDependentlist) == dependent)
    #endif
                        {
                            iterDependentlist = dependentlist.erase (iterDependentlist);
                            eraseCount++;
                            if (dependentlist.empty ())
                            {
                                map.erase (iterList);
                                break;
                            }
                        }
                        else
                        {
                            ++iterDependentlist;
                            mustFlush = false;
                        }
                    }
                }
            }
            if (mustFlush)
                cancelUpdates (unknown);
        }

        return kResultTrue;
        */
    }
    
    pub fn do_trigger_updates(&mut self, 
        u:                    *mut dyn FUnknown,
        message:              i32,
        suppress_update_done: bool) -> tresult {
        
        todo!();
        /*
            if (lockUpdates)
            return kResultFalse;
        IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (!unknown)
            return kResultFalse;

        // to avoid crashes due to stack overflow, we reduce the amount of memory reserved for the
        // dependents
        IDependent* smallDependents[Update::kMapSize / 10]; // 8kB for x64
        IDependent** dependents = smallDependents;
        int32 maxDependents = Update::kMapSize / 10;
        int32 count = 0;

        {
            FGuard guard (lock); // scope for lock

            Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
            Update::DependentMapIterConst iterList = map.find (unknown);
            if (iterList != map.end ())
            {
                const Update::DependentList& dependentlist = (*iterList).second;
                Update::DependentListIterConst iterDependentlist = dependentlist.begin ();
                while (iterDependentlist != dependentlist.end ())
                {
    #if CLASS_NAME_TRACKED
                    dependents[count] = (*iterDependentlist).dep;
    #else
                    dependents[count] = *iterDependentlist;
    #endif
                    count++;

                    if (count >= maxDependents)
                    {
                        if (dependents == smallDependents)
                        {
                            dependents = NEW IDependent*[Update::kMapSize];
                            memcpy (dependents, smallDependents, count * sizeof (dependents[0]));
                            maxDependents = Update::kMapSize;
                        }
                        else
                        {
                            SMTG_WARNING ("UpdateHandlerDependency overflow")
                            break;
                        }
                    }
                    ++iterDependentlist;
                }
            }

            // push update data on the stack
            if (count > 0)
            {
                Update::UpdateHandlerUpdateData data (unknown, dependents, count);
                table->updateData.push_back (data);
            }
        } // end scope

        int32 i = 0;
        while (i < count)
        {
            if (dependents[i])
                dependents[i]->update (unknown, message);
            i++;
        }

        if (dependents != smallDependents)
            delete[] dependents;

        // remove update data from the stack
        if (count > 0)
        {
            FGuard guard (lock);

            table->updateData.pop_back ();
        }

        // send update done message
        if (suppressUpdateDone == false)
            Update::updateDone (unknown, message);

        return count > 0 ? kResultTrue : kResultFalse; // Object was found and has been updated
        */
    }

    #[PLUGIN_API]
    pub fn trigger_updates(&mut self, 
        u:       *mut dyn FUnknown,
        message: i32) -> tresult {
        
        todo!();
        /*
            return doTriggerUpdates (u, message, false);
        */
    }

    #[PLUGIN_API]
    pub fn defer_updates(&mut self, 
        u:       *mut dyn FUnknown,
        message: i32) -> tresult {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (!unknown)
            return kResultFalse;

        FGuard guard (lock);

        Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
        Update::DependentMapIterConst iterList = map.find (unknown);

        bool hasDependency = (iterList != map.end ());
        if (hasDependency == false)
        {
            Update::updateDone (unknown, message);
            return kResultTrue;
        }

        bool found = false;
        Update::DeferedChangeListIterConst iter = table->defered.begin ();
        while (iter != table->defered.end ())
        {
            if ((*iter).obj == unknown && (*iter).msg == message)
            {
                found = true;
                break;
            }
            ++iter;
        }

        if (!found)
        {
            Update::UpdateDeferedChange change (unknown, message);
            table->defered.push_back (change);
        }

        return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn trigger_defered_updates(&mut self, unknown: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            Update::DeferedChangeList deferedAgain;
        if (!unknown)
        {
            while (table->defered.empty () == false)
            {
                // Remove first from queue
                lock.lock ();

                FUnknown* obj = table->defered.front ().obj;
                int32 msg = table->defered.front ().msg;
                table->defered.pop_front ();

                // check if this object is currently being updated. in this case, defer update again...
                bool canSignal = true;
                Update::UpdateDataListIterConst iter = table->updateData.begin ();
                while (iter != table->updateData.end ())
                {
                    if ((*iter).obj == obj)
                    {
                        canSignal = false;
                        break;
                    }
                    ++iter;
                }
                lock.unlock ();

                if (canSignal)
                {
                    triggerUpdates (obj, msg);
                }
                else
                {
                    Update::UpdateDeferedChange change (obj, msg);
                    deferedAgain.push_back (change);
                }
            }
        }
        else
        {
            IPtr<FUnknown> object = Update::getUnknownBase (unknown);
            Update::UpdateDeferedChange tmp (object);

            while (true)
            {
                lock.lock ();
                Update::DeferedChangeListIter it =
                    std::find (table->defered.begin (), table->defered.end (), tmp);
                if (it == table->defered.end ())
                {
                    lock.unlock ();
                    return kResultTrue;
                }

                if ((*it).obj != nullptr)
                {
                    int32 msg = (*it).msg;
                    table->defered.erase (it);

                    // check if this object is currently being updated. in this case, defer update
                    // again...
                    bool canSignal = true;
                    Update::UpdateDataListIterConst iter = table->updateData.begin ();
                    while (iter != table->updateData.end ())
                    {
                        if ((*iter).obj == object)
                        {
                            canSignal = false;
                            break;
                        }
                        ++iter;
                    }
                    lock.unlock ();

                    if (canSignal)
                    {
                        triggerUpdates (object, msg);
                    }
                    else
                    {
                        Update::UpdateDeferedChange change (object, msg);
                        deferedAgain.push_back (change);
                    }
                }
            }
        }

        if (deferedAgain.empty () == false)
        {
            FGuard guard (lock);

            Update::DeferedChangeListIterConst iter = deferedAgain.begin ();
            while (iter != deferedAgain.end ())
            {
                table->defered.push_back (*iter);
                ++iter;
            }
        }

        return kResultTrue;
        */
    }

    #[PLUGIN_API]
    pub fn cancel_updates(&mut self, u: *mut dyn FUnknown) -> tresult {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (!unknown)
            return kResultFalse;

        FGuard guard (lock);

        Update::UpdateDeferedChange change (unknown, 0);
        while (true)
        {
            auto iter = std::find (table->defered.begin (), table->defered.end (), change);
            if (iter != table->defered.end ())
                table->defered.erase (iter);
            else
                break;
        }

        return kResultTrue;
        */
    }
    
    pub fn count_dependencies(&mut self, object: *mut dyn FUnknown) -> usize {
        
        todo!();
        /*
            FGuard guard (lock);
        uint32 res = 0;

        IPtr<FUnknown> unknown = Update::getUnknownBase (object);
        if (unknown)
        {
            Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
            Update::DependentMapIter iterList = map.find (unknown);
            if (iterList != map.end ())
                return iterList->second.size ();
        }
        else
        {
            for (uint32 j = 0; j < Update::kHashSize; j++)
            {
                Update::DependentMap& map = table->depMap[j];
                res += countEntries (map);
            }
        }
        return res;
        */
    }

    #[cfg(DEVELOPMENT)]
    pub fn check_deferred(&mut self, object: *mut FUnknown) -> bool {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (object);

        FGuard guard (lock);

        Update::UpdateDeferedChange tmp (unknown);
        Update::DeferedChangeListIterConst it =
            std::find (table->defered.begin (), table->defered.end (), tmp);
        if (it != table->defered.end ())
            return true;

        return false;
        */
    }
    
    #[cfg(DEVELOPMENT)]
    pub fn has_dependencies(&mut self, u: *mut FUnknown) -> bool {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (u);
        if (!unknown)
            return false;

        FGuard guard (lock);

        Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
        Update::DependentMapIterConst iterList = map.find (unknown);
        bool hasDependency = (iterList != map.end ());

        return hasDependency;
        */
    }
    
    #[cfg(DEVELOPMENT)]
    pub fn print_for_object(&self, obj: *mut FObject)  {
        
        todo!();
        /*
            IPtr<FUnknown> unknown = Update::getUnknownBase (obj);
        if (!unknown)
            return;

        FUnknownPtr<IDependent> dep (obj);

        bool header = false;

        Update::DependentMap& map = table->depMap[Update::hashPointer (unknown)];
        Update::DependentMapIterConst iterList = map.begin ();
        while (iterList != map.end ())
        {
            const Update::DependentList& dependentlist = (*iterList).second;
            Update::DependentListIterConst iterDependentlist = dependentlist.begin ();
            while (iterDependentlist != dependentlist.end ())
            {
    #if CLASS_NAME_TRACKED
                if ((*iterList).first == unknown || (*iterDependentlist).dep == dep.getInterface ())
                {
                    if (!header)
                    {
                        FDebugPrint ("Dependencies for object %8" FORMAT_INT64A " %s\n", (uint64)obj,
                                     obj->isA ());
                        header = true;
                    }
                    FDebugPrint ("%s %8" FORMAT_INT64A "\n <- %s %8" FORMAT_INT64A "\n",
                                 (*iterDependentlist).depClass, (uint64) (*iterDependentlist).dep,
                                 (*iterDependentlist).objClass, (uint64) ((*iterList).first));
                }
    #else
                if ((*iterList).first == unknown || (*iterDependentlist) == dep.getInterface ())
                {
                    if (!header)
                    {
                        FDebugPrint ("Dependencies for object %8" FORMAT_INT64A " %s\n", (uint64)obj,
                                     obj->isA ());
                        header = true;
                    }
                    FDebugPrint ("%8" FORMAT_INT64A "\n <- %8" FORMAT_INT64A "\n",
                                 (uint64) (*iterDependentlist), (uint64) ((*iterList).first));
                }
    #endif
                ++iterDependentlist;
            }

            ++iterList;
        }
        */
    }
}
