crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_plugin_client/utility/aloe_FakeMouseMoveGenerator.h]

/**
  | Helper class to workaround windows
  | not getting mouse-moves...
  |
  */
#[cfg(target_os="macos")]
pub struct FakeMouseMoveGenerator<'a> {
    base:               Timer,
    last_screen_pos:    Point<f32>,
    safe_old_component: WeakReference<Component<'a>>,
}

#[cfg(target_os="macos")]
impl<'a> Default for FakeMouseMoveGenerator<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            startTimer (1000 / 30);
        */
    }
}

#[cfg(target_os="macos")]
impl<'a> FakeMouseMoveGenerator<'a> {

    pub fn component_contains_audio_processor_editor(comp: *mut Component<'a>) -> bool {
        
        todo!();
        /*
            if (dynamic_cast<AudioProcessorEditor*> (comp) != nullptr)
                return true;

            for (auto* child : comp->getChildren())
                if (componentContainsAudioProcessorEditor (child))
                    return true;

            return false;
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            // Workaround for windows not getting mouse-moves...
            auto screenPos = Desktop::getInstance().getMainMouseSource().getScreenPosition();

            if (screenPos != lastScreenPos)
            {
                lastScreenPos = screenPos;
                auto mods = ModifierKeys::currentModifiers;

                if (! mods.isAnyMouseButtonDown())
                {
                    if (auto* comp = Desktop::getInstance().findComponentAt (screenPos.roundToInt()))
                    {
                        if (componentContainsAudioProcessorEditor (comp->getTopLevelComponent()))
                        {
                            safeOldComponent = comp;

                            if (auto* peer = comp->getPeer())
                            {
                                if (! peer->isFocused())
                                {
                                    peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse,
                                                            peer->globalToLocal (Desktop::getInstance().getMainMouseSource().getRawScreenPosition()),
                                                            mods,
                                                            MouseInputSource::invalidPressure,
                                                            MouseInputSource::invalidOrientation,
                                                            Time::currentTimeMillis());
                                }
                            }

                            return;
                        }
                    }

                    if (safeOldComponent != nullptr)
                    {
                        if (auto* peer = safeOldComponent->getPeer())
                        {
                            peer->handleMouseEvent (MouseInputSource::InputSourceType::mouse,
                                                    MouseInputSource::offscreenMousePos,
                                                    mods,
                                                    MouseInputSource::invalidPressure,
                                                    MouseInputSource::invalidOrientation,
                                                    Time::currentTimeMillis());
                        }
                    }

                    safeOldComponent = nullptr;
                }
            }
        */
    }
}

///---------------------------
#[cfg(not(target_os="macos"))]
pub struct FakeMouseMoveGenerator {}
