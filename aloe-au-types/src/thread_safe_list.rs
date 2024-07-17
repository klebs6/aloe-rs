crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/AU/CoreAudioUtilityClasses/CAThreadSafeList.h]

/**
  | linked list of T's
  | 
  | T must define operator ==
  |
  */
pub struct ThreadSafeList<T> {

    /**
      | what's actually in the container - only
      | accessed on one thread
      |
      */
    active_list:  ThreadSafeListNodeStack<T>,

    /**
      | add or remove requests - threadsafe
      |
      */
    pending_list: ThreadSafeListNodeStack<T>,

    /**
      | free nodes for reuse - threadsafe
      |
      */
    free_list:    ThreadSafeListNodeStack<T>,
}

impl<T> Drop for ThreadSafeList<T> {

    fn drop(&mut self) {
        todo!();
        /*
            mActiveList.free_all();
            mPendingList.free_all();
            mFreeList.free_all();
        */
    }
}

impl<T> ThreadSafeList<T> {

    /**
      | These may be called on any thread
      |
      */
    pub fn deferred_add(&mut self, obj: &T)  {

        // can be called on any thread
        
        todo!();
        /*
            ThreadSafeListNode *node = AllocNode();
            node->mEventType = kAdd;
            node->mObject = obj;
            mPendingList.push_atomic(node);
            //mPendingList.dump("pending after add");
        */
    }

    /**
      | can be called on any thread
      |
      */
    pub fn deferred_remove(&mut self, obj: &T)  {
        
        todo!();
        /*
            ThreadSafeListNode *node = AllocNode();
            node->mEventType = kRemove;
            node->mObject = obj;
            mPendingList.push_atomic(node);
            //mPendingList.dump("pending after remove");
        */
    }

    /**
      | can be called on any thread
      |
      */
    pub fn deferred_clear(&mut self)  {
        
        todo!();
        /*
            ThreadSafeListNode *node = AllocNode();
            node->mEventType = kClear;
            mPendingList.push_atomic(node);
        */
    }

    /**
      | These must be called from only one thread
      |
      | must only be called from one thread
      */
    pub fn update(&mut self)  {
        
        todo!();
        /*
            ThreadSafeListNodeStack reversed;
            ThreadSafeListNode *event, *node, *next;
            bool workDone = false;

            // reverse the events so they are in order
            event = mPendingList.pop_all();
            while (event != NULL) {
                next = event->mNext;
                reversed.push_NA(event);
                event = next;
                workDone = true;
            }
            if (workDone) {
                //reversed.dump("pending popped");
                //mActiveList.dump("active before update");

                // now process them
                while ((event = reversed.pop_NA()) != NULL) {
                    switch (event->mEventType) {
                        case kAdd:
                            {
                                ThreadSafeListNode **pnode;
                                bool needToInsert = true;
                                for (pnode = mActiveList.phead(); *pnode != NULL; pnode = &node->mNext) {
                                    node = *pnode;
                                    if (node->mObject == event->mObject) {
                                        //printf("already active!!!\n");
                                        FreeNode(event);
                                        needToInsert = false;
                                        break;
                                    }
                                }
                                if (needToInsert) {
                                    // link the new event in at the end of the active list
                                    *pnode = event;
                                    event->mNext = NULL;
                                }
                            }
                            break;
                        case kRemove:
                            // find matching node in the active list, remove it
                            for (ThreadSafeListNode **pnode = mActiveList.phead(); *pnode != NULL; ) {
                                node = *pnode;
                                if (node->mObject == event->mObject) {
                                    *pnode = node->mNext;   // remove from linked list
                                    FreeNode(node);
                                    break;
                                }
                                pnode = &node->mNext;
                            }
                            // dispose the request node
                            FreeNode(event);
                            break;
                        case kClear:
                            for (node = mActiveList.head(); node != NULL; ) {
                                next = node->mNext;
                                FreeNode(node);
                                node = next;
                            }
                            FreeNode(event);
                            break;
                        default:
                            //printf("invalid node type %d!\n", event->mEventType);
                            break;
                    }
                }
                //mActiveList.dump("active after update");
            }
        */
    }
    
    pub fn begin(&self) -> ThreadSafeListIterator<T> {
        
        todo!();
        /*
            //mActiveList.dump("active at begin");
            return iterator(mActiveList.head());
        */
    }
    
    pub fn end(&self) -> ThreadSafeListIterator<T> {
        
        todo!();
        /*
            return iterator(NULL);
        */
    }
    
    pub fn alloc_node(&mut self) -> *mut ThreadSafeListNode<T> {
        
        todo!();
        /*
            ThreadSafeListNode *node = mFreeList.pop_atomic();
            if (node == NULL)
                node = (ThreadSafeListNode *)CA_malloc(sizeof(ThreadSafeListNode));
            return node;
        */
    }
    
    pub fn free_node(&mut self, node: *mut ThreadSafeListNode<T>)  {
        
        todo!();
        /*
            mFreeList.push_atomic(node);
        */
    }
}
