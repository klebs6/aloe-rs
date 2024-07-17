crate::ix!();

#[no_copy]
pub struct MidiKeyboardComponentUpDownButton<'a> {
    base:  Button<'a>,
    owner: &'a mut MidiKeyboardComponent<'a>,
    delta: i32,
}

impl<'a> MidiKeyboardComponentUpDownButton<'a> {

    pub fn new(
        c: &mut MidiKeyboardComponent,
        d: i32) -> Self {
    
        todo!();
        /*


            : Button ({}), owner (c), delta (d)
        */
    }
    
    pub fn clicked(&mut self)  {
        
        todo!();
        /*
            auto note = owner.getLowestVisibleKey();

                if (delta < 0)
                    note = (note - 1) / 12;
                else
                    note = note / 12 + 1;

                owner.setLowestVisibleKey (note * 12);
        */
    }
    
    pub fn paint_button(&mut self, 
        g:                                 &mut Graphics,
        should_draw_button_as_highlighted: bool,
        should_draw_button_as_down:        bool)  {
        
        todo!();
        /*
            owner.drawUpDownButton (g, getWidth(), getHeight(),
                                        shouldDrawButtonAsHighlighted, shouldDrawButtonAsDown,
                                        delta > 0);
        */
    }
}
