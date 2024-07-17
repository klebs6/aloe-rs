crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/public.sdk/source/vst/vsteditcontroller.cpp]

pub type ProgramListWithPitchNamesPitchNameMap     = HashMap<i16,String>;
pub type ProgramListWithPitchNamesPitchNamesVector = Vec<ProgramListWithPitchNamesPitchNameMap>;

/**
  | ProgramListWithPitchNames element.
  | \ingroup vstClasses
  |
  */
pub struct ProgramListWithPitchNames {
    base:        ProgramList,
    pitch_names: ProgramListWithPitchNamesPitchNamesVector,
}

obj_methods!{ ProgramListWithPitchNames, ProgramList }

impl ProgramListWithPitchNames {

    pub fn new(
        name:    String128,
        list_id: ProgramListID,
        unit_id: UnitID) -> Self {
    
        todo!();
        /*
        : program_list(name, listId, unitId),

        
        */
    }
    
    pub fn add_program(&mut self, name: String128) -> i32 {
        
        todo!();
        /*
            int32 index = ProgramList::addProgram (name);
        if (index >= 0)
            pitchNames.emplace_back ();
        return index;
        */
    }
    
    /**
      | Sets a name for the given program index
      | and a given pitch.
      |
      */
    pub fn set_pitch_name(&mut self, 
        program_index: i32,
        pitch:         i16,
        pitch_name:    String128) -> bool {
        
        todo!();
        /*
            if (programIndex < 0 || programIndex >= getCount ())
            return false;

        bool nameChanged = true;
        std::pair<PitchNameMap::iterator, bool> res =
            pitchNames[programIndex].insert (std::make_pair (pitch, pitchName));
        if (!res.second)
        {
            if (res.first->second == pitchName)
                nameChanged = false;
            else
                res.first->second = pitchName;
        }

        if (nameChanged)
            changed ();
        return true;
        */
    }
    
    /**
      | Removes the PitchName entry for the
      | given program index and a given pitch.
      | Returns true if it was found and removed.
      |
      */
    pub fn remove_pitch_name(&mut self, 
        program_index: i32,
        pitch:         i16) -> bool {
        
        todo!();
        /*
            bool result = false;
        if (programIndex >= 0 && programIndex < getCount ())
        {
            result = pitchNames.at (programIndex).erase (pitch) != 0;
        }
        if (result)
            changed ();
        return result;
        */
    }
    
    pub fn has_pitch_names(&mut self, program_index: i32) -> tresult {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < getCount ())
            return (pitchNames.at (programIndex).empty () == true) ? kResultFalse : kResultTrue;
        return kResultFalse;
        */
    }
    
    pub fn get_pitch_name(&mut self, 
        program_index: i32,
        midi_pitch:    i16,
        name:          String128) -> tresult {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < getCount ())
        {
            PitchNameMap::const_iterator it = pitchNames[programIndex].find (midiPitch);
            if (it != pitchNames[programIndex].end ())
            {
                it->second.copyTo16 (name, 0, 128);
                return kResultTrue;
            }
        }
        return kResultFalse;
        */
    }
}
