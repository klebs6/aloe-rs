crate::ix!();

pub fn announcements_component_default_description_label<'a>() -> Label<'a> {

    todo!();

    /*
       { {}, "This is a demo of posting system announcements that will be read out by an accessibility client.\n\n"
       "You can enter some text to be read out in the text box below, set a priority for the message and then "
       "post it using the \"Announce\" button." }
       */
}

/**
  | The top-level component containing
  | an example of how to post system announcements.
  | 
  | The AccessibilityHandler::postAnnouncement()
  | method will post some text to the native
  | screen reader application to be read
  | out along with a priority determining
  | how it should be read out (whether it
  | should interrupt other announcements,
  | etc.).
  |
  */
#[no_copy]
#[leak_detector]
pub struct AnnouncementsComponent<'a> {
    base:               Component<'a>,
    description_label:  Label<'a>,
    text_entry_box:     TextEditor<'a>,
    priority_combo_box: ComboBox<'a>,
    announce_button:    TextButton<'a>, // default = "Announce" 
}

impl<'a> Default for AnnouncementsComponent<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            addAndMakeVisible (descriptionLabel);

            textEntryBox.setMultiLine (true);
            textEntryBox.setReturnKeyStartsNewLine (true);
            textEntryBox.setText ("Announcement text.");
            addAndMakeVisible (textEntryBox);

            priorityComboBox.addItemList ({ "Priority - Low", "Priority - Medium", "Priority - High" }, 1);
            priorityComboBox.setSelectedId (2);
            addAndMakeVisible (priorityComboBox);

            announceButton.onClick = [this]
            {
                auto priority = [this]
                {
                    switch (priorityComboBox.getSelectedId())
                    {
                        case 1:   return AccessibilityHandler::AnnouncementPriority::low;
                        case 2:   return AccessibilityHandler::AnnouncementPriority::medium;
                        case 3:   return AccessibilityHandler::AnnouncementPriority::high;
                    }

                    jassertfalse;
                    return AccessibilityHandler::AnnouncementPriority::medium;
                }();

                AccessibilityHandler::postAnnouncement (textEntryBox.getText(), priority);
            };

            addAndMakeVisible (announceButton);

            setTitle ("Announcements");
            setHelpText ("Type some text into the box and click the announce button to have it read out.");
            setFocusContainerType (FocusContainerType::focusContainer)
        */
    }
}

impl<'a> Resized for AnnouncementsComponent<'a> {

    fn resized(&mut self)  {
        
        todo!();
        /*
            Grid grid;

            grid.templateRows = { Grid::TrackInfo (Grid::Fr (3)),
                                  Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (1)),
                                  Grid::TrackInfo (Grid::Fr (1)) };

            grid.templateColumns = { Grid::TrackInfo (Grid::Fr (3)),
                                     Grid::TrackInfo (Grid::Fr (2)) };

            grid.items = { GridItem (descriptionLabel).withMargin (2).withColumn ({ GridItem::Span (2) }),
                           GridItem (textEntryBox).withMargin (2).withArea ({ 2 }, { 1 }, { 5 }, { 2 }),
                           GridItem (priorityComboBox).withMargin (2).withArea ({ 5 }, { 1 }, { 6 }, { 2 }),
                           GridItem (announceButton).withMargin (2).withArea ({ 4 }, { 2 }, { 5 }, { 3 }) };

            grid.performLayout (getLocalBounds());
        */
    }
}
