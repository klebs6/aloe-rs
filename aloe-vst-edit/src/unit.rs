crate::ix!();

/**
  | Unit element. \ingroup vstClasses
  |
  */
pub struct Unit {
    base: FObject,
    info: UnitInfo,
}

obj_methods!{ Unit, FObject }

impl Default for Unit {

    fn default() -> Self {
    
        todo!();
        /*


            memset (&info, 0, sizeof (UnitInfo));
        */
    }
}

impl From<&UnitInfo> for Unit {

    fn from(info: &UnitInfo) -> Self {
    
        todo!();
        /*
        : info(info),

        
        */
    }
}
    
impl Unit {
    
    /**
      | Returns its info.
      |
      */
    pub fn get_info(&self) -> &UnitInfo {
        
        todo!();
        /*
            return info;
        */
    }

    /**
      | Returns its Unit ID.
      |
      */
    pub fn getid(&self) -> UnitID {
        
        todo!();
        /*
            return info.id;
        */
    }

    /**
      | Sets a new Unit ID.
      |
      */
    pub fn setid(&mut self, newid: UnitID)  {
        
        todo!();
        /*
            info.id = newID;
        */
    }

    /**
      | Returns its Unit Name.
      |
      */
    pub fn get_name(&self) -> *const TChar {
        
        todo!();
        /*
            return info.name;
        */
    }

    /**
      | Returns its ProgramList ID.
      |
      */
    pub fn get_program_listid(&self) -> ProgramListID {
        
        todo!();
        /*
            return info.programListId;
        */
    }

    /**
      | Sets a new ProgramList ID.
      |
      */
    pub fn set_program_listid(&mut self, newid: ProgramListID)  {
        
        todo!();
        /*
            info.programListId = newID;
        */
    }
    
    pub fn new(
        name:            String128,
        unit_id:         UnitID,
        parent_unit_id:  Option<UnitID>,
        program_list_id: Option<ProgramListID>

    ) -> Self {
    
        let parent_unit_id  = parent_unit_id.unwrap_or(ROOT_UNIT_ID);
        let program_list_id = program_list_id.unwrap_or(NO_PROGRAM_LIST_ID);

        todo!();
        /*
        setName (name);
        info.id = unitId;
        info.parentUnitId = parentUnitId;
        info.programListId = programListId;
        */
    }
    
    /**
      | Sets a new Unit Name.
      |
      */
    pub fn set_name(&mut self, new_name: String128)  {
        
        todo!();
        /*
            UString128 (newName).copyTo (info.name, 128);
        */
    }
}
