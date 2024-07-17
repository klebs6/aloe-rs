crate::ix!();

#[no_copy]
#[leak_detector]
pub struct VoiceModel<'a> {
    base:           ListBoxModel,
    voice_products: StringArray,
    purchases:      &'a mut VoicePurchases<'a>,
}

impl<'a> VoiceModel<'a> {
    
    pub fn new(voice_purchases: &mut VoicePurchases) -> Self {
    
        todo!();
        /*
        : purchases(voicePurchases),

            voiceProducts = purchases.getVoiceNames();
        */
    }
    
    pub fn get_num_rows(&mut self) -> i32 {
        
        todo!();
        /*
            return voiceProducts.size();
        */
    }
    
    pub fn refresh_component_for_row(&mut self, 
        row:      i32,
        selected: bool,
        existing: *mut Component) -> *mut Component {
        
        todo!();
        /*
            if (isPositiveAndBelow (row, voiceProducts.size()))
            {
                if (existing == nullptr)
                    existing = new VoiceRow (purchases);

                if (auto* voiceRow = dynamic_cast<VoiceRow*> (existing))
                    voiceRow->update (row, selected);

                return existing;
            }

            return nullptr;
        */
    }
    
    pub fn paint_list_box_item(&mut self, 
        _0:          i32,
        g:           &mut Graphics,
        w:           i32,
        h:           i32,
        is_selected: bool)  {
        
        todo!();
        /*
            auto r = Rectangle<int> (0, 0, w, h).reduced (4);

            auto& lf = Desktop::getInstance().getDefaultLookAndFeel();
            g.setColour (lf.findColour (isSelected ? (int) TextEditor::highlightColourId : (int) ListBox::backgroundColourId));
            g.fillRect (r);
        */
    }
}
