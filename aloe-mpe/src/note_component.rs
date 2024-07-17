crate::ix!();

#[no_copy]
#[leak_detector]
pub struct NoteComponent<'a> {
    base:            Component<'a>,
    note:            MPENote,
    colour:          Colour,
    centre:          Point<f32>,
    max_note_radius: f32, // default = 100.0f
}

impl<'a> NoteComponent<'a> {

    pub fn new(
        n:             &MPENote,
        colour_to_use: Colour) -> Self {
    
        todo!();
        /*
        : note(n),
        : colour(colourToUse),
        */
    }
    
    pub fn update(&mut self, 
        new_note:   &MPENote,
        new_centre: Point<f32>)  {
        
        todo!();
        /*
            note = newNote;
            centre = newCentre;

            setBounds (getSquareAroundCentre (jmax (getNoteOnRadius(), getNoteOffRadius(), getPressureRadius()))
                         .getUnion (getTextRectangle())
                         .getSmallestIntegerContainer()
                         .expanded (3));

            repaint();
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            if (note.keyState == MPENote::keyDown || note.keyState == MPENote::keyDownAndSustained)
                drawPressedNoteCircle (g, colour);
            else if (note.keyState == MPENote::sustained)
                drawSustainedNoteCircle (g, colour);
            else
                return;

            drawNoteLabel (g, colour);
        */
    }
    
    pub fn draw_pressed_note_circle(&mut self, 
        g:           &mut Graphics,
        zone_colour: Colour)  {
        
        todo!();
        /*
            g.setColour (zoneColour.withAlpha (0.3f));
            g.fillEllipse (translateToLocalBounds (getSquareAroundCentre (getNoteOnRadius())));
            g.setColour (zoneColour);
            g.drawEllipse (translateToLocalBounds (getSquareAroundCentre (getPressureRadius())), 2.0f);
        */
    }
    
    pub fn draw_sustained_note_circle(&mut self, 
        g:           &mut Graphics,
        zone_colour: Colour)  {
        
        todo!();
        /*
            g.setColour (zoneColour);
            Path circle, dashedCircle;
            circle.addEllipse (translateToLocalBounds (getSquareAroundCentre (getNoteOffRadius())));
            float dashLengths[] = { 3.0f, 3.0f };
            PathStrokeType (2.0, PathStrokeType::mitered).createDashedStroke (dashedCircle, circle, dashLengths, 2);
            g.fillPath (dashedCircle);
        */
    }
    
    pub fn draw_note_label(&mut self, 
        g:           &mut Graphics,
        zone_colour: Colour)  {
        
        todo!();
        /*
            auto textBounds = translateToLocalBounds (getTextRectangle()).getSmallestIntegerContainer();

            g.drawText ("+", textBounds, Justification::centred);
            g.drawText (MidiMessage::getMidiNoteName (note.initialNote, true, true, 3), textBounds, Justification::centredBottom);
            g.setFont (Font (22.0f, Font::bold));
            g.drawText (String (note.midiChannel), textBounds, Justification::centredTop);
        */
    }
    
    pub fn get_square_around_centre(&self, radius: f32) -> Rectangle<f32> {
        
        todo!();
        /*
            return Rectangle<float> (radius * 2.0f, radius * 2.0f).withCentre (centre);
        */
    }
    
    pub fn translate_to_local_bounds(&self, r: Rectangle<f32>) -> Rectangle<f32> {
        
        todo!();
        /*
            return r - getPosition().toFloat();
        */
    }
    
    pub fn get_text_rectangle(&self) -> Rectangle<f32> {
        
        todo!();
        /*
            return Rectangle<float> (30.0f, 50.0f).withCentre (centre);
        */
    }
    
    pub fn get_note_on_radius(&self) -> f32 {
        
        todo!();
        /*
            return note.noteOnVelocity .asUnsignedFloat() * maxNoteRadius;
        */
    }
    
    pub fn get_note_off_radius(&self) -> f32 {
        
        todo!();
        /*
            return note.noteOffVelocity.asUnsignedFloat() * maxNoteRadius;
        */
    }
    
    pub fn get_pressure_radius(&self) -> f32 {
        
        todo!();
        /*
            return note.pressure       .asUnsignedFloat() * maxNoteRadius;
        */
    }
}
