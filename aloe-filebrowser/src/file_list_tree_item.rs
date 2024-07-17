crate::ix!();

#[no_copy]
#[leak_detector]
pub struct FileListTreeItem<'a> {
    base:                   TreeViewItem<'a>,
    base2:                  TimeSliceClient,
    base3:                  AsyncUpdater<'a>,
    file:                   File,
    owner:                  &'a mut FileTreeComponent<'a>,
    parent_contents_list:   *mut DirectoryContentsList<'a>,
    index_in_contents_list: i32,
    sub_contents_list:      OptionalScopedPointer<DirectoryContentsList<'a>>,
    is_directory:           bool,
    thread:                 &'a mut TimeSliceThread,
    icon_update:            CriticalSection,
    icon:                   Image,
    file_size:              String,
    mod_time:               String,
}

impl<'a> ChangeListener for FileListTreeItem<'a> {

    fn change_listener_callback(&mut self, _0: *mut ChangeBroadcaster)  {
        
        todo!();
        /*
            rebuildItemsFromContentList();
        */
    }
}

impl<'a> ItemDropped for FileListTreeItem<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) 
    { 
        todo!() 
    }
}

impl<'a> Drop for FileListTreeItem<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            thread.removeTimeSliceClient (this);
            clearSubItems();
            removeSubContentsList();
         */
    }
}

impl<'a> FileListTreeItem<'a> {

    pub fn new(
        tree_comp:         &mut FileTreeComponent,
        parent_contents:   *mut DirectoryContentsList,
        index_in_contents: i32,
        f:                 &File,
        t:                 &mut TimeSliceThread) -> Self {
    
        todo!();
        /*
        : file(f),
        : owner(treeComp),
        : parent_contents_list(parentContents),
        : index_in_contents_list(indexInContents),
        : sub_contents_list(nullptr, false),
        : thread(t),

            DirectoryContentsList::FileInfo fileInfo;

            if (parentContents != nullptr
                 && parentContents->getFileInfo (indexInContents, fileInfo))
            {
                fileSize = File::descriptionOfSizeInBytes (fileInfo.fileSize);
                modTime = fileInfo.modificationTime.formatted ("%d %b '%y %H:%M");
                isDirectory = fileInfo.isDirectory;
            }
            else
            {
                isDirectory = true;
            }
        */
    }
    
    pub fn might_contain_sub_items(&mut self) -> bool {
        
        todo!();
        /*
            return isDirectory;
        */
    }
    
    pub fn get_unique_name(&self) -> String {
        
        todo!();
        /*
            return file.getFullPathName();
        */
    }
    
    pub fn get_item_height(&self) -> i32 {
        
        todo!();
        /*
            return owner.getItemHeight();
        */
    }
    
    pub fn get_drag_source_description(&mut self) -> Var {
        
        todo!();
        /*
            return owner.getDragAndDropDescription();
        */
    }
    
    pub fn item_openness_changed(&mut self, is_now_open: bool)  {
        
        todo!();
        /*
            if (isNowOpen)
            {
                clearSubItems();

                isDirectory = file.isDirectory();

                if (isDirectory)
                {
                    if (subContentsList == nullptr && parentContentsList != nullptr)
                    {
                        auto l = new DirectoryContentsList (parentContentsList->getFilter(), thread);

                        l->setDirectory (file,
                                         parentContentsList->isFindingDirectories(),
                                         parentContentsList->isFindingFiles());

                        setSubContentsList (l, true);
                    }

                    changeListenerCallback (nullptr);
                }
            }
        */
    }
    
    pub fn remove_sub_contents_list(&mut self)  {
        
        todo!();
        /*
            if (subContentsList != nullptr)
            {
                subContentsList->removeChangeListener (this);
                subContentsList.reset();
            }
        */
    }
    
    pub fn set_sub_contents_list(&mut self, 
        new_list:        *mut DirectoryContentsList,
        can_delete_list: bool)  {
        
        todo!();
        /*
            removeSubContentsList();

            subContentsList = OptionalScopedPointer<DirectoryContentsList> (newList, canDeleteList);
            newList->addChangeListener (this);
        */
    }
    
    pub fn select_file(&mut self, target: &File) -> bool {
        
        todo!();
        /*
            if (file == target)
            {
                setSelected (true, true);
                return true;
            }

            if (target.isAChildOf (file))
            {
                setOpen (true);

                for (int maxRetries = 500; --maxRetries > 0;)
                {
                    for (int i = 0; i < getNumSubItems(); ++i)
                        if (auto* f = dynamic_cast<FileListTreeItem*> (getSubItem (i)))
                            if (f->selectFile (target))
                                return true;

                    // if we've just opened and the contents are still loading, wait for it..
                    if (subContentsList != nullptr && subContentsList->isStillLoading())
                    {
                        Thread::sleep (10);
                        rebuildItemsFromContentList();
                    }
                    else
                    {
                        break;
                    }
                }
            }

            return false;
        */
    }
    
    pub fn rebuild_items_from_content_list(&mut self)  {
        
        todo!();
        /*
            clearSubItems();

            if (isOpen() && subContentsList != nullptr)
            {
                for (int i = 0; i < subContentsList->getNumFiles(); ++i)
                    addSubItem (new FileListTreeItem (owner, subContentsList, i,
                                                      subContentsList->getFile(i), thread));
            }
        */
    }
    
    pub fn paint_item(&mut self, 
        g:      &mut Graphics,
        width:  i32,
        height: i32)  {
        
        todo!();
        /*
            ScopedLock lock (iconUpdate);

            if (file != File())
            {
                updateIcon (true);

                if (icon.isNull())
                    thread.addTimeSliceClient (this);
            }

            owner.getLookAndFeel().drawFileBrowserRow (g, width, height,
                                                       file, file.getFileName(),
                                                       &icon, fileSize, modTime,
                                                       isDirectory, isSelected(),
                                                       indexInContentsList, owner);
        */
    }
    
    pub fn get_accessibility_name(&mut self) -> String {
        
        todo!();
        /*
            return file.getFileName();
        */
    }
    
    pub fn item_clicked(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            owner.sendMouseClickMessage (file, e);
        */
    }
    
    pub fn item_double_clicked(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            TreeViewItem::itemDoubleClicked (e);

            owner.sendDoubleClickMessage (file);
        */
    }
    
    pub fn item_selection_changed(&mut self, _0: bool)  {
        
        todo!();
        /*
            owner.sendSelectionChangeMessage();
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
            owner.repaint();
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
                    {
                        ScopedLock lock (iconUpdate);
                        icon = im;
                    }

                    triggerAsyncUpdate();
                }
            }
        */
    }
}
