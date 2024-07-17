crate::ix!();

pub enum MidiEventListEventConversionKind
{
    /**
      | Hosted plugins don't expect to receive
      | LegacyMIDICCEvents messages from the
      | host, so if we're converting midi from
      | the host to an eventlist, this mode
      | will avoid converting to Legacy events
      | where possible.
      */
    hostToPlugin,

    /**
      | If plugins generate MIDI internally,
      | then where possible we should preserve
      | these messages as LegacyMIDICCOut
      | events.
      */
    pluginToHost
}

///-------------------------
#[derive(Default)]
pub struct MidiEventListBasicOptional<Item> {
    item:     Item,
    is_valid: bool,
}

impl<Item> MidiEventListBasicOptional<Item> {

    pub fn new(i: &Item) -> Self {
    
        todo!();
        /*
            : item { i }, isValid { true }
        */
    }
}

pub struct Vst3MidiControlEvent
{
    controller_number: CtrlNumber,
    param_value:       ParamValue,
}

#[derive(Default)]
#[ALOE_DECLARE_Vst3_COM_REF_METHODS]
#[ALOE_DECLARE_Vst3_COM_QUERY_METHODS]
#[no_copy]
#[leak_detector]
pub struct MidiEventList {
    events:    Vec<Arc<Mutex<VstEvent>>>,
    ref_count: Atomic<i32>,
}

impl IEventList for MidiEventList {

    #[PLUGIN_API]
    fn get_event_count(&mut self) -> i32 {
        
        todo!();
        /*
            return (typename i32) events.size();
        */
    }

    /**
      | NB: This has to cope with out-of-range
      | indexes from some plugins.
      |
      */
    #[PLUGIN_API]
    fn get_event(&mut self, 
        index: i32,
        e:     &mut VstEvent) -> tresult {
        
        todo!();
        /*
            if (isPositiveAndBelow ((int) index, events.size()))
            {
                e = events.getReference ((int) index);
                return typename kResultTrue;
            }

            return typename kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn add_event(&mut self, e: &mut VstEvent) -> tresult {
        
        todo!();
        /*
            events.add (e);
            return typename kResultTrue;
        */
    }
}

impl FUnknown for MidiEventList {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut aloe_deps::c_void) -> i32 { todo!() }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl MidiEventList {

    pub fn clear(&mut self)  {
        
        todo!();
        /*
            events.clearQuick();
        */
    }
    
    pub fn to_midi_buffer(
        result:     &mut MidiBuffer,
        event_list: &mut dyn IEventList)  {
        
        todo!();
        /*
            const auto numEvents = eventList.getEventCount();

            for (typename i32 i = 0; i < numEvents; ++i)
            {
                typename VstEvent e;

                if (eventList.getEvent (i, e) != typename kResultOk)
                    continue;

                const auto message = toMidiMessage (e);

                if (message.isValid)
                    result.addEvent (message.item, e.sampleOffset);
            }
        */
    }
    
    pub fn host_to_plugin_event_list(
        result:            &mut dyn IEventList,
        midi_buffer:       &mut MidiBuffer,
        parameter_changes: *mut dyn IParameterChanges,
        midi_mapping:      &StoredMidiMapping)  {
        
        todo!();
        /*
            toEventList (result,
                         midiBuffer,
                         parameterChanges,
                         &midiMapping,
                         MidiEventListEventConversionKind::hostToPlugin);
        */
    }
    
    pub fn plugin_to_host_event_list(
        result:      &mut dyn IEventList,
        midi_buffer: &mut MidiBuffer)  {
        
        todo!();
        /*
            toEventList (result,
                         midiBuffer,
                         nullptr,
                         nullptr,
                         MidiEventListEventConversionKind::pluginToHost);
        */
    }
    
    pub fn to_event_list(
        result:            &mut dyn IEventList,
        midi_buffer:       &mut MidiBuffer,
        parameter_changes: *mut dyn IParameterChanges,
        midi_mapping:      *const StoredMidiMapping,
        kind:              MidiEventListEventConversionKind)  {
        
        todo!();
        /*
            enum { maxNumEvents = 2048 }; // Steinberg's Host Checker states that no more than 2048 events are allowed at once
            int numEvents = 0;

            for (const auto metadata : midiBuffer)
            {
                if (++numEvents > maxNumEvents)
                    break;

                auto msg = metadata.getMessage();

                if (midiMapping != nullptr && parameterChanges != nullptr)
                {
                    Vst3MidiControlEvent controlEvent;

                    if (toVst3ControlEvent (msg, controlEvent))
                    {
                        const auto controlParamID = midiMapping->getMapping (createSafeChannel (msg.getChannel()),
                                                                             controlEvent.controllerNumber);

                        if (controlParamID != typename VstkNoParamId)
                        {
                            typename i32 ignore;

                            if (auto* queue = parameterChanges->addParameterData (controlParamID, ignore))
                                queue->addPoint (metadata.samplePosition, controlEvent.paramValue, ignore);
                        }

                        continue;
                    }
                }

                auto maybeEvent = createVstEvent (msg, metadata.data, kind);

                if (! maybeEvent.isValid)
                    continue;

                auto& e = maybeEvent.item;
                e.busIndex = 0;
                e.sampleOffset = metadata.samplePosition;
                result.addEvent (e);
            }
        */
    }
    
    pub fn create_safe_channel(channel: i32) -> i16 {
        
        todo!();
        /*
            return (typename i16) jlimit (0, 15, channel - 1);
        */
    }
    
    pub fn create_safe_channel_i16(channel: i16) -> i32 {
        
        todo!();
        /*
            return (int) jlimit (1, 16, channel + 1);
        */
    }
    
    pub fn create_safe_note(note: i32) -> i16 {
        
        todo!();
        /*
            return (typename i16) jlimit (0, 127, note);
        */
    }
    
    pub fn create_safe_note_i16(note: i16) -> i32 {
        
        todo!();
        /*
            return jlimit (0, 127, (int) note);
        */
    }
    
    pub fn normalise_midi_value(value: i32) -> f32 {
        
        todo!();
        /*
            return jlimit (0.0f, 1.0f, (float) value / 127.0f);
        */
    }
    
    pub fn denormalise_to_midi_value(value: f32) -> i32 {
        
        todo!();
        /*
            return roundToInt (jlimit (0.0f, 127.0f, value * 127.0f));
        */
    }
    
    pub fn create_note_on_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            typename VstEvent e{};
            e.type              = typename VstEvent::kNoteOnEvent;
            e.noteOn.channel    = createSafeChannel (msg.getChannel());
            e.noteOn.pitch      = createSafeNote (msg.getNoteNumber());
            e.noteOn.velocity   = normaliseMidiValue (msg.getVelocity());
            e.noteOn.length     = 0;
            e.noteOn.tuning     = 0.0f;
            e.noteOn.noteId     = -1;
            return e;
        */
    }
    
    pub fn create_note_off_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            typename VstEvent e{};
            e.type              = typename VstEvent::kNoteOffEvent;
            e.noteOff.channel   = createSafeChannel (msg.getChannel());
            e.noteOff.pitch     = createSafeNote (msg.getNoteNumber());
            e.noteOff.velocity  = normaliseMidiValue (msg.getVelocity());
            e.noteOff.tuning    = 0.0f;
            e.noteOff.noteId    = -1;
            return e;
        */
    }
    
    pub fn create_sys_ex_event(
        msg:             &MidiMessage,
        midi_event_data: *const u8) -> VstEvent {
        
        todo!();
        /*
            typename VstEvent e{};
            e.type          = typename VstEvent::kDataEvent;
            e.data.bytes    = midiEventData + 1;
            e.data.size     = (uint32) msg.getSysExDataSize();
            e.data.type     = typename VstDataEvent::kMidiSysEx;
            return e;
        */
    }
    
    pub fn create_legacy_midi_event(
        channel:        i32,
        control_number: i32,
        value:          i32,
        value2:         Option<i32>

    ) -> VstEvent {

        let value2: i32 = value2.unwrap_or(0);

        todo!();
        /*
            typename VstEvent e{};
            e.type                      = typename VstEvent::kLegacyMIDICCOutEvent;
            e.midiCCOut.channel         = typename int8 (createSafeChannel (channel));
            e.midiCCOut.controlNumber   = uint8 (jlimit (0, 255, controlNumber));
            e.midiCCOut.value           = typename int8 (createSafeNote (value));
            e.midiCCOut.value2          = typename int8 (createSafeNote (value2));
            return e;
        */
    }
    
    pub fn create_poly_pressure_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            typename VstEvent e{};
            e.type                      = typename VstEvent::kPolyPressureEvent;
            e.polyPressure.channel      = createSafeChannel (msg.getChannel());
            e.polyPressure.pitch        = createSafeNote (msg.getNoteNumber());
            e.polyPressure.pressure     = normaliseMidiValue (msg.getAfterTouchValue());
            e.polyPressure.noteId       = -1;
            return e;
        */
    }
    
    pub fn create_channel_pressure_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          typename VstkAfterTouch,
                                          msg.getChannelPressureValue());
        */
    }
    
    pub fn create_controller_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          msg.getControllerNumber(),
                                          msg.getControllerValue());
        */
    }
    
    pub fn create_ctrl_poly_pressure_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          typename VstkCtrlPolyPressure,
                                          msg.getNoteNumber(),
                                          msg.getAfterTouchValue());
        */
    }
    
    pub fn create_pitch_wheel_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          typename VstkPitchBend,
                                          msg.getRawData()[1],
                                          msg.getRawData()[2]);
        */
    }
    
    pub fn create_program_change_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          typename VstkCtrlProgramChange,
                                          msg.getProgramChangeNumber());
        */
    }
    
    pub fn create_ctrl_quarter_frame_event(msg: &MidiMessage) -> VstEvent {
        
        todo!();
        /*
            return createLegacyMIDIEvent (msg.getChannel(),
                                          typename VstkCtrlQuarterFrame,
                                          msg.getQuarterFrameValue());
        */
    }
    
    pub fn create_vst_event(
        msg:             &MidiMessage,
        midi_event_data: *const u8,
        kind:            MidiEventListEventConversionKind) -> MidiEventListBasicOptional<VstEvent> {
        
        todo!();
        /*
            if (msg.isNoteOn())
                return createNoteOnEvent (msg);

            if (msg.isNoteOff())
                return createNoteOffEvent (msg);

            if (msg.isSysEx())
                return createSysExEvent (msg, midiEventData);

            if (msg.isChannelPressure())
                return createChannelPressureEvent (msg);

            if (msg.isPitchWheel())
                return createPitchWheelEvent (msg);

            if (msg.isProgramChange())
                return createProgramChangeEvent (msg);

            if (msg.isController())
                return createControllerEvent (msg);

            if (msg.isQuarterFrame())
                return createCtrlQuarterFrameEvent (msg);

            if (msg.isAftertouch())
            {
                switch (kind)
                {
                    case MidiEventListEventConversionKind::hostToPlugin:
                        return createPolyPressureEvent (msg);

                    case MidiEventListEventConversionKind::pluginToHost:
                        return createCtrlPolyPressureEvent (msg);
                }

                jassertfalse;
                return {};
            }

            return {};
        */
    }
    
    
    pub fn to_vst_3control_event(
        msg:    &MidiMessage,
        result: &mut Vst3MidiControlEvent) -> bool {
        
        todo!();
        /*
            if (msg.isController())
            {
                result = { (typename VstCtrlNumber) msg.getControllerNumber(), msg.getControllerValue() / 127.0};
                return true;
            }

            if (msg.isPitchWheel())
            {
                result = { typename VstkPitchBend, msg.getPitchWheelValue() / 16383.0};
                return true;
            }

            if (msg.isChannelPressure())
            {
                result = { typename VstkAfterTouch, msg.getChannelPressureValue() / 127.0};
                return true;
            }

            result.controllerNumber = -1;
            return false;
        */
    }
}

impl From<&LegacyMIDICCOutEvent> for MidiEventListBasicOptional<MidiMessage> {

    fn from(e: &LegacyMIDICCOutEvent) -> MidiEventListBasicOptional<MidiMessage> {
        
        todo!();
        /*
            if (e.controlNumber <= 127)
                return MidiMessage::controllerEvent (createSafeChannel (i16 (e.channel)),
                                                     createSafeNote (i16 (e.controlNumber)),
                                                     createSafeNote (i16 (e.value)));

            switch (e.controlNumber)
            {
                case typename VstkAfterTouch:
                    return MidiMessage::channelPressureChange (createSafeChannel (i16 (e.channel)),
                                                               createSafeNote (i16 (e.value)));

                case typename VstkPitchBend:
                    return MidiMessage::pitchWheel (createSafeChannel (i16 (e.channel)),
                                                    (e.value & 0x7f) | ((e.value2 & 0x7f) << 7));

                case typename VstkCtrlProgramChange:
                    return MidiMessage::programChange (createSafeChannel (i16 (e.channel)),
                                                       createSafeNote (i16 (e.value)));

                case typename VstkCtrlQuarterFrame:
                    return MidiMessage::quarterFrame (createSafeChannel (i16 (e.channel)),
                                                      createSafeNote (i16 (e.value)));

                case typename VstkCtrlPolyPressure:
                    return MidiMessage::aftertouchChange (createSafeChannel (i16 (e.channel)),
                                                          createSafeNote (i16 (e.value)),
                                                          createSafeNote (i16 (e.value2)));

                default:
                    // If this is hit, we're trying to convert a LegacyMIDICCOutEvent with an unknown controlNumber.
                    jassertfalse;
                    return {};
            }
        */
    }
}

impl From<&VstEvent> for MidiEventListBasicOptional<MidiMessage> {
    
    fn from(e: &VstEvent) -> MidiEventListBasicOptional<MidiMessage> {
        
        todo!();
        /*
            switch (e.type)
            {
                case typename VstEvent::kNoteOnEvent:
                    return MidiMessage::noteOn (createSafeChannel (e.noteOn.channel),
                                                createSafeNote (e.noteOn.pitch),
                                                (typename uint8) denormaliseToMidiValue (e.noteOn.velocity));

                case typename VstEvent::kNoteOffEvent:
                    return MidiMessage::noteOff (createSafeChannel (e.noteOff.channel),
                                                 createSafeNote (e.noteOff.pitch),
                                                 (typename uint8) denormaliseToMidiValue (e.noteOff.velocity));

                case typename VstEvent::kPolyPressureEvent:
                    return MidiMessage::aftertouchChange (createSafeChannel (e.polyPressure.channel),
                                                          createSafeNote (e.polyPressure.pitch),
                                                          (typename uint8) denormaliseToMidiValue (e.polyPressure.pressure));

                case typename VstEvent::kDataEvent:
                    return MidiMessage::createSysExMessage (e.data.bytes, (int) e.data.size);

                case typename VstEvent::kLegacyMIDICCOutEvent:
                    return toMidiMessage (e.midiCCOut);

                case typename VstEvent::kNoteExpressionValueEvent:
                case typename VstEvent::kNoteExpressionTextEvent:
                case typename VstEvent::kChordEvent:
                case typename VstEvent::kScaleEvent:
                    return {};

                default:
                    break;
            }

            // If this is hit, we've been sent an event type that doesn't exist in the Vst3 spec.
            jassertfalse;
            return {};
        */
    }
}
