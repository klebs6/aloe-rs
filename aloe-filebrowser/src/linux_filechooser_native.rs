crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FileChooserNative<'a> {
    base2:                 Timer,
    owner:                 &'a mut FileChooser<'a>,
    is_directory:          bool,
    is_save:               bool,
    select_multiple_files: bool,
    warn_about_overwrite:  bool,
    child:                 ChildProcess,
    args:                  Vec<String>,
    separator:             String,
}

impl<'a> FileChooserPimpl for FileChooserNative<'a> {

    fn run_modally(&mut self)  {
        
        todo!();
        /*
            #if ALOE_MODAL_LOOPS_PERMITTED
                    child.start (args, ChildProcess::wantStdOut);

                    while (child.isRunning())
                        if (! MessageManager::getInstance()->runDispatchLoopUntil (20))
                            break;

                    finish (false);
                    #else
                    jassertfalse;
                    #endif
        */
    }
    
    fn launch(&mut self)  {
        
        todo!();
        /*
            child.start (args, ChildProcess::wantStdOut);
                    startTimer (100);
        */
    }
}

impl<'a> Drop for FileChooserNative<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                    finish (true);
                 */
    }
}

impl<'a> FileChooserNative<'a> {

    pub fn new(
        file_chooser: &mut FileChooser,
        flags:        i32) -> Self {
    
        todo!();
        /*


            : owner (fileChooser),
                    isDirectory         ((flags & FileBrowserComponent::canSelectDirectories)   != 0),
                    isSave              ((flags & FileBrowserComponent::saveMode)               != 0),
                    selectMultipleFiles ((flags & FileBrowserComponent::canSelectMultipleItems) != 0),
                    warnAboutOverwrite  ((flags & FileBrowserComponent::warnAboutOverwriting)   != 0)
                    const File previousWorkingDirectory (File::getCurrentWorkingDirectory());

                    // use kdialog for KDE sessions or if zenity is missing
                    if (exeIsAvailable ("kdialog") && (isKdeFullSession() || ! exeIsAvailable ("zenity")))
                        addKDialogArgs();
                    else
                        addZenityArgs();
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (! child.isRunning())
                    {
                        stopTimer();
                        finish (false);
                    }
        */
    }
    
    pub fn finish(&mut self, should_kill: bool)  {
        
        todo!();
        /*
            String result;
                    Vec<Url> selection;

                    if (shouldKill)
                        child.kill();
                    else
                        result = child.readAllProcessOutput().trim();

                    if (result.isNotEmpty())
                    {
                        Vec<String> tokens;

                        if (selectMultipleFiles)
                            tokens.addTokens (result, separator, "\"");
                        else
                            tokens.add (result);

                        for (auto& token : tokens)
                            selection.add (Url (File::getCurrentWorkingDirectory().getChildFile (token)));
                    }

                    if (! shouldKill)
                    {
                        child.waitForProcessToFinish (60 * 1000);
                        owner.finished (selection);
                    }
        */
    }
    
    pub fn get_top_windowid() -> u64 {
        
        todo!();
        /*
            if (TopLevelWindow* top = TopLevelWindow::getActiveTopLevelWindow())
                        return (uint64) (pointer_sized_uint) top->getWindowHandle();

                    return 0;
        */
    }
    
    pub fn is_kde_full_session() -> bool {
        
        todo!();
        /*
            return SystemStats::getEnvironmentVariable ("KDE_FULL_SESSION", String())
                        .equalsIgnoreCase ("true");
        */
    }
    
    pub fn add_kdialog_args(&mut self)  {
        
        todo!();
        /*
            args.add ("kdialog");

                    if (owner.title.isNotEmpty())
                        args.add ("--title=" + owner.title);

                    if (uint64 topWindowID = getTopWindowID())
                    {
                        args.add ("--attach");
                        args.add (String (topWindowID));
                    }

                    if (selectMultipleFiles)
                    {
                        separator = "\n";
                        args.add ("--multiple");
                        args.add ("--separate-output");
                        args.add ("--getopenfilename");
                    }
                    else
                    {
                        if (isSave)             args.add ("--getsavefilename");
                        else if (isDirectory)   args.add ("--getexistingdirectory");
                        else                    args.add ("--getopenfilename");
                    }

                    File startPath;

                    if (owner.startingFile.exists())
                    {
                        startPath = owner.startingFile;
                    }
                    else if (owner.startingFile.getParentDirectory().exists())
                    {
                        startPath = owner.startingFile.getParentDirectory();
                    }
                    else
                    {
                        startPath = File::getSpecialLocation (File::userHomeDirectory);

                        if (isSave)
                            startPath = startPath.getChildFile (owner.startingFile.getFileName());
                    }

                    args.add (startPath.getFullPathName());
                    args.add ("(" + owner.filters.replaceCharacter (';', ' ') + ")");
        */
    }
    
    pub fn add_zenity_args(&mut self)  {
        
        todo!();
        /*
            args.add ("zenity");
                    args.add ("--file-selection");

                    if (warnAboutOverwrite)
                        args.add("--confirm-overwrite");

                    if (owner.title.isNotEmpty())
                        args.add ("--title=" + owner.title);

                    if (selectMultipleFiles)
                    {
                        separator = ":";
                        args.add ("--multiple");
                        args.add ("--separator=" + separator);
                    }
                    else
                    {
                        if (isSave)
                            args.add ("--save");
                    }

                    if (isDirectory)
                        args.add ("--directory");

                    if (owner.filters.isNotEmpty() && owner.filters != "*" && owner.filters != "*.*")
                    {
                        Vec<String> tokens;
                        tokens.addTokens (owner.filters, ";,|", "\"");

                        args.add ("--file-filter=" + tokens.joinIntoString (" "));
                    }

                    if (owner.startingFile.isDirectory())
                        owner.startingFile.setAsCurrentWorkingDirectory();
                    else if (owner.startingFile.getParentDirectory().exists())
                        owner.startingFile.getParentDirectory().setAsCurrentWorkingDirectory();
                    else
                        File::getSpecialLocation (File::userHomeDirectory).setAsCurrentWorkingDirectory();

                    auto filename = owner.startingFile.getFileName();

                    if (! filename.isEmpty())
                        args.add ("--filename=" + filename);

                    // supplying the window ID of the topmost window makes sure that Zenity pops up..
                    if (uint64 topWindowID = getTopWindowID())
                        setenv ("WINDOWID", String (topWindowID).toRawUTF8(), true);
        */
    }
}
