crate::ix!();

pub trait EditControllerEx1Interface: 
    SetProgramName 
    + NotifyUnitSelection {}

pub type EditControllerEx1ProgramListVector = Vec<IPtr<ProgramList>>;
pub type EditControllerEx1ProgramIndexMap   = HashMap<ProgramListID,usize>;
pub type EditControllerEx1UnitVector        = Vec<IPtr<Unit>>;

/**
  | Advanced implementation (support
  | IUnitInfo) for a VST 3 edit controller.
  | \ingroup vstClasses
  | 
  | - [extends EditController]
  |
  */
pub struct EditControllerEx1 {
    base:              EditController,
    units:             EditControllerEx1UnitVector,
    program_lists:     EditControllerEx1ProgramListVector,
    program_index_map: EditControllerEx1ProgramIndexMap,
    selected_unit:     UnitID,
}

impl FUnknown for EditControllerEx1 {

    fn query_interface(&mut self, _: [i8; 16], _: *mut *mut c_void) -> i32 { 
        todo!() 
    }

    fn add_ref(&mut self) -> u32 { todo!() }

    fn release(&mut self) -> u32 { todo!() }
}

impl IUnitInfo for EditControllerEx1 {

    fn get_unit_info(
        &mut self, 
        _: i32, 
        _: &mut aloe_vst_units::UnitInfo

    ) -> i32 { 

        todo!() 
    }

    fn get_program_list_count(&mut self) -> i32 { 
        todo!() 
    }

    fn get_program_list_info(
        &mut self, 
        _: i32, 
        _: &mut aloe_vst_units::ProgramListInfo

    ) -> i32 { 

        todo!() 
    }

    fn get_program_name(
        &mut self, 
        _: i32, 
        _: i32, 
        _: [u16; 128]

    ) -> i32 { 

        todo!() 
    }

    fn get_program_info(
        &mut self, 
        _: i32, 
        _: i32, 
        _: &str, 
        _: [u16; 128]

    ) -> i32 { 

        todo!() 
    }

    fn has_program_pitch_names(&mut self, _: i32, _: i32) -> i32 { 
        todo!() 
    }

    fn get_program_pitch_name(
        &mut self, 
        _: i32, 
        _: i32, 
        _: i16, 
        _: [u16; 128]

    ) -> i32 { 

        todo!() 
    }

    #[PLUGIN_API]
    fn get_unit_count(&mut self) -> i32 {
        
        todo!();
        /*
            return static_cast<i32> (units.size ());
        */
    }
    
    #[PLUGIN_API]
    fn get_selected_unit(&mut self) -> UnitID {
        
        todo!();
        /*
            return selectedUnit;
        */
    }
    
    #[PLUGIN_API]
    fn select_unit(&mut self, unit_id: UnitID) -> tresult {
        
        todo!();
        /*
            selectedUnit = unitId;
            return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    fn get_unit_by_bus(&mut self, 
        ty:        MediaType,
        dir:       BusDirection,
        bus_index: i32,
        channel:   i32,
        unit_id:   &mut UnitID) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    fn set_unit_program_data(&mut self, 
        list_or_unit_id: i32,
        program_index:   i32,
        data:            *mut dyn IBStream) -> tresult {
        
        todo!();
        /*
            return kResultFalse;
        */
    }
}

impl EditControllerEx1 {
    
    lazy_static!{
        /*
        OBJ_METHODS (EditControllerEx1, EditController)
            DEFINE_INTERFACES
                DEF_INTERFACE (IUnitInfo)
            END_DEFINE_INTERFACES (EditController)
            REFCOUNT_METHODS (EditController)
        */
    }

    pub fn new() -> Self {
    
        todo!();
        /*
        : selected_unit(kRootUnitId),

            UpdateHandler::instance ();
        */
    }
    
    #[PLUGIN_API]
    pub fn terminate(&mut self) -> tresult {
        
        todo!();
        /*
            units.clear ();

        for (const auto& programList : programLists)
        {
            if (programList)
                programList->removeDependent (this);
        }
        programLists.clear ();
        programIndexMap.clear ();

        return EditController::terminate ();
        */
    }
    
    /**
      | Adds a given unit.
      |
      */
    pub fn add_unit(&mut self, unit: *mut Unit) -> bool {
        
        todo!();
        /*
            units.emplace_back (unit, false);
        return true;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_unit_info(&mut self, 
        unit_index: i32,
        info:       &mut UnitInfo) -> tresult {
        
        todo!();
        /*
            if (Unit* unit = units.at (unitIndex))
        {
            info = unit->getInfo ();
            return kResultTrue;
        }
        return kResultFalse;
        */
    }
    
    pub fn notify_unit_selection(&mut self) -> tresult {
        
        todo!();
        /*
            tresult result = kResultFalse;
        FUnknownPtr<IUnitHandler> unitHandler (componentHandler);
        if (unitHandler)
            result = unitHandler->notifyUnitSelection (selectedUnit);
        return result;
        */
    }
    
    /**
      | Adds a given program list.
      |
      */
    pub fn add_program_list(&mut self, list: *mut ProgramList) -> bool {
        
        todo!();
        /*
            programIndexMap[list->getID ()] = programLists.size ();
        programLists.emplace_back (list, false);
        list->addDependent (this);
        return true;
        */
    }
    
    /**
      | Returns the ProgramList associated
      | to a given listId.
      |
      */
    pub fn get_program_list(&self, list_id: ProgramListID) -> *mut ProgramList {
        
        todo!();
        /*
            auto it = programIndexMap.find (listId);
        return it == programIndexMap.end () ? nullptr : programLists[it->second];
        */
    }
    
    /**
      | Notifies the host about program list
      | changes.
      |
      */
    pub fn notify_program_list_change(
        &mut self, 
        list_id:       ProgramListID,
        program_index: Option<i32>

    ) -> tresult {

        let program_index: i32 = program_index.unwrap_or(ALL_PROGRAM_INVALID);
        
        todo!();
        /*
            tresult result = kResultFalse;
        FUnknownPtr<IUnitHandler> unitHandler (componentHandler);
        if (unitHandler)
            result = unitHandler->notifyProgramListChange (listId, programIndex);
        return result;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_list_count(&mut self) -> i32 {
        
        todo!();
        /*
            return static_cast<i32> (programLists.size ());
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_list_info(&mut self, 
        list_index: i32,
        info:       &mut ProgramListInfo) -> tresult {
        
        todo!();
        /*
            if (listIndex < 0 || listIndex >= static_cast<i32> (programLists.size ()))
            return kResultFalse;
        info = programLists[listIndex]->getInfo ();
        return kResultTrue;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_name(&mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        name:          String128) -> tresult {
        
        todo!();
        /*
            ProgramIndexMap::const_iterator it = programIndexMap.find (listId);
        if (it != programIndexMap.end ())
        {
            return programLists[it->second]->getProgramName (programIndex, name);
        }
        return kResultFalse;
        */
    }
    
    pub fn set_program_name(&mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        name:          String128) -> tresult {
        
        todo!();
        /*
            ProgramIndexMap::const_iterator it = programIndexMap.find (listId);
        if (it != programIndexMap.end ())
        {
            return programLists[it->second]->setProgramName (programIndex, name);
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_info(&mut self, 
        list_id:         ProgramListID,
        program_index:   i32,
        attribute_id:    &str,
        attribute_value: String128) -> tresult {
        
        todo!();
        /*
            ProgramIndexMap::const_iterator it = programIndexMap.find (listId);
        if (it != programIndexMap.end ())
        {
            return programLists[it->second]->getProgramInfo (programIndex, attributeId, attributeValue);
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn has_program_pitch_names(&mut self, 
        list_id:       ProgramListID,
        program_index: i32) -> tresult {
        
        todo!();
        /*
            ProgramIndexMap::const_iterator it = programIndexMap.find (listId);
        if (it != programIndexMap.end ())
        {
            return programLists[it->second]->hasPitchNames (programIndex);
        }
        return kResultFalse;
        */
    }
    
    #[PLUGIN_API]
    pub fn get_program_pitch_name(&mut self, 
        list_id:       ProgramListID,
        program_index: i32,
        midi_pitch:    i16,
        name:          String128) -> tresult {
        
        todo!();
        /*
            ProgramIndexMap::const_iterator it = programIndexMap.find (listId);
        if (it != programIndexMap.end ())
        {
            return programLists[it->second]->getPitchName (programIndex, midiPitch, name);
        }
        return kResultFalse;
        */
    }
    
    pub fn update(&mut self, 
        changed_unknown: *mut dyn FUnknown,
        message:         i32)  {
        
        todo!();
        /*
            auto* programList = FCast<ProgramList> (changedUnknown);
        if (programList)
        {
            FUnknownPtr<IUnitHandler> unitHandler (componentHandler);
            if (unitHandler)
                unitHandler->notifyProgramListChange (programList->getID (), kAllProgramInvalid);
        }
        */
    }
}
