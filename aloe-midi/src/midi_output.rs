crate::ix!();

/**
  | Represents a midi output device.
  | 
  | To create one of these, use the static
  | getAvailableDevices() method to find
  | out what outputs are available, and
  | then use the openDevice() method to
  | try to open one.
  | 
  | @see MidiInput
  | 
  | @tags{Audio}
  |
  */
#[no_copy]
#[leak_detector]
pub struct MidiOutput {
    base: Thread,
    device_info:   MidiDeviceInfo,
    internal:      Box<MidiOutputPimpl>,
    lock:          CriticalSection,
    first_message: *mut MidiOutputPendingMessage, // default = nullptr
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/native/aloe_android_Midi.cpp]

#[cfg(target_os="android")]
impl Drop for MidiOutput {

    fn drop(&mut self) {
        todo!();
        /* 
        stopBackgroundThread();
 */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_devices/midi_io/aloe_MidiDevices.cpp]
impl MidiOutput {

    #[cfg(target_os="android")]
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        AndroidMidiDeviceManager manager;
        return manager.getDevices (false);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        return getAvailableDevices().getFirst();
        */
    }
    
    #[cfg(target_os="android")]
    pub fn open_device(&mut self, device_identifier: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23 || deviceIdentifier.isEmpty())
            return {};

        AndroidMidiDeviceManager manager;

        if (auto* port = manager.openMidiOutputPortWithID (deviceIdentifier.getIntValue()))
        {
            std::unique_ptr<MidiOutput> midiOutput (new MidiOutput ({}, deviceIdentifier));
            midiOutput->internal.reset (port);
            midiOutput->setName (port->getName());

            return midiOutput;
        }

        return {};
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_devices(&mut self) -> StringArray {
        
        todo!();
        /*
            if (getAndroidSDKVersion() < 23)
            return {};

        StringArray deviceNames;

        for (auto& d : getAvailableDevices())
            deviceNames.add (d.name);

        return deviceNames;
        */
    }
    
    #[cfg(target_os="android")]
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return (getAndroidSDKVersion() < 23 ? -1 : 0);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn open_device(&mut self, index: i32) -> Box<MidiOutput> {
        
        todo!();
        /*
            return openDevice (getAvailableDevices()[index].identifier);
        */
    }
    
    #[cfg(target_os="android")]
    pub fn send_message_now(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
            if (auto* androidMidi = internal.get())
        {
            auto* env = getEnv();
            auto messageSize = message.getRawDataSize();

            LocalRef<jbyteArray> messageContent (env->NewByteArray (messageSize));
            auto content = messageContent.get();

            auto* rawBytes = env->GetByteArrayElements (content, nullptr);
            std::memcpy (rawBytes, message.getRawData(), static_cast<size_t> (messageSize));
            env->ReleaseByteArrayElements (content, rawBytes, 0);

            androidMidi->send (content, (jint) 0, (jint) messageSize);
        }
        */
    }

    /**
      | Returns a list of the available midi
      | output devices.
      | 
      | You can open one of the devices by passing
      | its identifier into the openDevice()
      | method.
      | 
      | @see MidiDeviceInfo, getDevices,
      | getDefaultDeviceIndex, openDevice
      |
      */
    pub fn get_available_devices() -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns the MidiDeviceInfo of the default
      | midi output device to use.
      |
      */
    pub fn get_default_device() -> MidiDeviceInfo {
        
        todo!();
        /*
        
        */
    }

    /**
      | Tries to open one of the midi output devices.
      | 
      | This will return a MidiOutput object
      | if it manages to open it, you can then
      | send messages to this device.
      | 
      | If the device can't be opened, this will
      | return an empty object.
      | 
      | -----------
      | @param deviceIdentifier
      | 
      | the ID of the device to open - use the getAvailableDevices()
      | method to find the available devices
      | that can be opened @see getDevices
      |
      */
    pub fn open_device(device_identifier: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
        
        */
    }

    /**
      | This will try to create a new midi output
      | device (only available on Linux, macOS
      | and iOS).
      | 
      | This will attempt to create a new midi
      | output device with the specified name
      | that other apps can connect to and use
      | as their midi input.
      | 
      | NB - if you are calling this method on
      | iOS you must have enabled the "Audio
      | Background Capability" setting in
      | the iOS exporter otherwise this method
      | will fail.
      | 
      | Returns an empty object if a device can't
      | be created.
      | 
      | -----------
      | @param deviceName
      | 
      | the name of the device to create
      |
      */
    #[cfg(any(target_os="linux",target_os="bsd",target_os="macos",target_os="ios"))]
    pub fn create_new_device(device_name: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns the MidiDeviceInfo struct
      | containing some information about
      | this device.
      |
      */
    pub fn get_device_info(&self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return deviceInfo;
        */
    }

    /**
      | Returns the identifier of this device.
      |
      */
    pub fn get_identifier(&self) -> String {
        
        todo!();
        /*
            return deviceInfo.identifier;
        */
    }

    /**
      | Returns the name of this device.
      |
      */
    pub fn get_name(&self) -> String {
        
        todo!();
        /*
            return deviceInfo.name;
        */
    }

    /**
      | Sets a custom name for the device.
      |
      */
    pub fn set_name(&mut self, new_name: &String)  {
        
        todo!();
        /*
            deviceInfo.name = newName;
        */
    }

    /**
      | Sends out a MIDI message immediately.
      |
      */
    pub fn send_message_now(&mut self, message: &MidiMessage)  {
        
        todo!();
        /*
        
        */
    }

    /**
      | Returns true if the background thread
      | used to send blocks of data is running.
      | @see startBackgroundThread, stopBackgroundThread
      |
      */
    pub fn is_background_thread_running(&self) -> bool {
        
        todo!();
        /*
            return isThreadRunning();
        */
    }
    
    pub fn new(
        device_name:       &String,
        device_identifier: &String) -> Self {
    
        todo!();
        /*
        : thread("midi out"),
        : device_info(deviceName, deviceIdentifier),

        
        */
    }
    
    /**
      | Sends out a sequence of MIDI messages
      | immediately.
      |
      */
    pub fn send_block_of_messages_now(&mut self, buffer: &MidiBuffer)  {
        
        todo!();
        /*
            for (const auto metadata : buffer)
            sendMessageNow (metadata.getMessage());
        */
    }
    
    /**
      | This lets you supply a block of messages
      | that will be sent out at some point in
      | the future.
      | 
      | The MidiOutput class has an internal
      | thread that can send out timestamped
      | messages - this appends a set of messages
      | to its internal buffer, ready for sending.
      | 
      | This will only work if you've already
      | started the thread with startBackgroundThread().
      | 
      | A time is specified, at which the block
      | of messages should be sent. This time
      | uses the same time base as Time::getMillisecondCounter(),
      | and must be in the future.
      | 
      | The samplesPerSecondForBuffer parameter
      | indicates the number of samples per
      | second used by the MidiBuffer. Each
      | event in a MidiBuffer has a sample position,
      | and the samplesPerSecondForBuffer
      | value is needed to convert this sample
      | position to a real time.
      |
      */
    pub fn send_block_of_messages(&mut self, 
        buffer:                          &MidiBuffer,
        millisecond_counter_to_start_at: f64,
        samples_per_second_for_buffer:   f64)  {
        
        todo!();
        /*
            // You've got to call startBackgroundThread() for this to actually work..
        jassert (isThreadRunning());

        // this needs to be a value in the future - RTFM for this method!
        jassert (millisecondCounterToStartAt > 0);

        auto timeScaleFactor = 1000.0 / samplesPerSecondForBuffer;

        for (const auto metadata : buffer)
        {
            auto eventTime = millisecondCounterToStartAt + timeScaleFactor * metadata.samplePosition;
            auto* m = new MidiOutputPendingMessage (metadata.data, metadata.numBytes, eventTime);

            const ScopedLock sl (lock);

            if (firstMessage == nullptr || firstMessage->message.getTimeStamp() > eventTime)
            {
                m->next = firstMessage;
                firstMessage = m;
            }
            else
            {
                auto* mm = firstMessage;

                while (mm->next != nullptr && mm->next->message.getTimeStamp() <= eventTime)
                    mm = mm->next;

                m->next = mm->next;
                mm->next = m;
            }
        }

        notify();
        */
    }
    
    /**
      | Gets rid of any midi messages that had
      | been added by sendBlockOfMessages().
      |
      */
    pub fn clear_all_pending_messages(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        while (firstMessage != nullptr)
        {
            auto* m = firstMessage;
            firstMessage = firstMessage->next;
            delete m;
        }
        */
    }
    
    /**
      | Starts up a background thread so that
      | the device can send blocks of data. Call
      | this to get the device ready, before
      | using sendBlockOfMessages().
      |
      */
    pub fn start_background_thread(&mut self)  {
        
        todo!();
        /*
            startThread (9);
        */
    }
    
    /**
      | Stops the background thread, and clears
      | any pending midi events. @see startBackgroundThread
      |
      */
    pub fn stop_background_thread(&mut self)  {
        
        todo!();
        /*
            stopThread (5000);
        */
    }
    
    pub fn run(&mut self)  {
        
        todo!();
        /*
            while (! threadShouldExit())
        {
            auto now = Time::getMillisecondCounter();
            uint32 eventTime = 0;
            uint32 timeToWait = 500;

            MidiOutputPendingMessage* message;

            {
                const ScopedLock sl (lock);
                message = firstMessage;

                if (message != nullptr)
                {
                    eventTime = (uint32) roundToInt (message->message.getTimeStamp());

                    if (eventTime > now + 20)
                    {
                        timeToWait = eventTime - (now + 20);
                        message = nullptr;
                    }
                    else
                    {
                        firstMessage = message->next;
                    }
                }
            }

            if (message != nullptr)
            {
                std::unique_ptr<MidiOutputPendingMessage> messageDeleter (message);

                if (eventTime > now)
                {
                    Time::waitForMillisecondCounter (eventTime);

                    if (threadShouldExit())
                        break;
                }

                if (eventTime > now - 200)
                    sendMessageNow (message->message);
            }
            else
            {
                jassert (timeToWait < 1000 * 30);
                wait ((int) timeToWait);
            }
        }

        clearAllPendingMessages();
        */
    }

    #[cfg(feature = "bela")]
    pub fn send_message_now(&mut self, _0: &MidiMessage)  {
        
        todo!();
        /*
        
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn get_available_devices(&mut self) -> Vec<MidiDeviceInfo> {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn get_default_device(&mut self) -> MidiDeviceInfo {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn open_device_with_string(&mut self, _0: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn create_new_device(&mut self, _0: &String) -> Box<MidiOutput> {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn get_devices(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return {};
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn get_default_device_index(&mut self) -> i32 {
        
        todo!();
        /*
            return 0;
        */
    }
    
    #[cfg(feature = "bela")]
    pub fn open_device(&mut self, _0: i32) -> Box<MidiOutput> {
        
        todo!();
        /*
            return {};
        */
    }
}
