/*!
  | \defgroup vst3typedef Vst 3 Data Types
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstphysicalui.h]

/**
  | Extended plug-in interface IEditController
  | for note expression event support:
  | Vst::INoteExpressionPhysicalUIMapping
  | \ingroup vstIPlug vst3611
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.6.11]
  | 
  | - [optional]
  | 
  | With this plug-in interface, the host
  | can retrieve the preferred physical
  | mapping associated to note expression
  | supported by the plug-in.
  | 
  | When the mapping changes (for example
  | when switching presets) the plug-in
  | needs to inform the host about it via
  | \ref IComponentHandler::restartComponent
  | (kNoteExpressionChanged).
  | 
  | \section INoteExpressionPhysicalUIMappingExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | // here an example of how a Vst3 plug-in could support this INoteExpressionPhysicalUIMapping interface.
  | // we need to define somewhere the iids:
  | 
  | //in MyController class declaration
  | class MyController : public Vst::EditController, public Vst::INoteExpressionPhysicalUIMapping
  | {
  |     // ...
  |     //--- INoteExpressionPhysicalUIMapping ---------------------------------
  |     tresult PLUGIN_API getPhysicalUIMapping (int32 busIndex, int16 channel, PhysicalUIMapList& list) SMTG_OVERRIDE;
  |     // ...
  | 
  |     OBJ_METHODS (MyController, Vst::EditController)
  |     DEFINE_INTERFACES
  |         // ...
  |         DEF_INTERFACE (Vst::INoteExpressionPhysicalUIMapping)
  |     END_DEFINE_INTERFACES (Vst::EditController)
  |     //...
  | }
  | 
  | // In mycontroller.cpp
  | #include "pluginterfaces/vst/ivstnoteexpression.h"
  | 
  | namespace Steinberg {
  |     namespace Vst {
  |         DEF_CLASS_IID (INoteExpressionPhysicalUIMapping)
  |     }
  | }
  | 
  | tresult PLUGIN_API MyController::getPhysicalUIMapping (int32 busIndex, int16 channel, PhysicalUIMapList& list)
  | {
  |     if (busIndex == 0 && channel == 0)
  |     {
  |         for (uint32 i = 0; i < list.count; ++i)
  |         {
  |             NoteExpressionTypeID type = kInvalidTypeID;
  |             if (kPUIXMovement == list.map[i].physicalUITypeID)
  |                 list.map[i].noteExpressionTypeID = kCustomStart + 1;
  |             else if (kPUIYMovement == list.map[i].physicalUITypeID)
  |                 list.map[i].noteExpressionTypeID = kCustomStart + 2;
  |         }
  |         return kResultTrue;
  |     }
  |     return kResultFalse;
  | }
  |
  */
pub trait INoteExpressionPhysicalUIMapping: FUnknown {

    /**
      | Fills the list of mapped [physical UI
      | (in) - note expression (out)] for a given
      | bus index and channel.
      |
      */
    #[PLUGIN_API]
    fn get_physical_ui_mapping(&mut self, 
            bus_index: i32,
            channel:   i16,
            list:      &mut PhysicalUIMapList) -> tresult;
}

lazy_static!{
    /*
    static const FUID inote_expression_physical_ui_mapping_iid;
    */
}

declare_class_iid!{
    INoteExpressionPhysicalUIMapping, 
    0xB03078FF, 
    0x94D24AC8, 
    0x90CCD303, 
    0xD4133324
}
