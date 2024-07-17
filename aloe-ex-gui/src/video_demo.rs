crate::ix!();

#[cfg(any(target_os="macos",target_os="windows"))]
#[no_copy]
#[leak_detector]
pub struct VideoDemo<'a> {
    base:                   Component<'a>,
    base2:                  DragAndDropContainer<'a>,
    file_chooser:           Box<FileChooser<'a>>,
    movies_wildcard_filter: WildcardFileFilter,             // default = { "*", "*", "Movies File Filter"  }
    directory_thread:       TimeSliceThread,                // default = { "Movie File Scanner Thread"  }
    movie_list:             DirectoryContentsList<'a>,          //{ &moviesWildcardFilter, directoryThread };
    file_tree:              FileTreeComponent<'a>,              // default = { movieList  }
    stretchable_manager:    StretchableLayoutManager,
    resizer_bar:            StretchableLayoutResizerBar<'a>,    //{ &stretchableManager, 1, false };
    load_left_button:       TextButton<'a>,                     // default = { "Load Left"  }
    load_right_button:      TextButton<'a>,                     // default = { "Load Right"  }
    movie_comp_left:        MovieComponentWithFileBrowser<'a>,
    movie_comp_right:       MovieComponentWithFileBrowser<'a>,
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> FileBrowserListener for VideoDemo<'a> {

    fn selection_changed(&mut self)  {
        
        todo!();
        /*
            // we're just going to update the drag description of out tree so that rows can be dragged onto the file players
            fileTree.setDragAndDropDescription (fileTree.getSelectedFile().getFullPathName());
        */
    }
    
    fn file_clicked(&mut self, 
        _0: &File,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    fn file_double_clicked(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
    
    fn browser_root_changed(&mut self, _0: &File)  {
        
        todo!();
        /*
        
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> Default for VideoDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            movieList.setDirectory (File::getSpecialLocation (File::userMoviesDirectory), true, true);
            directoryThread.startThread (1);

            fileTree.setTitle ("Files");
            fileTree.addListener (this);
            fileTree.setColour (FileTreeComponent::backgroundColourId, Colours::lightgrey.withAlpha (0.6f));
            addAndMakeVisible (fileTree);

            addAndMakeVisible (resizerBar);

            loadLeftButton .onClick = [this] { movieCompLeft .setFile (fileTree.getSelectedFile (0)); };
            loadRightButton.onClick = [this] { movieCompRight.setFile (fileTree.getSelectedFile (0)); };

            addAndMakeVisible (loadLeftButton);
            addAndMakeVisible (loadRightButton);

            addAndMakeVisible (movieCompLeft);
            addAndMakeVisible (movieCompRight);

            // we have to set up our StretchableLayoutManager so it know the limits and preferred sizes of it's contents
            stretchableManager.setItemLayout (0,            // for the fileTree
                                              -0.1, -0.9,   // must be between 50 pixels and 90% of the available space
                                              -0.3);        // and its preferred size is 30% of the total available space

            stretchableManager.setItemLayout (1,            // for the resize bar
                                              5, 5, 5);     // hard limit to 5 pixels

            stretchableManager.setItemLayout (2,            // for the movie components
                                              -0.1, -0.9,   // size must be between 50 pixels and 90% of the available space
                                              -0.7);        // and its preferred size is 70% of the total available space

            setSize (500, 500)
        */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> Drop for VideoDemo<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            fileTree.removeListener (this);
         */
    }
}

#[cfg(any(target_os="macos",target_os="windows"))]
impl<'a> VideoDemo<'a> {

    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            // make a list of two of our child components that we want to reposition
            Component* comps[] = { &fileTree, &resizerBar, nullptr };

            // this will position the 3 components, one above the other, to fit
            // vertically into the rectangle provided.
            stretchableManager.layOutComponents (comps, 3,
                                                 0, 0, getWidth(), getHeight(),
                                                 true, true);

            // now position out two video components in the space that's left
            auto area = getLocalBounds().removeFromBottom (getHeight() - resizerBar.getBottom());

            {
                auto buttonArea = area.removeFromTop (30);
                loadLeftButton .setBounds (buttonArea.removeFromLeft (buttonArea.getWidth() / 2).reduced (5));
                loadRightButton.setBounds (buttonArea.reduced (5));
            }

            movieCompLeft .setBounds (area.removeFromLeft (area.getWidth() / 2).reduced (5));
            movieCompRight.setBounds (area.reduced (5));
        */
    }
    
    pub fn select_video_file(&mut self)  {
        
        todo!();
        /*
            fileChooser.reset (new FileChooser ("Choose a file to open...", File::getCurrentWorkingDirectory(),
                                                "*", false));

            fileChooser->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles,
                                      [this] (const FileChooser& chooser)
                                      {
                                          String chosen;
                                          auto results = chooser.getURLResults();

                                          // TODO: support non local files too
                                          if (results.size() > 0)
                                              movieCompLeft.setFile (results[0].getLocalFile());
                                      });
        */
    }
}

#[cfg(any(target_os="ios",target_os="android"))]
#[no_copy]
#[leak_detector]
#[weak_referenceable]
pub struct VideoDemo<'a> {
    base:                            Component<'a>,
    base2:                           Timer,

    load_local_button:               TextButton, // default = { "Load Local"  }
    load_url_button:                 TextButton, // default = { "Load Url"  }
    volume_label:                    Label, // default = { "volumeLabel", "Vol:"  }
    volume_slider:                   Slider, // default = { Slider::LinearHorizontal, Slider::NoTextBox  }
    video_comp_with_native_controls: VideoComponent,
    video_comp_no_native_controls:   VideoComponent,

    #[cfg(any(target_os="ios",target_os="macos"))]
    cur_video_comp:                  *mut VideoComponent, //= &videoCompWithNativeControls;

    #[cfg(not(any(target_os="ios",target_os="macos")))]
    cur_video_comp:                  *mut VideoComponent, //= &videoCompNoNativeControls;

    is_first_setup:                  bool, // default = true
    position_slider:                 Slider, // default = { Slider::LinearHorizontal, Slider::NoTextBox  }
    position_slider_dragging:        bool, // default = false
    was_playing_before_drag_start:   bool, // default = false
    current_position_label:          Label, // default = { "currentPositionLabel", "-:- / -:-"  }
    play_speed_combo_box:            ComboBox, // default = { "playSpeedComboBox"  }
    seek_to_start_button:            TextButton, // default = { "|<"  }
    play_button:                     TextButton, // default = { "Play"  }
    pause_button:                    TextButton, // default = { "Pause"  }
    unload_button:                   TextButton, // default = { "Unload"  }
    file_chooser:                    Box<FileChooser>,
}

#[cfg(any(target_os="ios",target_os="android"))]
impl<'a> Default for VideoDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*
        : video_comp_with_native_controls(true),
        : video_comp_no_native_controls(false),

            loadLocalButton  .onClick = [this] { selectVideoFile(); };
            loadUrlButton    .onClick = [this] { showVideoUrlPrompt(); };
            seekToStartButton.onClick = [this] { seekVideoToStart(); };
            playButton       .onClick = [this] { playVideo(); };
            pauseButton      .onClick = [this] { pauseVideo(); };
            unloadButton     .onClick = [this] { unloadVideoFile(); };

            volumeLabel         .setColour (Label::textColourId, Colours::white);
            currentPositionLabel.setColour (Label::textColourId, Colours::white);

            volumeLabel         .setJustificationType (Justification::right);
            currentPositionLabel.setJustificationType (Justification::right);

            volumeSlider  .setRange (0.0, 1.0);
            positionSlider.setRange (0.0, 1.0);

            volumeSlider  .setSliderSnapsToMousePosition (false);
            positionSlider.setSliderSnapsToMousePosition (false);

            volumeSlider.setSkewFactor (1.5);
            volumeSlider.setValue (1.0, dontSendNotification);
           #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
            curVideoComp->onGlobalMediaVolumeChanged = [this]() { volumeSlider.setValue (curVideoComp->getAudioVolume(), dontSendNotification); };
           #endif

            volumeSlider  .onValueChange = [this]() { curVideoComp->setAudioVolume ((float) volumeSlider.getValue()); };
            positionSlider.onValueChange = [this]() { seekVideoToNormalisedPosition (positionSlider.getValue()); };

            positionSlider.onDragStart = [this]()
                                         {
                                             positionSliderDragging = true;
                                             wasPlayingBeforeDragStart = curVideoComp->isPlaying();

                                             if (wasPlayingBeforeDragStart)
                                                 curVideoComp->stop();
                                         };

            positionSlider.onDragEnd   = [this]()
                                         {
                                             if (wasPlayingBeforeDragStart)
                                                 curVideoComp->play();

                                             wasPlayingBeforeDragStart = false;

                                             // Ensure the slider does not temporarily jump back on consecutive timer callback.
                                             Timer::callAfterDelay (500, [this]() { positionSliderDragging = false; });
                                         };

            playSpeedComboBox.addItem ("25%", 25);
            playSpeedComboBox.addItem ("50%", 50);
            playSpeedComboBox.addItem ("100%", 100);
            playSpeedComboBox.addItem ("200%", 200);
            playSpeedComboBox.addItem ("400%", 400);
            playSpeedComboBox.setSelectedId (100, dontSendNotification);
            playSpeedComboBox.onChange = [this]() { curVideoComp->setPlaySpeed (playSpeedComboBox.getSelectedId() / 100.0); };

            setTransportControlsEnabled (false);

            addAndMakeVisible (loadLocalButton);
            addAndMakeVisible (loadUrlButton);
            addAndMakeVisible (volumeLabel);
            addAndMakeVisible (volumeSlider);
            addChildComponent (videoCompWithNativeControls);
            addChildComponent (videoCompNoNativeControls);
            addAndMakeVisible (positionSlider);
            addAndMakeVisible (currentPositionLabel);

            addAndMakeVisible (playSpeedComboBox);
            addAndMakeVisible (seekToStartButton);
            addAndMakeVisible (playButton);
            addAndMakeVisible (unloadButton);
            addChildComponent (pauseButton);

            setSize (500, 500);

            RuntimePermissions::request (RuntimePermissions::readExternalStorage,
                                         [] (bool granted)
                                         {
                                             if (! granted)
                                             {
                                                 AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                                                   "Permissions warning",
                                                                                   "External storage access permission not granted, some files"
                                                                                   " may be inaccessible.");
                                             }
                                         });

            setPortraitOrientationEnabled (true)
        */
    }
}

#[cfg(any(target_os="ios",target_os="android"))]
impl<'a> Drop for VideoDemo<'a> {
    fn drop(&mut self) {
        todo!();
        /* 
            curVideoComp->onPlaybackStarted = nullptr;
            curVideoComp->onPlaybackStopped = nullptr;
            curVideoComp->onErrorOccurred   = nullptr;
            curVideoComp->onGlobalMediaVolumeChanged = nullptr;

            setPortraitOrientationEnabled (false);
         */
    }
}

#[cfg(any(target_os="ios",target_os="android"))]
impl<'a> VideoDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            int marginSize = 5;
            int buttonHeight = 20;

            area.reduce (0, marginSize);

            auto topArea = area.removeFromTop (buttonHeight);
            loadLocalButton.setBounds (topArea.removeFromLeft (topArea.getWidth() / 6));
            loadUrlButton.setBounds (topArea.removeFromLeft (loadLocalButton.getWidth()));
            volumeLabel.setBounds (topArea.removeFromLeft (loadLocalButton.getWidth()));
            volumeSlider.setBounds (topArea.reduced (10, 0));

            auto transportArea = area.removeFromBottom (buttonHeight);
            auto positionArea  = area.removeFromBottom (buttonHeight).reduced (marginSize, 0);

            playSpeedComboBox.setBounds (transportArea.removeFromLeft (jmax (50, transportArea.getWidth() / 5)));

            auto controlWidth = transportArea.getWidth() / 3;

            currentPositionLabel.setBounds (positionArea.removeFromRight (jmax (150, controlWidth)));
            positionSlider.setBounds (positionArea);

            seekToStartButton.setBounds (transportArea.removeFromLeft (controlWidth));
            playButton       .setBounds (transportArea.removeFromLeft (controlWidth));
            unloadButton     .setBounds (transportArea.removeFromLeft (controlWidth));
            pauseButton.setBounds (playButton.getBounds());

            area.removeFromTop (marginSize);
            area.removeFromBottom (marginSize);

            videoCompWithNativeControls.setBounds (area);
            videoCompNoNativeControls.setBounds (area);

            if (positionSlider.getWidth() > 0)
                positionSlider.setMouseDragSensitivity (positionSlider.getWidth());
        */
    }
    
    pub fn set_portrait_orientation_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            auto allowedOrientations = Desktop::getInstance().getOrientationsEnabled();

            if (shouldBeEnabled)
                allowedOrientations |= Desktop::upright;
            else
                allowedOrientations &= ~Desktop::upright;

            Desktop::getInstance().setOrientationsEnabled (allowedOrientations);
        */
    }
    
    pub fn set_transport_controls_enabled(&mut self, should_be_enabled: bool)  {
        
        todo!();
        /*
            positionSlider   .setEnabled (shouldBeEnabled);
            playSpeedComboBox.setEnabled (shouldBeEnabled);
            seekToStartButton.setEnabled (shouldBeEnabled);
            playButton       .setEnabled (shouldBeEnabled);
            unloadButton     .setEnabled (shouldBeEnabled);
            pauseButton      .setEnabled (shouldBeEnabled);
        */
    }
    
    pub fn select_video_file(&mut self)  {
        
        todo!();
        /*
            fileChooser.reset (new FileChooser ("Choose a video file to open...", File::getCurrentWorkingDirectory(),
                                                "*", true));

            fileChooser->launchAsync (FileBrowserComponent::openMode | FileBrowserComponent::canSelectFiles,
                                      [this] (const FileChooser& chooser)
                                      {
                                          auto results = chooser.getURLResults();

                                          if (results.size() > 0)
                                              loadVideo (results[0]);
                                      });
        */
    }
    
    pub fn load_video(&mut self, url: &Url)  {
        
        todo!();
        /*
            unloadVideoFile();

           #if ALOE_IOS || ALOE_MAC
            askIfUseNativeControls (url);
           #else
            loadUrl (url);
            setupVideoComp (false);
           #endif
        */
    }
    
    pub fn ask_if_use_native_controls(&mut self, url: &Url)  {
        
        todo!();
        /*
            auto* aw = new AlertWindow ("Choose viewer type", {}, MessageBoxIconType::NoIcon);

            aw->addButton ("Yes", 1, KeyPress (KeyPress::returnKey));
            aw->addButton ("No", 0, KeyPress (KeyPress::escapeKey));
            aw->addTextBlock ("Do you want to use the viewer with native controls?");

            auto callback = ModalCallbackFunction::forComponent (videoViewerTypeChosen, this, url);
            aw->enterModalState (true, callback, true);
        */
    }
    
    pub fn video_viewer_type_chosen(
        result: i32,
        owner:  *mut VideoDemo,
        url:    Url)  {
        
        todo!();
        /*
            if (owner != nullptr)
            {
                owner->setupVideoComp (result != 0);
                owner->loadUrl (url);
            }
        */
    }
    
    pub fn setup_video_comp(&mut self, use_native_viewer_with_native_controls: bool)  {
        
        todo!();
        /*
            auto* oldVideoComp = curVideoComp;

            if (useNativeViewerWithNativeControls)
                curVideoComp = &videoCompWithNativeControls;
            else
                curVideoComp = &videoCompNoNativeControls;

            if (isFirstSetup || oldVideoComp != curVideoComp)
            {
                oldVideoComp->onPlaybackStarted = nullptr;
                oldVideoComp->onPlaybackStopped = nullptr;
                oldVideoComp->onErrorOccurred   = nullptr;
                oldVideoComp->setVisible (false);

                curVideoComp->onPlaybackStarted = [this]() { processPlaybackStarted(); };
                curVideoComp->onPlaybackStopped = [this]() { processPlaybackPaused(); };
                curVideoComp->onErrorOccurred   = [this](const String& errorMessage) { errorOccurred (errorMessage); };
                curVideoComp->setVisible (true);

               #if ALOE_SYNC_VIDEO_VOLUME_WITH_OS_MEDIA_VOLUME
                oldVideoComp->onGlobalMediaVolumeChanged = nullptr;
                curVideoComp->onGlobalMediaVolumeChanged = [this]() { volumeSlider.setValue (curVideoComp->getAudioVolume(), dontSendNotification); };
               #endif
            }

            isFirstSetup = false;
        */
    }
    
    pub fn load_url(&mut self, url: &Url)  {
        
        todo!();
        /*
            curVideoComp->loadAsync (url, [this] (const Url& u, Result r) { videoLoadingFinished (u, r); });
        */
    }
    
    pub fn show_video_url_prompt(&mut self)  {
        
        todo!();
        /*
            auto* aw = new AlertWindow ("Enter Url for video to load", {}, MessageBoxIconType::NoIcon);

            aw->addButton ("OK", 1, KeyPress (KeyPress::returnKey));
            aw->addButton ("Cancel", 0, KeyPress (KeyPress::escapeKey));
            aw->addTextEditor ("videoUrlTextEditor", "https://www.rmp-streaming.com/media/bbb-360p.mp4");

            auto callback = ModalCallbackFunction::forComponent (videoUrlPromptClosed, this, Component::SafePointer<AlertWindow> (aw));
            aw->enterModalState (true, callback, true);
        */
    }
    
    pub fn video_url_prompt_closed(
        result: i32,
        owner:  *mut VideoDemo,
        aw:     Component::SafePointer<AlertWindow>)  {
        
        todo!();
        /*
            if (result != 0 && owner != nullptr && aw != nullptr)
            {
                auto url = aw->getTextEditorContents ("videoUrlTextEditor");

                if (url.isNotEmpty())
                    owner->loadVideo (url);
            }
        */
    }
    
    pub fn video_loading_finished(&mut self, 
        url:    &Url,
        result: Result)  {
        
        todo!();
        /*
            ignoreUnused (url);

            if (result.wasOk())
            {
                resized(); // update to reflect the video's aspect ratio

                setTransportControlsEnabled (true);

                currentPositionLabel.setText (getPositionString (0.0, curVideoComp->getVideoDuration()), sendNotification);
                positionSlider.setValue (0.0, dontSendNotification);
                playSpeedComboBox.setSelectedId (100, dontSendNotification);
            }
            else
            {
                AlertWindow::showMessageBoxAsync (MessageBoxIconType::WarningIcon,
                                                  "Couldn't load the file!",
                                                  result.getErrorMessage());
            }
        */
    }
    
    pub fn get_position_string(
        play_position_seconds: f64,
        duration_seconds:      f64) -> String {
        
        todo!();
        /*
            auto positionMs = static_cast<int> (1000 * playPositionSeconds);
            int posMinutes = positionMs / 60000;
            int posSeconds = (positionMs % 60000) / 1000;
            int posMillis = positionMs % 1000;

            auto totalMs = static_cast<int> (1000 * durationSeconds);
            int totMinutes = totalMs / 60000;
            int totSeconds = (totalMs % 60000) / 1000;
            int totMillis = totalMs % 1000;

            return String::formatted ("%02d:%02d:%03d / %02d:%02d:%03d",
                                      posMinutes, posSeconds, posMillis,
                                      totMinutes, totSeconds, totMillis);
        */
    }
    
    pub fn update_position_slider_and_label(&mut self)  {
        
        todo!();
        /*
            auto position = curVideoComp->getPlayPosition();
            auto duration = curVideoComp->getVideoDuration();

            currentPositionLabel.setText (getPositionString (position, duration), sendNotification);

            if (! positionSliderDragging)
                positionSlider.setValue (duration != 0 ? (position / duration) : 0.0, dontSendNotification);
        */
    }
    
    pub fn seek_video_to_start(&mut self)  {
        
        todo!();
        /*
            seekVideoToNormalisedPosition (0.0);
        */
    }
    
    pub fn seek_video_to_normalised_position(&mut self, normalised_pos: f64)  {
        
        todo!();
        /*
            normalisedPos = jlimit (0.0, 1.0, normalisedPos);

            auto duration = curVideoComp->getVideoDuration();
            auto newPos = jlimit (0.0, duration, duration * normalisedPos);

            curVideoComp->setPlayPosition (newPos);
            currentPositionLabel.setText (getPositionString (newPos, curVideoComp->getVideoDuration()), sendNotification);
            positionSlider.setValue (normalisedPos, dontSendNotification);
        */
    }
    
    pub fn play_video(&mut self)  {
        
        todo!();
        /*
            curVideoComp->play();
        */
    }
    
    pub fn process_playback_started(&mut self)  {
        
        todo!();
        /*
            playButton.setVisible (false);
            pauseButton.setVisible (true);

            startTimer (20);
        */
    }
    
    pub fn pause_video(&mut self)  {
        
        todo!();
        /*
            curVideoComp->stop();
        */
    }
    
    pub fn process_playback_paused(&mut self)  {
        
        todo!();
        /*
            // On seeking to a new pos, the playback may be temporarily paused.
            if (positionSliderDragging)
                return;

            pauseButton.setVisible (false);
            playButton.setVisible (true);
        */
    }
    
    pub fn error_occurred(&mut self, error_message: &String)  {
        
        todo!();
        /*
            AlertWindow::showMessageBoxAsync (MessageBoxIconType::InfoIcon,
                                              "An error has occurred",
                                              errorMessage + ", video will be unloaded.");

            unloadVideoFile();
        */
    }
    
    pub fn unload_video_file(&mut self)  {
        
        todo!();
        /*
            curVideoComp->closeVideo();

            setTransportControlsEnabled (false);
            stopTimer();

            pauseButton.setVisible (false);
            playButton.setVisible (true);

            currentPositionLabel.setText ("-:- / -:-", sendNotification);
            positionSlider.setValue (0.0, dontSendNotification);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            updatePositionSliderAndLabel();
        */
    }
}
