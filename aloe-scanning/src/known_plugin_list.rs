crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_KnownPluginList.h]

/**
  | Manages a list of plugin types.
  | 
  | This can be easily edited, saved and
  | loaded, and used to create instances
  | of the plugin types in it.
  | 
  | @see PluginListComponent
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct KnownPluginList<'a> {
    base:             ChangeBroadcaster<'a>,
    types:            Vec<PluginDescription>,
    blacklist:        StringArray,
    scanner:          Box<KnownPluginListCustomScanner>,
    scan_lock:        CriticalSection,
    types_array_lock: CriticalSection,
}

impl<'a> Default for KnownPluginList<'a> {

    fn default() -> Self {
        todo!();
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_KnownPluginList.cpp]
impl<'a> KnownPluginList<'a> {
    
    /**
      | Clears the list.
      |
      */
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            ScopedLock lock (typesArrayLock);

        if (! types.isEmpty())
        {
            types.clear();
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Returns the number of types currently
      | in the list.
      |
      */
    pub fn get_num_types(&self) -> i32 {
        
        todo!();
        /*
            ScopedLock lock (typesArrayLock);
        return types.size();
        */
    }
    
    /**
      | Returns a copy of the current list.
      |
      */
    pub fn get_types(&self) -> Vec<PluginDescription> {
        
        todo!();
        /*
            ScopedLock lock (typesArrayLock);
        return types;
        */
    }
    
    /**
      | Returns the subset of plugin types for
      | a given format.
      |
      */
    pub fn get_types_for_format(&self, format: &mut AudioPluginFormat) -> Vec<PluginDescription> {
        
        todo!();
        /*
            Vec<PluginDescription> result;

        for (auto& d : getTypes())
            if (d.pluginFormatName == format.getName())
                result.add (d);

        return result;
        */
    }
    
    /**
      | Looks for a type in the list which comes
      | from this file.
      |
      */
    pub fn get_type_for_file(&self, file_or_identifier: &String) -> Box<PluginDescription> {
        
        todo!();
        /*
            ScopedLock lock (typesArrayLock);

        for (auto& desc : types)
            if (desc.fileOrIdentifier == fileOrIdentifier)
                return std::make_unique<PluginDescription> (desc);

        return {};
        */
    }
    
    /**
      | Looks for a type in the list which matches
      | a plugin type ID.
      | 
      | The identifierString parameter must
      | have been created by
      | 
      | PluginDescription::createIdentifierString().
      |
      */
    pub fn get_type_for_identifier_string(&self, identifier_string: &String) -> Box<PluginDescription> {
        
        todo!();
        /*
            ScopedLock lock (typesArrayLock);

        for (auto& desc : types)
            if (desc.matchesIdentifierString (identifierString))
                return std::make_unique<PluginDescription> (desc);

        return {};
        */
    }
    
    /**
      | Adds a type manually from its description.
      |
      */
    pub fn add_type(&mut self, ty: &PluginDescription) -> bool {
        
        todo!();
        /*
            {
            ScopedLock lock (typesArrayLock);

            for (auto& desc : types)
            {
                if (desc.isDuplicateOf (type))
                {
                    // strange - found a duplicate plugin with different info..
                    jassert (desc.name == type.name);
                    jassert (desc.isInstrument == type.isInstrument);

                    desc = type;
                    return false;
                }
            }

            types.insert (0, type);
        }

        sendChangeMessage();
        return true;
        */
    }
    
    /**
      | Removes a type.
      |
      */
    pub fn remove_type(&mut self, ty: &PluginDescription)  {
        
        todo!();
        /*
            {
            ScopedLock lock (typesArrayLock);

            for (int i = types.size(); --i >= 0;)
                if (types.getUnchecked (i).isDuplicateOf (type))
                    types.remove (i);
        }

        sendChangeMessage();
        */
    }
    
    /**
      | Returns true if the specified file is
      | already known about and if it hasn't
      | been modified since our entry was created.
      |
      */
    pub fn is_listing_up_to_date(&self, 
        file_or_identifier: &String,
        format_to_use:      &mut AudioPluginFormat) -> bool {
        
        todo!();
        /*
            if (getTypeForFile (fileOrIdentifier) == nullptr)
            return false;

        ScopedLock lock (typesArrayLock);

        for (auto& d : types)
            if (d.fileOrIdentifier == fileOrIdentifier && formatToUse.pluginNeedsRescanning (d))
                return false;

        return true;
        */
    }
    
    /**
      | Supplies a custom scanner to be used
      | in future scans.
      | 
      | The KnownPluginList will take ownership
      | of the object passed in.
      |
      */
    pub fn set_custom_scanner(&mut self, new_scanner: Box<KnownPluginListCustomScanner>)  {
        
        todo!();
        /*
            if (scanner != newScanner)
            scanner = std::move (newScanner);
        */
    }
    
    /**
      | Looks for all types that can be loaded
      | from a given file, and adds them to the
      | list.
      | 
      | If dontRescanIfAlreadyInList is true,
      | then the file will only be loaded and
      | re-tested if it's not already in the
      | list, or if the file's modification
      | time has changed since the list was created.
      | If dontRescanIfAlreadyInList is false,
      | the file will always be reloaded and
      | tested.
      | 
      | Returns true if any new types were added,
      | and all the types found in this file (even
      | if it was already known and hasn't been
      | re-scanned) get returned in the array.
      |
      */
    pub fn scan_and_add_file(&mut self, 
        file_or_identifier:             &String,
        dont_rescan_if_already_in_list: bool,
        types_found:                    &mut Vec<Box<PluginDescription>>,
        format:                         &mut AudioPluginFormat) -> bool {
        
        todo!();
        /*
            const ScopedLock sl (scanLock);

        if (dontRescanIfAlreadyInList
             && getTypeForFile (fileOrIdentifier) != nullptr)
        {
            bool needsRescanning = false;

            ScopedLock lock (typesArrayLock);

            for (auto& d : types)
            {
                if (d.fileOrIdentifier == fileOrIdentifier && d.pluginFormatName == format.getName())
                {
                    if (format.pluginNeedsRescanning (d))
                        needsRescanning = true;
                    else
                        typesFound.add (new PluginDescription (d));
                }
            }

            if (! needsRescanning)
                return false;
        }

        if (blacklist.contains (fileOrIdentifier))
            return false;

        Vec<Box<PluginDescription>> found;

        {
            const ScopedUnlock sl2 (scanLock);

            if (scanner != nullptr)
            {
                if (! scanner->findPluginTypesFor (format, found, fileOrIdentifier))
                    addToBlacklist (fileOrIdentifier);
            }
            else
            {
                format.findAllTypesForFile (found, fileOrIdentifier);
            }
        }

        for (auto* desc : found)
        {
            if (desc == nullptr)
            {
                jassertfalse;
                continue;
            }

            addType (*desc);
            typesFound.add (new PluginDescription (*desc));
        }

        return ! found.isEmpty();
        */
    }
    
    /**
      | Scans and adds a bunch of files that might
      | have been dragged-and-dropped.
      | 
      | If any types are found in the files, their
      | descriptions are returned in the array.
      |
      */
    pub fn scan_and_add_drag_and_dropped_files(&mut self, 
        format_manager: &mut AudioPluginFormatManager,
        files:          &StringArray,
        types_found:    &mut Vec<Box<PluginDescription>>)  {
        
        todo!();
        /*
            for (const auto& filenameOrID : files)
        {
            bool found = false;

            for (auto format : formatManager.getFormats())
            {
                if (format->fileMightContainThisPluginType (filenameOrID)
                     && scanAndAddFile (filenameOrID, true, typesFound, *format))
                {
                    found = true;
                    break;
                }
            }

            if (! found)
            {
                File f (filenameOrID);

                if (f.isDirectory())
                {
                    StringArray s;

                    for (auto& subFile : f.findChildFiles (File::findFilesAndDirectories, false))
                        s.add (subFile.getFullPathName());

                    scanAndAddDragAndDroppedFiles (formatManager, s, typesFound);
                }
            }
        }

        scanFinished();
        */
    }
    
    /**
      | Tells a custom scanner that a scan has
      | finished, and it can release any resources.
      |
      */
    pub fn scan_finished(&mut self)  {
        
        todo!();
        /*
            if (scanner != nullptr)
            scanner->scanFinished();
        */
    }
    
    /**
      | Returns the list of blacklisted files.
      |
      */
    pub fn get_blacklisted_files(&self) -> &StringArray {
        
        todo!();
        /*
            return blacklist;
        */
    }
    
    /**
      | Adds a plugin ID to the black-list.
      |
      */
    pub fn add_to_blacklist(&mut self, pluginid: &String)  {
        
        todo!();
        /*
            if (! blacklist.contains (pluginID))
        {
            blacklist.add (pluginID);
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Removes a plugin ID from the black-list.
      |
      */
    pub fn remove_from_blacklist(&mut self, pluginid: &String)  {
        
        todo!();
        /*
            const int index = blacklist.indexOf (pluginID);

        if (index >= 0)
        {
            blacklist.remove (index);
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Clears all the blacklisted files.
      |
      */
    pub fn clear_blacklisted_files(&mut self)  {
        
        todo!();
        /*
            if (blacklist.size() > 0)
        {
            blacklist.clear();
            sendChangeMessage();
        }
        */
    }
    
    /**
      | Sorts the list.
      |
      */
    pub fn sort(&mut self, 
        method:   KnownPluginListSortMethod,
        forwards: bool)  {
        
        todo!();
        /*
            if (method != defaultOrder)
        {
            Vec<PluginDescription> oldOrder, newOrder;

            {
                ScopedLock lock (typesArrayLock);

                oldOrder.addArray (types);
                std::stable_sort (types.begin(), types.end(), PluginSorter (method, forwards));
                newOrder.addArray (types);
            }

            auto hasOrderChanged = [&]
            {
                for (int i = 0; i < oldOrder.size(); ++i)
                     if (! oldOrder[i].isDuplicateOf (newOrder[i]))
                         return true;

                return false;
            }();

            if (hasOrderChanged)
                sendChangeMessage();
        }
        */
    }
    
    /**
      | Creates some XML that can be used to store
      | the state of this list.
      |
      */
    pub fn create_xml(&self) -> Box<XmlElement> {
        
        todo!();
        /*
            auto e = std::make_unique<XmlElement> ("KNOWNPLUGINS");

        {
            ScopedLock lock (typesArrayLock);

            for (int i = types.size(); --i >= 0;)
                e->prependChildElement (types.getUnchecked (i).createXml().release());
        }

        for (auto& b : blacklist)
            e->createNewChildElement ("BLACKLISTED")->setAttribute ("id", b);

        return e;
        */
    }
    
    /**
      | Recreates the state of this list from
      | its stored XML format.
      |
      */
    pub fn recreate_from_xml(&mut self, xml: &XmlElement)  {
        
        todo!();
        /*
            clear();
        clearBlacklistedFiles();

        if (xml.hasTagName ("KNOWNPLUGINS"))
        {
            for (auto* e : xml.getChildIterator())
            {
                PluginDescription info;

                if (e->hasTagName ("BLACKLISTED"))
                    blacklist.add (e->getStringAttribute ("id"));
                else if (info.loadFromXml (*e))
                    addType (info);
            }
        }
        */
    }
    
    /**
      | Creates a KnownPluginListPluginTree object representing
      | the list of plug-ins.
      |
      */
    pub fn create_tree_with_types(
        &mut self, 
        types:       &[PluginDescription],
        sort_method: KnownPluginListSortMethod

    ) -> Box<KnownPluginListPluginTree> {
        
        todo!();
        /*
            Vec<PluginDescription> sorted;
        sorted.addArray (types);

        std::stable_sort (sorted.begin(), sorted.end(), PluginSorter (sortMethod, true));

        auto tree = std::make_unique<KnownPluginListPluginTree>();

        if (sortMethod == sortByCategory || sortMethod == sortByManufacturer || sortMethod == sortByFormat)
        {
            PluginTreeUtils::buildTreeByCategory (*tree, sorted, sortMethod);
        }
        else if (sortMethod == sortByFileSystemLocation)
        {
            PluginTreeUtils::buildTreeByFolder (*tree, sorted);
        }
        else
        {
            for (auto& p : sorted)
                tree->plugins.add (p);
        }

        return tree;
        */
    }
    
    /**
      | Adds the plug-in types to a popup menu
      | so that the user can select one.
      | 
      | Depending on the sort method, it may
      | add sub-menus for categories, manufacturers,
      | etc.
      | 
      | Use getIndexChosenByMenu() to find
      | out the type that was chosen.
      |
      */
    pub fn add_these_types_to_menu(
        &mut self, 
        menu:                      &mut PopupMenu,
        types:                     &[PluginDescription],
        sort_method:               KnownPluginListSortMethod,
        currently_ticked_pluginid: &String

    ) {
        
        todo!();
        /*
            auto tree = createTree (types, sortMethod);
        PluginTreeUtils::addToMenu (*tree, menu, types, currentlyTickedPluginID);
        */
    }
    
    /**
      | Converts a menu item index that has been
      | chosen into its index in the list.
      | 
      | Returns -1 if it's not an ID that was used.
      | @see addToMenu
      |
      */
    pub fn get_index_chosen_by_menu_with_types(
        &mut self, 
        types:            &[PluginDescription],
        menu_result_code: i32

    ) -> i32 {
        
        todo!();
        /*
            auto i = menuResultCode - PluginTreeUtils::menuIdBase;
        return isPositiveAndBelow (i, types.size()) ? i : -1;
        */
    }
    
    pub fn add_to_menu(&self, 
        menu:                      &mut PopupMenu,
        sort_method:               KnownPluginListSortMethod,
        currently_ticked_pluginid: &String)  {
        
        todo!();
        /*
            addToMenu (menu, getTypes(), sortMethod, currentlyTickedPluginID);
        */
    }
    
    pub fn get_index_chosen_by_menu(&self, menu_result_code: i32) -> i32 {
        
        todo!();
        /*
            return getIndexChosenByMenu (getTypes(), menuResultCode);
        */
    }
    
    pub fn create_tree(&self, sort_method: KnownPluginListSortMethod) -> Box<KnownPluginListPluginTree> {
        
        todo!();
        /*
            return createTree (getTypes(), sortMethod);
        */
    }
}
