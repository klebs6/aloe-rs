crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/aloe_VSTMidiEventList.h]

/**
  | Holds a set of VSTMidiEvent objects
  | and makes it easy to add events to the
  | list.
  | 
  | This is used by both the VST hosting code
  | and the plugin wrapper.
  | 
  | @tags{Audio}
  |
  */
pub struct VSTMidiEventList {
    events:               HeapBlock<VstEvents>,
    num_events_used:      i32,
    num_events_allocated: i32,
}

impl Default for VSTMidiEventList {
    
    fn default() -> Self {
        todo!();
        /*
        : num_events_used(0),
        : num_events_allocated(0),

        
        */
    }
}

impl Drop for VSTMidiEventList {

    fn drop(&mut self) {
        todo!();
        /*
            freeEvents();
        */
    }
}

impl VSTMidiEventList {
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            numEventsUsed = 0;

            if (events != nullptr)
                events->numEvents = 0;
        */
    }
    
    pub fn add_event(&mut self, 
        midi_data:    *const c_void,
        num_bytes:    i32,
        frame_offset: i32)  {
        
        todo!();
        /*
            ensureSize (numEventsUsed + 1);

            void* const ptr = (typename Vst2::VstMidiEvent*) (events->events [numEventsUsed]);
            auto* const e = (typename Vst2::VstMidiEvent*) ptr;
            events->numEvents = ++numEventsUsed;

            if (numBytes <= 4)
            {
                if (e->type == typename Vst2::kVstSysExType)
                {
                    delete[] (((typename Vst2::VstMidiSysexEvent*) ptr)->sysexDump);
                    e->type = typename Vst2::kVstMidiType;
                    e->byteSize = sizeof (typename Vst2::VstMidiEvent);
                    e->noteLength = 0;
                    e->noteOffset = 0;
                    e->detune = 0;
                    e->noteOffVelocity = 0;
                }

                e->deltaFrames = frameOffset;
                memcpy (e->midiData, midiData, (size_t) numBytes);
            }
            else
            {
                auto* const se = (typename Vst2::VstMidiSysexEvent*) ptr;

                if (se->type == typename Vst2::kVstSysExType)
                    delete[] se->sysexDump;

                se->sysexDump = new char [(size_t) numBytes];
                memcpy (se->sysexDump, midiData, (size_t) numBytes);

                se->type = typename Vst2::kVstSysExType;
                se->byteSize = sizeof (typename Vst2::VstMidiSysexEvent);
                se->deltaFrames = frameOffset;
                se->flags = 0;
                se->dumpBytes = numBytes;
                se->resvd1 = 0;
                se->resvd2 = 0;
            }
        */
    }

    /**
       Handy method to pull the events out of an
       event buffer supplied by the host or
       plugin.
      */
    pub fn add_events_to_midi_buffer(
        events: *const VstEvents,
        dest:   &mut MidiBuffer)  {
        
        todo!();
        /*
            for (int i = 0; i < events->numEvents; ++i)
            {
                const typename Vst2::VstEvent* const e = events->events[i];

                if (e != nullptr)
                {
                    const void* const ptr = events->events[i];

                    if (e->type == typename Vst2::kVstMidiType)
                    {
                        dest.addEvent ((const uint8*) ((const typename Vst2::VstMidiEvent*) ptr)->midiData,
                                       4, e->deltaFrames);
                    }
                    else if (e->type == typename Vst2::kVstSysExType)
                    {
                        const auto* se = (const typename Vst2::VstMidiSysexEvent*) ptr;
                        dest.addEvent ((const uint8*) se->sysexDump,
                                       (int) se->dumpBytes,
                                       e->deltaFrames);
                    }
                }
            }
        */
    }
    
    pub fn ensure_size(&mut self, num_events_needed: i32)  {
        
        todo!();
        /*
            if (numEventsNeeded > numEventsAllocated)
            {
                numEventsNeeded = (numEventsNeeded + 32) & ~31;

                const size_t size = 20 + (size_t) numEventsNeeded * sizeof (typename Vst2::VstEvent*);

                if (events == nullptr)
                    events.calloc (size, 1);
                else
                    events.realloc (size, 1);

                for (int i = numEventsAllocated; i < numEventsNeeded; ++i)
                    events->events[i] = allocateVSTEvent();

                numEventsAllocated = numEventsNeeded;
            }
        */
    }
    
    pub fn free_events(&mut self)  {
        
        todo!();
        /*
            if (events != nullptr)
            {
                for (int i = numEventsAllocated; --i >= 0;)
                    freeVSTEvent (events->events[i]);

                events.free();
                numEventsUsed = 0;
                numEventsAllocated = 0;
            }
        */
    }
    
    pub fn allocate_vst_event() -> *mut VstEvent {
        
        todo!();
        /*
            constexpr auto size = jmax (sizeof (typename Vst2::VstMidiEvent), sizeof (typename Vst2::VstMidiSysexEvent));

            if (auto* e = static_cast<typename Vst2::VstEvent*> (std::calloc (1, size)))
            {
                e->type = typename Vst2::kVstMidiType;
                e->byteSize = sizeof (typename Vst2::VstMidiEvent);
                return e;
            }

            return nullptr;
        */
    }
    
    pub fn free_vst_event(e: *mut VstEvent)  {
        
        todo!();
        /*
            if (e->type == typename Vst2::kVstSysExType)
            {
                delete[] (reinterpret_cast<typename Vst2::VstMidiSysexEvent*> (e)->sysexDump);
            }

            std::free (e);
        */
    }
}

pub struct VstEvent {}
pub struct VstEvents {}
