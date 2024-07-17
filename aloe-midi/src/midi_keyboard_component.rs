crate::ix!();

pub const whiteNotes: &[u8] = &[ 0, 2, 4, 5, 7, 9, 11 ];
pub const blackNotes: &[u8] = &[ 1, 3, 6, 8, 10 ];

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_MidiKeyboardComponent.h]

/**
  | A component that displays a piano keyboard,
  | whose notes can be clicked on.
  | 
  | This component will mimic a physical
  | midi keyboard, showing the current
  | state of a MidiKeyboardState object.
  | When the on-screen keys are clicked
  | on, it will play these notes by calling
  | the noteOn() and noteOff() methods
  | of its MidiKeyboardState object.
  | 
  | Another feature is that the computer
  | keyboard can also be used to play notes.
  | By default it maps the top two rows of
  | a standard qwerty keyboard to the notes,
  | but these can be remapped if needed.
  | It will only respond to keypresses when
  | it has the keyboard focus, so to disable
  | this feature you can call setWantsKeyboardFocus
  | (false).
  | 
  | The component is also a ChangeBroadcaster,
  | so if you want to be informed when the
  | keyboard is scrolled, you can register
  | a ChangeListener for callbacks.
  | 
  | @see MidiKeyboardState
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MidiKeyboardComponent<'a> {
    base:                            Component<'a>,
    base3:                           ChangeBroadcaster<'a>,
    base4:                           Timer,
    state:                           &'a mut MidiKeyboardState,
    black_note_length_ratio:         f32, // default = 0.7
    black_note_width_ratio:          f32, // default = 0.7
    x_offset:                        f32, // default = 0
    key_width:                       f32, // default = 16.0
    scroll_button_width:             i32, // default = 12
    orientation:                     MidiKeyboardComponentOrientation,
    midi_channel:                    i32, // default = 1
    midi_in_channel_mask:            i32, // default = 0xffff
    velocity:                        f32, // default = 1.0
    mouse_over_notes:                Vec<i32>,
    mouse_down_notes:                Vec<i32>,
    keys_pressed:                    BigInteger,
    keys_currently_drawn_down:       BigInteger,
    should_check_state:              bool, // default = false
    range_start:                     i32, // default = 0
    range_end:                       i32, // default = 127
    first_key:                       f32, //= 12 * 4.0;
    can_scroll:                      bool, // default = true
    use_mouse_position_for_velocity: bool, // default = true
    scroll_down:                     Box<Button<'a>>,
    scroll_up:                       Box<Button<'a>>,
    key_presses:                     Vec<KeyPress>,
    key_press_notes:                 Vec<i32>,
    key_mapping_octave:              i32, // default = 6
    octave_num_for_middlec:          i32, // default = 3
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_utils/gui/aloe_MidiKeyboardComponent.cpp]
impl<'a> MidiKeyboardStateListener for MidiKeyboardComponent<'a> {

    fn handle_note_on(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            shouldCheckState = true; // (probably being called from the audio thread, so avoid blocking in here)
        */
    }
    
    fn handle_note_off(&mut self, 
        _0:               *mut MidiKeyboardState,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            shouldCheckState = true; // (probably being called from the audio thread, so avoid blocking in here)
        */
    }
}

impl<'a> Drop for MidiKeyboardComponent<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
        state.removeListener (this);
 */
    }
}

impl<'a> MidiKeyboardComponent<'a> {

    /**
      | Returns the midi channel that the keyboard
      | is using for midi messages. @see setMidiChannel
      |
      */
    pub fn get_midi_channel(&self) -> i32 {
        
        todo!();
        /*
            return midiChannel;
        */
    }

    /**
      | Returns the current set of midi channels
      | represented by the component.
      | 
      | This is the value that was set with setMidiChannelsToDisplay().
      |
      */
    pub fn get_midi_channels_to_display(&self) -> i32 {
        
        todo!();
        /*
            return midiInChannelMask;
        */
    }

    /**
      | Returns the width that was set by setKeyWidth().
      |
      */
    pub fn get_key_width(&self) -> f32 {
        
        todo!();
        /*
            return keyWidth;
        */
    }

    /**
      | Returns the width that was set by setScrollButtonWidth().
      |
      */
    pub fn get_scroll_button_width(&self) -> i32 {
        
        todo!();
        /*
            return scrollButtonWidth;
        */
    }

    /**
      | Returns the keyboard's current direction.
      |
      */
    pub fn get_orientation(&self) -> MidiKeyboardComponentOrientation {
        
        todo!();
        /*
            return orientation;
        */
    }

    /**
      | Returns the first note in the available
      | range. @see setAvailableRange
      |
      */
    pub fn get_range_start(&self) -> i32 {
        
        todo!();
        /*
            return rangeStart;
        */
    }

    /**
      | Returns the last note in the available
      | range. @see setAvailableRange
      |
      */
    pub fn get_range_end(&self) -> i32 {
        
        todo!();
        /*
            return rangeEnd;
        */
    }

    /**
      | Returns the number of the first key shown
      | in the component. @see setLowestVisibleKey
      |
      */
    pub fn get_lowest_visible_key(&self) -> i32 {
        
        todo!();
        /*
            return (int) firstKey;
        */
    }

    /**
      | Returns the length of the black notes
      | as a proportion of the white note length.
      |
      */
    pub fn get_black_note_length_proportion(&self) -> f32 {
        
        todo!();
        /*
            return blackNoteLengthRatio;
        */
    }

    /**
      | Returns the width of the black notes
      | as a proportion of the white note width.
      |
      */
    pub fn get_black_note_width_proportion(&self) -> f32 {
        
        todo!();
        /*
            return blackNoteWidthRatio;
        */
    }

    /**
      | Returns the absolute width of the black
      | notes. This will be their vertical or
      | horizontal width, depending on the
      | keyboard's orientation.
      |
      */
    pub fn get_black_note_width(&self) -> f32 {
        
        todo!();
        /*
            return keyWidth * blackNoteWidthRatio;
        */
    }

    /**
      | This returns the value set by setOctaveForMiddleC().
      | @see setOctaveForMiddleC
      |
      */
    pub fn get_octave_for_middlec(&self) -> i32 {
        
        todo!();
        /*
            return octaveNumForMiddleC;
        */
    }
    
    /**
      | Creates a MidiKeyboardComponent.
      | 
      | -----------
      | @param state
      | 
      | the midi keyboard model that this component
      | will represent
      | ----------
      | @param orientation
      | 
      | whether the keyboard is horizontal
      | or vertical
      |
      */
    pub fn new(
        s: &mut MidiKeyboardState,
        o: MidiKeyboardComponentOrientation) -> Self {
    
        todo!();
        /*
        : state(s),
        : orientation(o),

            scrollDown.reset (new MidiKeyboardComponentUpDownButton (*this, -1));
        scrollUp  .reset (new MidiKeyboardComponentUpDownButton (*this, 1));

        addChildComponent (scrollDown.get());
        addChildComponent (scrollUp.get());

        // initialise with a default set of qwerty key-mappings..
        int note = 0;

        for (char c : "awsedftgyhujkolp;")
            setKeyPressForNote (KeyPress (c, 0, 0), note++);

        mouseOverNotes.insertMultiple (0, -1, 32);
        mouseDownNotes.insertMultiple (0, -1, 32);

        colourChanged();
        setWantsKeyboardFocus (true);

        state.addListener (this);

        startTimerHz (20);
        */
    }
    
    /**
      | Changes the width used to draw the white
      | keys.
      |
      */
    pub fn set_key_width(&mut self, width_in_pixels: f32)  {
        
        todo!();
        /*
            jassert (widthInPixels > 0);

        if (keyWidth != widthInPixels) // Prevent infinite recursion if the width is being computed in a 'resized()' call-back
        {
            keyWidth = widthInPixels;
            resized();
        }
        */
    }
    
    /**
      | Changes the width used to draw the buttons
      | that scroll the keyboard up/down in
      | octaves.
      |
      */
    pub fn set_scroll_button_width(&mut self, width_in_pixels: i32)  {
        
        todo!();
        /*
            jassert (widthInPixels > 0);

        if (scrollButtonWidth != widthInPixels)
        {
            scrollButtonWidth = widthInPixels;
            resized();
        }
        */
    }
    
    /**
      | Changes the keyboard's current direction.
      |
      */
    pub fn set_orientation(&mut self, new_orientation: MidiKeyboardComponentOrientation)  {
        
        todo!();
        /*
            if (orientation != newOrientation)
        {
            orientation = newOrientation;
            resized();
        }
        */
    }
    
    /**
      | Sets the range of midi notes that the
      | keyboard will be limited to.
      | 
      | By default the range is 0 to 127 (inclusive),
      | but you can limit this if you only want
      | a restricted set of the keys to be shown.
      | 
      | Note that the values here are inclusive
      | and must be between 0 and 127.
      |
      */
    pub fn set_available_range(&mut self, 
        lowest_note:  i32,
        highest_note: i32)  {
        
        todo!();
        /*
            jassert (lowestNote >= 0 && lowestNote <= 127);
        jassert (highestNote >= 0 && highestNote <= 127);
        jassert (lowestNote <= highestNote);

        if (rangeStart != lowestNote || rangeEnd != highestNote)
        {
            rangeStart = jlimit (0, 127, lowestNote);
            rangeEnd = jlimit (0, 127, highestNote);
            firstKey = jlimit ((float) rangeStart, (float) rangeEnd, firstKey);
            resized();
        }
        */
    }
    
    /**
      | If the keyboard extends beyond the size
      | of the component, this will scroll it
      | to show the given key at the start.
      | 
      | Whenever the keyboard's position is
      | changed, this will use the ChangeBroadcaster
      | base class to send a callback to any ChangeListeners
      | that have been registered.
      |
      */
    pub fn set_lowest_visible_key(&mut self, note_number: i32)  {
        
        todo!();
        /*
            setLowestVisibleKeyFloat ((float) noteNumber);
        */
    }
    
    pub fn set_lowest_visible_key_float(&mut self, note_number: f32)  {
        
        todo!();
        /*
            noteNumber = jlimit ((float) rangeStart, (float) rangeEnd, noteNumber);

        if (noteNumber != firstKey)
        {
            bool hasMoved = (((int) firstKey) != (int) noteNumber);
            firstKey = noteNumber;

            if (hasMoved)
                sendChangeMessage();

            resized();
        }
        */
    }
    
    /**
      | If set to true, then scroll buttons will
      | appear at either end of the keyboard
      | if there are too many notes to fit them
      | all in the component at once.
      |
      */
    pub fn set_scroll_buttons_visible(&mut self, new_can_scroll: bool)  {
        
        todo!();
        /*
            if (canScroll != newCanScroll)
        {
            canScroll = newCanScroll;
            resized();
        }
        */
    }
    
    pub fn colour_changed(&mut self)  {
        
        todo!();
        /*
            setOpaque (findColour (whiteNoteColourId).isOpaque());
        repaint();
        */
    }
    
    /**
      | Changes the midi channel number that
      | will be used for events triggered by
      | clicking on the component.
      | 
      | The channel must be between 1 and 16 (inclusive).
      | This is the channel that will be passed
      | on to the MidiKeyboardState::noteOn()
      | method when the user clicks the component.
      | 
      | Although this is the channel used for
      | outgoing events, the component can
      | display incoming events from more than
      | one channel - see setMidiChannelsToDisplay()
      | 
      | @see setVelocity
      |
      */
    pub fn set_midi_channel(&mut self, midi_channel_number: i32)  {
        
        todo!();
        /*
            jassert (midiChannelNumber > 0 && midiChannelNumber <= 16);

        if (midiChannel != midiChannelNumber)
        {
            resetAnyKeysInUse();
            midiChannel = jlimit (1, 16, midiChannelNumber);
        }
        */
    }
    
    /**
      | Sets a mask to indicate which incoming
      | midi channels should be represented
      | by key movements.
      | 
      | The mask is a set of bits, where bit 0 =
      | midi channel 1, bit 1 = midi channel 2,
      | etc.
      | 
      | If the MidiKeyboardState has a key down
      | for any of the channels whose bits are
      | set in this mask, the on-screen keys
      | will also go down.
      | 
      | By default, this mask is set to 0xffff
      | (all channels displayed).
      | 
      | @see setMidiChannel
      |
      */
    pub fn set_midi_channels_to_display(&mut self, midi_channel_mask: i32)  {
        
        todo!();
        /*
            midiInChannelMask = midiChannelMask;
        shouldCheckState = true;
        */
    }
    
    /**
      | Changes the velocity used in midi note-on
      | messages that are triggered by clicking
      | on the component.
      | 
      | Values are 0 to 1.0, where 1.0 is the heaviest.
      | 
      | @see setMidiChannel
      |
      */
    pub fn set_velocity(&mut self, 
        v:                  f32,
        use_mouse_position: bool)  {
        
        todo!();
        /*
            velocity = jlimit (0.0f, 1.0f, v);
        useMousePositionForVelocity = useMousePosition;
        */
    }
    
    /**
      | Calculates the position of a given midi-note.
      | 
      | This can be overridden to create layouts
      | with custom key-widths.
      | 
      | -----------
      | @param midiNoteNumber
      | 
      | the note to find
      | ----------
      | @param keyWidth
      | 
      | the desired width in pixels of one key
      | - see setKeyWidth()
      | 
      | -----------
      | @return
      | 
      | the start and length of the key along
      | the axis of the keyboard
      |
      */
    pub fn get_key_position(&self, 
        midi_note_number: i32,
        target_key_width: f32) -> Range<f32> {
        
        todo!();
        /*
            jassert (midiNoteNumber >= 0 && midiNoteNumber < 128);

        static const float notePos[] = { 0.0f, 1 - blackNoteWidthRatio * 0.6f,
                                         1.0f, 2 - blackNoteWidthRatio * 0.4f,
                                         2.0f,
                                         3.0f, 4 - blackNoteWidthRatio * 0.7f,
                                         4.0f, 5 - blackNoteWidthRatio * 0.5f,
                                         5.0f, 6 - blackNoteWidthRatio * 0.3f,
                                         6.0f };

        auto octave = midiNoteNumber / 12;
        auto note   = midiNoteNumber % 12;

        auto start = (float) octave * 7.0f * targetKeyWidth + notePos[note] * targetKeyWidth;
        auto width = MidiMessage::isMidiNoteBlack (note) ? blackNoteWidthRatio * targetKeyWidth : targetKeyWidth;

        return { start, start + width };
        */
    }
    
    pub fn get_key_pos(&self, midi_note_number: i32) -> Range<f32> {
        
        todo!();
        /*
            return getKeyPosition (midiNoteNumber, keyWidth)
                 - xOffset
                 - getKeyPosition (rangeStart, keyWidth).getStart();
        */
    }
    
    /**
      | Returns the rectangle for a given key
      | if within the displayable range
      |
      */
    pub fn get_rectangle_for_key(&self, note: i32) -> Rectangle<f32> {
        
        todo!();
        /*
            jassert (note >= rangeStart && note <= rangeEnd);

        auto pos = getKeyPos (note);
        auto x = pos.getStart();
        auto w = pos.getLength();

        if (MidiMessage::isMidiNoteBlack (note))
        {
            auto blackNoteLength = getBlackNoteLength();

            switch (orientation)
            {
                case horizontalKeyboard:            return { x, 0, w, blackNoteLength };
                case verticalKeyboardFacingLeft:    return { (float) getWidth() - blackNoteLength, x, blackNoteLength, w };
                case verticalKeyboardFacingRight:   return { 0, (float) getHeight() - x - w, blackNoteLength, w };
                default:                            jassertfalse; break;
            }
        }
        else
        {
            switch (orientation)
            {
                case horizontalKeyboard:            return { x, 0, w, (float) getHeight() };
                case verticalKeyboardFacingLeft:    return { 0, x, (float) getWidth(), w };
                case verticalKeyboardFacingRight:   return { 0, (float) getHeight() - x - w, (float) getWidth(), w };
                default:                            jassertfalse; break;
            }
        }

        return {};
        */
    }
    
    /**
      | Returns the position within the component
      | of the left-hand edge of a key.
      | 
      | Depending on the keyboard's orientation,
      | this may be a horizontal or vertical
      | distance, in either direction.
      |
      */
    pub fn get_key_start_position(&self, midi_note_number: i32) -> f32 {
        
        todo!();
        /*
            return getKeyPos (midiNoteNumber).getStart();
        */
    }
    
    /**
      | Returns the total width needed to fit
      | all the keys in the available range.
      |
      */
    pub fn get_total_keyboard_width(&self) -> f32 {
        
        todo!();
        /*
            return getKeyPos (rangeEnd).getEnd();
        */
    }
    
    /**
      | Returns the key at a given coordinate.
      |
      */
    pub fn get_note_at_position(&mut self, p: Point<f32>) -> i32 {
        
        todo!();
        /*
            float v;
        return xyToNote (p, v);
        */
    }
    
    pub fn xy_to_note(&mut self, 
        pos:                     Point<f32>,
        mouse_position_velocity: &mut f32) -> i32 {
        
        todo!();
        /*
            if (! reallyContains (pos.toInt(), false))
            return -1;

        auto p = pos;

        if (orientation != horizontalKeyboard)
        {
            p = { p.y, p.x };

            if (orientation == verticalKeyboardFacingLeft)
                p = { p.x, (float) getWidth() - p.y };
            else
                p = { (float) getHeight() - p.x, p.y };
        }

        return remappedXYToNote (p + Point<float> (xOffset, 0), mousePositionVelocity);
        */
    }
    
    pub fn remapped_xy_to_note(&self, 
        pos:                     Point<f32>,
        mouse_position_velocity: &mut f32) -> i32 {
        
        todo!();
        /*
            auto blackNoteLength = getBlackNoteLength();

        if (pos.getY() < blackNoteLength)
        {
            for (int octaveStart = 12 * (rangeStart / 12); octaveStart <= rangeEnd; octaveStart += 12)
            {
                for (int i = 0; i < 5; ++i)
                {
                    auto note = octaveStart + blackNotes[i];

                    if (note >= rangeStart && note <= rangeEnd)
                    {
                        if (getKeyPos (note).contains (pos.x - xOffset))
                        {
                            mousePositionVelocity = jmax (0.0f, pos.y / blackNoteLength);
                            return note;
                        }
                    }
                }
            }
        }

        for (int octaveStart = 12 * (rangeStart / 12); octaveStart <= rangeEnd; octaveStart += 12)
        {
            for (int i = 0; i < 7; ++i)
            {
                auto note = octaveStart + whiteNotes[i];

                if (note >= rangeStart && note <= rangeEnd)
                {
                    if (getKeyPos (note).contains (pos.x - xOffset))
                    {
                        auto whiteNoteLength = (orientation == horizontalKeyboard) ? getHeight() : getWidth();
                        mousePositionVelocity = jmax (0.0f, pos.y / (float) whiteNoteLength);
                        return note;
                    }
                }
            }
        }

        mousePositionVelocity = 0;
        return -1;
        */
    }
    
    pub fn repaint_note(&mut self, note_num: i32)  {
        
        todo!();
        /*
            if (noteNum >= rangeStart && noteNum <= rangeEnd)
            repaint (getRectangleForKey (noteNum).getSmallestIntegerContainer());
        */
    }
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (findColour (whiteNoteColourId));

        auto lineColour = findColour (keySeparatorLineColourId);
        auto textColour = findColour (textLabelColourId);

        for (int octave = 0; octave < 128; octave += 12)
        {
            for (int white = 0; white < 7; ++white)
            {
                auto noteNum = octave + whiteNotes[white];

                if (noteNum >= rangeStart && noteNum <= rangeEnd)
                    drawWhiteNote (noteNum, g, getRectangleForKey (noteNum),
                                   state.isNoteOnForChannels (midiInChannelMask, noteNum),
                                   mouseOverNotes.contains (noteNum), lineColour, textColour);
            }
        }

        float x1 = 0.0f, y1 = 0.0f, x2 = 0.0f, y2 = 0.0f;
        auto width = getWidth();
        auto height = getHeight();

        if (orientation == verticalKeyboardFacingLeft)
        {
            x1 = (float) width - 1.0f;
            x2 = (float) width - 5.0f;
        }
        else if (orientation == verticalKeyboardFacingRight)
            x2 = 5.0f;
        else
            y2 = 5.0f;

        auto x = getKeyPos (rangeEnd).getEnd();
        auto shadowCol = findColour (shadowColourId);

        if (! shadowCol.isTransparent())
        {
            g.setGradientFill (ColourGradient (shadowCol, x1, y1, shadowCol.withAlpha (0.0f), x2, y2, false));

            switch (orientation)
            {
                case horizontalKeyboard:            g.fillRect (0.0f, 0.0f, x, 5.0f); break;
                case verticalKeyboardFacingLeft:    g.fillRect ((float) width - 5.0f, 0.0f, 5.0f, x); break;
                case verticalKeyboardFacingRight:   g.fillRect (0.0f, 0.0f, 5.0f, x); break;
                default: break;
            }
        }

        if (! lineColour.isTransparent())
        {
            g.setColour (lineColour);

            switch (orientation)
            {
                case horizontalKeyboard:            g.fillRect (0.0f, (float) height - 1.0f, x, 1.0f); break;
                case verticalKeyboardFacingLeft:    g.fillRect (0.0f, 0.0f, 1.0f, x); break;
                case verticalKeyboardFacingRight:   g.fillRect ((float) width - 1.0f, 0.0f, 1.0f, x); break;
                default: break;
            }
        }

        auto blackNoteColour = findColour (blackNoteColourId);

        for (int octave = 0; octave < 128; octave += 12)
        {
            for (int black = 0; black < 5; ++black)
            {
                auto noteNum = octave + blackNotes[black];

                if (noteNum >= rangeStart && noteNum <= rangeEnd)
                    drawBlackNote (noteNum, g, getRectangleForKey (noteNum),
                                   state.isNoteOnForChannels (midiInChannelMask, noteNum),
                                   mouseOverNotes.contains (noteNum), blackNoteColour);
            }
        }
        */
    }
    
    /**
      | Draws a white note in the given rectangle.
      | 
      | isOver indicates whether the mouse
      | is over the key, isDown indicates whether
      | the key is currently pressed down.
      | 
      | When doing this, be sure to note the keyboard's
      | orientation.
      |
      */
    pub fn draw_white_note(&mut self, 
        midi_note_number: i32,
        g:                &mut Graphics,
        area:             Rectangle<f32>,
        is_down:          bool,
        is_over:          bool,
        line_colour:      Colour,
        text_colour:      Colour)  {
        
        todo!();
        /*
            auto c = Colours::transparentWhite;

        if (isDown)  c = findColour (keyDownOverlayColourId);
        if (isOver)  c = c.overlaidWith (findColour (mouseOverKeyOverlayColourId));

        g.setColour (c);
        g.fillRect (area);

        auto text = getWhiteNoteText (midiNoteNumber);

        if (text.isNotEmpty())
        {
            auto fontHeight = jmin (12.0f, keyWidth * 0.9f);

            g.setColour (textColour);
            g.setFont (Font (fontHeight).withHorizontalScale (0.8f));

            switch (orientation)
            {
                case horizontalKeyboard:            g.drawText (text, area.withTrimmedLeft (1.0f).withTrimmedBottom (2.0f), Justification::centredBottom, false); break;
                case verticalKeyboardFacingLeft:    g.drawText (text, area.reduced (2.0f), Justification::centredLeft,   false); break;
                case verticalKeyboardFacingRight:   g.drawText (text, area.reduced (2.0f), Justification::centredRight,  false); break;
                default: break;
            }
        }

        if (! lineColour.isTransparent())
        {
            g.setColour (lineColour);

            switch (orientation)
            {
                case horizontalKeyboard:            g.fillRect (area.withWidth (1.0f)); break;
                case verticalKeyboardFacingLeft:    g.fillRect (area.withHeight (1.0f)); break;
                case verticalKeyboardFacingRight:   g.fillRect (area.removeFromBottom (1.0f)); break;
                default: break;
            }

            if (midiNoteNumber == rangeEnd)
            {
                switch (orientation)
                {
                    case horizontalKeyboard:            g.fillRect (area.expanded (1.0f, 0).removeFromRight (1.0f)); break;
                    case verticalKeyboardFacingLeft:    g.fillRect (area.expanded (0, 1.0f).removeFromBottom (1.0f)); break;
                    case verticalKeyboardFacingRight:   g.fillRect (area.expanded (0, 1.0f).removeFromTop (1.0f)); break;
                    default: break;
                }
            }
        }
        */
    }
    
    /**
      | Draws a black note in the given rectangle.
      | 
      | isOver indicates whether the mouse
      | is over the key, isDown indicates whether
      | the key is currently pressed down.
      | 
      | When doing this, be sure to note the keyboard's
      | orientation.
      |
      */
    pub fn draw_black_note(&mut self, 
        midi_note_number: i32,
        g:                &mut Graphics,
        area:             Rectangle<f32>,
        is_down:          bool,
        is_over:          bool,
        note_fill_colour: Colour)  {
        
        todo!();
        /*
            auto c = noteFillColour;

        if (isDown)  c = c.overlaidWith (findColour (keyDownOverlayColourId));
        if (isOver)  c = c.overlaidWith (findColour (mouseOverKeyOverlayColourId));

        g.setColour (c);
        g.fillRect (area);

        if (isDown)
        {
            g.setColour (noteFillColour);
            g.drawRect (area);
        }
        else
        {
            g.setColour (c.brighter());
            auto sideIndent = 1.0f / 8.0f;
            auto topIndent = 7.0f / 8.0f;
            auto w = area.getWidth();
            auto h = area.getHeight();

            switch (orientation)
            {
                case horizontalKeyboard:            g.fillRect (area.reduced (w * sideIndent, 0).removeFromTop   (h * topIndent)); break;
                case verticalKeyboardFacingLeft:    g.fillRect (area.reduced (0, h * sideIndent).removeFromRight (w * topIndent)); break;
                case verticalKeyboardFacingRight:   g.fillRect (area.reduced (0, h * sideIndent).removeFromLeft  (w * topIndent)); break;
                default: break;
            }
        }
        */
    }
    
    /**
      | This sets the octave number which is
      | shown as the octave number for middle
      | C.
      | 
      | This affects only the default implementation
      | of getWhiteNoteText(), which passes
      | this octave number to MidiMessage::getMidiNoteName()
      | in order to get the note text. See MidiMessage::getMidiNoteName()
      | for more info about the parameter.
      | 
      | By default this value is set to 3.
      | 
      | @see getOctaveForMiddleC
      |
      */
    pub fn set_octave_for_middlec(&mut self, octave_num: i32)  {
        
        todo!();
        /*
            octaveNumForMiddleC = octaveNum;
        repaint();
        */
    }
    
    /**
      | Allows text to be drawn on the white notes.
      | By default this is used to label the C
      | in each octave, but could be used for
      | other things. @see setOctaveForMiddleC
      |
      */
    pub fn get_white_note_text(&mut self, midi_note_number: i32) -> String {
        
        todo!();
        /*
            if (midiNoteNumber % 12 == 0)
            return MidiMessage::getMidiNoteName (midiNoteNumber, true, true, octaveNumForMiddleC);

        return {};
        */
    }
    
    /**
      | Draws the up and down buttons that scroll
      | the keyboard up/down in octaves.
      |
      */
    pub fn draw_up_down_button(&mut self, 
        g:                &mut Graphics,
        w:                i32,
        h:                i32,
        mouse_over:       bool,
        button_down:      bool,
        moves_octaves_up: bool)  {
        
        todo!();
        /*
            g.fillAll (findColour (upDownButtonBackgroundColourId));

        float angle = 0;

        switch (orientation)
        {
            case horizontalKeyboard:            angle = movesOctavesUp ? 0.0f  : 0.5f;  break;
            case verticalKeyboardFacingLeft:    angle = movesOctavesUp ? 0.25f : 0.75f; break;
            case verticalKeyboardFacingRight:   angle = movesOctavesUp ? 0.75f : 0.25f; break;
            default:                            jassertfalse; break;
        }

        Path path;
        path.addTriangle (0.0f, 0.0f, 0.0f, 1.0f, 1.0f, 0.5f);
        path.applyTransform (AffineTransform::rotation (MathConstants<float>::twoPi * angle, 0.5f, 0.5f));

        g.setColour (findColour (upDownButtonArrowColourId)
                      .withAlpha (buttonDown ? 1.0f : (mouseOver ? 0.6f : 0.4f)));

        g.fillPath (path, path.getTransformToScaleToFit (1.0f, 1.0f, (float) w - 2.0f, (float) h - 2.0f, true));
        */
    }
    
    /**
      | Sets the length of the black notes as
      | a proportion of the white note length.
      |
      */
    pub fn set_black_note_length_proportion(&mut self, ratio: f32)  {
        
        todo!();
        /*
            jassert (ratio >= 0.0f && ratio <= 1.0f);

        if (blackNoteLengthRatio != ratio)
        {
            blackNoteLengthRatio = ratio;
            resized();
        }
        */
    }
    
    /**
      | Returns the absolute length of the black
      | notes. This will be their vertical or
      | horizontal length, depending on the
      | keyboard's orientation.
      |
      */
    pub fn get_black_note_length(&self) -> f32 {
        
        todo!();
        /*
            auto whiteNoteLength = orientation == horizontalKeyboard ? getHeight() : getWidth();
        return (float) whiteNoteLength * blackNoteLengthRatio;
        */
    }
    
    /**
      | Sets the width of the black notes as a
      | proportion of the white note width.
      |
      */
    pub fn set_black_note_width_proportion(&mut self, ratio: f32)  {
        
        todo!();
        /*
            jassert (ratio >= 0.0f && ratio <= 1.0f);

        if (blackNoteWidthRatio != ratio)
        {
            blackNoteWidthRatio = ratio;
            resized();
        }
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto w = getWidth();
        auto h = getHeight();

        if (w > 0 && h > 0)
        {
            if (orientation != horizontalKeyboard)
                std::swap (w, h);

            auto kx2 = getKeyPos (rangeEnd).getEnd();

            if ((int) firstKey != rangeStart)
            {
                auto kx1 = getKeyPos (rangeStart).getStart();

                if (kx2 - kx1 <= (float) w)
                {
                    firstKey = (float) rangeStart;
                    sendChangeMessage();
                    repaint();
                }
            }

            scrollDown->setVisible (canScroll && firstKey > (float) rangeStart);

            xOffset = 0;

            if (canScroll)
            {
                auto scrollButtonW = jmin (scrollButtonWidth, w / 2);
                auto r = getLocalBounds();

                if (orientation == horizontalKeyboard)
                {
                    scrollDown->setBounds (r.removeFromLeft  (scrollButtonW));
                    scrollUp  ->setBounds (r.removeFromRight (scrollButtonW));
                }
                else if (orientation == verticalKeyboardFacingLeft)
                {
                    scrollDown->setBounds (r.removeFromTop    (scrollButtonW));
                    scrollUp  ->setBounds (r.removeFromBottom (scrollButtonW));
                }
                else
                {
                    scrollDown->setBounds (r.removeFromBottom (scrollButtonW));
                    scrollUp  ->setBounds (r.removeFromTop    (scrollButtonW));
                }

                auto endOfLastKey = getKeyPos (rangeEnd).getEnd();

                float mousePositionVelocity;
                auto spaceAvailable = w;
                auto lastStartKey = remappedXYToNote ({ endOfLastKey - (float) spaceAvailable, 0 }, mousePositionVelocity) + 1;

                if (lastStartKey >= 0 && ((int) firstKey) > lastStartKey)
                {
                    firstKey = (float) jlimit (rangeStart, rangeEnd, lastStartKey);
                    sendChangeMessage();
                }

                xOffset = getKeyPos ((int) firstKey).getStart();
            }
            else
            {
                firstKey = (float) rangeStart;
            }

            scrollUp->setVisible (canScroll && getKeyPos (rangeEnd).getStart() > (float) w);
            repaint();
        }
        */
    }
    
    pub fn reset_any_keys_in_use(&mut self)  {
        
        todo!();
        /*
            if (! keysPressed.isZero())
        {
            for (int i = 128; --i >= 0;)
                if (keysPressed[i])
                    state.noteOff (midiChannel, i, 0.0f);

            keysPressed.clear();
        }

        for (int i = mouseDownNotes.size(); --i >= 0;)
        {
            auto noteDown = mouseDownNotes.getUnchecked(i);

            if (noteDown >= 0)
            {
                state.noteOff (midiChannel, noteDown, 0.0f);
                mouseDownNotes.set (i, -1);
            }

            mouseOverNotes.set (i, -1);
        }
        */
    }
    
    pub fn update_note_under_mouse(
        &mut self, 
        e:       &MouseEvent,
        is_down: bool

    ) {
        
        todo!();
        /*
            updateNoteUnderMouse (e.getEventRelativeTo (this).position, isDown, e.source.getIndex());
        */
    }
    
    pub fn update_note_under_mouse_with_pos(
        &mut self, 
        pos:        Point<f32>,
        is_down:    bool,
        finger_num: i32

    ) {
        
        todo!();
        /*
            float mousePositionVelocity = 0.0f;
        auto newNote = xyToNote (pos, mousePositionVelocity);
        auto oldNote = mouseOverNotes.getUnchecked (fingerNum);
        auto oldNoteDown = mouseDownNotes.getUnchecked (fingerNum);
        auto eventVelocity = useMousePositionForVelocity ? mousePositionVelocity * velocity : velocity;

        if (oldNote != newNote)
        {
            repaintNote (oldNote);
            repaintNote (newNote);
            mouseOverNotes.set (fingerNum, newNote);
        }

        if (isDown)
        {
            if (newNote != oldNoteDown)
            {
                if (oldNoteDown >= 0)
                {
                    mouseDownNotes.set (fingerNum, -1);

                    if (! mouseDownNotes.contains (oldNoteDown))
                        state.noteOff (midiChannel, oldNoteDown, eventVelocity);
                }

                if (newNote >= 0 && ! mouseDownNotes.contains (newNote))
                {
                    state.noteOn (midiChannel, newNote, eventVelocity);
                    mouseDownNotes.set (fingerNum, newNote);
                }
            }
        }
        else if (oldNoteDown >= 0)
        {
            mouseDownNotes.set (fingerNum, -1);

            if (! mouseDownNotes.contains (oldNoteDown))
                state.noteOff (midiChannel, oldNoteDown, eventVelocity);
        }
        */
    }
    
    pub fn mouse_move(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateNoteUnderMouse (e, false);
        */
    }
    
    pub fn mouse_drag(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            float mousePositionVelocity;
        auto newNote = xyToNote (e.position, mousePositionVelocity);

        if (newNote >= 0 && mouseDraggedToKey (newNote, e))
            updateNoteUnderMouse (e, true);
        */
    }
    
    /**
      | Callback when the mouse is clicked on
      | a key.
      | 
      | You could use this to do things like handle
      | right-clicks on keys, etc.
      | 
      | Return true if you want the click to trigger
      | the note, or false if you want to handle
      | it yourself and not have the note played.
      | 
      | @see mouseDraggedToKey
      |
      */
    pub fn mouse_down_on_key(&mut self, 
        _0: i32,
        _1: &MouseEvent) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    /**
      | Callback when the mouse is dragged from
      | one key onto another.
      | 
      | Return true if you want the drag to trigger
      | the new note, or false if you want to handle
      | it yourself and not have the note played.
      | 
      | @see mouseDownOnKey
      |
      */
    pub fn mouse_dragged_to_key(&mut self, 
        _0: i32,
        _1: &MouseEvent) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
    
    /**
      | Callback when the mouse is released
      | from a key. @see mouseDownOnKey
      |
      */
    pub fn mouse_up_on_key(&mut self, 
        _0: i32,
        _1: &MouseEvent)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn mouse_down(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            float mousePositionVelocity;
        auto newNote = xyToNote (e.position, mousePositionVelocity);

        if (newNote >= 0 && mouseDownOnKey (newNote, e))
            updateNoteUnderMouse (e, true);
        */
    }
    
    pub fn mouse_up(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateNoteUnderMouse (e, false);

        float mousePositionVelocity;
        auto note = xyToNote (e.position, mousePositionVelocity);

        if (note >= 0)
            mouseUpOnKey (note, e);
        */
    }
    
    pub fn mouse_enter(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateNoteUnderMouse (e, false);
        */
    }
    
    pub fn mouse_exit(&mut self, e: &MouseEvent)  {
        
        todo!();
        /*
            updateNoteUnderMouse (e, false);
        */
    }
    
    pub fn mouse_wheel_move(&mut self, 
        _0:    &MouseEvent,
        wheel: &MouseWheelDetails)  {
        
        todo!();
        /*
            auto amount = (orientation == horizontalKeyboard && wheel.deltaX != 0)
                           ? wheel.deltaX : (orientation == verticalKeyboardFacingLeft ? wheel.deltaY
                                                                                       : -wheel.deltaY);

        setLowestVisibleKeyFloat (firstKey - amount * keyWidth);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            if (shouldCheckState)
        {
            shouldCheckState = false;

            for (int i = rangeStart; i <= rangeEnd; ++i)
            {
                bool isOn = state.isNoteOnForChannels (midiInChannelMask, i);

                if (keysCurrentlyDrawnDown[i] != isOn)
                {
                    keysCurrentlyDrawnDown.setBit (i, isOn);
                    repaintNote (i);
                }
            }
        }
        */
    }
    
    /**
      | Deletes all key-mappings. @see setKeyPressForNote
      |
      */
    pub fn clear_key_mappings(&mut self)  {
        
        todo!();
        /*
            resetAnyKeysInUse();
        keyPressNotes.clear();
        keyPresses.clear();
        */
    }
    
    /**
      | Maps a key-press to a given note.
      | 
      | -----------
      | @param key
      | 
      | the key that should trigger the note
      | ----------
      | @param midiNoteOffsetFromC
      | 
      | how many semitones above C the triggered
      | note should be. The actual midi note
      | that gets played will be this value +
      | (12 * the current base octave). To change
      | the base octave, see setKeyPressBaseOctave()
      |
      */
    pub fn set_key_press_for_note(&mut self, 
        key:                    &KeyPress,
        midi_note_offset_fromc: i32)  {
        
        todo!();
        /*
            removeKeyPressForNote (midiNoteOffsetFromC);

        keyPressNotes.add (midiNoteOffsetFromC);
        keyPresses.add (key);
        */
    }
    
    /**
      | Removes any key-mappings for a given
      | note. For a description of what the note
      | number means, see setKeyPressForNote().
      |
      */
    pub fn remove_key_press_for_note(&mut self, midi_note_offset_fromc: i32)  {
        
        todo!();
        /*
            for (int i = keyPressNotes.size(); --i >= 0;)
        {
            if (keyPressNotes.getUnchecked (i) == midiNoteOffsetFromC)
            {
                keyPressNotes.remove (i);
                keyPresses.remove (i);
            }
        }
        */
    }
    
    /**
      | Changes the base note above which key-press-triggered
      | notes are played.
      | 
      | The set of key-mappings that trigger
      | notes can be moved up and down to cover
      | the entire scale using this method.
      | 
      | The value passed in is an octave number
      | between 0 and 10 (inclusive), and indicates
      | which C is the base note to which the key-mapped
      | notes are relative.
      |
      */
    pub fn set_key_press_base_octave(&mut self, new_octave_number: i32)  {
        
        todo!();
        /*
            jassert (newOctaveNumber >= 0 && newOctaveNumber <= 10);

        keyMappingOctave = newOctaveNumber;
        */
    }
    
    pub fn key_state_changed(&mut self, is_key_down: bool) -> bool {
        
        todo!();
        /*
            bool keyPressUsed = false;

        for (int i = keyPresses.size(); --i >= 0;)
        {
            auto note = 12 * keyMappingOctave + keyPressNotes.getUnchecked (i);

            if (keyPresses.getReference(i).isCurrentlyDown())
            {
                if (! keysPressed[note])
                {
                    keysPressed.setBit (note);
                    state.noteOn (midiChannel, note, velocity);
                    keyPressUsed = true;
                }
            }
            else
            {
                if (keysPressed[note])
                {
                    keysPressed.clearBit (note);
                    state.noteOff (midiChannel, note, 0.0f);
                    keyPressUsed = true;
                }
            }
        }

        return keyPressUsed;
        */
    }
    
    pub fn key_pressed(&mut self, key: &KeyPress) -> bool {
        
        todo!();
        /*
            return keyPresses.contains (key);
        */
    }
    
    pub fn focus_lost(&mut self, _0: FocusChangeType)  {
        
        todo!();
        /*
            resetAnyKeysInUse();
        */
    }
}
