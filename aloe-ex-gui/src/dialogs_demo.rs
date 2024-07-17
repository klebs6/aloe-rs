crate::ix!();

#[no_copy]
#[leak_detector]
pub struct DialogsDemo<'a> {
    base:               Component<'a>,
    window_buttons:     Vec<Box<TextButton<'a>>>,
    native_button:      ToggleButton<'a>,
    image_preview:      ImagePreviewComponent<'a>,
    fc:                 Box<FileChooser<'a>>,
    async_alert_window: Box<AlertWindow<'a>>,
}

impl<'a> Paint for DialogsDemo<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
}

impl<'a> Resized for DialogsDemo<'a> {
    
    fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds().reduced (5, 15);
            Rectangle<int> topRow;

            for (auto* b : windowButtons)
            {
                auto index = windowButtons.indexOf (b);

                if (topRow.getWidth() < 10 || index == loadChooser)
                    topRow = area.removeFromTop (26);

                if (index == progressWindow)
                    area.removeFromTop (20);

                b->setBounds (topRow.removeFromLeft (area.getWidth() / 2).reduced (4, 2));
            }

            area.removeFromTop (15);
            nativeButton.setBounds (area.removeFromTop (24));
        */
    }
}


impl<'a> Default for DialogsDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (nativeButton);
            nativeButton.setButtonText ("Use Native Windows");
            nativeButton.onClick = [this] { getLookAndFeel().setUsingNativeAlertWindows (nativeButton.getToggleState()); };

            StringArray windowNames { "Plain Alert Window",
                                      "Alert Window With Warning Icon",
                                      "Alert Window With Info Icon",
                                      "Alert Window With Question Icon",
                                      "OK Cancel Alert Window",
                                      "Alert Window With Extra Components",
                                      "CalloutBox",
                                      "Thread With Progress Window",
                                      "'Load' File Browser",
                                      "'Load' File Browser With Image Preview",
                                      "'Choose Directory' File Browser",
                                      "'Save' File Browser",
                                      "Share Text",
                                      "Share Files",
                                      "Share Images" };

            // warn in case we add any windows
            jassert (windowNames.size() == numDialogs);

            for (auto windowName : windowNames)
            {
                auto* newButton = new TextButton();

                addAndMakeVisible (windowButtons.add (newButton));
                newButton->setButtonText (windowName);

                auto index = windowNames.indexOf (windowName);
                newButton->onClick = [this, index, newButton] { showWindow (*newButton, static_cast<DialogType> (index)); };
            }

            setSize (500, 500);

            RuntimePermissions::request (RuntimePermissions::readExternalStorage,
                                         [] (bool granted)
                                         {
                                             if (! granted)
                                                 AlertWindow::showAsync (MessageBoxOptions()
                                                                           .withIconType (MessageBoxIconType::WarningIcon)
                                                                           .withTitle ("Permissions warning")
                                                                           .withMessage ("External storage access permission not granted, some files"
                                                                                         " may be inaccessible.")
                                                                           .withButton ("OK"),
                                                                         nullptr);
                                         })
        */
    }
}

impl<'a> DialogsDemo<'a> {
    
    pub fn show_window(&mut self, 
        button: &mut Component,
        ty:     DialogType)  {
        
        todo!();
        /*
            if (type >= plainAlertWindow && type <= questionAlertWindow)
            {
                MessageBoxIconType icon = MessageBoxIconType::NoIcon;

                if (type == warningAlertWindow)   icon = MessageBoxIconType::WarningIcon;
                if (type == infoAlertWindow)      icon = MessageBoxIconType::InfoIcon;
                if (type == questionAlertWindow)  icon = MessageBoxIconType::QuestionIcon;

                AlertWindow::showMessageBoxAsync (icon, "This is an AlertWindow",
                                                  "And this is the AlertWindow's message. Blah blah blah blah blah blah blah blah blah blah blah blah blah.",
                                                  "OK");
            }
            else if (type == okCancelAlertWindow)
            {
                AlertWindow::showOkCancelBox (MessageBoxIconType::QuestionIcon, "This is an ok/cancel AlertWindow",
                                              "And this is the AlertWindow's message. Blah blah blah blah blah blah blah blah blah blah blah blah blah.",
                                              {}, {}, {},
                                              ModalCallbackFunction::create (AlertBoxResultChosen{}));
            }
            else if (type == calloutBoxWindow)
            {
                auto colourSelector = std::make_unique<ColourSelector>();

                colourSelector->setName ("background");
                colourSelector->setCurrentColour (findColour (TextButton::buttonColourId));
                colourSelector->setColour (ColourSelector::backgroundColourId, Colours::transparentBlack);
                colourSelector->setSize (300, 400);

                CallOutBox::launchAsynchronously (std::move (colourSelector), button.getScreenBounds(), nullptr);
            }
            else if (type == extraComponentsAlertWindow)
            {
                asyncAlertWindow = std::make_unique<AlertWindow> ("AlertWindow demo..",
                                                                  "This AlertWindow has a couple of extra components added to show how to add drop-down lists and text entry boxes.",
                                                                  MessageBoxIconType::QuestionIcon);

                asyncAlertWindow->addTextEditor ("text", "enter some text here", "text field:");
                asyncAlertWindow->addComboBox ("option", { "option 1", "option 2", "option 3", "option 4" }, "some options");
                asyncAlertWindow->addButton ("OK",     1, KeyPress (KeyPress::returnKey, 0, 0));
                asyncAlertWindow->addButton ("Cancel", 0, KeyPress (KeyPress::escapeKey, 0, 0));

                asyncAlertWindow->enterModalState (true, ModalCallbackFunction::create (AsyncAlertBoxResultChosen { *this }));
            }
            else if (type == progressWindow)
            {
                // This will launch our ThreadWithProgressWindow in a modal state. (Our subclass
                // will take care of deleting the object when the task has finished)
                (new DemoBackgroundThread())->launchThread();
            }
            else if (type >= loadChooser && type <= saveChooser)
            {
                auto useNativeVersion = nativeButton.getToggleState();

                if (type == loadChooser)
                {
                    fc.reset (new FileChooser ("Choose a file to open...", File::getCurrentWorkingDirectory(),
                                               "*", useNativeVersion));

                    fc->launchAsync (FileBrowserComponent::canSelectMultipleItems | FileBrowserComponent::openMode
                                         | FileBrowserComponent::canSelectFiles,
                                     [] (const FileChooser& chooser)
                                     {
                                         String chosen;
                                         auto results = chooser.getURLResults();

                                         for (auto result : results)
                                             chosen << (result.isLocalFile() ? result.getLocalFile().getFullPathName()
                                                                             : result.toString (false)) << "\n";

                                         AlertWindow::showAsync (MessageBoxOptions()
                                                                   .withIconType (MessageBoxIconType::InfoIcon)
                                                                   .withTitle ("File Chooser...")
                                                                   .withMessage ("You picked: " + chosen)
                                                                   .withButton ("OK"),
                                                                 nullptr);
                                     });
                }
                else if (type == loadWithPreviewChooser)
                {
                    imagePreview.setSize (200, 200);

                    fc.reset (new FileChooser ("Choose an image to open...", File::getCurrentWorkingDirectory(),
                                               "*.jpg;*.jpeg;*.png;*.gif", useNativeVersion));

                    fc->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles
                                        | FileBrowserComponent::canSelectMultipleItems,
                                     [] (const FileChooser& chooser)
                                     {
                                         String chosen;
                                         auto results = chooser.getURLResults();

                                         for (auto result : results)
                                             chosen << (result.isLocalFile() ? result.getLocalFile().getFullPathName()
                                                                             : result.toString (false)) << "\n";

                                         AlertWindow::showAsync (MessageBoxOptions()
                                                                   .withIconType (MessageBoxIconType::InfoIcon)
                                                                   .withTitle ("File Chooser...")
                                                                   .withMessage ("You picked: " + chosen)
                                                                   .withButton ("OK"),
                                                                 nullptr);
                                     },
                                     &imagePreview);
                }
                else if (type == saveChooser)
                {
                    auto fileToSave = File::createTempFile ("saveChooserDemo");

                    if (fileToSave.createDirectory().wasOk())
                    {
                        fileToSave = fileToSave.getChildFile ("Aloe.png");
                        fileToSave.deleteFile();

                        FileOutputStream outStream (fileToSave);

                        if (outStream.openedOk())
                            if (auto inStream = createAssetInputStream ("aloe_icon.png"))
                                outStream.writeFromInputStream (*inStream, -1);
                    }

                    fc.reset (new FileChooser ("Choose a file to save...",
                                               File::getCurrentWorkingDirectory().getChildFile (fileToSave.getFileName()),
                                               "*",  useNativeVersion));

                    fc->launchAsync (FileBrowserComponent::saveMode | FileBrowserComponent::canSelectFiles,
                                     [fileToSave] (const FileChooser& chooser)
                                     {
                                         auto result = chooser.getURLResult();
                                         auto name = result.isEmpty() ? String()
                                                                      : (result.isLocalFile() ? result.getLocalFile().getFullPathName()
                                                                                              : result.toString (true));

                                         // Android and iOS file choosers will create placeholder files for chosen
                                         // paths, so we may as well write into those files.
                                       #if ALOE_ANDROID || ALOE_IOS
                                         if (! result.isEmpty())
                                         {
                                             std::unique_ptr<InputStream>  wi (fileToSave.createInputStream());
                                             std::unique_ptr<OutputStream> wo (result.createOutputStream());

                                             if (wi.get() != nullptr && wo.get() != nullptr)
                                             {
                                                 auto numWritten = wo->writeFromInputStream (*wi, -1);
                                                 jassertquiet (numWritten > 0);
                                                 wo->flush();
                                             }
                                         }
                                       #endif

                                         AlertWindow::showAsync (MessageBoxOptions()
                                                                   .withIconType (MessageBoxIconType::InfoIcon)
                                                                   .withTitle ("File Chooser...")
                                                                   .withMessage ("You picked: " + name)
                                                                   .withButton ("OK"),
                                                                 nullptr);
                                     });
                }
                else if (type == directoryChooser)
                {
                    fc.reset (new FileChooser ("Choose a directory...",
                                               File::getCurrentWorkingDirectory(),
                                               "*",
                                               useNativeVersion));

                    fc->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectDirectories,
                                     [] (const FileChooser& chooser)
                                     {
                                         auto result = chooser.getURLResult();
                                         auto name = result.isLocalFile() ? result.getLocalFile().getFullPathName()
                                                                          : result.toString (true);

                                         AlertWindow::showAsync (MessageBoxOptions()
                                                                   .withIconType (MessageBoxIconType::InfoIcon)
                                                                   .withTitle ("File Chooser...")
                                                                   .withMessage ("You picked: " + name)
                                                                   .withButton ("OK"),
                                                                 nullptr);
                                     });
                }
            }
            else if (type == shareText)
            {
                ContentSharer::getInstance()->shareText ("I love Aloe!",
                                                         [] (bool success, const String& error)
                    {
                        auto resultString = success ? String ("success") : ("failure\n (error: " + error + ")");

                        AlertWindow::showAsync (MessageBoxOptions()
                                                  .withIconType (MessageBoxIconType::InfoIcon)
                                                  .withTitle ("Sharing Text Result")
                                                  .withMessage ("Sharing text finished\nwith " + resultString)
                                                  .withButton ("OK"),
                                                nullptr);
                    });
            }
            else if (type == shareFile)
            {
                File fileToSave = File::createTempFile ("DialogsDemoSharingTest");

                if (fileToSave.createDirectory().wasOk())
                {
                    fileToSave = fileToSave.getChildFile ("SharingDemoFile.txt");
                    fileToSave.replaceWithText ("Make it fast!");

                    Vec<Url> urls;
                    urls.add (Url (fileToSave));

                    ContentSharer::getInstance()->shareFiles (urls,
                        [] (bool success, const String& error)
                        {
                            auto resultString = success ? String ("success") : ("failure\n (error: " + error + ")");

                            AlertWindow::showAsync (MessageBoxOptions()
                                                      .withIconType (MessageBoxIconType::InfoIcon)
                                                      .withTitle ("Sharing Files Result")
                                                      .withMessage ("Sharing files finished\nwith " + resultString)
                                                      .withButton ("OK"),
                                                    nullptr);
                        });

                }
            }
            else if (type == shareImage)
            {
                auto myImage = getImageFromAssets ("aloe_icon.png");

                Image myImage2 (Image::RGB, 500, 500, true);
                Graphics g (myImage2);
                g.setColour (Colours::green);
                ColourGradient gradient (Colours::yellow, 170, 170, Colours::cyan, 170, 20, true);
                g.setGradientFill (gradient);
                g.fillEllipse (20, 20, 300, 300);

                Vec<Image> images { myImage, myImage2 };

                ContentSharer::getInstance()->shareImages (images,
                                                           [] (bool success, const String& error)
                                                           {
                                                               String resultString = success ? String ("success")
                                                                                             : ("failure\n (error: " + error + ")");

                                                               AlertWindow::showAsync (MessageBoxOptions()
                                                                                         .withIconType (MessageBoxIconType::InfoIcon)
                                                                                         .withTitle ("Sharing Images Result")
                                                                                         .withMessage ("Sharing images finished\nwith " + resultString)
                                                                                         .withButton ("OK"),
                                                                                       nullptr);
                                                           });
            }
        */
    }
}
