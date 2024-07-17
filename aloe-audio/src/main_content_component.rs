crate::ix!();

pub const STANDALONE_FILTER_WINDOW_MAIN_CONTENT_COMPONENT_NOTIFICATION_AREA_HEIGHT: usize = 30;
    
#[no_copy]
#[leak_detector]
pub struct StandaloneFilterWindowMainContentComponent<'a> {
    base:                     Component<'a>,
    owner:                    &'a mut StandaloneFilterWindow<'a>,
    notification:             StandaloneFilterWindowMainContentComponentNotificationArea<'a>,
    editor:                   Box<AudioProcessorEditor<'a>>,
    input_muted_value:        Value<'a>,
    should_show_notification: bool, // default = false
}

impl<'a> ValueListener for StandaloneFilterWindowMainContentComponent<'a> {

    fn value_changed(&mut self, value: &mut Value)  {
        
        todo!();
        /*
            inputMutedChanged (value.getValue());
        */
    }
}

impl<'a> ComponentListener for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentBroughtToFront for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentVisibilityChanged for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentChildrenChanged for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentParentHierarchyChanged for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentNameChanged for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentBeingDeleted for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentEnablementChanged for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ComponentMovedOrResized for StandaloneFilterWindowMainContentComponent<'a> {

}

impl<'a> ButtonListener for StandaloneFilterWindowMainContentComponent<'a> {

    fn button_clicked(&mut self, _0: *mut Button)  {
        
        todo!();
        /*
            #if ALOE_IOS || ALOE_ANDROID
                owner.pluginHolder->getMuteInputValue().setValue (false);
               #else
                owner.pluginHolder->showAudioSettingsDialog();
               #endif
        */
    }
}

impl<'a> Drop for StandaloneFilterWindowMainContentComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
                if (editor != nullptr)
                {
                    editor->removeComponentListener (this);
                    owner.pluginHolder->processor->editorBeingDeleted (editor.get());
                    editor = nullptr;
                }
             */
    }
}

impl<'a> StandaloneFilterWindowMainContentComponent<'a> {

    pub fn new(filter_window: &mut StandaloneFilterWindow) -> Self {
    
        todo!();
        /*


            : owner (filterWindow), notification (this),
                  editor (owner.getAudioProcessor()->hasEditor() ? owner.getAudioProcessor()->createEditorIfNeeded()
                                                                 : new GenericAudioProcessorEditor (*owner.getAudioProcessor()))

                inputMutedValue.referTo (owner.pluginHolder->getMuteInputValue());

                if (editor != nullptr)
                {
                    editor->addComponentListener (this);
                    componentMovedOrResized (*editor, false, true);

                    addAndMakeVisible (editor.get());
                }

                addChildComponent (notification);

                if (owner.pluginHolder->getProcessorHasPotentialFeedbackLoop())
                {
                    inputMutedValue.addListener (this);
                    shouldShowNotification = inputMutedValue.getValue();
                }

                inputMutedChanged (shouldShowNotification);
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto r = getLocalBounds();

                if (shouldShowNotification)
                    notification.setBounds (r.removeFromTop (StandaloneFilterWindowMainContentComponentNotificationArea::height));

                if (editor != nullptr)
                    editor->setBoundsConstrained (editor->getLocalArea (this, r.toFloat())
                                                         .withPosition (r.getTopLeft().toFloat().transformedBy (editor->getTransform().inverted()))
                                                    .toNearestInt());
        */
    }
    
    pub fn input_muted_changed(&mut self, new_input_muted_value: bool)  {
        
        todo!();
        /*
            shouldShowNotification = newInputMutedValue;
                notification.setVisible (shouldShowNotification);

               #if ALOE_IOS || ALOE_ANDROID
                resized();
               #else
                if (editor != nullptr)
                {
                    const int extraHeight = shouldShowNotification ? StandaloneFilterWindowMainContentComponentNotificationArea::height : 0;
                    const auto rect = getSizeToContainEditor();

                    if (auto* editorConstrainer = editor->getConstrainer())
                    {
                        const auto borders = owner.getContentComponentBorder();
                        const auto extraWindowWidth = borders.getLeftAndRight();
                        const auto extraWindowHeight = extraHeight + borders.getTopAndBottom();

                        owner.setResizeLimits (jmax (10, editorConstrainer->getMinimumWidth()  + extraWindowWidth),
                                               jmax (10, editorConstrainer->getMinimumHeight() + extraWindowHeight),
                                               editorConstrainer->getMaximumWidth()  + extraWindowWidth,
                                               editorConstrainer->getMaximumHeight() + extraWindowHeight);
                    }

                    setSize (rect.getWidth(), rect.getHeight() + extraHeight);
                }
               #endif
        */
    }
    
    pub fn component_moved_or_resized(&mut self, 
        _0: &mut Component,
        _1: bool,
        _2: bool)  {
        
        todo!();
        /*
            if (editor != nullptr)
                {
                    auto rect = getSizeToContainEditor();

                    setSize (rect.getWidth(),
                             rect.getHeight() + (shouldShowNotification ? StandaloneFilterWindowMainContentComponentNotificationArea::height : 0));
                }
        */
    }
    
    pub fn get_size_to_contain_editor(&self) -> Rectangle<i32> {
        
        todo!();
        /*
            if (editor != nullptr)
                    return getLocalArea (editor.get(), editor->getLocalBounds());

                return {};
        */
    }
}
