crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_PluginDescription.h]

/**
  | A small class to represent some facts
  | about a particular type of plug-in.
  | 
  | This class is for storing and managing
  | the details about a plug-in without
  | actually having to load an instance
  | of it.
  | 
  | A KnownPluginList contains a list of
  | PluginDescription objects.
  | 
  | @see KnownPluginList
  | 
  | @tags{Audio}
  |
  */
#[derive(Default)]
#[leak_detector]
pub struct PluginDescription {

    /**
      | The name of the plug-in.
      |
      */
    name:                  String,

    /**
      | A more descriptive name for the plug-in.
      | 
      | This may be the same as the 'name' field,
      | but some plug-ins may provide an alternative
      | name.
      |
      */
    descriptive_name:      String,

    /**
      | The plug-in format, e.g. "Vst", "AudioUnit",
      | etc.
      |
      */
    plugin_format_name:    String,

    /**
      | A category, such as "Dynamics", "Reverbs",
      | etc.
      |
      */
    category:              String,

    /**
      | The manufacturer.
      |
      */
    manufacturer_name:     String,

    /**
      | The version. This string doesn't have
      | any particular format.
      |
      */
    version:               String,

    /**
      | Either the file containing the plug-in
      | module, or some other unique way of identifying
      | it.
      | 
      | E.g. for an AU, this would be an ID string
      | that the component manager could use
      | to retrieve the plug-in. For a Vst, it's
      | the file path.
      |
      */
    file_or_identifier:    String,

    /**
      | The last time the plug-in file was changed.
      | 
      | This is handy when scanning for new or
      | changed plug-ins.
      |
      */
    last_file_mod_time:    Time,

    /**
      | The last time that this information
      | was updated. This would typically have
      | been during a scan when this plugin was
      | first tested or found to have changed.
      |
      */
    last_info_update_time: Time,

    /**
      | Deprecated: New projects should use
      | uniqueId instead.
      | 
      | A unique ID for the plug-in.
      | 
      | -----------
      | @note
      | 
      | this might not be unique between formats,
      | e.g. a Vst and some other format might
      | actually have the same id.
      | 
      | @see createIdentifierString
      |
      */
    deprecated_uid:        i32, // default = 0

    /**
      | A unique ID for the plug-in.
      | 
      | -----------
      | @note
      | 
      | this might not be unique between formats,
      | e.g. a Vst and some other format might
      | actually have the same id.
      | 
      | The uniqueId field replaces the deprecatedUid
      | field, and fixes an issue where Vst3
      | plugins with matching FUIDs would generate
      | different uid values depending on the
      | platform. The deprecatedUid field
      | is kept for backwards compatibility,
      | allowing existing hosts to migrate
      | from the old uid to the new uniqueId.
      | 
      | @see createIdentifierString
      |
      */
    unique_id:             i32, // default = 0

    /**
      | True if the plug-in identifies itself
      | as a synthesiser.
      |
      */
    is_instrument:         bool, // default = false

    /**
      | The number of inputs.
      |
      */
    num_input_channels:    i32, // default = 0

    /**
      | The number of outputs.
      |
      */
    num_output_channels:   i32, // default = 0

    /**
      | True if the plug-in is part of a multi-type
      | container, e.g. a Vst Shell.
      |
      */
    has_shared_container:  bool, // default = false
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/processors/aloe_PluginDescription.cpp]
impl PluginDescription {

    /**
      | Returns true if the two descriptions
      | refer to the same plug-in.
      | 
      | This isn't quite as simple as them just
      | having the same file (because of shell
      | plug-ins).
      |
      */
    pub fn is_duplicate_of(&self, other: &PluginDescription) -> bool {
        
        todo!();
        /*
            const auto tie = [] (const PluginDescription& d)
        {
            return std::tie (d.fileOrIdentifier, d.deprecatedUid, d.uniqueId);
        };

        return tie (*this) == tie (other);
        */
    }
    
    /**
      | Return true if this description is equivalent
      | to another one which created the given
      | identifier string.
      | 
      | -----------
      | @note
      | 
      | this isn't quite as simple as them just
      | calling createIdentifierString()
      | and comparing the strings, because
      | the identifiers can differ (thanks
      | to shell plug-ins).
      |
      */
    pub fn matches_identifier_string(&self, identifier_string: &String) -> bool {
        
        todo!();
        /*
            const auto matches = [&] (int uid)
        {
            return identifierString.endsWithIgnoreCase (getPluginDescSuffix (*this, uid));
        };

        return matches (uniqueId) || matches (deprecatedUid);
        */
    }
    
    /**
      | Returns a string that can be saved and
      | used to uniquely identify the plugin
      | again.
      | 
      | This contains less info than the XML
      | encoding, and is independent of the
      | plug-in's file location, so can be used
      | to store a plug-in ID for use across different
      | machines.
      |
      */
    pub fn create_identifier_string(&self) -> String {
        
        todo!();
        /*
            return pluginFormatName + "-" + name + getPluginDescSuffix (*this, uniqueId);
        */
    }
    
    /**
      | Creates an XML object containing these
      | details.
      | 
      | @see loadFromXml
      |
      */
    pub fn create_xml(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            auto e = std::make_unique<XmlElement> ("PLUGIN");

        e->setAttribute ("name", name);

        if (descriptiveName != name)
            e->setAttribute ("descriptiveName", descriptiveName);

        e->setAttribute ("format", pluginFormatName);
        e->setAttribute ("category", category);
        e->setAttribute ("manufacturer", manufacturerName);
        e->setAttribute ("version", version);
        e->setAttribute ("file", fileOrIdentifier);
        e->setAttribute ("uniqueId", String::toHexString (uniqueId));
        e->setAttribute ("isInstrument", isInstrument);
        e->setAttribute ("fileTime", String::toHexString (lastFileModTime.toMilliseconds()));
        e->setAttribute ("infoUpdateTime", String::toHexString (lastInfoUpdateTime.toMilliseconds()));
        e->setAttribute ("numInputs", numInputChannels);
        e->setAttribute ("numOutputs", numOutputChannels);
        e->setAttribute ("isShell", hasSharedContainer);

        e->setAttribute ("uid", String::toHexString (deprecatedUid));

        return e;
        */
    }
    
    /**
      | Reloads the info in this structure from
      | an XML record that was previously saved
      | with createXML().
      | 
      | Returns true if the XML was a valid plug-in
      | description.
      |
      */
    pub fn load_from_xml(&mut self, xml: &XmlElement) -> bool {
        
        todo!();
        /*
            if (xml.hasTagName ("PLUGIN"))
        {
            name                = xml.getStringAttribute ("name");
            descriptiveName     = xml.getStringAttribute ("descriptiveName", name);
            pluginFormatName    = xml.getStringAttribute ("format");
            category            = xml.getStringAttribute ("category");
            manufacturerName    = xml.getStringAttribute ("manufacturer");
            version             = xml.getStringAttribute ("version");
            fileOrIdentifier    = xml.getStringAttribute ("file");
            isInstrument        = xml.getBoolAttribute ("isInstrument", false);
            lastFileModTime     = Time (xml.getStringAttribute ("fileTime").getHexValue64());
            lastInfoUpdateTime  = Time (xml.getStringAttribute ("infoUpdateTime").getHexValue64());
            numInputChannels    = xml.getIntAttribute ("numInputs");
            numOutputChannels   = xml.getIntAttribute ("numOutputs");
            hasSharedContainer  = xml.getBoolAttribute ("isShell", false);

            deprecatedUid       = xml.getStringAttribute ("uid").getHexValue32();
            uniqueId            = xml.getStringAttribute ("uniqueId", "0").getHexValue32();

            return true;
        }

        return false;
        */
    }
}

pub fn get_plugin_desc_suffix(
        d:   &PluginDescription,
        uid: i32) -> String {
    
    todo!();
        /*
            return "-" + String::toHexString (d.fileOrIdentifier.hashCode())
             + "-" + String::toHexString (uid);
        */
}
