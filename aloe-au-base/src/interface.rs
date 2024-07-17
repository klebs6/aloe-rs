crate::ix!();

pub trait CanScheduleParameters {

    fn can_schedule_parameters(&self) -> bool;
}

pub trait StreamFormatWritable {

    /**
      | scope will always be input or output
      |
      */
    fn stream_format_writable(
        &mut self, 
        scope:   AudioUnitScope,
        element: AudioUnitElement
    ) -> bool;
}

pub trait AUBaseInterface: 
AUPostConstructor 
+ AUPostDestructor 
+ AUCreateExtendedElements 
+ AUInitialize 
+ AUCleanup 
+ AUReset 
+ AUGetPropertyInfo 
+ AUGetProperty 
+ AUSetProperty 
+ AURemovePropertyValue 
+ AUAddPropertyListener 
+ AURemovePropertyListener 
+ AUSetRenderNotification 
+ AURemoveRenderNotification 
+ AUGetParameter 
+ AUSetParameter 
+ AUScheduleParameter 
+ AUProcessBufferLists 
+ AUProcessMultipleBufferLists 
+ AUComplexRender 
+ AURenderBus 
+ AURender 
+ AUBusCountWritable 
+ AUSetBusCount 
+ AUSetConnection 
+ AUSetInputCallback 
+ AUGetParameterList 
+ AUGetParameterInfo 
+ GetParameterHistoryInfo 
+ SaveState 
+ SaveExtendedScopes 
+ RestoreState 
+ GetParameterValueStrings 
+ CopyClumpName 
+ GetPresets 
+ NewFactoryPresetSet 
+ NewCustomPresetSet 
+ GetNumCustomUiComponents 
+ GetUiComponentDescs 
+ CopyIconLocation 
+ GetLatency 
+ GetTailTime 
+ SupportsTail 
+ SupportedNumChannels 
+ ValidFormat 
+ GetStreamFormat 
+ ChangeStreamFormat 
+ GetScopeExtended 
+ PropertyChanged 
+ CreateElement 
+ Start 
+ Stop 
+ PrepareInstrument 
+ ReleaseInstrument 
+ MidiEvent 
+ SysEx 
+ StartNote 
+ StopNote 
+ ReallocateBuffers 
+ DeallocateIoBuffers 
+ SetMaxFramesPerSlice 
+ CanSetMaxFrames 
+ GetChannelLayoutTags 
+ GetAudioChannelLayout 
+ SetAudioChannelLayout 
+ RemoveAudioChannelLayout 
+ ProcessForScheduledParams 
+ ProcessScheduledSlice 
{ }

/*
   | Virtual methods (mostly) directly
   | corresponding to the entry points.  Many of
   | these have useful implementations here and
   | will not need overriding.
   */

pub trait AUInitialize {

    /**
      | ... so that overrides to this
      | method can assume that they
      | will only be called exactly
      | once.
      */
    fn initialize(&mut self) -> OSStatus { 
        todo!();
    }
}
    
pub trait AUCleanup {

    fn cleanup(&mut self)  {
        
    }
}
    
pub trait AUReset {

    fn reset(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus 
    {
        todo!();
    }
}
    
pub trait AUGetPropertyInfo {

    fn get_property_info(&mut self, 
        inid:          AudioUnitPropertyID,
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        out_data_size: &mut u32,
        out_writable:  &mut bool) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUGetProperty {

    fn get_property(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_data:   *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUSetProperty {

    fn set_property(&mut self, 
        inid:         AudioUnitPropertyID,
        in_scope:     AudioUnitScope,
        in_element:   AudioUnitElement,
        in_data:      *const c_void,
        in_data_size: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait AURemovePropertyValue {
    
    fn remove_property_value(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUAddPropertyListener {

    fn add_property_listener(&mut self, 
        inid:            AudioUnitPropertyID,
        in_proc:         AudioUnitPropertyListenerProc,
        in_proc_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AURemovePropertyListener {

    fn remove_property_listener(&mut self, 
        inid:              AudioUnitPropertyID,
        in_proc:           AudioUnitPropertyListenerProc,
        in_proc_ref_con:   *mut c_void,
        ref_con_specified: bool) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUSetRenderNotification {

    fn set_render_notification(&mut self, 
        in_proc:    AURenderCallback,
        in_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AURemoveRenderNotification {

    fn remove_render_notification(&mut self, 
        in_proc:    AURenderCallback,
        in_ref_con: *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUGetParameter {

    fn get_parameter(&mut self, 
        inid:       AudioUnitParameterID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement,
        out_value:  &mut AudioUnitParameterValue) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUSetParameter {

    fn set_parameter(&mut self, 
        inid:                       AudioUnitParameterID,
        in_scope:                   AudioUnitScope,
        in_element:                 AudioUnitElement,
        in_value:                   AudioUnitParameterValue,
        in_buffer_offset_in_frames: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUScheduleParameter {

    fn schedule_parameter(&mut self, 
        in_parameter_event: *const AudioUnitParameterEvent,
        in_num_events:      u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUProcessBufferLists {

    fn process_buffer_lists(&mut self, 
        io_action_flags:      &mut AudioUnitRenderActionFlags,
        in_buffer:            &AudioBufferList,
        out_buffer:           &mut AudioBufferList,
        in_frames_to_process: u32) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait AUProcessMultipleBufferLists {

    fn process_multiple_buffer_lists(&mut self, 
        io_action_flags:               &mut AudioUnitRenderActionFlags,
        in_frames_to_process:          u32,
        in_number_input_buffer_lists:  u32,
        in_input_buffer_lists:         *const *const AudioBufferList,
        in_number_output_buffer_lists: u32,
        io_output_buffer_lists:        *mut *mut AudioBufferList) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait AUComplexRender {

    fn complex_render(&mut self, 
        io_action_flags:         &mut AudioUnitRenderActionFlags,
        in_time_stamp:           &AudioTimeStamp,
        in_output_bus_number:    u32,
        in_number_of_packets:    u32,
        out_number_of_packets:   *mut u32,
        out_packet_descriptions: *mut AudioStreamPacketDescription,
        io_data:                 &mut AudioBufferList,
        out_metadata:            *mut c_void,
        out_metadata_byte_size:  *mut u32) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}

pub trait AURenderBus {

    /**
      | Override this method if your AU processes
      | multiple output busses completely
      | independently -- you'll want to just call
      | Render without the NeedsToRender check.
      | Otherwise, override Render().
      |
      | N.B. Implementations of this method can
      | assume that the output's buffer list has
      | already been prepared and access it with
      | GetOutput(inBusNumber)->GetBufferList()
      | instead of
      | GetOutput(inBusNumber)->PrepareBuffer(nFrames)
      | -- if PrepareBuffer is called, a copy may
      | occur after rendering.
      */
    fn render_bus(&mut self, 
        io_action_flags:  &mut AudioUnitRenderActionFlags,
        in_time_stamp:    &AudioTimeStamp,
        in_bus_number:    u32,
        in_number_frames: u32) -> OSStatus {
        
        todo!();
        /*
            if (NeedsToRender(inTimeStamp))
                return Render(ioActionFlags, inTimeStamp, inNumberFrames);
            return noErr;   // was presumably already rendered via another bus
        */
    }
}

pub trait AURender {

    /**
      | N.B. For a unit with only one output bus,
      | it can assume in its implementation of this
      | method that the output's buffer list has
      | already been prepared and access it with
      | GetOutput(0)->GetBufferList() instead of
      | GetOutput(0)->PrepareBuffer(nFrames) -- if
      | PrepareBuffer is called, a copy may occur
      | after rendering.
      */
    fn render(&mut self, 
        io_action_flags:  &mut AudioUnitRenderActionFlags,
        in_time_stamp:    &AudioTimeStamp,
        in_number_frames: u32) -> OSStatus {
        
        todo!();
        /*
            return noErr;
        */
    }
}

pub trait AUBusCountWritable {

    /**
      | These are generated from
      | DispatchGetProperty/DispatchGetPropertyInfo/DispatchSetProperty
      |
      */
    fn bus_count_writable(&mut self, in_scope: AudioUnitScope) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}
    
pub trait AUSetBusCount {

    fn set_bus_count(&mut self, 
        in_scope: AudioUnitScope,
        in_count: u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUSetConnection {

    fn set_connection(&mut self, in_connection: &AudioUnitConnection) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUSetInputCallback {

    fn set_input_callback(&mut self, 
        in_propertyid: u32,
        in_element:    AudioUnitElement,
        in_proc:       AURenderCallback,
        in_ref_con:    *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait AUGetParameterList {

    /**
      | outParameterList may be a null pointer
      |
      */
    fn get_parameter_list(&mut self, 
        in_scope:           AudioUnitScope,
        out_parameter_list: *mut AudioUnitParameterID,
        out_num_parameters: &mut u32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait AUGetParameterInfo {

    fn get_parameter_info(&mut self, 
        in_scope:           AudioUnitScope,
        in_parameterid:     AudioUnitParameterID,
        out_parameter_info: &mut AudioUnitParameterInfo) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetParameterHistoryInfo {

    fn get_parameter_history_info(&mut self, 
        in_scope:                        AudioUnitScope,
        in_parameterid:                  AudioUnitParameterID,
        out_updates_per_second:          &mut Float32,
        out_history_duration_in_seconds: &mut Float32) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait SaveState {

    fn save_state(&mut self, out_data: *mut CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait SaveExtendedScopes {

    fn save_extended_scopes(&mut self, out_data: CFMutableDataRef)  { }
}
    
pub trait RestoreState {

    fn restore_state(&mut self, in_data: CFPropertyListRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetParameterValueStrings {

    fn get_parameter_value_strings(&mut self, 
        in_scope:       AudioUnitScope,
        in_parameterid: AudioUnitParameterID,
        out_strings:    *mut CFArrayRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CopyClumpName {

    fn copy_clump_name(&mut self, 
        in_scope:               AudioUnitScope,
        in_clumpid:             u32,
        in_desired_name_length: u32,
        out_clump_name:         *mut CFStringRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetPresets {

    fn get_presets(&self, out_data: *mut CFArrayRef) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait NewFactoryPresetSet {

    /**
      | Called when someone sets a new, valid
      | preset
      |
      | If this is a valid preset, then the
      | subclass sets its state to that preset
      | and returns noErr.
      |
      | If not a valid preset, return an error,
      | and the pre-existing preset is restored
      */
    fn new_factory_preset_set(&mut self, in_new_factory_preset: &AUPreset) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait NewCustomPresetSet {

    fn new_custom_preset_set(&mut self, in_new_custom_preset: &AUPreset) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetNumCustomUiComponents {

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    fn get_num_custom_ui_components(&mut self) -> i32 {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetUiComponentDescs {

    #[cfg(not(CA_USE_AUDIO_PLUGIN_ONLY))]
    fn get_ui_component_descs(&mut self, in_desc_array: *mut ComponentDescription)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CopyIconLocation {

    fn copy_icon_location(&mut self) -> CFURLRef {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetLatency {

    /**
      | default is no latency, and unimplemented
      | tail time
      |
      */
    fn get_latency(&mut self) -> f64 {
        
        todo!();
        /*
            return 0.0;
        */
    }
}
    
pub trait GetTailTime {

    fn get_tail_time(&mut self) -> f64 {
        
        todo!();
        /*
            return 0;
        */
    }
}
    
pub trait SupportsTail {

    fn supports_tail(&mut self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
}

pub trait SupportedNumChannels {

    /**
      | pass in a pointer to get the
      | struct, and num channel infos you
      | can pass in NULL to just get the
      | number a return value of 0 (the
      | default in AUBase) means the
      | property is not supported...
      */
    fn supported_num_channels(&mut self, out_info: *const *const AUChannelInfo) -> u32 {
        
        todo!();
        /*
        
        */
    }
}

pub trait ValidFormat {

    /**
      | Will only be called after
      | StreamFormatWritable has succeeded.
      |
      | Default implementation requires
      | canonical format: native-endian
      | 32-bit float, any sample rate, any
      | number of channels; override when
      | other formats are supported.
      | A subclass's override can choose to
      | always return true and trap invalid
      | formats in ChangeStreamFormat.
      */
    fn valid_format(&mut self, 
        in_scope:      AudioUnitScope,
        in_element:    AudioUnitElement,
        in_new_format: &CAStreamBasicDescription) -> bool {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetStreamFormat {

    fn get_stream_format(&mut self, 
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement) -> &CAStreamBasicDescription {
        
        todo!();
        /*
        
        */
    }
}
pub trait ChangeStreamFormat {

    /**
      | Will only be called after
      | StreamFormatWritable and ValidFormat
      | have succeeded.
      |
      */
    fn change_stream_format(&mut self, 
        in_scope:       AudioUnitScope,
        in_element:     AudioUnitElement,
        in_prev_format: &CAStreamBasicDescription,
        in_new_format:  &CAStreamBasicDescription) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetScopeExtended {

    fn get_scope_extended(&mut self, in_scope: AudioUnitScope) -> *mut AUScope {
        
        todo!();
        /*
            return NULL;
        */
    }
}
    
pub trait PropertyChanged {

    fn property_changed(&mut self, 
        inid:       AudioUnitPropertyID,
        in_scope:   AudioUnitScope,
        in_element: AudioUnitElement)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CreateElement {

    fn create_element(&mut self, 
        scope:   AudioUnitScope,
        element: AudioUnitElement) -> *mut AUElement {
        
        todo!();
        /*
        
        */
    }
}

pub trait Start {

    /**
      | output unit methods
      |
      */
    fn start(&mut self) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait Stop {

    fn stop(&mut self) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}

pub trait PrepareInstrument {

    /**
      | these methods are deprecated, so we
      | don't include them except for compatability
      |
      */
    #[cfg(not(CA_BASIC_AU_FEATURES))]
    #[cfg(not(TARGET_OS_IPHONE))]
    fn prepare_instrument(&mut self, in_instrument: MusicDeviceInstrumentID) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait ReleaseInstrument {

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    #[cfg(not(TARGET_OS_IPHONE))]
    fn release_instrument(&mut self, in_instrument: MusicDeviceInstrumentID) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}

pub trait MidiEvent {

    /**
      | music device/music effect methods
      | -- incomplete
      |
      */
    #[cfg(not(CA_BASIC_AU_FEATURES))]
    fn midi_event(&mut self, 
        in_status:              u32,
        in_data1:               u32,
        in_data2:               u32,
        in_offset_sample_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait SysEx {

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    fn sys_ex(&mut self, 
        in_data:   *const UInt8,
        in_length: u32) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait StartNote {

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    fn start_note(&mut self, 
        in_instrument:          MusicDeviceInstrumentID,
        in_groupid:             MusicDeviceGroupID,
        out_note_instanceid:    *mut NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params:              &MusicDeviceNoteParams) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}
    
pub trait StopNote {

    #[cfg(not(CA_BASIC_AU_FEATURES))]
    fn stop_note(&mut self, 
        in_groupid:             MusicDeviceGroupID,
        in_note_instanceid:     NoteInstanceID,
        in_offset_sample_frame: u32) -> OSStatus {
        
        todo!();
        /*
            return kAudio_UnimplementedError;
        */
    }
}

pub trait ReallocateBuffers {

    /**
      | needs to be called when mMaxFramesPerSlice
      | changes
      |
      */
    fn reallocate_buffers(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait DeallocateIoBuffers {

    fn deallocate_io_buffers(&mut self)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait SetMaxFramesPerSlice {

    fn set_max_frames_per_slice(&mut self, n_frames: u32)  {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait CanSetMaxFrames {

    fn can_set_max_frames(&self) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetChannelLayoutTags {

    fn get_channel_layout_tags(&mut self, 
        scope:           AudioUnitScope,
        element:         AudioUnitElement,
        out_layout_tags: *mut AudioChannelLayoutTag) -> u32 {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait GetAudioChannelLayout {

    fn get_audio_channel_layout(&mut self, 
        scope:          AudioUnitScope,
        element:        AudioUnitElement,
        out_layout_ptr: *mut AudioChannelLayout,
        out_writable:   &mut bool) -> u32 {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait SetAudioChannelLayout {

    fn set_audio_channel_layout(&mut self, 
        scope:     AudioUnitScope,
        element:   AudioUnitElement,
        in_layout: *const AudioChannelLayout) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}
    
pub trait RemoveAudioChannelLayout {

    fn remove_audio_channel_layout(&mut self, 
        scope:   AudioUnitScope,
        element: AudioUnitElement) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait ProcessForScheduledParams {

    /**
      | Usually, you won't override this method.
      | You only need to call this if your DSP code
      | is prepared to handle scheduled immediate
      | and ramped parameter changes.
      |
      | Before calling this method, it is assumed
      | you have already called PullInput() on the
      | input busses for which the DSP code
      | depends.  ProcessForScheduledParams() will
      | call (potentially repeatedly) virtual
      | method ProcessScheduledSlice() to perform
      | the actual DSP for a given sub-division of
      | the buffer.  The job of
      | ProcessForScheduledParams() is to
      | sub-divide the buffer into smaller pieces
      | according to the scheduled times found in
      | the ParameterEventList (usually coming
      | directly from a previous call to
      | ScheduleParameter() ), setting the
      | appropriate immediate or ramped parameter
      | values for the corresponding scopes and
      | elements, then calling
      | ProcessScheduledSlice() to do the actual
      | DSP for each of these divisions.
      */
    fn process_for_scheduled_params(&mut self, 
        in_param_list:        &mut ParameterEventList,
        in_frames_to_process: u32,
        in_user_data:         *mut c_void) -> OSStatus {
        
        todo!();
        /*
        
        */
    }
}

pub trait ProcessScheduledSlice {

    /**
      |  This method is called (potentially
      |  repeatedly) by ProcessForScheduledParams()
      |  in order to perform the actual DSP
      |  required for this portion of the entire
      |  buffer being processed.  The entire buffer
      |  can be divided up into smaller "slices"
      |  according to the timestamps on the
      |  scheduled parameters...
      |
      |  sub-classes wishing to handle scheduled
      |  parameter changes should override this
      |  method in order to do the appropriate DSP.
      |  AUEffectBase already overrides this for
      |  standard effect AudioUnits.
      */
    fn process_scheduled_slice(&mut self, 
        in_user_data:               *mut c_void,
        in_start_frame_in_buffer:   u32,
        in_slice_frames_to_process: u32,
        in_total_buffer_frames:     u32) -> OSStatus {

        // default impl does nothing...
        
        todo!();
        /*
            return noErr;}{
        */
    }
}
