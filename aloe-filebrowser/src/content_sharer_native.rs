crate::ix!();

#[cfg(target_os="ios")]
pub struct ContentSharerNativeImpl<'a> {
    base:              ContentSharerImpl,
    base2:             Component<'a>,
    owner:             &'a mut ContentSharer,
    peer:              *mut UIViewComponentPeer, // default = nullptr
    controller:        NSUniquePtr<UIActivityViewController>,
    popover_delegate:  NSUniquePtr<NSObject<UIPopoverPresentationControllerDelegate>>,
    succeeded:         bool, // default = false
    error_description: String,
}

#[cfg(target_os="ios")]
impl Drop for ContentSharerNativeImpl {

    fn drop(&mut self) {
        todo!();
        /* 
                exitModalState (0);
             */
    }
}

#[cfg(target_os="ios")]
impl ContentSharerNativeImpl {
    
    pub fn new(cs: &mut ContentSharer) -> Self {
    
        todo!();
        /*
        : owner(cs),

            static PopoverDelegateClass cls;
                popoverDelegate.reset ([cls.createInstance() init]);
        */
    }
    
    pub fn share_files(&mut self, files: &[Url])  {
        
        todo!();
        /*
            auto urls = [NSMutableArray arrayWithCapacity: (NSUInteger) files.size()];

                for (const auto& f : files)
                {
                    NSString* nativeFilePath = nil;

                    if (f.isLocalFile())
                    {
                        nativeFilePath = aloeStringToNS (f.getLocalFile().getFullPathName());
                    }
                    else
                    {
                        auto filePath = f.toString (false);

                        auto* fileDirectory = filePath.contains ("/")
                            ? aloeStringToNS (filePath.upToLastOccurrenceOf ("/", false, false))
                            : [NSString string];

                        auto fileName = aloeStringToNS (filePath.fromLastOccurrenceOf ("/", false, false)
                            .upToLastOccurrenceOf (".", false, false));

                        auto fileExt = aloeStringToNS (filePath.fromLastOccurrenceOf (".", false, false));

                        if ([fileDirectory length] == NSUInteger (0))
                            nativeFilePath = [[NSBundle mainBundle] pathForResource: fileName
                                ofType: fileExt];
                        else
                            nativeFilePath = [[NSBundle mainBundle] pathForResource: fileName
                                ofType: fileExt
                                inDirectory: fileDirectory];
                    }

                    if (nativeFilePath != nil)
                        [urls addObject: [NSURL fileURLWithPath: nativeFilePath]];
                }

                share (urls);
        */
    }
    
    pub fn share_text(&mut self, text: &String)  {
        
        todo!();
        /*
            auto array = [NSArray arrayWithObject: aloeStringToNS (text)];
                share (array);
        */
    }
    
    pub fn share(&mut self, items: *mut NSArray)  {
        
        todo!();
        /*
            if ([items count] == 0)
                {
                    jassertfalse;
                    owner.sharingFinished (false, "No valid items found for sharing.");
                    return;
                }

                controller.reset ([[UIActivityViewController alloc] initWithActivityItems: items
                    applicationActivities: nil]);

                controller.get().excludedActivityTypes = nil;

                controller.get().completionWithItemsHandler = ^ (UIActivityType type, BOOL completed,
                    NSArray* returnedItems, NSError* error)
                {
                    ignoreUnused (type);
                    ignoreUnused (returnedItems);

                    succeeded = completed;

                    if (error != nil)
                        errorDescription = nsStringToAloe ([error localizedDescription]);

                    exitModalState (0);
                };

                controller.get().modalTransitionStyle = UIModalTransitionStyleCoverVertical;

                auto bounds = Desktop::getInstance().getDisplays().getPrimaryDisplay()->userArea;
                setBounds (bounds);

                setAlwaysOnTop (true);
                setVisible (true);
                addToDesktop (0);

                enterModalState (true,
                    ModalCallbackFunction::create ([this] (int)
                        {
                            owner.sharingFinished (succeeded, errorDescription);
                        }),
                        false);
        */
    }
    
    pub fn is_ipad() -> bool {
        
        todo!();
        /*
            return [UIDevice currentDevice].userInterfaceIdiom == UIUserInterfaceIdiomPad;
        */
    }
    
    pub fn parent_hierarchy_changed(&mut self)  {
        
        todo!();
        /*
            auto* newPeer = dynamic_cast<UIViewComponentPeer*> (getPeer());

                if (peer != newPeer)
                {
                    peer = newPeer;

                    if (isIPad())
                    {
                        controller.get().preferredContentSize = peer->view.frame.size;

                        auto screenBounds = [UIScreen mainScreen].bounds;

                        auto* popoverController = controller.get().popoverPresentationController;
                        popoverController.sourceView = peer->view;
                        popoverController.sourceRect = CGRectMake (0.f, screenBounds.size.height - 10.f, screenBounds.size.width, 10.f);
                        popoverController.canOverlapSourceViewRect = YES;
                        popoverController.delegate = popoverDelegate.get();
                    }

                    if (auto* parentController = peer->controller)
                        [parentController showViewController: controller.get() sender: parentController];
                }
        */
    }
}
