crate::ix!();

pub trait PerformOnDescription {
    fn perform_on_description(&mut self, _0: &mut PluginDescription) -> Result<(),()>;
}

pub fn get_normalisedtuid(tuid: &TUID) -> [u32; 4] {
    
    todo!();
        /*
            const FUID fuid { tuid };
        return { { fuid.getLong1(), fuid.getLong2(), fuid.getLong3(), fuid.getLong4() } };
        */
}

pub fn get_hash_for_range<Range>(range: Range) -> i32 {

    todo!();
        /*
            uint32 value = 0;

        for (const auto& item : range)
            value = (value * 31) + (uint32) item;

        return (int) value;
        */
}

pub fn fill_description_with<ObjectType>(
    description: &mut PluginDescription,
    object:      &mut ObjectType
) {

    todo!();
        /*
            description.version  = toString (object.version).trim();
        description.category = toString (object.subCategories).trim();

        if (description.manufacturerName.trim().isEmpty())
            description.manufacturerName = toString (object.vendor).trim();
        */
}

pub fn create_plugin_description(
    description: &mut PluginDescription,
    plugin_file: &File,
    company:     &String,
    name:        &String,
    info:        &PClassInfo,
    info2:       *mut PClassInfo2,
    infow:       *mut PClassInfoW,
    num_inputs:  i32,
    num_outputs: i32

) {
    
    todo!();
        /*
            description.fileOrIdentifier    = pluginFile.getFullPathName();
        description.lastFileModTime     = pluginFile.getLastModificationTime();
        description.lastInfoUpdateTime  = Time::getCurrentTime();
        description.manufacturerName    = company;
        description.name                = name;
        description.descriptiveName     = name;
        description.pluginFormatName    = "Vst3";
        description.numInputChannels    = numInputs;
        description.numOutputChannels   = numOutputs;

        description.deprecatedUid       = getHashForRange (info.cid);
        description.uniqueId            = getHashForRange (getNormalisedTUID (info.cid));

        if (infoW != nullptr)      fillDescriptionWith (description, *infoW);
        else if (info2 != nullptr) fillDescriptionWith (description, *info2);

        if (description.category.isEmpty())
            description.category = toString (info.category).trim();

        description.isInstrument = description.category.containsIgnoreCase ("Instrument"); // This seems to be the only way to find that out! ARGH!
        */
}

pub fn get_num_single_direction_buses_for(
    component:            *mut dyn VstIComponent,
    check_inputs:         bool,
    check_audio_channels: bool

) -> i32 {
    
    todo!();
        /*
            jassert (component != nullptr);

        return (int) component->getBusCount (checkAudioChannels ? VstkAudio : VstkEvent,
                                             checkInputs ? VstkInput : VstkOutput);
        */
}

/**
  | Gives the total number of channels for
  | a particular type of bus direction and
  | media type
  |
  */
pub fn get_num_single_direction_channels_for(
    component:            *mut dyn VstIComponent,
    check_inputs:         bool,
    check_audio_channels: bool

) -> i32 {

    todo!();
        /*
            jassert (component != nullptr);

        const VstBusDirections direction  = checkInputs ? VstkInput : VstkOutput;
        const VstMediaTypes mediaType     = checkAudioChannels ? VstkAudio : VstkEvent;
        const i32 numBuses     = component->getBusCount (mediaType, direction);

        int numChannels = 0;

        for (i32 i = numBuses; --i >= 0;)
        {
            VstBusInfo busInfo;
            warnOnFailure (component->getBusInfo (mediaType, direction, i, busInfo));
            numChannels += ((busInfo.flags & VstBusInfo::kDefaultActive) != 0 ? (int) busInfo.channelCount : 0);
        }

        return numChannels;
        */
}

pub fn set_state_for_all_buses_of_type(
    component:               *mut dyn VstIComponent,
    state:                   bool,
    activate_inputs:         bool,
    activate_audio_channels: bool

) {

    todo!();
        /*
            jassert (component != nullptr);

        const VstBusDirections direction  = activateInputs ? VstkInput : VstkOutput;
        const VstMediaTypes mediaType     = activateAudioChannels ? VstkAudio : VstkEvent;
        const i32 numBuses     = component->getBusCount (mediaType, direction);

        for (i32 i = numBuses; --i >= 0;)
            warnOnFailure (component->activateBus (mediaType, direction, i, state));
        */
}
