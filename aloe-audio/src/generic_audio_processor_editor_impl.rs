crate::ix!();

#[no_copy]
#[leak_detector]
pub struct GenericAudioProcessorEditorImpl<'a> {
    owner:             &'a mut GenericAudioProcessorEditor<'a>,
    legacy_parameters: LegacyAudioParametersWrapper,
    view:              Viewport<'a>,
}

impl<'a> Drop for GenericAudioProcessorEditorImpl<'a> {

    fn drop(&mut self) {
        todo!();
        /*
            view.setViewedComponent (nullptr, false);
        */
    }
}

impl<'a> GenericAudioProcessorEditorImpl<'a> {

    pub fn new(parent: &mut GenericAudioProcessorEditor) -> Self {
    
        todo!();
        /*
        : owner(parent),

            ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6011)
            auto* p = parent.getAudioProcessor();
            jassert (p != nullptr);

            legacyParameters.update (*p, false);

            owner.setOpaque (true);

            view.setViewedComponent (new ParametersPanel (*p, legacyParameters.params));
            owner.addAndMakeVisible (view);

            view.setScrollBarsShown (true, false);
            ALOE_END_IGNORE_WARNINGS_MSVC
        */
    }
    
    pub fn resize(&mut self, size: Rectangle<i32>)  {
        
        todo!();
        /*
            view.setBounds (size);
            auto content = view.getViewedComponent();
            content->setSize (view.getMaximumVisibleWidth(), content->getHeight());
        */
    }
}
