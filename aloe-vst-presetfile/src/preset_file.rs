crate::ix!();

pub const PRESET_FILE_MAX_ENTRIES: usize = 128;

/**
  | Internal structure used for chunk handling
  |
  */
pub struct PresetFileEntry
{
    id:     ChunkID,
    offset: TSize,
    size:   TSize,
}

/**
  | Handler for a VST 3 Preset File. \ingroup
  | vstClasses \see \ref presetformat
  |
  */
pub struct PresetFile {

    stream:      *mut dyn IBStream,

    /**
      | classID is the FUID of the component
      | (processor) part
      |
      */
    classid:     FUID,

    entries:     [PresetFileEntry; PRESET_FILE_MAX_ENTRIES],
    entry_count: i32,
}

impl Drop for PresetFile {

    fn drop(&mut self) {
        todo!();
        /*
            if (stream)
            stream->release ();
        */
    }
}

impl PresetFile {
    
    /**
      | Shortcut helper to create preset from
      | component/controller state. classID
      | is the FUID of the component (processor)
      | part.
      |
      */
    pub fn save_preset(
        &mut self, 
        stream:          *mut dyn IBStream,
        classid:         &FUID,
        component:       *mut dyn VstIComponent,
        edit_controller: *mut dyn IEditController,
        xml_buffer:      *const u8,
        xml_size:        Option<i32>

    ) -> bool {

        let xml_size: i32 = xml_size.unwrap_or(-1);
        
        todo!();
        /*
            PresetFile pf (stream);
        pf.setClassID (classID);
        if (!pf.writeHeader ())
            return false;

        if (!pf.storeComponentState (component))
            return false;

        if (editController && !pf.storeControllerState (editController))
            return false;

        if (xmlBuffer && !pf.writeMetaInfo (xmlBuffer, xmlSize))
            return false;

        return pf.writeChunkList ();
        */
    }
    
    pub fn save_preset_with_component_stream(
        &mut self, 
        stream:           *mut dyn IBStream,
        classid:          &FUID,
        component_stream: *mut dyn IBStream,
        edit_stream:      *mut dyn IBStream,
        xml_buffer:       *const u8,
        xml_size:         Option<i32>

    ) -> bool {

        let xml_size: i32 = xml_size.unwrap_or(-1);
        
        todo!();
        /*
            PresetFile pf (stream);
        pf.setClassID (classID);
        if (!pf.writeHeader ())
            return false;

        if (!pf.storeComponentState (componentStream))
            return false;

        if (editStream && !pf.storeControllerState (editStream))
            return false;

        if (xmlBuffer && !pf.writeMetaInfo (xmlBuffer, xmlSize))
            return false;

        return pf.writeChunkList ();
        */
    }
    
    /**
      | Shortcut helper to load preset with
      | component/controller state. classID
      | is the FUID of the component (processor)
      | part.
      |
      */
    pub fn load_preset(&mut self, 
        stream:               *mut dyn IBStream,
        classid:              &FUID,
        component:            *mut dyn VstIComponent,
        edit_controller:      *mut dyn IEditController,
        other_class_id_array: *mut Vec<FUID>) -> bool {
        
        todo!();
        /*
            PresetFile pf (stream);
        if (!pf.readChunkList ())
            return false;

        if (pf.getClassID () != classID)
        {
            if (otherClassIDArray)
            {
                // continue to load only if found in supported ID else abort load
                if (std::find (otherClassIDArray->begin (), otherClassIDArray->end (),
                               pf.getClassID ()) == otherClassIDArray->end ())
                    return false;
            }
            else
                return false;
        }

        if (!pf.restoreComponentState (component))
            return false;

        if (editController)
        {
            // assign component state to controller
            if (!pf.restoreComponentState (editController))
                return false;

            // restore controller-only state (if present)
            if (pf.contains (kControllerState) && !pf.restoreControllerState (editController))
                return false;
        }
        return true;
        */
    }
    
    /**
      | Constructor of Preset file based on
      | a stream
      |
      */
    pub fn new(stream: *mut dyn IBStream) -> Self {
    
        todo!();
        /*
        : stream(stream),
        : entry_count(0),

            memset (entries, 0, sizeof (entries));

        if (stream)
            stream->addRef ();
        */
    }
    
    /**
      | Returns an entry for a given chunk type.
      |
      */
    pub fn get_entry(&self, which: ChunkType) -> *const PresetFileEntry {
        
        todo!();
        /*
            const ChunkID& id = getChunkID (which);
        for (int32 i = 0; i < entryCount; i++)
            if (isEqualID (entries[i].id, id))
                return &entries[i];
        return nullptr;
        */
    }
    
    /**
      | Returns the last available entry.
      |
      */
    pub fn get_last_entry(&self) -> *const PresetFileEntry {
        
        todo!();
        /*
            return entryCount > 0 ? &entries[entryCount - 1] : nullptr;
        */
    }
    
    pub fn readid(&mut self, id: ChunkID) -> bool {
        
        todo!();
        /*
            int32 numBytesRead = 0;
        stream->read (id, sizeof (ChunkID), &numBytesRead);
        return numBytesRead == sizeof (ChunkID);
        */
    }
    
    pub fn writeid(&mut self, id: ChunkID) -> bool {
        
        todo!();
        /*
            int32 numBytesWritten = 0;
        stream->write ((void*)id, sizeof (ChunkID), &numBytesWritten);
        return numBytesWritten == sizeof (ChunkID);
        */
    }
    
    pub fn read_equalid(&mut self, id: ChunkID) -> bool {
        
        todo!();
        /*
            ChunkID temp = {0};
        return readID (temp) && isEqualID (temp, id);
        */
    }
    
    pub fn read_size(&mut self, size: &mut TSize) -> bool {
        
        todo!();
        /*
            int32 numBytesRead = 0;
        stream->read (&size, sizeof (TSize), &numBytesRead);
    #if BYTEORDER == kBigEndian
        SWAP_64 (size)
    #endif
        return numBytesRead == sizeof (TSize);
        */
    }
    
    pub fn write_size(&mut self, size: TSize) -> bool {
        
        todo!();
        /*
            #if BYTEORDER == kBigEndian
        SWAP_64 (size)
    #endif
        int32 numBytesWritten = 0;
        stream->write (&size, sizeof (TSize), &numBytesWritten);
        return numBytesWritten == sizeof (TSize);
        */
    }
    
    pub fn read_int32(&mut self, value: &mut i32) -> bool {
        
        todo!();
        /*
            int32 numBytesRead = 0;
        stream->read (&value, sizeof (int32), &numBytesRead);
    #if BYTEORDER == kBigEndian
        SWAP_32 (value)
    #endif
        return numBytesRead == sizeof (int32);
        */
    }
    
    pub fn write_int32(&mut self, value: i32) -> bool {
        
        todo!();
        /*
            #if BYTEORDER == kBigEndian
        SWAP_32 (value)
    #endif
        int32 numBytesWritten = 0;
        stream->write (&value, sizeof (int32), &numBytesWritten);
        return numBytesWritten == sizeof (int32);
        */
    }
    
    pub fn seek_to(&mut self, offset: TSize) -> bool {
        
        todo!();
        /*
            int64 result = -1;
        stream->seek (offset, IBStream::kIBSeekSet, &result);
        return result == offset;
        */
    }
    
    /**
      | Reads and build the chunk list (including
      | the header chunk).
      |
      */
    pub fn read_chunk_list(&mut self) -> bool {
        
        todo!();
        /*
            seekTo (0);
        entryCount = 0;

        char8 classString[kClassIDSize + 1] = {0};

        // Read header
        int32 version = 0;
        TSize listOffset = 0;
        if (!(readEqualID (getChunkID (kHeader)) && readInt32 (version) &&
              verify (stream->read (classString, kClassIDSize)) && readSize (listOffset) &&
              listOffset > 0 && seekTo (listOffset)))
            return false;

        classID.fromString (classString);

        // Read list
        int32 count = 0;
        if (!readEqualID (getChunkID (kChunkList)))
            return false;
        if (!readInt32 (count))
            return false;

        if (count > kMaxEntries)
            count = kMaxEntries;

        for (int32 i = 0; i < count; i++)
        {
            Entry& e = entries[i];
            if (!(readID (e.id) && readSize (e.offset) && readSize (e.size)))
                break;

            entryCount++;
        }

        return entryCount > 0;
        */
    }
    
    /**
      | Writes into the stream the main header.
      |
      */
    pub fn write_header(&mut self) -> bool {
        
        todo!();
        /*
            // header id + version + class id + list offset (unknown yet)

        char8 classString[kClassIDSize + 1] = {0};
        classID.toString (classString);

        return seekTo (0) && writeID (getChunkID (kHeader)) && writeInt32 (kFormatVersion) &&
               verify (stream->write (classString, kClassIDSize)) && writeSize (0);
        */
    }
    
    /**
      | Writes into the stream the chunk list
      | (should be at the end).
      |
      */
    pub fn write_chunk_list(&mut self) -> bool {
        
        todo!();
        /*
            // Update list offset
        TSize pos = 0;
        stream->tell (&pos);
        if (!(seekTo (kListOffsetPos) && writeSize (pos) && seekTo (pos)))
            return false;

        // Write list
        if (!writeID (getChunkID (kChunkList)))
            return false;
        if (!writeInt32 (entryCount))
            return false;

        for (int32 i = 0; i < entryCount; i++)
        {
            Entry& e = entries[i];
            if (!(writeID (e.id) && writeSize (e.offset) && writeSize (e.size)))
                return false;
        }
        return true;
        */
    }
    
    pub fn begin_chunk(
        &mut self, 
        e:     &mut PresetFileEntry,
        which: ChunkType

    ) -> bool {
        
        todo!();
        /*
            if (entryCount >= kMaxEntries)
            return false;

        const ChunkID& id = getChunkID (which);
        memcpy (e.id, &id, sizeof (ChunkID));
        stream->tell (&e.offset);
        e.size = 0;
        return true;
        */
    }
    
    pub fn end_chunk(&mut self, e: &mut PresetFileEntry) -> bool {
        
        todo!();
        /*
            if (entryCount >= kMaxEntries)
            return false;

        TSize pos = 0;
        stream->tell (&pos);
        e.size = pos - e.offset;
        entries[entryCount++] = e;
        return true;
        */
    }
    
    /**
      | Reads the meta XML info and its size,
      | the size could be retrieved by passing
      | zero as xmlBuffer.
      |
      */
    pub fn read_meta_info(&mut self, 
        xml_buffer: *mut u8,
        size:       &mut i32) -> bool {
        
        todo!();
        /*
            bool result = false;
        const Entry* e = getEntry (kMetaInfo);
        if (e)
        {
            if (xmlBuffer)
            {
                result = seekTo (e->offset) && verify (stream->read (xmlBuffer, size, &size));
            }
            else
            {
                size = (int32)e->size;
                result = size > 0;
            }
        }
        return result;
        */
    }
    
    /**
      | Writes the meta XML info, -1 means null-terminated,
      | forceWriting to true will force to rewrite
      | the XML Info when the chunk already exists.
      |
      */
    pub fn write_meta_info(
        &mut self, 
        xml_buffer:    *const u8,
        size:          Option<i32>,
        force_writing: Option<bool>

    ) -> bool {

        let size: i32 = size.unwrap_or(-1);
        let force_writing: bool = force_writing.unwrap_or(false);
        
        todo!();
        /*
            if (contains (kMetaInfo)) // already exists!
        {
            if (!forceWriting)
                return false;
        }
        if (!prepareMetaInfoUpdate ())
            return false;

        if (size == -1)
            size = (int32)strlen (xmlBuffer);

        Entry e = {};
        return beginChunk (e, kMetaInfo) && verify (stream->write ((void*)xmlBuffer, size)) &&
               endChunk (e);
        */
    }
    
    /**
      | checks if meta info chunk is the last
      | one and jump to correct position.
      |
      */
    pub fn prepare_meta_info_update(&mut self) -> bool {
        
        todo!();
        /*
            TSize writePos = 0;
        const Entry* e = getEntry (kMetaInfo);
        if (e)
        {
            // meta info must be the last entry!
            if (e != getLastEntry ())
                return false;

            writePos = e->offset;
            entryCount--;
        }
        else
        {
            // entries must be sorted ascending by offset!
            e = getLastEntry ();
            writePos = e ? e->offset + e->size : kHeaderSize;
        }

        return seekTo (writePos);
        */
    }
    
    /**
      | Writes a given data of a given size as
      | "which" chunk type.
      |
      */
    pub fn write_chunk(
        &mut self, 
        data:  *const c_void,
        size:  i32,
        which: Option<ChunkType>

    ) -> bool {

        let which: ChunkType = which.unwrap_or(ChunkType::ComponentState);
        
        todo!();
        /*
            if (contains (which)) // already exists!
            return false;

        Entry e = {};
        return beginChunk (e, which) && verify (stream->write ((void*)data, size)) && endChunk (e);
        */
    }
    
    /**
      | Seeks to the begin of the Component State.
      |
      */
    pub fn seek_to_component_state(&mut self) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kComponentState);
        return e && seekTo (e->offset);
        */
    }
    
    /**
      | Stores the component state (only one
      | time).
      |
      */
    pub fn store_component_state(&mut self, component: *mut dyn VstIComponent) -> bool {
        
        todo!();
        /*
            if (contains (kComponentState)) // already exists!
            return false;

        Entry e = {};
        return beginChunk (e, kComponentState) && verify (component->getState (stream)) && endChunk (e);
        */
    }
    
    /**
      | Stores the component state from stream
      | (only one time).
      |
      */
    pub fn store_component_state_from_stream(&mut self, component_stream: *mut dyn IBStream) -> bool {
        
        todo!();
        /*
            if (contains (kComponentState)) // already exists!
            return false;

        Entry e = {};
        return beginChunk (e, kComponentState) && copyStream (componentStream, stream) && endChunk (e);
        */
    }
    
    /**
      | Restores the component state.
      |
      */
    pub fn restore_component_state(&mut self, component: *mut dyn VstIComponent) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kComponentState);
        if (e)
        {
            auto* readOnlyBStream = new ReadOnlyBStream (stream, e->offset, e->size);
            FReleaser readOnlyBStreamReleaser (readOnlyBStream);
            return verify (component->setState (readOnlyBStream));
        }
        return false;
        */
    }
    
    /**
      | Restores the component state and apply
      | it to the controller.
      |
      */
    pub fn restore_component_state_and_apply_to_controller(&mut self, edit_controller: *mut dyn IEditController) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kComponentState);
        if (e)
        {
            auto* readOnlyBStream = new ReadOnlyBStream (stream, e->offset, e->size);
            FReleaser readOnlyBStreamReleaser (readOnlyBStream);
            return verify (editController->setComponentState (readOnlyBStream));
        }
        return false;
        */
    }
    
    /**
      | Seeks to the begin of the Controller
      | State.
      |
      */
    pub fn seek_to_controller_state(&mut self) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kControllerState);
        return e && seekTo (e->offset);
        */
    }
    
    /**
      | Stores the controller state (only one
      | time).
      |
      */
    pub fn store_controller_state(&mut self, edit_controller: *mut dyn IEditController) -> bool {
        
        todo!();
        /*
            if (contains (kControllerState)) // already exists!
            return false;

        Entry e = {};
        return beginChunk (e, kControllerState) && verify (editController->getState (stream)) &&
               endChunk (e);
        */
    }
    
    /**
      | Stores the controller state from stream
      | (only one time).
      |
      */
    pub fn store_controller_state_from_stream(&mut self, edit_stream: *mut dyn IBStream) -> bool {
        
        todo!();
        /*
            if (contains (kControllerState)) // already exists!
            return false;

        Entry e = {};
        return beginChunk (e, kControllerState) && copyStream (editStream, stream) && endChunk (e);
        */
    }
    
    /**
      | Restores the controller state.
      |
      */
    pub fn restore_controller_state(&mut self, edit_controller: *mut dyn IEditController) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kControllerState);
        if (e)
        {
            auto* readOnlyBStream = new ReadOnlyBStream (stream, e->offset, e->size);
            FReleaser readOnlyBStreamReleaser (readOnlyBStream);
            return verify (editController->setState (readOnlyBStream));
        }
        return false;
        */
    }
    
    /**
      | Store program data or unit data from
      | stream (including the header chunk).
      | 
      | -----------
      | @param inStream
      | 
      | \param listID could be ProgramListID
      | or UnitID.
      |
      */
    pub fn store_program_data_or_unit_data_from_stream(
        &mut self, 
        in_stream: *mut dyn IBStream,
        listid:    ProgramListID

    ) -> bool {
        
        todo!();
        /*
            if (contains (kProgramData)) // already exists!
            return false;

        writeHeader ();

        Entry e = {};
        if (beginChunk (e, kProgramData))
        {
            if (writeInt32 (listID))
            {
                if (!copyStream (inStream, stream))
                    return false;

                return endChunk (e);
            }
        }
        return false;
        */
    }
    
    /**
      | Stores a IProgramListData with a given
      | identifier and index (including the
      | header chunk).
      |
      */
    pub fn store_program_data_with_a_given_identifier_and_index(
        &mut self, 
        program_list_data: *mut dyn IProgramListData,
        listid:            ProgramListID,
        program_index:     i32

    ) -> bool {
        
        todo!();
        /*
            if (contains (kProgramData)) // already exists!
            return false;

        writeHeader ();

        Entry e = {};
        return beginChunk (e, kProgramData) && writeInt32 (listID) &&
               verify (programListData->getProgramData (listID, programIndex, stream)) && endChunk (e);
        */
    }
    
    /**
      | Restores a IProgramListData with a
      | given identifier and index.
      |
      */
    pub fn restore_program_data_with_identifier_and_index(
        &mut self, 
        program_list_data: *mut dyn IProgramListData,
        program_listid:    *mut ProgramListID,
        program_index:     Option<i32>

    ) -> bool {

        let program_index: i32 = program_index.unwrap_or(0);
        
        todo!();
        /*
            const Entry* e = getEntry (kProgramData);
        ProgramListID savedProgramListID = -1;
        if (e && seekTo (e->offset))
        {
            if (readInt32 (savedProgramListID))
            {
                if (programListID && *programListID != savedProgramListID)
                    return false;

                int32 alreadyRead = sizeof (int32);
                auto* readOnlyBStream =
                    new ReadOnlyBStream (stream, e->offset + alreadyRead, e->size - alreadyRead);
                FReleaser readOnlyBStreamReleaser (readOnlyBStream);
                return programListData && verify (programListData->setProgramData (
                                              savedProgramListID, programIndex, readOnlyBStream));
            }
        }
        return false;
        */
    }
    
    /**
      | Stores a IUnitData with a given unitID
      | (including the header chunk).
      |
      */
    pub fn store_program_data(
        &mut self, 
        unit_data: *mut dyn IUnitData,
        unitid:    UnitID

    ) -> bool {
        
        todo!();
        /*
            if (contains (kProgramData)) // already exists!
            return false;

        writeHeader ();

        Entry e = {};
        return beginChunk (e, kProgramData) && writeInt32 (unitID) &&
               verify (unitData->getUnitData (unitID, stream)) && endChunk (e);
        */
    }
    
    /**
      | Restores a IUnitData with a given unitID
      | (optional).
      |
      */
    pub fn restore_program_data_with_unit_id(
        &mut self, 
        unit_data: *mut dyn IUnitData,
        unit_id:   *mut UnitID) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kProgramData);
        UnitID savedUnitID = -1;
        if (e && seekTo (e->offset))
        {
            if (readInt32 (savedUnitID))
            {
                if (unitId && *unitId != savedUnitID)
                    return false;

                int32 alreadyRead = sizeof (int32);
                auto* readOnlyBStream =
                    new ReadOnlyBStream (stream, e->offset + alreadyRead, e->size - alreadyRead);
                FReleaser readOnlyStreamReleaser (readOnlyBStream);
                return (unitData && verify (unitData->setUnitData (savedUnitID, readOnlyBStream)));
            }
        }
        return false;
        */
    }
    
    /**
      | for keeping the controller part in sync
      | concerning preset data stream, unitProgramListID
      | could be ProgramListID or UnitID.
      |
      */
    pub fn restore_program_data(
        &mut self, 
        unit_info:           *mut dyn IUnitInfo,
        unit_program_listid: i32,
        program_index:       Option<i32>

    ) -> bool {

        let program_index: i32 = program_index.unwrap_or(-1);
        
        todo!();
        /*
            const Entry* e = getEntry (kProgramData);
        int32 savedProgramListID = -1;
        if (e && seekTo (e->offset))
        {
            if (readInt32 (savedProgramListID))
            {
                if (unitProgramListID != savedProgramListID)
                    return false;

                int32 alreadyRead = sizeof (int32);
                auto* readOnlyBStream =
                    new ReadOnlyBStream (stream, e->offset + alreadyRead, e->size - alreadyRead);
                FReleaser readOnlyStreamReleaser (readOnlyBStream);
                return (unitInfo && unitInfo->setUnitProgramData (unitProgramListID, programIndex,
                                                                  readOnlyBStream));
            }
        }
        return false;
        */
    }
    
    /**
      | Gets the unitProgramListID saved in
      | the kProgramData chunk (if available).
      |
      */
    pub fn get_unit_program_listid(&mut self, unit_program_listid: &mut i32) -> bool {
        
        todo!();
        /*
            const Entry* e = getEntry (kProgramData);
        if (e && seekTo (e->offset))
        {
            if (readInt32 (unitProgramListID))
            {
                return true;
            }
        }
        return false;
        */
    }

    /**
      | Returns the associated stream.
      |
      */
    pub fn get_stream(&mut self) -> *mut dyn IBStream {
        
        todo!();
        /*
            return stream;
        */
    }

    /**
      | Returns the associated classID (component
      | ID: Processor part (not the controller!)).
      |
      */
    pub fn get_classid(&self) -> &FUID {
        
        todo!();
        /*
            return classID;
        */
    }

    /**
      | Sets the associated classID (component
      | ID: Processor part (not the controller!)).
      |
      */
    pub fn set_classid(&mut self, uid: &FUID)  {
        
        todo!();
        /*
            classID = uid;
        */
    }

    /**
      | Returns the number of total entries
      | in the current stream.
      |
      */
    pub fn get_entry_count(&self) -> i32 {
        
        todo!();
        /*
            return entryCount;
        */
    }

    /**
      | Returns the entry at a given position.
      |
      */
    pub fn at(&self, index: i32) -> &PresetFileEntry {
        
        todo!();
        /*
            return entries[index];
        */
    }

    /**
      | Checks if a given chunk type exist in
      | the stream.
      |
      */
    pub fn contains(&self, which: ChunkType) -> bool {
        
        todo!();
        /*
            return getEntry (which) != nullptr;
        */
    }

    /*
       for storing and restoring the whole plug-in
       state (component and controller states)
      */

    /* ---when plug-in uses IProgramListData----------------------- */

    /* ---when plug-in uses IUnitData------------------------------ */
}
