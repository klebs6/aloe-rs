crate::ix!();

pub type ProgramListStringMap         = HashMap<String,String>;
pub type ProgramListStringVector      = Vec<String>;
pub type ProgramListProgramInfoVector = Vec<ProgramListStringMap>;

pub trait ProgramListInterface:
    GetProgramName
    + SetProgramName
    + GetProgramInfo
    + HasPitchNames
    + GetPitchName
    + AddProgram
    + SetProgramInfo
    + GetParameter {}

obj_methods!{ ProgramList, FObject }

/**
  | ProgramList element. \ingroup vstClasses
  |
  */
pub struct ProgramList {
    base:          FObject,
    info:          ProgramListInfo,
    unit_id:       UnitID,
    program_names: ProgramListStringVector,
    program_infos: ProgramListProgramInfoVector,
    parameter:     *mut Parameter,
}

impl From<&ProgramList> for ProgramList {

     fn from(program_list: &ProgramList) -> Self {
    
        todo!();
        /*
        : info(programList.info),
        : unit_id(programList.unitId),
        : program_names(programList.programNames),
        : parameter(nullptr),

        
        */
    }
}

impl ProgramList {
    
    pub fn get_info(&self) -> &ProgramListInfo {
        
        todo!();
        /*
            return info;
        */
    }
    
    pub fn getid(&self) -> ProgramListID {
        
        todo!();
        /*
            return info.id;
        */
    }
    
    pub fn get_name(&self) -> *const TChar {
        
        todo!();
        /*
            return info.name;
        */
    }
    
    pub fn get_count(&self) -> i32 {
        
        todo!();
        /*
            return info.programCount;
        */
    }

    pub fn new(
        name:    String128,
        list_id: ProgramListID,
        unit_id: UnitID) -> Self {
    
        todo!();
        /*
        : unit_id(unitId),
        : parameter(nullptr),

            UString128 (name).copyTo (info.name, 128);
        info.id = listId;
        info.programCount = 0;
        */
    }
    
    pub fn add_program(&mut self, name: String128) -> i32 {
        
        todo!();
        /*
            ++info.programCount;
        programNames.emplace_back (name);
        programInfos.emplace_back ();
        return static_cast<i32> (programNames.size ()) - 1;
        */
    }
    
    pub fn set_program_info(&mut self, 
        program_index: i32,
        attribute_id:  &str,
        value:         String128) -> bool {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < static_cast<i32> (programNames.size ()))
        {
            programInfos.at (programIndex).insert (std::make_pair (attributeId, value));
            return true;
        }
        return false;
        */
    }
    
    pub fn get_program_info(&mut self, 
        program_index: i32,
        attribute_id:  &str,
        value:         String128) -> tresult {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < static_cast<i32> (programNames.size ()))
        {
            ProgramListStringMap::const_iterator it = programInfos[programIndex].find (attributeId);
            if (it != programInfos[programIndex].end ())
            {
                if (!it->second.isEmpty ())
                {
                    it->second.copyTo16 (value, 0, 128);
                    return kResultTrue;
                }
            }
        }
        return kResultFalse;
        */
    }
    
    pub fn get_program_name(&mut self, 
        program_index: i32,
        name:          String128) -> tresult {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < static_cast<i32> (programNames.size ()))
        {
            programNames.at (programIndex).copyTo16 (name, 0, 128);
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    pub fn set_program_name(&mut self, 
        program_index: i32,
        name:          String128) -> tresult {
        
        todo!();
        /*
            if (programIndex >= 0 && programIndex < static_cast<i32> (programNames.size ()))
        {
            programNames.at (programIndex) = name;
            if (parameter)
            {
                static_cast<StringListParameter*> (parameter)->replaceString (programIndex, name);
            }
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    pub fn get_parameter(&mut self) -> *mut Parameter {
        
        todo!();
        /*
            if (parameter == nullptr)
        {
            auto* listParameter = new StringListParameter (
                info.name, info.id, nullptr,
                ParameterInfo::kCanAutomate | ParameterInfo::kIsList | ParameterInfo::kIsProgramChange,
                unitId);
            for (const auto& programName : programNames)
            {
                listParameter->appendString (programName);
            }
            parameter = listParameter;
        }
        return parameter;
        */
    }
}
