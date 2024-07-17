crate::ix!();

pub struct TextPropertyComponentLabelComp<'a> {
    base:                        Label<'a>,
    owner:                       &'a mut TextPropertyComponent<'a>,
    max_chars:                   i32,
    is_multiline:                bool,
    interested_in_file_drag:     bool, // default = true
    text_to_display_when_empty:  String,
    alpha_to_use_for_empty_text: f32, // default = 0.0f
}

impl<'a> FileDragAndDropTarget for TextPropertyComponentLabelComp<'a> {

}

impl<'a> ShouldDrawDragImageWhenOver for TextPropertyComponentLabelComp<'a> { }

impl<'a> ItemDropped for TextPropertyComponentLabelComp<'a> {

    fn item_dropped(&mut self, _: &DragAndDropTargetSourceDetails<'_>) { 
        todo!() 
    }
}

impl<'a> ItemDragExit                for TextPropertyComponentLabelComp<'a> { }
impl<'a> ItemDragMove                for TextPropertyComponentLabelComp<'a> { }
impl<'a> ItemDragEnter               for TextPropertyComponentLabelComp<'a> { }

impl<'a> IsInterestedInDragSource for TextPropertyComponentLabelComp<'a> {

    fn is_interested_in_drag_source(&mut self, _: &DragAndDropTargetSourceDetails<'_>) -> bool {
        todo!()
    }
}

impl<'a> TextPropertyComponentLabelComp<'a> {

    pub fn new(
        tpc:        &mut TextPropertyComponent,
        char_limit: i32,
        multiline:  bool,
        editable:   bool) -> Self {
    
        todo!();
        /*


            : Label ({}, {}),
              owner (tpc),
              maxChars (charLimit),
              isMultiline (multiline)

            setEditable (editable, editable);

            updateColours();
        */

    }
    
    pub fn is_interested_in_file_drag(&mut self, _0: &Vec<String>) -> bool {
        
        todo!();
        /*
            return interestedInFileDrag;
        */

    }
    
    pub fn files_dropped(&mut self, 
        files: &Vec<String>,
        _1:    i32,
        _2:    i32)  {
        
        todo!();
        /*
            setText (getText() + files.joinIntoString (isMultiline ? "\n" : ", "), sendNotificationSync);
            showEditor();
        */

    }
    
    pub fn create_editor_component(&mut self) -> *mut TextEditor {
        
        todo!();
        /*
            auto* ed = Label::createEditorComponent();
            ed->setInputRestrictions (maxChars);

            if (isMultiline)
            {
                ed->setMultiLine (true, true);
                ed->setReturnKeyStartsNewLine (true);
            }

            return ed;
        */

    }
    
    pub fn text_was_edited(&mut self)  {
        
        todo!();
        /*
            owner.textWasEdited();
        */

    }
    
    pub fn update_colours(&mut self)  {
        
        todo!();
        /*
            setColour (backgroundColourId, owner.findColour (TextPropertyComponent::backgroundColourId));
            setColour (outlineColourId,    owner.findColour (TextPropertyComponent::outlineColourId));
            setColour (textColourId,       owner.findColour (TextPropertyComponent::textColourId));
            repaint();
        */

    }
    
    pub fn set_interested_in_file_drag(&mut self, is_interested: bool)  {
        
        todo!();
        /*
            interestedInFileDrag = isInterested;
        */

    }
    
    pub fn set_text_to_display_when_empty(&mut self, 
        text:  &String,
        alpha: f32)  {
        
        todo!();
        /*
            textToDisplayWhenEmpty = text;
            alphaToUseForEmptyText = alpha;
        */

    }
    
    pub fn paint_over_children(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (getText().isEmpty() && ! isBeingEdited())
            {
                auto& lf = owner.getLookAndFeel();
                auto textArea = lf.getLabelBorderSize (*this).subtractedFrom (getLocalBounds());
                auto labelFont = lf.getLabelFont (*this);

                g.setColour (owner.findColour (TextPropertyComponent::textColourId).withAlpha (alphaToUseForEmptyText));
                g.setFont (labelFont);

                g.drawFittedText (textToDisplayWhenEmpty, textArea, getJustificationType(),
                                  jmax (1, (int) ((float) textArea.getHeight() / labelFont.getHeight())),
                                  getMinimumHorizontalScale());
            }
        */

    }
}
