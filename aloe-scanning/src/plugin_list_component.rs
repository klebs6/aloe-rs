crate::ix!();

pub fn can_show_folder_for_plugin(
        list:  &mut KnownPluginList,
        index: i32) -> bool {
    
    todo!();
        /*
            return File::createFileWithoutCheckingPath (list.getTypes()[index].fileOrIdentifier).exists();
        */
}

pub fn show_folder_for_plugin(
        list:  &mut KnownPluginList,
        index: i32)  {
    
    todo!();
        /*
            if (canShowFolderForPlugin (list, index))
            File (list.getTypes()[index].fileOrIdentifier).revealToUser();
        */
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_PluginListComponent.h]

/**
  | A component displaying a list of plugins,
  | with options to scan for them, add, remove
  | and sort them.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct PluginListComponent<'a> {
    base:                 Component<'a>,
    format_manager:       &'a mut AudioPluginFormatManager,
    list:                 &'a mut KnownPluginList<'a>,
    dead_mans_pedal_file: File,
    table:                TableListBox<'a>,
    options_button:       TextButton<'a>,
    properties_to_use:    *mut PropertiesFile<'a>,
    dialog_title:         String,
    dialog_text:          String,
    allow_async:          bool,
    num_threads:          i32,
    table_model:          Box<dyn TableListBoxModel>,
    current_scanner:      Box<PluginListComponentScanner<'a>>,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/scanning/aloe_PluginListComponent.cpp]
impl<'a> ChangeListener for PluginListComponent<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            table.getHeader().reSortTable();
        updateList();
        */
    }
}

impl<'a> FileDragAndDropTarget for PluginListComponent<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for PluginListComponent<'a> {

}

impl<'a> ItemDropped for PluginListComponent<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) { 
        todo!() 
    }
}

impl<'a> ItemDragExit for PluginListComponent<'a> {

}

impl<'a> ItemDragMove for PluginListComponent<'a> {

}

impl<'a> ItemDragEnter for PluginListComponent<'a> {

}

impl<'a> IsInterestedInDragSource for PluginListComponent<'a> {

    fn is_interested_in_drag_source(&mut self, _: &DragAndDropTargetSourceDetails<'_>) -> bool { 
        todo!() 
    }
}

impl<'a> Drop for PluginListComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            list.removeChangeListener (this);
        */
    }
}

impl<'a> PluginListComponent<'a> {

    /**
      | Returns the table used to display the
      | plugin list.
      |
      */
    pub fn get_table_list_box(&mut self) -> &mut TableListBox {
        
        todo!();
        /*
            return table;
        */
    }

    /**
      | Returns the button used to display the
      | options menu - you can make this invisible
      | if you want to hide it and use some other
      | method for showing the menu.
      |
      */
    pub fn get_options_button(&mut self) -> &mut TextButton {
        
        todo!();
        /*
            return optionsButton;
        */
    }
    
    /**
      | Creates the list component.
      | 
      | For info about the deadMansPedalFile,
      | see the PluginDirectoryScanner constructor.
      | 
      | The properties file, if supplied, is
      | used to store the user's last search
      | paths.
      |
      */
    pub fn new(
        manager:                                                &mut AudioPluginFormatManager,
        list_to_edit:                                           &mut KnownPluginList,
        dead_mans_pedal:                                        &File,
        props:                                                  *mut PropertiesFile,
        allow_plugins_which_require_asynchronous_instantiation: Option<bool>

    ) -> Self {

        let allow_plugins_which_require_asynchronous_instantiation: bool =
                 allow_plugins_which_require_asynchronous_instantiation.unwrap_or(false);
    
        todo!();
        /*


            : formatManager (manager),
          list (listToEdit),
          deadMansPedalFile (deadMansPedal),
          optionsButton ("Options..."),
          propertiesToUse (props),
          allowAsync (allowPluginsWhichRequireAsynchronousInstantiation),
          numThreads (allowAsync ? 1 : 0)

        tableModel.reset (new PluginListComponentTableModel (*this, listToEdit));

        TableHeaderComponent& header = table.getHeader();

        header.addColumn (TRANS("Name"),         PluginListComponentTableModel::nameCol,         200, 100, 700, TableHeaderComponent::defaultFlags | TableHeaderComponent::sortedForwards);
        header.addColumn (TRANS("Format"),       PluginListComponentTableModel::typeCol,         80, 80, 80,    TableHeaderComponent::notResizable);
        header.addColumn (TRANS("Category"),     PluginListComponentTableModel::categoryCol,     100, 100, 200);
        header.addColumn (TRANS("Manufacturer"), PluginListComponentTableModel::manufacturerCol, 200, 100, 300);
        header.addColumn (TRANS("Description"),  PluginListComponentTableModel::descCol,         300, 100, 500, TableHeaderComponent::notSortable);

        table.setHeaderHeight (22);
        table.setRowHeight (20);
        table.setModel (tableModel.get());
        table.setMultipleSelectionEnabled (true);
        addAndMakeVisible (table);

        addAndMakeVisible (optionsButton);
        optionsButton.onClick = [this]
        {
            createOptionsMenu().showMenuAsync (typename PopupMenu::Options()
                                                  .withDeletionCheck (*this)
                                                  .withTargetComponent (optionsButton));
        };

        optionsButton.setTriggeredOnMouseDown (true);

        setSize (400, 600);
        list.addChangeListener (this);
        updateList();
        table.getHeader().reSortTable();

        PluginDirectoryScanner::applyBlacklistingsFromDeadMansPedal (list, deadMansPedalFile);
        deadMansPedalFile.deleteFile();
        */
    }
    
    /**
      | Changes the text in the panel's options
      | button.
      |
      */
    pub fn set_options_button_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            optionsButton.setButtonText (newText);
        resized();
        */
    }
    
    /**
      | Changes the text in the progress dialog
      | box that is shown when scanning.
      |
      */
    pub fn set_scan_dialog_text(&mut self, 
        title:   &String,
        content: &String)  {
        
        todo!();
        /*
            dialogTitle = title;
        dialogText = content;
        */
    }
    
    /**
      | Sets how many threads to simultaneously
      | scan for plugins.
      | 
      | If this is 0, then all scanning happens
      | on the message thread (this is the default
      | when allowPluginsWhichRequireAsynchronousInstantiation
      | is false). If allowPluginsWhichRequireAsynchronousInstantiation
      | is true then numThreads must not be zero
      | (it is one by default).
      |
      */
    pub fn set_number_of_threads_for_scanning(&mut self, num: i32)  {
        
        todo!();
        /*
            numThreads = num;
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds().reduced (2);

        if (optionsButton.isVisible())
        {
            optionsButton.setBounds (r.removeFromBottom (24));
            optionsButton.changeWidthToFitText (24);
            r.removeFromBottom (3);
        }

        table.setBounds (r);
        */
    }
    
    pub fn update_list(&mut self)  {
        
        todo!();
        /*
            table.updateContent();
        table.repaint();
        */
    }
    
    /**
      | Removes the plugins currently selected
      | in the table.
      |
      */
    pub fn remove_selected_plugins(&mut self)  {
        
        todo!();
        /*
            auto selected = table.getSelectedRows();

        for (int i = table.getNumRows(); --i >= 0;)
            if (selected.contains (i))
                removePluginItem (i);
        */
    }
    
    /**
      | Sets a custom table model to be used.
      | 
      | This will take ownership of the model
      | and delete it when no longer needed.
      |
      */
    pub fn set_table_model(&mut self, model: *mut dyn TableListBoxModel)  {
        
        todo!();
        /*
            table.setModel (nullptr);
        tableModel.reset (model);
        table.setModel (tableModel.get());

        table.getHeader().reSortTable();
        table.updateContent();
        table.repaint();
        */
    }
    
    pub fn remove_missing_plugins(&mut self)  {
        
        todo!();
        /*
            auto types = list.getTypes();

        for (int i = types.size(); --i >= 0;)
        {
            auto type = types.getUnchecked (i);

            if (! formatManager.doesPluginStillExist (type))
                list.removeType (type);
        }
        */
    }
    
    pub fn remove_plugin_item(&mut self, index: i32)  {
        
        todo!();
        /*
            if (index < list.getNumTypes())
            list.removeType (list.getTypes()[index]);
        else
            list.removeFromBlacklist (list.getBlacklistedFiles() [index - list.getNumTypes()]);
        */
    }
    
    /**
      | Returns a pop-up menu that contains
      | all the options for scanning and updating
      | the list.
      |
      */
    pub fn create_options_menu(&mut self) -> PopupMenu {
        
        todo!();
        /*
            PopupMenu menu;
        menu.addItem (typename PopupMenu::Item (TRANS("Clear list"))
                        .setAction ([this] { list.clear(); }));

        menu.addSeparator();

        for (auto format : formatManager.getFormats())
            if (format->canScanForPlugins())
                menu.addItem (typename PopupMenu::Item ("Remove all " + format->getName() + " plug-ins")
                                .setEnabled (! list.getTypesForFormat (*format).isEmpty())
                                .setAction ([this, format]
                                            {
                                                for (auto& pd : list.getTypesForFormat (*format))
                                                    list.removeType (pd);
                                            }));

        menu.addSeparator();

        menu.addItem (typename PopupMenu::Item (TRANS("Remove selected plug-in from list"))
                        .setEnabled (table.getNumSelectedRows() > 0)
                        .setAction ([this] { removeSelectedPlugins(); }));

        menu.addItem (typename PopupMenu::Item (TRANS("Remove any plug-ins whose files no longer exist"))
                        .setAction ([this] { removeMissingPlugins(); }));

        menu.addSeparator();

        auto selectedRow = table.getSelectedRow();

        menu.addItem (typename PopupMenu::Item (TRANS("Show folder containing selected plug-in"))
                        .setEnabled (canShowFolderForPlugin (list, selectedRow))
                        .setAction ([this, selectedRow] { showFolderForPlugin (list, selectedRow); }));

        menu.addSeparator();

        for (auto format : formatManager.getFormats())
            if (format->canScanForPlugins())
                menu.addItem (typename PopupMenu::Item ("Scan for new or updated " + format->getName() + " plug-ins")
                                .setAction ([this, format]  { scanFor (*format); }));

        return menu;
        */
    }
    
    /**
      | Returns a menu that can be shown if a row
      | is right-clicked, containing actions
      | like "remove plugin" or "show folder"
      | etc.
      |
      */
    pub fn create_menu_for_row(&mut self, row_number: i32) -> PopupMenu {
        
        todo!();
        /*
            PopupMenu menu;

        if (rowNumber >= 0 && rowNumber < tableModel->getNumRows())
        {
            menu.addItem (typename PopupMenu::Item (TRANS("Remove plug-in from list"))
                            .setAction ([this, rowNumber] { removePluginItem (rowNumber); }));

            menu.addItem (typename PopupMenu::Item (TRANS("Show folder containing plug-in"))
                            .setEnabled (canShowFolderForPlugin (list, rowNumber))
                            .setAction ([this, rowNumber] { showFolderForPlugin (list, rowNumber); }));
        }

        return menu;
        */
    }
    
    pub fn is_interested_in_file_drag(&mut self, files: &StringArray) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    pub fn files_dropped(&mut self, 
        files: &StringArray,
        _1:    i32,
        _2:    i32)  {
        
        todo!();
        /*
            Vec<Box<PluginDescription>> typesFound;
        list.scanAndAddDragAndDroppedFiles (formatManager, files, typesFound);
        */
    }
    
    /**
      | Returns the last search path stored
      | in a given properties file for the specified
      | format.
      |
      */
    pub fn get_last_search_path(&mut self, 
        properties: &mut PropertiesFile,
        format:     &mut AudioPluginFormat) -> FileSearchPath {
        
        todo!();
        /*
            auto key = "lastPluginScanPath_" + format.getName();

        if (properties.containsKey (key) && properties.getValue (key, {}).trim().isEmpty())
            properties.removeValue (key);

        return FileSearchPath (properties.getValue (key, format.getDefaultLocationsToSearch().toString()));
        */
    }
    
    /**
      | Stores a search path in a properties
      | file for the given format.
      |
      */
    pub fn set_last_search_path(&mut self, 
        properties: &mut PropertiesFile,
        format:     &mut AudioPluginFormat,
        new_path:   &FileSearchPath)  {
        
        todo!();
        /*
            auto key = "lastPluginScanPath_" + format.getName();

        if (newPath.getNumPaths() == 0)
            properties.removeValue (key);
        else
            properties.setValue (key, newPath.toString());
        */
    }
    
    /**
      | Triggers an asynchronous scan for the
      | given format.
      |
      */
    pub fn scan_for(&mut self, format: &mut AudioPluginFormat)  {
        
        todo!();
        /*
            scanFor (format, StringArray());
        */
    }
    
    /**
      | Triggers an asynchronous scan for the
      | given format and scans only the given
      | files or identifiers. @see AudioPluginFormat::searchPathsForPlugins
      |
      */
    pub fn scan_for_files_or_identifiers(
        &mut self, 
        format:                       &mut AudioPluginFormat,
        files_or_identifiers_to_scan: &StringArray

    ) {
        
        todo!();
        /*
            currentScanner.reset (new PluginListComponentScanner (*this, format, filesOrIdentifiersToScan, propertiesToUse, allowAsync, numThreads,
                                           dialogTitle.isNotEmpty() ? dialogTitle : TRANS("Scanning for plug-ins..."),
                                           dialogText.isNotEmpty()  ? dialogText  : TRANS("Searching for all possible plug-in files...")));
        */
    }
    
    /**
      | Returns true if there's currently a
      | scan in progress.
      |
      */
    pub fn is_scanning(&self) -> bool {
        
        todo!();
        /*
            return currentScanner != nullptr;
        */
    }
    
    pub fn scan_finished(&mut self, failed_files: &StringArray)  {
        
        todo!();
        /*
            StringArray shortNames;

        for (auto& f : failedFiles)
            shortNames.add (File::createFileWithoutCheckingPath (f).getFileName());

        currentScanner.reset(); // mustn't delete this before using the failed files array

        if (shortNames.size() > 0)
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon,
                                              TRANS("Scan complete"),
                                              TRANS("Note that the following files appeared to be plugin files, but failed to load correctly")
                                                + ":\n\n"
                                                + shortNames.joinIntoString (", "));
        */
    }
}
