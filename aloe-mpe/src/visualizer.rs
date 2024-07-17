crate::ix!();

#[no_copy]
#[leak_detector]
pub struct Visualiser<'a> {
    base:            Component<'a>,
    base3:           AsyncUpdater<'a>,
    note_components: Vec<Box<NoteComponent<'a>>>,
    lock:            CriticalSection,
    active_notes:    Vec<MPENote>,
    colour_picker:   &'a mut ZoneColourPicker,
}

impl<'a> MpeSetupComponentListener for Visualiser<'a> {

    fn zone_changed(&mut self, 
        is_lower:         bool,
        num_member_chans: i32,
        per_note_pb:      i32,
        master_pb:        i32) 
    {
        todo!();
    }

    fn all_zones_cleared(&mut self) 
    {
        todo!();
    }

    fn legacy_mode_changed(&mut self, 
        legacy_mode_enabled: bool,
        pitchbend_range:     i32,
        channel_range:       Range<i32>)
    {
        todo!();
    }

    fn voice_stealing_enabled_changed(&mut self, voice_stealing_enabled: bool)
    {
        todo!();
    }

    fn number_of_voices_changed(&mut self, number_of_voices: i32)
    {
        todo!();
    }
}

impl<'a> Visualiser<'a> {

    pub fn new(zone_colour_picker: &mut ZoneColourPicker) -> Self {
    
        todo!();
        /*
        : colour_picker(zoneColourPicker),
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (Colours::black);

            auto noteDistance = float (getWidth()) / 128;
            for (auto i = 0; i < 128; ++i)
            {
                auto x = noteDistance * (float) i;
                auto noteHeight = int (MidiMessage::isMidiNoteBlack (i) ? 0.7 * getHeight() : getHeight());

                g.setColour (MidiMessage::isMidiNoteBlack (i) ? Colours::white : Colours::grey);
                g.drawLine (x, 0.0f, x, (float) noteHeight);

                if (i > 0 && i % 12 == 0)
                {
                    g.setColour (Colours::grey);
                    auto octaveNumber = (i / 12) - 2;
                    g.drawText ("C" + String (octaveNumber), (int) x - 15, getHeight() - 30, 30, 30, Justification::centredBottom);
                }
            }
        */
    }
    
    pub fn note_added(&mut self, new_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
            activeNotes.add (newNote);
            triggerAsyncUpdate();
        */
    }
    
    pub fn note_pressure_changed(&mut self, note: MPENote)  {
        
        todo!();
        /*
            noteChanged (note);
        */
    }
    
    pub fn note_pitchbend_changed(&mut self, note: MPENote)  {
        
        todo!();
        /*
            noteChanged (note);
        */
    }
    
    pub fn note_timbre_changed(&mut self, note: MPENote)  {
        
        todo!();
        /*
            noteChanged (note);
        */
    }
    
    pub fn note_key_state_changed(&mut self, note: MPENote)  {
        
        todo!();
        /*
            noteChanged (note);
        */
    }
    
    pub fn note_changed(&mut self, changed_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            for (auto& note : activeNotes)
                if (note.noteID == changedNote.noteID)
                    note = changedNote;

            triggerAsyncUpdate();
        */
    }
    
    pub fn note_released(&mut self, finished_note: MPENote)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            for (auto i = activeNotes.size(); --i >= 0;)
                if (activeNotes.getReference(i).noteID == finishedNote.noteID)
                    activeNotes.remove (i);

            triggerAsyncUpdate();
        */
    }
    
    pub fn find_active_note(&self, noteid: i32) -> *const MPENote {
        
        todo!();
        /*
            for (auto& note : activeNotes)
                if (note.noteID == noteID)
                    return &note;

            return nullptr;
        */
    }
    
    pub fn find_note_component(&self, noteid: i32) -> *mut NoteComponent {
        
        todo!();
        /*
            for (auto& noteComp : noteComponents)
                if (noteComp->note.noteID == noteID)
                    return noteComp;

            return nullptr;
        */
    }
    
    pub fn handle_async_update(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

            for (auto i = noteComponents.size(); --i >= 0;)
                if (findActiveNote (noteComponents.getUnchecked(i)->note.noteID) == nullptr)
                    noteComponents.remove (i);

            for (auto& note : activeNotes)
                if (findNoteComponent (note.noteID) == nullptr)
                    addAndMakeVisible (noteComponents.add (new NoteComponent (note, colourPicker.getColourForMidiChannel(note.midiChannel))));

            for (auto& noteComp : noteComponents)
                if (auto* noteInfo = findActiveNote (noteComp->note.noteID))
                    noteComp->update (*noteInfo, getCentrePositionForNote (*noteInfo));
        */
    }
    
    pub fn get_centre_position_for_note(&self, note: MPENote) -> Point<f32> {
        
        todo!();
        /*
            auto n = float (note.initialNote) + float (note.totalPitchbendInSemitones);
            auto x = (float) getWidth() * n / 128;
            auto y = (float) getHeight() * (1 - note.timbre.asUnsignedFloat());

            return { x, y };
        */
    }
}
