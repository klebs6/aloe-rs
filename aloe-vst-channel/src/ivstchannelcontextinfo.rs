/*!
  | For Channel Context Info Interface
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstchannelcontextinfo.h]

/**
  | Channel context interface: Vst::ChannelContextIInfoListener
  | \ingroup vstIHost vst365
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.6.5]
  | 
  | - [optional]
  | 
  | Allows the host to inform the plug-in
  | about the context in which the plug-in
  | is instantiated, mainly channel based
  | info (color, name, index,...). Index
  | can be defined inside a namespace (for
  | example, index start from 1 to N for Type
  | Input/Output Channel (Index namespace)
  | and index start from 1 to M for Type Audio
  | Channel).\n
  | 
  | As soon as the plug-in provides this
  | ChannelContextIInfoListener interface, the host
  | will call setChannelContextInfos
  | for each change occurring to this channel
  | (new name, new color, new indexation,...)
  | 
  | \section IChannelContextExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | tresult PLUGIN_API MyPlugin::setChannelContextInfos (IAttributeList* list)
  | {
  |     if (list)
  |     {
  |         // optional we can ask for the Channel Name Length
  |         int64 length;
  |         if (list->getInt (ChannelContext::kChannelNameLengthKey, length) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get the Channel Name where we, as plug-in, are instantiated
  |         String128 name;
  |         if (list->getString (ChannelContext::kChannelNameKey, name, sizeof (name)) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get the Channel UID
  |         if (list->getString (ChannelContext::kChannelUIDKey, name, sizeof (name)) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get Channel Index
  |         int64 index;
  |         if (list->getInt (ChannelContext::kChannelIndexKey, index) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get the Channel Color
  |         int64 color;
  |         if (list->getInt (ChannelContext::kChannelColorKey, color) == kResultTrue)
  |         {
  |             uint32 channelColor = (uint32)color;
  |             String str;
  |             str.printf ("%x%x%x%x", ChannelContext::GetAlpha (channelColor),
  |             ChannelContext::GetRed (channelColor),
  |             ChannelContext::GetGreen (channelColor),
  |             ChannelContext::GetBlue (channelColor));
  |             String128 string128;
  |             typename Steinberg::UString (string128, 128).fromAscii (str);
  |             ...
  |         }
  | 
  |         // get Channel Index Namespace Order of the current used index namespace
  |         if (list->getInt (ChannelContext::kChannelIndexNamespaceOrderKey, index) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get the channel Index Namespace Length
  |         if (list->getInt (ChannelContext::kChannelIndexNamespaceLengthKey, length) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get the channel Index Namespace
  |         String128 namespaceName;
  |         if (list->getString (ChannelContext::kChannelIndexNamespaceKey, namespaceName, sizeof (namespaceName)) == kResultTrue)
  |         {
  |             ...
  |         }
  | 
  |         // get plug-in Channel Location
  |         int64 location;
  |         if (list->getInt (ChannelContext::kChannelPluginLocationKey, location) == kResultTrue)
  |         {
  |             String128 string128;
  |             switch (location)
  |             {
  |                 case ChannelContext::kPreVolumeFader:
  |                     typename Steinberg::UString (string128, 128).fromAscii ("PreVolFader");
  |                 break;
  |                 case ChannelContext::kPostVolumeFader:
  |                     typename Steinberg::UString (string128, 128).fromAscii ("PostVolFader");
  |                 break;
  |                 case ChannelContext::kUsedAsPanner:
  |                     typename Steinberg::UString (string128, 128).fromAscii ("UsedAsPanner");
  |                 break;
  |                 default: typename Steinberg::UString (string128, 128).fromAscii ("unknown!");
  |                 break;
  |             }
  |         }
  | 
  |         // do not forget to call addRef () if you want to keep this list
  |     }
  | }
  |
  */
pub trait ChannelContextIInfoListener: FUnknown {

    type AttrID;

    /**
      | Receive the channel context infos from
      | host.
      |
      */
    #[PLUGIN_API]
    fn set_channel_context_infos(&mut self, list: *mut dyn IAttributeList<AttrID = Self::AttrID>) -> tresult;
}

lazy_static!{
    /*
    static const FUID channel_context_iinfo_listeneriid;
    */
}

declare_class_iid!{
    ChannelContextIInfoListener, 
    0x0F194781, 
    0x8D984ADA, 
    0xBBA0C1EF, 
    0xC011D8D0
}

/**
  | Values used for kChannelPluginLocationKey
  |
  */
pub enum ChannelContextChannelPluginLocation
{
    kPreVolumeFader = 0,
    kPostVolumeFader,
    kUsedAsPanner
}

/* ----- \defgroup vst3typedef Vst 3 Data Types  ----- */

/**
  | ARGB (Alpha-Red-Green-Blue)
  |
  */
pub type ChannelContextColorSpec      = u32;
pub type ChannelContextColorComponent = u8;

/**
  | Returns the Blue part of the given ChannelContextColorSpec
  |
  */
#[inline] pub fn channel_context_get_blue(cs: ChannelContextColorSpec) -> ChannelContextColorComponent {
    
    todo!();
        /*
            return (ChannelContextColorComponent)(cs & 0x000000FF);
        */
}

/**
  | Returns the Green part of the given ChannelContextColorSpec
  |
  */
#[inline] pub fn channel_context_get_green(cs: ChannelContextColorSpec) -> ChannelContextColorComponent {
    
    todo!();
        /*
            return (ChannelContextColorComponent)((cs >> 8) & 0x000000FF);
        */
}

/**
  | Returns the Red part of the given ChannelContextColorSpec
  |
  */
#[inline] pub fn channel_context_get_red(cs: ChannelContextColorSpec) -> ChannelContextColorComponent {
    
    todo!();
        /*
            return (ChannelContextColorComponent)((cs >> 16) & 0x000000FF);
        */
}

/**
  | Returns the Alpha part of the given ChannelContextColorSpec
  |
  */
#[inline] pub fn channel_context_get_alpha(cs: ChannelContextColorSpec) -> ChannelContextColorComponent {
    
    todo!();
        /*
            return (ChannelContextColorComponent)((cs >> 24) & 0x000000FF);
        */
}

/*
  | Keys used as AttrID (Attribute ID) in
  | the return IAttributeList of
  | 
  | ChannelContextIInfoListener::setChannelContextInfos
  |
  */

/**
  | string (TChar) [optional]: unique
  | id string used to identify a channel
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_UID_KEY: &'static str = "channel uid";

/**
  | integer (int64) [optional]: number
  | of characters in kChannelUIDKey
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_UID_LENGTH_KEY: &'static str = "channel uid length";

/**
  | string (TChar) [optional]: name of
  | the channel like displayed in the mixer
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_NAME_KEY: &'static str = "channel name";

/**
  | integer (int64) [optional]: number
  | of characters in kChannelNameKey
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_NAME_LENGTH_KEY: &'static str = "channel name length";

/**
  | color (ChannelContextColorSpec) [optional]: used
  | color for the channel in mixer or track
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_COLOR_KEY: &'static str = "channel color";

/**
  | integer (int64) [optional]: index
  | of the channel in a channel index namespace,
  | start with 1 not * 0!
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_INDEX_KEY: &'static str = "channel index";

/**
  | integer (int64) [optional]: define
  | the order of the current used index namespace,
  | start with 1 not 0!
  | 
  | For example: index namespace is "Input"
  | -> order 1, index namespace is "Channel"
  | -> order 2, index namespace is "Output"
  | -> order 3
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_INDEX_NAMESPACE_ORDER_KEY: &'static str = "channel index namespace order";

/**
  | string (TChar) [optional]: name of
  | the channel index namespace for example
  | "Input", "Output", "Channel", ...
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_INDEX_NAMESPACE_KEY: &'static str = "channel index namespace";

/**
  | integer (int64) [optional]: number
  | of characters in kChannelIndexNamespaceKey
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_INDEX_NAMESPACE_LENGTH_KEY: &'static str = "channel index namespace length";

/**
  | PNG image representation as binary
  | [optional]
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_IMAGE_KEY: &'static str = "channel image";

/**
  | integer (int64) [optional]: routing
  | position of the plug-in in the channel
  | (see ChannelContextChannelPluginLocation)
  |
  */
pub const CHANNEL_CONTEXT_CHANNEL_PLUGIN_LOCATION_KEY: &'static str = "channel plugin location";
