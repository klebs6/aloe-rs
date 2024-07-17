crate::ix!();

pub struct MouseInputSourceList<'a> {
    base:         Timer,
    sources:      Vec<MouseInputSourceInternal<'a>>,
    source_array: Vec<MouseInputSource<'a>>,
}

impl<'a> Default for MouseInputSourceList<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            #if ALOE_ANDROID || ALOE_IOS
                auto mainMouseInputType = MouseInputSource::MouseInputSourceType::touch;
               #else
                auto mainMouseInputType = MouseInputSource::MouseInputSourceType::mouse;
               #endif

                addSource (0, mainMouseInputType)
        */
    }
}

impl<'a> MouseInputSourceList<'a> {

    pub fn can_use_touch(&mut self) -> bool {
        
        todo!();
        /*
        
        */
    }
    
    pub fn add_source(
        &mut self, 
        index: i32,
        ty:    MouseInputSourceType

    ) -> *mut MouseInputSource {
        
        todo!();
        /*
            auto* s = new MouseInputSourceInternal (index, type);
                sources.add (s);
                sourceArray.add (MouseInputSource (s));

                return &sourceArray.getReference (sourceArray.size() - 1);
        */
    }
    
    pub fn get_mouse_source(&mut self, index: i32) -> *mut MouseInputSource {
        
        todo!();
        /*
            return isPositiveAndBelow (index, sourceArray.size()) ? &sourceArray.getReference (index)
                                                                      : nullptr;
        */
    }
    
    pub fn get_or_create_mouse_input_source(
        &mut self, 
        ty:          MouseInputSourceType,
        touch_index: Option<i32>

    ) -> *mut MouseInputSource {

        let touch_index: i32 = touch_index.unwrap_or(0);

        todo!();
        /*
            if (type == MouseInputSource::MouseInputSourceType::mouse || type == MouseInputSource::MouseInputSourceType::pen)
                {
                    for (auto& m : sourceArray)
                        if (type == m.getType())
                            return &m;

                    addSource (0, type);
                }
                else if (type == MouseInputSource::MouseInputSourceType::touch)
                {
                    jassert (touchIndex >= 0 && touchIndex < 100); // sanity-check on number of fingers

                    for (auto& m : sourceArray)
                        if (type == m.getType() && touchIndex == m.getIndex())
                            return &m;

                    if (canUseTouch())
                        return addSource (touchIndex, type);
                }

                return nullptr;
        */
    }
    
    pub fn get_num_dragging_mouse_sources(&self) -> i32 {
        
        todo!();
        /*
            int num = 0;

                for (auto* s : sources)
                    if (s->isDragging())
                        ++num;

                return num;
        */
    }
    
    pub fn get_dragging_mouse_source(&mut self, index: i32) -> *mut MouseInputSource {
        
        todo!();
        /*
            int num = 0;

                for (auto& s : sourceArray)
                {
                    if (s.isDragging())
                    {
                        if (index == num)
                            return &s;

                        ++num;
                    }
                }

                return nullptr;
        */
    }
    
    pub fn begin_drag_auto_repeat(&mut self, interval: i32)  {
        
        todo!();
        /*
            if (interval > 0)
                {
                    if (getTimerInterval() != interval)
                        startTimer (interval);
                }
                else
                {
                    stopTimer();
                }
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            bool anyDragging = false;

                for (auto* s : sources)
                {
                    // NB: when doing auto-repeat, we need to force an update of the current position and button state,
                    // because on some OSes the queue can get overloaded with messages so that mouse-events don't get through..
                    if (s->isDragging() && ComponentPeer::getCurrentModifiersRealtime().isAnyMouseButtonDown())
                    {
                        s->lastScreenPos = s->getRawScreenPosition();
                        s->triggerFakeMove();
                        anyDragging = true;
                    }
                }

                if (! anyDragging)
                    stopTimer();
        */
    }
    
    #[cfg(target_os="linux")]
    pub fn add_source(&mut self) -> bool {
        
        todo!();
        /*
            if (sources.isEmpty())
        {
            addSource (0, MouseInputSource::InputSourceType::mouse);
            return true;
        }

        return false;
        */

    }
    
    #[cfg(target_os="linux")]
    pub fn can_use_touch(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */

    }
}
