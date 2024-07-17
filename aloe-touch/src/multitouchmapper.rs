crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_gui_basics/native/aloe_MultiTouchMapper.h]

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct MultiTouchMapper<'a,IDType> {
    current_touches: Vec<MultiTouchMapperTouchInfo<'a,IDType>>,
}

impl<'a,IDType> MultiTouchMapper<'a,IDType> {

    pub fn get_index_of_touch(&mut self, 
        peer:    *mut ComponentPeer,
        touchid: IDType) -> i32 {
        
        todo!();
        /*
            jassert (touchID != 0); // need to rethink this if IDs can be 0!
            MultiTouchMapperTouchInfo info {touchID, peer};

            int touchIndex = currentTouches.indexOf (info);

            if (touchIndex < 0)
            {
                auto emptyTouchIndex = currentTouches.indexOf ({});
                touchIndex = (emptyTouchIndex >= 0 ? emptyTouchIndex : currentTouches.size());

                currentTouches.set (touchIndex, info);
            }

            return touchIndex;
        */

    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            currentTouches.clear();
        */

    }
    
    pub fn clear_touch(&mut self, index: i32)  {
        
        todo!();
        /*
            currentTouches.set (index, {});
        */

    }
    
    pub fn are_any_touches_active(&self) -> bool {
        
        todo!();
        /*
            for (auto& t : currentTouches)
                if (t.touchId != 0)
                    return true;

            return false;
        */

    }
    
    pub fn delete_all_touches_for_peer(&mut self, peer: *mut ComponentPeer)  {
        
        todo!();
        /*
            for (auto& t : currentTouches)
                if (t.owner == peer)
                    t.touchId = 0;
        */

    }
}
