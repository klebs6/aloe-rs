crate::ix!();

/**
  | Extended plug-in interface IEditController
  | for note expression event support:
  | Vst::INoteExpressionController
  | \ingroup vstIPlug vst350
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.5.0]
  | 
  | - [optional]
  | 
  | With this plug-in interface, the host
  | can retrieve all necessary note expression
  | information supported by the plug-in.
  | 
  | Note expression information (\ref
  | NoteExpressionTypeInfo) are specific
  | for given channel and event bus.
  | 
  | -----------
  | @note
  | 
  | there is only one NoteExpressionTypeID
  | per given channel of an event bus.
  | 
  | The method getNoteExpressionStringByValue
  | allows conversion from a normalized
  | value to a string representation and
  | the getNoteExpressionValueByString
  | method from a string to a normalized
  | value.
  | 
  | When the note expression state changes
  | (for example when switching presets)
  | the plug-in needs to inform the host
  | about it via \ref IComponentHandler::restartComponent
  | (kNoteExpressionChanged).
  |
  */
pub trait INoteExpressionController: FUnknown {

    /**
      | Returns number of supported note change
      | types for event bus index and channel.
      |
      */
    #[PLUGIN_API]
    fn get_note_expression_count(&mut self, 
            bus_index: i32,
            channel:   i16) -> i32;

    /**
      | Returns note change type info.
      |
      */
    #[PLUGIN_API]
    fn get_note_expression_info(&mut self, 
            bus_index:             i32,
            channel:               i16,
            note_expression_index: i32,
            info:                  &mut NoteExpressionTypeInfo) -> tresult;

    /**
      | Gets a user readable representation
      | of the normalized note change value.
      |
      */
    #[PLUGIN_API]
    fn get_note_expression_string_by_value(&mut self, 
            bus_index:        i32,
            channel:          i16,
            id:               NoteExpressionTypeID,
            value_normalized: NoteExpressionValue,
            string:           String128) -> tresult;

    /**
      | Converts the user readable representation
      | to the normalized note change value.
      |
      */
    #[PLUGIN_API]
    fn get_note_expression_value_by_string(&mut self, 
            bus_index:        i32,
            channel:          i16,
            id:               NoteExpressionTypeID,
            string:           *const TChar,
            value_normalized: &mut NoteExpressionValue) -> tresult;
}

lazy_static!{
    /*
    static const FUID inote_expression_controller_iid;
    */
}

declare_class_iid!{
    INoteExpressionController, 
    0xB7F8F859, 
    0x41234872, 
    0x91169581, 
    0x4F3721A3
}
