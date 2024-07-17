/*!
  | \defgroup vst3typedef Vst 3 Data Types
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstprefetchablesupport.h]

/**
  | Prefetchable Support Type
  |
  */
pub type PrefetchableSupport = u32;

/**
  | Prefetchable Support Enum
  |
  */
pub enum ePrefetchableSupport
{
    /**
      | every instance of the plug does not support
      | prefetch processing
      |
      */
    IsNeverPrefetchable = 0,   

    /**
      | in the current state the plug support
      | prefetch processing
      |
      */
    IsYetPrefetchable,         

    /**
      | in the current state the plug does not
      | support prefetch processing
      |
      */
    IsNotYetPrefetchable,      

    NumPrefetchableSupport
}

/**
  | Indicates that the plug-in could or
  | not support Prefetch (dynamically):
  | Vst::IPrefetchableSupport \ingroup
  | vstIPlug vst365
  | 
  | - [plug imp]
  | 
  | - [extends IComponent]
  | 
  | - [released: 3.6.5]
  | 
  | - [optional]
  | 
  | The plug-in should implement this interface
  | if it needs to dynamically change between
  | prefetchable or not.
  | 
  | By default (without implementing this
  | interface) the host decides in which
  | mode the plug-in is processed.
  | 
  | For more info about the prefetch processing
  | mode check the ProcessModes::kPrefetch
  | documentation.
  | 
  | \section IPrefetchableSupportExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | tresult PLUGIN_API myPlug::getPrefetchableSupport (PrefetchableSupport& prefetchable)
  | {
  |     prefetchable = kIsNeverPrefetchable;
  | 
  |     switch (myPrefetchableMode)
  |     {
  |         case 0: prefetchable = kIsNeverPrefetchable; break;
  |         case 1: prefetchable = kIsYetPrefetchable; break;
  |         case 2: prefetchable = kIsNotYetPrefetchable; break;
  |     }
  |     return kResultOk;
  | }
  |
  */
pub trait IPrefetchableSupport: FUnknown {

    /**
      | retrieve the current prefetch support.
      | Use IComponentHandler::restartComponent
      | (kPrefetchableSupportChanged) to
      | inform the host that this support has
      | changed.
      |
      */
    #[PLUGIN_API]
    fn get_prefetchable_support(&mut self, prefetchable: &mut PrefetchableSupport) -> tresult;
}

lazy_static!{
    /*
    static const FUID iprefetchable_support_iid;
    */
}

declare_class_iid!{
    IPrefetchableSupport, 
    0x8AE54FDA, 
    0xE93046B9, 
    0xA28555BC, 
    0xDC98E21E
}
