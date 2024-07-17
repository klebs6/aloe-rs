crate::ix!();

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
lazy_static!{
    /*
    static AlsaClient* instance;
    AlsaClient* AlsaClient::instance = nullptr;
    */
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
pub struct MidiInputThread {
    base:         Thread,
    client:       &mut AlsaClient,
    concatenator: MidiDataConcatenator, // default = 2048 
}

#[cfg(target_os="linux")]
#[cfg(feature = "alsa")]
impl MidiInputThread {

    pub fn new(c: &mut AlsaClient) -> Self {
    
        todo!();
        /*
            : Thread ("Aloe MIDI Input"), client (c)
                jassert (client.get() != nullptr);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            auto seqHandle = client.get();

                const int maxEventSize = 16 * 1024;
                snd_midi_event_t* midiParser;

                if (snd_midi_event_new (maxEventSize, &midiParser) >= 0)
                {
                    auto numPfds = snd_seq_poll_descriptors_count (seqHandle, POLLIN);
                    HeapBlock<pollfd> pfd (numPfds);
                    snd_seq_poll_descriptors (seqHandle, pfd, (unsigned int) numPfds, POLLIN);

                    HeapBlock<uint8> buffer (maxEventSize);

                    while (! threadShouldExit())
                    {
                        if (poll (pfd, (nfds_t) numPfds, 100) > 0) // there was a "500" here which is a bit long when we exit the program and have to wait for a timeout on this poll call
                        {
                            if (threadShouldExit())
                                break;

                            do
                            {
                                snd_seq_event_t* inputEvent = nullptr;

                                if (snd_seq_event_input (seqHandle, &inputEvent) >= 0)
                                {
                                    // xxx what about SYSEXes that are too big for the buffer?
                                    auto numBytes = snd_midi_event_decode (midiParser, buffer,
                                                                           maxEventSize, inputEvent);

                                    snd_midi_event_reset_decode (midiParser);

                                    concatenator.pushMidiData (buffer, (int) numBytes,
                                                               Time::getMillisecondCounter() * 0.001,
                                                               inputEvent, client);

                                    snd_seq_free_event (inputEvent);
                                }
                            }
                            while (snd_seq_event_input_pending (seqHandle, 0) > 0);
                        }
                    }

                    snd_midi_event_free (midiParser);
                }
        */
    }
}
