crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FileListComponentItemComponent<'a> {
    base:         Component<'a>,
    base2:        TimeSliceClient,
    base3:        AsyncUpdater<'a>,
    owner:        &'a mut FileListComponent<'a>,
    thread:       &'a mut TimeSliceThread,
    file:         File,
    file_size:    String,
    mod_time:     String,
    icon:         Image,
    index:        i32, // default = 0
    highlighted:  bool, // default = false
    is_directory: bool, // default = false
}

impl<'a> Drop for FileListComponentItemComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                thread.removeTimeSliceClient (this);
             */
    }
}

impl<'a> FileListComponentItemComponent<'a> {

    pub fn new(
        fc: &mut FileListComponent,
        t:  &mut TimeSliceThread) -> Self {
    
        todo!();
        /*
        : owner(fc),
        : thread(t),

        
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            getLookAndFeel().drawFileBrowserRow (g, getWidth(), getHeight(),
                                                     file, file.getFileName(),
                                                     &icon, fileSize, modTime,
                                                     isDirectory, highlighted,
                                                     index, owner);
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            owner.selectRowsBasedOnModifierKeys (index, e.mods, true);
                owner.sendMouseClickMessage (file, e);
        */
    }
    
    pub fn mouse_double_click(&mut self, _0: &MouseEvent)  {
        
        todo!();
        /*
            owner.sendDoubleClickMessage (file);
        */
    }
    
    pub fn update(
        &mut self, 
        root:            &File,
        file_info:       *const DirectoryContentsListFileInfo,
        new_index:       i32,
        now_highlighted: bool

    ) {
        
        todo!();
        /*
            thread.removeTimeSliceClient (this);

                if (nowHighlighted != highlighted || newIndex != index)
                {
                    index = newIndex;
                    highlighted = nowHighlighted;
                    repaint();
                }

                File newFile;
                String newFileSize, newModTime;

                if (fileInfo != nullptr)
                {
                    newFile = root.getChildFile (fileInfo->filename);
                    newFileSize = File::descriptionOfSizeInBytes (fileInfo->fileSize);
                    newModTime = fileInfo->modificationTime.formatted ("%d %b '%y %H:%M");
                }

                if (newFile != file
                     || fileSize != newFileSize
                     || modTime != newModTime)
                {
                    file = newFile;
                    fileSize = newFileSize;
                    modTime = newModTime;
                    icon = Image();
                    isDirectory = fileInfo != nullptr && fileInfo->isDirectory;

                    repaint();
                }

                if (file != File() && icon.isNull() && ! isDirectory)
                {
                    updateIcon (true);

                    if (! icon.isValid())
                        thread.addTimeSliceClient (this);
                }
        */
    }
    
    pub fn use_time_slice(&mut self) -> i32 {
        
        todo!();
        /*
            updateIcon (false);
                return -1;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            repaint();
        */
    }
    
    pub fn create_accessibility_handler(&mut self) -> Box<AccessibilityHandler> {
        
        todo!();
        /*
            return createIgnoredAccessibilityHandler (*this);
        */
    }
    
    pub fn update_icon(&mut self, only_update_if_cached: bool)  {
        
        todo!();
        /*
            if (icon.isNull())
                {
                    auto hashCode = (file.getFullPathName() + "_iconCacheSalt").hashCode();
                    auto im = ImageCache::getFromHashCode (hashCode);

                    if (im.isNull() && ! onlyUpdateIfCached)
                    {
                        im = aloe_createIconForFile (file);

                        if (im.isValid())
                            ImageCache::addImageToCache (im, hashCode);
                    }

                    if (im.isValid())
                    {
                        icon = im;
                        triggerAsyncUpdate();
                    }
                }
        */
    }
}
