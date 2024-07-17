crate::ix!();

/**
  | A simple holder component with some
  | content, a title and an info tooltip
  | containing a brief description.
  | 
  | This component sets its accessibility
  | title and help text properties and also
  | acts as a focus container for its children.
  |
  */
#[no_copy]
#[leak_detector]
pub struct ContentComponent<'a> {
    base:        Component<'a>,
    title_label: Label<'a>,
    info_icon:   InfoIcon<'a>,
    content:     &'a mut Component<'a>,
}

impl<'a> Paint for ContentComponent<'a> {

    fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.setColour (Colours::black);
            g.drawRoundedRectangle (getLocalBounds().reduced (2).toFloat(), 5.0f, 3.0f);
        */
    }
}

impl<'a> Resized for ContentComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            auto bounds = getLocalBounds().reduced (5);

            auto topArea = bounds.removeFromTop (30);
            infoIcon.setBounds (topArea.removeFromLeft (30).reduced (5));
            titleLabel.setBounds (topArea.reduced (5));

            content.setBounds (bounds);
        */
    }
}

impl<'a> ContentComponent<'a> {

    pub fn new(
        title:              &String,
        info:               &String,
        content_to_display: &mut Component) -> Self {
    
        todo!();
        /*

            : titleLabel ({}, title),
               content (contentToDisplay)

            addAndMakeVisible (titleLabel);
            addAndMakeVisible (infoIcon);

            setTitle (title);
            setDescription (info);
            setFocusContainerType (FocusContainerType::focusContainer);

            infoIcon.setTooltip (info);
            infoIcon.setHelpText (info);

            addAndMakeVisible (content);
        */
    }
}
