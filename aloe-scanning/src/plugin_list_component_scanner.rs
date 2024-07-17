crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PluginListComponentScanner<'a> {
    base:                         Timer,
    owner:                        &'a mut PluginListComponent<'a>,
    format_to_scan:               &'a mut AudioPluginFormat,
    files_or_identifiers_to_scan: StringArray,
    properties_to_use:            *mut PropertiesFile<'a>,
    scanner:                      Box<PluginDirectoryScanner<'a>>,
    path_chooser_window:          AlertWindow<'a>,
    progress_window:              AlertWindow<'a>,
    path_list:                    FileSearchPathListComponent<'a>,
    plugin_being_scanned:         String,
    progress:                     f64, // default = 0
    num_threads:                  i32,
    allow_async:                  bool,
    finished:                     bool, // default = false
    timer_reentrancy_check:       bool, // default = false
    pool:                         Box<ThreadPool<'a>>,
}

impl<'a> Drop for PluginListComponentScanner<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            if (pool != nullptr)
                {
                    pool->removeAllJobs (true, 60000);
                    pool.reset();
                }
        */
    }
}

impl<'a> PluginListComponentScanner<'a> {

    pub fn new(
        plc:                                                    &mut PluginListComponent,
        format:                                                 &mut AudioPluginFormat,
        files_or_identifiers:                                   &StringArray,
        properties:                                             *mut PropertiesFile,
        allow_plugins_which_require_asynchronous_instantiation: bool,
        threads:                                                i32,
        title:                                                  &String,
        text:                                                   &String) -> Self {
    
        todo!();
        /*


            : owner (plc), formatToScan (format), filesOrIdentifiersToScan (filesOrIdentifiers), propertiesToUse (properties),
                  pathChooserWindow (TRANS("Select folders to scan..."), String(), MessageBoxIconType::NoIcon),
                  progressWindow (title, text, MessageBoxIconType::NoIcon),
                  numThreads (threads), allowAsync (allowPluginsWhichRequireAsynchronousInstantiation)

                FileSearchPath path (formatToScan.getDefaultLocationsToSearch());

                // You need to use at least one thread when scanning plug-ins asynchronously
                jassert (! allowAsync || (numThreads > 0));

                // If the filesOrIdentifiersToScan argument isn't empty, we should only scan these
                // If the path is empty, then paths aren't used for this format.
                if (filesOrIdentifiersToScan.isEmpty() && path.getNumPaths() > 0)
                {
                   #if ! ALOE_IOS
                    if (propertiesToUse != nullptr)
                        path = getLastSearchPath (*propertiesToUse, formatToScan);
                   #endif

                    pathList.setSize (500, 300);
                    pathList.setPath (path);

                    pathChooserWindow.addCustomComponent (&pathList);
                    pathChooserWindow.addButton (TRANS("Scan"),   1, KeyPress (KeyPress::returnKey));
                    pathChooserWindow.addButton (TRANS("Cancel"), 0, KeyPress (KeyPress::escapeKey));

                    pathChooserWindow.enterModalState (true,
                                                       ModalCallbackFunction::forComponent (startScanCallback,
                                                                                            &pathChooserWindow, this),
                                                       false);
                }
                else
                {
                    startScan();
                }
        */
    }
    
    pub fn start_scan_callback(
        result:  i32,
        alert:   *mut AlertWindow,
        scanner: *mut PluginListComponentScanner)  {
        
        todo!();
        /*
            if (alert != nullptr && scanner != nullptr)
                {
                    if (result != 0)
                        scanner->warnUserAboutStupidPaths();
                    else
                        scanner->finishedScan();
                }
        */
    }

    /**
       Try to dissuade people from to scanning
       their entire C: drive, or other system
       folders.
      */
    pub fn warn_user_about_stupid_paths(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < pathList.getPath().getNumPaths(); ++i)
                {
                    auto f = pathList.getPath()[i];

                    if (isStupidPath (f))
                    {
                        AlertWindow::showOkCancelBox (MessageBoxIconType::WarningIcon,
                                                      TRANS("Plugin Scanning"),
                                                      TRANS("If you choose to scan folders that contain non-plugin files, "
                                                            "then scanning may take a long time, and can cause crashes when "
                                                            "attempting to load unsuitable files.")
                                                        + newLine
                                                        + TRANS ("Are you sure you want to scan the folder \"XYZ\"?")
                                                           .replace ("XYZ", f.getFullPathName()),
                                                      TRANS ("Scan"),
                                                      String(),
                                                      nullptr,
                                                      ModalCallbackFunction::create (warnAboutStupidPathsCallback, this));
                        return;
                    }
                }

                startScan();
        */
    }
    
    pub fn is_stupid_path(f: &File) -> bool {
        
        todo!();
        /*
            Vec<File> roots;
                File::findFileSystemRoots (roots);

                if (roots.contains (f))
                    return true;

                File::SpecialLocationType pathsThatWouldBeStupidToScan[]
                    = { File::globalApplicationsDirectory,
                        File::userHomeDirectory,
                        File::userDocumentsDirectory,
                        File::userDesktopDirectory,
                        File::tempDirectory,
                        File::userMusicDirectory,
                        File::userMoviesDirectory,
                        File::userPicturesDirectory };

                for (auto location : pathsThatWouldBeStupidToScan)
                {
                    auto sillyFolder = File::getSpecialLocation (location);

                    if (f == sillyFolder || sillyFolder.isAChildOf (f))
                        return true;
                }

                return false;
        */
    }
    
    pub fn warn_about_stupid_paths_callback(
        result:  i32,
        scanner: *mut PluginListComponentScanner)  {
        
        todo!();
        /*
            if (result != 0)
                    scanner->startScan();
                else
                    scanner->finishedScan();
        */
    }
    
    pub fn start_scan(&mut self)  {
        
        todo!();
        /*
            pathChooserWindow.setVisible (false);

                scanner.reset (new PluginDirectoryScanner (owner.list, formatToScan, pathList.getPath(),
                                                           true, owner.deadMansPedalFile, allowAsync));

                if (! filesOrIdentifiersToScan.isEmpty())
                {
                    scanner->setFilesOrIdentifiersToScan (filesOrIdentifiersToScan);
                }
                else if (propertiesToUse != nullptr)
                {
                    setLastSearchPath (*propertiesToUse, formatToScan, pathList.getPath());
                    propertiesToUse->saveIfNeeded();
                }

                progressWindow.addButton (TRANS("Cancel"), 0, KeyPress (KeyPress::escapeKey));
                progressWindow.addProgressBarComponent (progress);
                progressWindow.enterModalState();

                if (numThreads > 0)
                {
                    pool.reset (new ThreadPool (numThreads));

                    for (int i = numThreads; --i >= 0;)
                        pool->addJob (new PluginListComponentScannerScanJob (*this), true);
                }

                startTimer (20);
        */
    }
    
    pub fn finished_scan(&mut self)  {
        
        todo!();
        /*
            owner.scanFinished (scanner != nullptr ? scanner->getFailedFiles()
                                                       : StringArray());
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (timerReentrancyCheck)
                    return;

                if (pool == nullptr)
                {
                    const ScopedValueSetter<bool> setter (timerReentrancyCheck, true);

                    if (doNextScan())
                        startTimer (20);
                }

                if (! progressWindow.isCurrentlyModal())
                    finished = true;

                if (finished)
                    finishedScan();
                else
                    progressWindow.setMessage (TRANS("Testing") + ":\n\n" + pluginBeingScanned);
        */
    }
    
    pub fn do_next_scan(&mut self) -> bool {
        
        todo!();
        /*
            if (scanner->scanNextFile (true, pluginBeingScanned))
                {
                    progress = scanner->getProgress();
                    return true;
                }

                finished = true;
                return false;
        */
    }
}
