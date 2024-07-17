crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/synthesisers/aloe_Synthesizer.h]

/**
  | Base class for a musical device that
  | can play sounds.
  | 
  | To create a synthesiser, you'll need
  | to create a subclass of SynthesizerSound
  | to describe each sound available to
  | your synth, and a subclass of SynthesizerVoice
  | which can play back one of these sounds.
  | 
  | Then you can use the addVoice() and addSound()
  | methods to give the synthesiser a set
  | of sounds, and a set of voices it can use
  | to play them. If you only give it one voice
  | it will be monophonic - the more voices
  | it has, the more polyphony it'll have
  | available.
  | 
  | Then repeatedly call the renderNextBlock()
  | method to produce the audio. Any midi
  | events that go in will be scanned for
  | note on/off messages, and these are
  | used to start and stop the voices playing
  | the appropriate sounds.
  | 
  | While it's playing, you can also cause
  | notes to be triggered by calling the
  | noteOn(), noteOff() and other controller
  | methods.
  | 
  | Before rendering, be sure to call the
  | setCurrentPlaybackSampleRate()
  | to tell it what the target playback rate
  | is. This value is passed on to the voices
  | so that they can pitch their output correctly.
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct Synthesizer {

    /**
      | This is used to control access to the
      | rendering callback and the note trigger
      | methods.
      |
      */
    lock:                            CriticalSection,

    voices:                          Vec<Box<SynthesizerVoice>>,
    sounds:                          ReferenceCountedArray<dyn SynthesizerSound>,

    /**
      | The last pitch-wheel values for each
      | midi channel.
      |
      */
    last_pitch_wheel_values:         [i32; 16],

    sample_rate:                     f64,  // default = 0
    last_note_on_counter:            u32,  // default = 0
    minimum_sub_block_size:          i32,  // default = 32
    sub_block_subdivision_is_strict: bool, // default = false
    should_steal_notes:              bool, // default = true
    sustain_pedals_down:             BigInteger,
}

impl Default for Synthesizer {
    
    /**
      | Creates a new synthesiser. You'll need
      | to add some sounds and voices before
      | it'll make any sound.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
            for (int i = 0; i < numElementsInArray (lastPitchWheelValues); ++i)
            lastPitchWheelValues[i] = 0x2000;
        */
    }
    
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/synthesisers/aloe_Synthesizer.cpp]
impl Synthesizer {

    /**
      | Returns the number of voices that have
      | been added.
      |
      */
    pub fn get_num_voices(&self) -> i32 {
        
        todo!();
        /*
            return voices.size();
        */
    }

    /**
      | Returns the number of sounds that have
      | been added to the synth.
      |
      */
    pub fn get_num_sounds(&self) -> i32 {
        
        todo!();
        /*
            return sounds.size();
        */
    }

    /**
      | Returns one of the sounds.
      |
      */
    pub fn get_sound(&self, index: i32) -> SynthesizerSoundPtr {
        
        todo!();
        /*
            return sounds[index];
        */
    }

    /**
      | Returns true if note-stealing is enabled.
      | @see setNoteStealingEnabled
      |
      */
    pub fn is_note_stealing_enabled(&self) -> bool {
        
        todo!();
        /*
            return shouldStealNotes;
        */
    }

    /**
      | Returns the current target sample rate
      | at which rendering is being done. Subclasses
      | may need to know this so that they can
      | pitch things correctly.
      |
      */
    pub fn get_sample_rate(&self) -> f64 {
        
        todo!();
        /*
            return sampleRate;
        */
    }

    /**
      | Returns one of the voices that have been
      | added.
      |
      */
    pub fn get_voice(&self, index: i32) -> *mut SynthesizerVoice {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return voices [index];
        */
    }
    
    /**
      | Deletes all voices.
      |
      */
    pub fn clear_voices(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        voices.clear();
        */
    }
    
    /**
      | Adds a new voice to the synth.
      | 
      | All the voices should be the same class
      | of object and are treated equally.
      | 
      | The object passed in will be managed
      | by the synthesiser, which will delete
      | it later on when no longer needed. The
      | caller should not retain a pointer to
      | the voice.
      |
      */
    pub fn add_voice(&mut self, new_voice: *mut SynthesizerVoice) -> *mut SynthesizerVoice {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        newVoice->setCurrentPlaybackSampleRate (sampleRate);
        return voices.add (newVoice);
        */
    }
    
    /**
      | Deletes one of the voices.
      |
      */
    pub fn remove_voice(&mut self, index: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        voices.remove (index);
        */
    }
    
    /**
      | Deletes all sounds.
      |
      */
    pub fn clear_sounds(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        sounds.clear();
        */
    }
    
    /**
      | Adds a new sound to the synthesiser.
      | 
      | The object passed in is reference counted,
      | so will be deleted when the synthesiser
      | and all voices are no longer using it.
      |
      */
    pub fn add_sound(&mut self, new_sound: &SynthesizerSoundPtr) -> SynthesizerSoundPtr {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        return sounds.add (newSound);
        */
    }
    
    /**
      | Removes and deletes one of the sounds.
      |
      */
    pub fn remove_sound(&mut self, index: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);
        sounds.remove (index);
        */
    }
    
    /**
      | If set to true, then the synth will try
      | to take over an existing voice if it runs
      | out and needs to play another note.
      | 
      | The value of this boolean is passed into
      | findFreeVoice(), so the result will
      | depend on the implementation of this
      | method.
      |
      */
    pub fn set_note_stealing_enabled(&mut self, should_steal: bool)  {
        
        todo!();
        /*
            shouldStealNotes = shouldSteal;
        */
    }
    
    /**
      | Sets a minimum limit on the size to which
      | audio sub-blocks will be divided when
      | rendering.
      | 
      | When rendering, the audio blocks that
      | are passed into renderNextBlock()
      | will be split up into smaller blocks
      | that lie between all the incoming midi
      | messages, and it is these smaller sub-blocks
      | that are rendered with multiple calls
      | to renderVoices().
      | 
      | Obviously in a pathological case where
      | there are midi messages on every sample,
      | then renderVoices() could be called
      | once per sample and lead to poor performance,
      | so this setting allows you to set a lower
      | limit on the block size.
      | 
      | The default setting is 32, which means
      | that midi messages are accurate to about
      | < 1ms accuracy, which is probably fine
      | for most purposes, but you may want to
      | increase or decrease this value for
      | your synth.
      | 
      | If shouldBeStrict is true, the audio
      | sub-blocks will strictly never be smaller
      | than numSamples.
      | 
      | If shouldBeStrict is false (default),
      | the first audio sub-block in the buffer
      | is allowed to be smaller, to make sure
      | that the first MIDI event in a buffer
      | will always be sample-accurate (this
      | can sometimes help to avoid quantisation
      | or phasing issues).
      |
      */
    pub fn set_minimum_rendering_subdivision_size(
        &mut self, 
        num_samples:      i32,
        should_be_strict: Option<bool>

    ) {

        let should_be_strict: bool = should_be_strict.unwrap_or(false);
        
        todo!();
        /*
            jassert (numSamples > 0); // it wouldn't make much sense for this to be less than 1
        minimumSubBlockSize = numSamples;
        subBlockSubdivisionIsStrict = shouldBeStrict;
        */
    }
    
    /**
      | Tells the synthesiser what the sample
      | rate is for the audio it's being used
      | to render.
      | 
      | This value is propagated to the voices
      | so that they can use it to render the correct
      | pitches.
      |
      */
    pub fn set_current_playback_sample_rate(&mut self, new_rate: f64)  {
        
        todo!();
        /*
            if (sampleRate != newRate)
        {
            const ScopedLock sl (lock);
            allNotesOff (0, false);
            sampleRate = newRate;

            for (auto* voice : voices)
                voice->setCurrentPlaybackSampleRate (newRate);
        }
        */
    }
    
    pub fn process_next_block<FloatType: num::Float>(
        &mut self, 
        output_audio: &mut AudioBuffer<FloatType>,
        midi_data:    &MidiBuffer,
        start_sample: i32,
        num_samples:  i32

    ) {
    
        todo!();
        /*
            // must set the sample rate before using this!
        jassert (sampleRate != 0);
        const int targetChannels = outputAudio.getNumChannels();

        auto midiIterator = midiData.findNextSamplePosition (startSample);

        bool firstEvent = true;

        const ScopedLock sl (lock);

        for (; numSamples > 0; ++midiIterator)
        {
            if (midiIterator == midiData.cend())
            {
                if (targetChannels > 0)
                    renderVoices (outputAudio, startSample, numSamples);

                return;
            }

            const auto metadata = *midiIterator;
            const int samplesToNextMidiMessage = metadata.samplePosition - startSample;

            if (samplesToNextMidiMessage >= numSamples)
            {
                if (targetChannels > 0)
                    renderVoices (outputAudio, startSample, numSamples);

                handleMidiEvent (metadata.getMessage());
                break;
            }

            if (samplesToNextMidiMessage < ((firstEvent && ! subBlockSubdivisionIsStrict) ? 1 : minimumSubBlockSize))
            {
                handleMidiEvent (metadata.getMessage());
                continue;
            }

            firstEvent = false;

            if (targetChannels > 0)
                renderVoices (outputAudio, startSample, samplesToNextMidiMessage);

            handleMidiEvent (metadata.getMessage());
            startSample += samplesToNextMidiMessage;
            numSamples  -= samplesToNextMidiMessage;
        }

        std::for_each (midiIterator,
                       midiData.cend(),
                       [&] (const MidiMessageMetadata& meta) { handleMidiEvent (meta.getMessage()); });
        */
    }
    
    /**
      | Creates the next block of audio output.
      | 
      | This will process the next numSamples
      | of data from all the voices, and add that
      | output to the audio block supplied,
      | starting from the offset specified.
      | Note that the data will be added to the
      | current contents of the buffer, so you
      | should clear it before calling this
      | method if necessary.
      | 
      | The midi events in the inputMidi buffer
      | are parsed for note and controller events,
      | and these are used to trigger the voices.
      | Note that the startSample offset applies
      | both to the audio output buffer and the
      | midi input buffer, so any midi events
      | with timestamps outside the specified
      | region will be ignored.
      |
      */
    pub fn render_next_block<T: Float>(&mut self, 
        output_audio: &mut AudioBuffer<T>,
        input_midi:   &MidiBuffer,
        start_sample: i32,
        num_samples:  i32)  {
        
        todo!();
        /*
            processNextBlock (outputAudio, inputMidi, startSample, numSamples);
        */
    }
    
    /**
      | Renders the voices for the given range.
      | By default this just calls renderNextBlock()
      | on each voice, but you may need to override
      | it to handle custom cases.
      |
      */
    pub fn render_voices<FloatType: num::Float>(
        &mut self, 
        buffer:       &mut AudioBuffer<FloatType>,
        start_sample: i32,
        num_samples:  i32
    ) {
        
        todo!();
        /*
            for (auto* voice : voices)
            voice->renderNextBlock (buffer, startSample, numSamples);
        */
    }
    
    /**
      | Can be overridden to do custom handling
      | of incoming midi events.
      |
      */
    pub fn handle_midi_event(&mut self, m: &MidiMessage)  {
        
        todo!();
        /*
            const int channel = m.getChannel();

        if (m.isNoteOn())
        {
            noteOn (channel, m.getNoteNumber(), m.getFloatVelocity());
        }
        else if (m.isNoteOff())
        {
            noteOff (channel, m.getNoteNumber(), m.getFloatVelocity(), true);
        }
        else if (m.isAllNotesOff() || m.isAllSoundOff())
        {
            allNotesOff (channel, true);
        }
        else if (m.isPitchWheel())
        {
            const int wheelPos = m.getPitchWheelValue();
            lastPitchWheelValues [channel - 1] = wheelPos;
            handlePitchWheel (channel, wheelPos);
        }
        else if (m.isAftertouch())
        {
            handleAftertouch (channel, m.getNoteNumber(), m.getAfterTouchValue());
        }
        else if (m.isChannelPressure())
        {
            handleChannelPressure (channel, m.getChannelPressureValue());
        }
        else if (m.isController())
        {
            handleController (channel, m.getControllerNumber(), m.getControllerValue());
        }
        else if (m.isProgramChange())
        {
            handleProgramChange (channel, m.getProgramChangeNumber());
        }
        */
    }
    
    /**
      | Triggers a note-on event.
      | 
      | The default method here will find all
      | the sounds that want to be triggered
      | by this note/channel. For each sound,
      | it'll try to find a free voice, and use
      | the voice to start playing the sound.
      | 
      | Subclasses might want to override this
      | if they need a more complex algorithm.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | The midiChannel parameter is the channel,
      | between 1 and 16 inclusive.
      |
      */
    pub fn note_on(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* sound : sounds)
        {
            if (sound->appliesToNote (midiNoteNumber) && sound->appliesToChannel (midiChannel))
            {
                // If hitting a note that's still ringing, stop it first (it could be
                // still playing because of the sustain or sostenuto pedal).
                for (auto* voice : voices)
                    if (voice->getCurrentlyPlayingNote() == midiNoteNumber && voice->isPlayingChannel (midiChannel))
                        stopVoice (voice, 1.0f, true);

                startVoice (findFreeVoice (sound, midiChannel, midiNoteNumber, shouldStealNotes),
                            sound, midiChannel, midiNoteNumber, velocity);
            }
        }
        */
    }
    
    /**
      | Starts a specified voice playing a particular
      | sound. You'll probably never need to
      | call this, it's used internally by noteOn(),
      | but may be needed by subclasses for custom
      | behaviours.
      |
      */
    pub fn start_voice(
        &mut self, 
        voice:            *mut SynthesizerVoice,
        sound:            &mut dyn SynthesizerSound,
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32

    ) {
        
        todo!();
        /*
            if (voice != nullptr && sound != nullptr)
        {
            if (voice->currentlyPlayingSound != nullptr)
                voice->stopNote (0.0f, false);

            voice->currentlyPlayingNote = midiNoteNumber;
            voice->currentPlayingMidiChannel = midiChannel;
            voice->noteOnTime = ++lastNoteOnCounter;
            voice->currentlyPlayingSound = sound;
            voice->setKeyDown (true);
            voice->setSostenutoPedalDown (false);
            voice->setSustainPedalDown (sustainPedalsDown[midiChannel]);

            voice->startNote (midiNoteNumber, velocity, sound,
                              lastPitchWheelValues [midiChannel - 1]);
        }
        */
    }
    
    /**
      | Stops a given voice. You should never
      | need to call this, it's used internally
      | by noteOff, but is protected in case
      | it's useful for some custom subclasses.
      | It basically just calls through to SynthesizerVoice::stopNote(),
      | and has some assertions to sanity-check
      | a few things.
      |
      */
    pub fn stop_voice(&mut self, 
        voice:          *mut SynthesizerVoice,
        velocity:       f32,
        allow_tail_off: bool)  {
        
        todo!();
        /*
            jassert (voice != nullptr);

        voice->stopNote (velocity, allowTailOff);

        // the subclass MUST call clearCurrentNote() if it's not tailing off! RTFM for stopNote()!
        jassert (allowTailOff || (voice->getCurrentlyPlayingNote() < 0 && voice->getCurrentlyPlayingSound() == nullptr));
        */
    }
    
    /**
      | Triggers a note-off event.
      | 
      | This will turn off any voices that are
      | playing a sound for the given note/channel.
      | 
      | If allowTailOff is true, the voices
      | will be allowed to fade out the notes
      | gracefully (if they can do). If this
      | is false, the notes will all be cut off
      | immediately.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | The midiChannel parameter is the channel,
      | between 1 and 16 inclusive.
      |
      */
    pub fn note_off(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        velocity:         f32,
        allow_tail_off:   bool)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
        {
            if (voice->getCurrentlyPlayingNote() == midiNoteNumber
                  && voice->isPlayingChannel (midiChannel))
            {
                if (auto sound = voice->getCurrentlyPlayingSound())
                {
                    if (sound->appliesToNote (midiNoteNumber)
                         && sound->appliesToChannel (midiChannel))
                    {
                        jassert (! voice->keyIsDown || voice->isSustainPedalDown() == sustainPedalsDown [midiChannel]);

                        voice->setKeyDown (false);

                        if (! (voice->isSustainPedalDown() || voice->isSostenutoPedalDown()))
                            stopVoice (voice, velocity, allowTailOff);
                    }
                }
            }
        }
        */
    }
    
    /**
      | Turns off all notes.
      | 
      | This will turn off any voices that are
      | playing a sound on the given midi channel.
      | 
      | If midiChannel is 0 or less, then all
      | voices will be turned off, regardless
      | of which channel they're playing. Otherwise
      | it represents a valid midi channel,
      | from 1 to 16 inclusive.
      | 
      | If allowTailOff is true, the voices
      | will be allowed to fade out the notes
      | gracefully (if they can do). If this
      | is false, the notes will all be cut off
      | immediately.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      |
      */
    pub fn all_notes_off(&mut self, 
        midi_channel:   i32,
        allow_tail_off: bool)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
            if (midiChannel <= 0 || voice->isPlayingChannel (midiChannel))
                voice->stopNote (1.0f, allowTailOff);

        sustainPedalsDown.clear();
        */
    }
    
    /**
      | Sends a pitch-wheel message to any active
      | voices.
      | 
      | This will send a pitch-wheel message
      | to any voices that are playing sounds
      | on the given midi channel.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | -----------
      | @param midiChannel
      | 
      | the midi channel, from 1 to 16 inclusive
      | ----------
      | @param wheelValue
      | 
      | the wheel position, from 0 to 0x3fff,
      | as returned by MidiMessage::getPitchWheelValue()
      |
      */
    pub fn handle_pitch_wheel(&mut self, 
        midi_channel: i32,
        wheel_value:  i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
            if (midiChannel <= 0 || voice->isPlayingChannel (midiChannel))
                voice->pitchWheelMoved (wheelValue);
        */
    }
    
    /**
      | Sends a midi controller message to any
      | active voices.
      | 
      | This will send a midi controller message
      | to any voices that are playing sounds
      | on the given midi channel.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | -----------
      | @param midiChannel
      | 
      | the midi channel, from 1 to 16 inclusive
      | ----------
      | @param controllerNumber
      | 
      | the midi controller type, as returned
      | by MidiMessage::getControllerNumber()
      | ----------
      | @param controllerValue
      | 
      | the midi controller value, between
      | 0 and 127, as returned by MidiMessage::getControllerValue()
      |
      */
    pub fn handle_controller(&mut self, 
        midi_channel:      i32,
        controller_number: i32,
        controller_value:  i32)  {
        
        todo!();
        /*
            switch (controllerNumber)
        {
            case 0x40:  handleSustainPedal   (midiChannel, controllerValue >= 64); break;
            case 0x42:  handleSostenutoPedal (midiChannel, controllerValue >= 64); break;
            case 0x43:  handleSoftPedal      (midiChannel, controllerValue >= 64); break;
            default:    break;
        }

        const ScopedLock sl (lock);

        for (auto* voice : voices)
            if (midiChannel <= 0 || voice->isPlayingChannel (midiChannel))
                voice->controllerMoved (controllerNumber, controllerValue);
        */
    }
    
    /**
      | Sends an aftertouch message.
      | 
      | This will send an aftertouch message
      | to any voices that are playing sounds
      | on the given midi channel and note number.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | -----------
      | @param midiChannel
      | 
      | the midi channel, from 1 to 16 inclusive
      | ----------
      | @param midiNoteNumber
      | 
      | the midi note number, 0 to 127
      | ----------
      | @param aftertouchValue
      | 
      | the aftertouch value, between 0 and
      | 127, as returned by MidiMessage::getAftertouchValue()
      |
      */
    pub fn handle_aftertouch(&mut self, 
        midi_channel:     i32,
        midi_note_number: i32,
        aftertouch_value: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
            if (voice->getCurrentlyPlayingNote() == midiNoteNumber
                  && (midiChannel <= 0 || voice->isPlayingChannel (midiChannel)))
                voice->aftertouchChanged (aftertouchValue);
        */
    }
    
    /**
      | Sends a channel pressure message.
      | 
      | This will send a channel pressure message
      | to any voices that are playing sounds
      | on the given midi channel.
      | 
      | This method will be called automatically
      | according to the midi data passed into
      | renderNextBlock(), but may be called
      | explicitly too.
      | 
      | -----------
      | @param midiChannel
      | 
      | the midi channel, from 1 to 16 inclusive
      | ----------
      | @param channelPressureValue
      | 
      | the pressure value, between 0 and 127,
      | as returned by MidiMessage::getChannelPressureValue()
      |
      */
    pub fn handle_channel_pressure(&mut self, 
        midi_channel:           i32,
        channel_pressure_value: i32)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
            if (midiChannel <= 0 || voice->isPlayingChannel (midiChannel))
                voice->channelPressureChanged (channelPressureValue);
        */
    }
    
    /**
      | Handles a sustain pedal event.
      |
      */
    pub fn handle_sustain_pedal(&mut self, 
        midi_channel: i32,
        is_down:      bool)  {
        
        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);
        const ScopedLock sl (lock);

        if (isDown)
        {
            sustainPedalsDown.setBit (midiChannel);

            for (auto* voice : voices)
                if (voice->isPlayingChannel (midiChannel) && voice->isKeyDown())
                    voice->setSustainPedalDown (true);
        }
        else
        {
            for (auto* voice : voices)
            {
                if (voice->isPlayingChannel (midiChannel))
                {
                    voice->setSustainPedalDown (false);

                    if (! (voice->isKeyDown() || voice->isSostenutoPedalDown()))
                        stopVoice (voice, 1.0f, true);
                }
            }

            sustainPedalsDown.clearBit (midiChannel);
        }
        */
    }
    
    /**
      | Handles a sostenuto pedal event.
      |
      */
    pub fn handle_sostenuto_pedal(&mut self, 
        midi_channel: i32,
        is_down:      bool)  {
        
        todo!();
        /*
            jassert (midiChannel > 0 && midiChannel <= 16);
        const ScopedLock sl (lock);

        for (auto* voice : voices)
        {
            if (voice->isPlayingChannel (midiChannel))
            {
                if (isDown)
                    voice->setSostenutoPedalDown (true);
                else if (voice->isSostenutoPedalDown())
                    stopVoice (voice, 1.0f, true);
            }
        }
        */
    }
    
    /**
      | Can be overridden to handle soft pedal
      | events.
      |
      */
    pub fn handle_soft_pedal(&mut self, 
        midi_channel: i32,
        is_down:      bool)  {
        
        todo!();
        /*
            ignoreUnused (midiChannel);
        jassert (midiChannel > 0 && midiChannel <= 16);
        */
    }
    
    /**
      | Can be overridden to handle an incoming
      | program change message. The base class
      | implementation of this has no effect,
      | but you may want to make your own synth
      | react to program changes.
      |
      */
    pub fn handle_program_change(&mut self, 
        midi_channel:   i32,
        program_number: i32)  {
        
        todo!();
        /*
            ignoreUnused (midiChannel, programNumber);
        jassert (midiChannel > 0 && midiChannel <= 16);
        */
    }
    
    /**
      | Searches through the voices to find
      | one that's not currently playing, and
      | which can play the given sound.
      | 
      | Returns nullptr if all voices are busy
      | and stealing isn't enabled.
      | 
      | To implement a custom note-stealing
      | algorithm, you can either override
      | this method, or (preferably) override
      | findVoiceToSteal().
      |
      */
    pub fn find_free_voice(
        &self, 
        sound_to_play:           &mut dyn SynthesizerSound,
        midi_channel:            i32,
        midi_note_number:        i32,
        steal_if_none_available: bool

    ) -> *mut SynthesizerVoice {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (auto* voice : voices)
            if ((! voice->isVoiceActive()) && voice->canPlaySound (soundToPlay))
                return voice;

        if (stealIfNoneAvailable)
            return findVoiceToSteal (soundToPlay, midiChannel, midiNoteNumber);

        return nullptr;
        */
    }
    
    /**
      | Chooses a voice that is most suitable
      | for being re-used. The default method
      | will attempt to find the oldest voice
      | that isn't the bottom or top note being
      | played. If that's not suitable for your
      | synth, you can override this method
      | and do something more cunning instead.
      |
      */
    pub fn find_voice_to_steal(
        &self, 
        sound_to_play:    &mut dyn SynthesizerSound,
        midi_channel:     i32,
        midi_note_number: i32

    ) -> *mut SynthesizerVoice {
        
        todo!();
        /*
            // This voice-stealing algorithm applies the following heuristics:
        // - Re-use the oldest notes first
        // - Protect the lowest & topmost notes, even if sustained, but not if they've been released.

        // apparently you are trying to render audio without having any voices...
        jassert (! voices.isEmpty());

        // These are the voices we want to protect (ie: only steal if unavoidable)
        SynthesizerVoice* low = nullptr; // Lowest sounding note, might be sustained, but NOT in release phase
        SynthesizerVoice* top = nullptr; // Highest sounding note, might be sustained, but NOT in release phase

        // this is a list of voices we can steal, sorted by how long they've been running
        Vec<SynthesizerVoice*> usableVoices;
        usableVoices.ensureStorageAllocated (voices.size());

        for (auto* voice : voices)
        {
            if (voice->canPlaySound (soundToPlay))
            {
                jassert (voice->isVoiceActive()); // We wouldn't be here otherwise

                usableVoices.add (voice);

                // NB: Using a functor rather than a lambda here due to scare-stories about
                // compilers generating code containing heap allocations..
                struct Sorter
                {
                    bool operator() (const SynthesizerVoice* a, const SynthesizerVoice* b) const  { return a->wasStartedBefore (*b); }
                };

                std::sort (usableVoices.begin(), usableVoices.end(), Sorter());

                if (! voice->isPlayingButReleased()) // Don't protect released notes
                {
                    auto note = voice->getCurrentlyPlayingNote();

                    if (low == nullptr || note < low->getCurrentlyPlayingNote())
                        low = voice;

                    if (top == nullptr || note > top->getCurrentlyPlayingNote())
                        top = voice;
                }
            }
        }

        // Eliminate pathological cases (ie: only 1 note playing): we always give precedence to the lowest note(s)
        if (top == low)
            top = nullptr;

        // The oldest note that's playing with the target pitch is ideal..
        for (auto* voice : usableVoices)
            if (voice->getCurrentlyPlayingNote() == midiNoteNumber)
                return voice;

        // Oldest voice that has been released (no finger on it and not held by sustain pedal)
        for (auto* voice : usableVoices)
            if (voice != low && voice != top && voice->isPlayingButReleased())
                return voice;

        // Oldest voice that doesn't have a finger on it:
        for (auto* voice : usableVoices)
            if (voice != low && voice != top && ! voice->isKeyDown())
                return voice;

        // Oldest voice that isn't protected
        for (auto* voice : usableVoices)
            if (voice != low && voice != top)
                return voice;

        // We've only got "protected" voices now: lowest note takes priority
        jassert (low != nullptr);

        // Duophonic synth: give priority to the bass note:
        if (top != nullptr)
            return top;

        return low;
        */
    }
}
