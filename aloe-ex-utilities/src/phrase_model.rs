crate::ix!();

#[derive(Default)]
#[no_copy]
#[leak_detector]
pub struct PhraseModel {
    base:    ListBoxModel,
    phrases: StringArray, //{"I love Aloe!", "The five dimensions of touch", "Make it fast!"};
}

impl PhraseModel {
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return phrases.size();
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        row:         i32,
        g:           &mut Graphics,
        w:           i32,
        h:           i32,
        is_selected: bool)  {
        
        todo!();
        /*
            Rectangle<int> r (0, 0, w, h);

            auto& lf = Desktop::getInstance().getDefaultLookAndFeel();
            g.setColour (lf.findColour (isSelected ? (int) TextEditor::highlightColourId : (int) ListBox::backgroundColourId));
            g.fillRect (r);

            g.setColour (lf.findColour (ListBox::textColourId));

            g.setFont (18);

            String phrase = (isPositiveAndBelow (row, phrases.size()) ? phrases[row] : String{});
            g.drawText (phrase, 10, 0, w, h, Justification::centredLeft);
        */
    }
}
