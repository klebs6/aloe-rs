crate::ix!();

#[no_copy]
#[leak_detector]
pub struct MidiInputPimpl {
    buffer:        Vec<u8>,
    midi_input:    *const MidiInput,
    midi_port:     String,
    midi_callback: *const dyn MidiInputCallback,

    #[cfg(feature = "todo---where-is-this")]
    midi:          Midi,

    concatenator:  MidiDataConcatenator, // default = 512 
}

#[cfg(feature = "bela")]
pub mod midi_input_pimpl {
    use super::*;

    lazy_static!{
        /*
        static Vec<MidiInputPimpl*> midiInputs;
        Vec<MidiInput::MidiInputPimpl*> MidiInput::MidiInputPimpl::midiInputs;
        */
    }
}

#[cfg(feature = "bela")]
impl Drop for MidiInputPimpl {
    fn drop(&mut self) {
        todo!();
        /* 
            stop();
            midiInputs.removeAllInstancesOf (this);
         */
    }
}

#[cfg(feature = "bela")]
impl MidiInputPimpl {

    pub fn new(
        port:     &String,
        input:    *mut MidiInput,
        callback: *mut dyn MidiInputCallback) -> Self {
    
        todo!();
        /*


            : midiInput (input), midiPort (port), midiCallback (callback)

            jassert (midiCallback != nullptr);
            midiInputs.add (this);

            buffer.resize (32);
        */
    }
    
    pub fn start(&mut self)  {
        
        todo!();
        /*
            midi.readFrom (midiPort.toRawUTF8());
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            midi.enableParser (false);
        */
    }
    
    pub fn poll(&mut self)  {
        
        todo!();
        /*
            size_t receivedBytes = 0;

            for (;;)
            {
                auto data = midi.getInput();

                if (data < 0)
                    break;

                buffer[receivedBytes] = (uint8) data;
                receivedBytes++;

                if (receivedBytes == buffer.size())
                {
                    pushMidiData (static_cast<int> (receivedBytes));
                    receivedBytes = 0;
                }
            }

            if (receivedBytes > 0)
                pushMidiData ((int) receivedBytes);
        */
    }
    
    pub fn get_devices(input: bool) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            Vec<MidiDeviceInfo> devices;

            for (auto& card : findAllALSACardIDs())
                findMidiDevices (devices, input, card);

            return devices;
        */
    }
    
    pub fn push_midi_message(&mut self, message: &mut MidiMessage)  {
        
        todo!();
        /*
            concatenator.pushMidiData (message.getRawData(), message.getRawDataSize(), Time::getMillisecondCounter() * 0.001, midiInput, *midiCallback);
        */
    }
    
    pub fn push_midi_data(&mut self, length: i32)  {
        
        todo!();
        /*
            concatenator.pushMidiData (buffer.data(), length, Time::getMillisecondCounter() * 0.001, midiInput, *midiCallback);
        */
    }
    
    pub fn find_all_alsa_cardi_ds() -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> cards;
            int card = -1;

            for (;;)
            {
                auto status = snd_card_next (&card);

                if (status != 0 || card < 0)
                    break;

                cards.add (card);
            }

            return cards;
        */
    }

    /**
       Adds all midi devices to the devices array
       of the given input/output type on the given
       card
      */
    pub fn find_midi_devices(
        devices:  &mut Vec<MidiDeviceInfo>,
        input:    bool,
        card_num: i32)  {
        
        todo!();
        /*
            snd_ctl_t* ctl = nullptr;
            auto status = snd_ctl_open (&ctl, ("hw:" + String (cardNum)).toRawUTF8(), 0);

            if (status < 0)
                return;

            int device = -1;

            for (;;)
            {
                status = snd_ctl_rawmidi_next_device (ctl, &device);

                if (status < 0 || device < 0)
                    break;

                snd_rawmidi_info_t* info;
                snd_rawmidi_info_alloca (&info);

                snd_rawmidi_info_set_device (info, (unsigned int) device);
                snd_rawmidi_info_set_stream (info, input ? SND_RAWMIDI_STREAM_INPUT
                                                         : SND_RAWMIDI_STREAM_OUTPUT);

                snd_ctl_rawmidi_info (ctl, info);

                auto subCount = snd_rawmidi_info_get_subdevices_count (info);

                for (size_t sub = 0; sub < subCount; ++sub)
                {
                    snd_rawmidi_info_set_subdevice (info, sub);

                    status = snd_ctl_rawmidi_info (ctl, info);

                    if (status == 0)
                    {
                        String deviceName ("hw:" + String (cardNum) + "," + String (device) + "," + String (sub));
                        devices.add (MidiDeviceInfo (deviceName, deviceName));
                    }
                }
            }

            snd_ctl_close (ctl);
        */
    }
}
