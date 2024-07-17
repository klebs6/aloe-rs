crate::ix!();

pub trait EditControllerInterface:
    BeginEdit
    + PerformEdit
    + EndEdit
    + StartGroupEdit
    + FinishGroupEdit
    + EditorDestroyed
    + EditorAttached
    + EditorRemoved
    + GetParameterObject
    + GetParameterInfoByTag
    + SetDirty
    + RequestOpenEditor {}

pub trait NotifyUnitSelection {

    /**
      | Notifies the host about the selected
      | Unit.
      |
      */
    fn notify_unit_selection(&mut self) -> tresult;

}

pub trait SetProgramNameWithListID {

    fn set_program_name_with_list_id(&mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        name:          String128) -> tresult {
        
        todo!();
        /*
        
        */
    }
}

pub trait GetProgramName {

    fn get_program_name(&mut self, 
            program_index: i32,
            name:          String128) -> tresult;
}

pub trait SetProgramName {

    fn set_program_name(&mut self, 
            program_index: i32,
            name:          String128) -> tresult;

}

pub trait GetProgramInfo {

    fn get_program_info(&mut self, 
            program_index: i32,
            attribute_id:  &str,
            value:         String128) -> tresult;

}

pub trait HasPitchNames {

    fn has_pitch_names(&mut self, program_index: i32) -> tresult {
        
        todo!();
        /*
            (void)programIndex;
            return kResultFalse;
        */
    }
}

pub trait GetPitchName {

    fn get_pitch_name(&mut self, 
        program_index: i32,
        midi_pitch:    i16,
        name:          String128) -> tresult {
        
        todo!();
        /*
            (void)programIndex;
            (void)midiPitch;
            (void)name;
            return kResultFalse;
        */
    }
}

pub trait AddProgram {

    /**
      | Adds a program to the end of the list.
      | returns the index of the program.
      |
      */
    fn add_program(&mut self, name: String128) -> i32;
}

pub trait SetProgramInfo {

    /**
      | Sets a program attribute value.
      |
      */
    fn set_program_info(&mut self, 
            program_index: i32,
            attribute_id:  &str,
            value:         String128) -> bool;
}

pub trait GetParameter {

    /**
      | Creates and returns the program parameter.
      |
      */
    fn get_parameter(&mut self) -> *mut Parameter;

}

pub trait BeginEdit {

    /**
      | to be called before a serie of performEdit
      |
      */
    fn begin_edit(&mut self, tag: ParamID) -> tresult;
}

pub trait PerformEdit {

    /**
      | will inform the host about the value
      | change
      |
      */
    fn perform_edit(&mut self, 
            tag:              ParamID,
            value_normalized: ParamValue) -> tresult;
}

pub trait EndEdit {

    /**
      | to be called after a serie of performEdit
      |
      */
    fn end_edit(&mut self, tag: ParamID) -> tresult;
}

pub trait StartGroupEdit {

    /**
      | calls IComponentHandler2::startGroupEdit()
      | if host supports it
      |
      */
    fn start_group_edit(&mut self) -> tresult;
}

pub trait FinishGroupEdit {

    /**
      | calls IComponentHandler2::finishGroupEdit()
      | if host supports it
      |
      */
    fn finish_group_edit(&mut self) -> tresult;
}

pub trait EditorDestroyed {

    /**
      | called from EditorView if it was destroyed
      |
      */
    fn editor_destroyed(&mut self, editor: *mut EditorView)  { }
}

pub trait EditorAttached {

    /**
      | called from EditorView if it was attached
      | to a parent
      |
      */
    fn editor_attached(&mut self, editor: *mut EditorView)  { }
}

pub trait EditorRemoved {

    /**
      | called from EditorView if it was removed
      | from a parent
      |
      */
    fn editor_removed(&mut self, editor: *mut EditorView)  { }
}


pub trait GetParameterObject {

    /**
      | Gets for a given tag the parameter object.
      |
      */
    fn get_parameter_object(&mut self, tag: ParamID) -> *mut Parameter {
        
        todo!();
        /*
            return parameters.getParameter (tag);
        */
    }
}

pub trait GetParameterInfoByTag {

    /**
      | Gets for a given tag the parameter information.
      |
      */
    fn get_parameter_info_by_tag(&mut self, 
        tag:  ParamID,
        info: &mut ParameterInfo) -> tresult;
}

pub trait SetDirty {

    /**
      | Calls IComponentHandler2::setDirty
      | (state) if host supports it.
      |
      */
    fn set_dirty(&mut self, state: TBool) -> tresult;
}

pub trait RequestOpenEditor {

    /**
      | Calls IComponentHandler2::requestOpenEditor
      | (name) if host supports it.
      |
      */
    fn request_open_editor(&mut self, name: Option<&str>) -> tresult {

        let name = name.unwrap_or(VIEW_TYPE_EDITOR);

        todo!();
        /*
        
        */
    }
}
