crate::ix!();

pub type JackClient  = jack_sys::jack_client_t;
pub type JackPort    = jack_sys::jack_port_t;
pub type JackPortId  = jack_sys::jack_port_id_t;
pub type JackNframes = jack_sys::jack_nframes_t;
pub type JackStatus  = jack_sys::jack_status_t;

///-------------------------
pub struct JackAudioIODevice<'a> {
    base:                            AudioIODevice,
    input_name:                      String,
    output_name:                     String,
    device_is_open:                  bool, // default = false
    client:                          *mut JackClient, // default = nullptr
    last_error:                      String,
    callback:                        *mut dyn AudioIODeviceCallback, // default = nullptr
    callback_lock:                   CriticalSection,
    in_chans:                        HeapBlock<*mut f32>,
    out_chans:                       HeapBlock<*mut f32>,
    total_number_of_input_channels:  i32, // default = 0
    total_number_of_output_channels: i32, // default = 0
    input_ports:                     Vec<*mut JackPort>,
    output_ports:                    Vec<*mut JackPort>,
    active_input_channels:           BigInteger,
    active_output_channels:          BigInteger,
    xruns:                           Atomic<i32>, // default = 0 
    notify_channels_changed:         fn() -> (),
    main_thread_dispatcher:          JackAudioIODeviceMainThreadDispatcher<'a>, // { *this };
}

impl<'a> Drop for JackAudioIODevice<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            close();
            if (client != nullptr)
            {
                jack_client_close (client);
                client = nullptr;
            }
         */
    }
}

impl<'a> JackAudioIODevice<'a> {

    pub fn new(
        in_name:   &String,
        out_name:  &String,
        notify_in: fn() -> ()) -> Self {
    
        todo!();
        /*


            : AudioIODevice (outName.isEmpty() ? inName : outName, "JACK"),
              inputName (inName),
              outputName (outName),
              notifyChannelsChanged (std::move (notifyIn))

            jassert (outName.isNotEmpty() || inName.isNotEmpty());

            jack_status_t status = {};
            client = jack_client_open (ALOE_JACK_CLIENT_NAME, JackNoStartServer, &status);

            if (client == nullptr)
            {
                ALOE_JACK_LOG_STATUS (status);
            }
            else
            {
                jack_set_error_function (errorCallback);

                // open input ports
                const Vec<String> inputChannels (getInputChannelNames());
                for (int i = 0; i < inputChannels.size(); ++i)
                {
                    String inputChannelName;
                    inputChannelName << "in_" << ++totalNumberOfInputChannels;

                    inputPorts.add (jack_port_register (client, inputChannelName.toUTF8(),
                                                              JACK_DEFAULT_AUDIO_TYPE, JackPortIsInput, 0));
                }

                // open output ports
                const Vec<String> outputChannels (getOutputChannelNames());
                for (int i = 0; i < outputChannels.size(); ++i)
                {
                    String outputChannelName;
                    outputChannelName << "out_" << ++totalNumberOfOutputChannels;

                    outputPorts.add (jack_port_register (client, outputChannelName.toUTF8(),
                                                               JACK_DEFAULT_AUDIO_TYPE, JackPortIsOutput, 0));
                }

                inChans.calloc (totalNumberOfInputChannels + 2);
                outChans.calloc (totalNumberOfOutputChannels + 2);
            }
        */
    }
    
    pub fn get_channel_names(&self, 
        client_name: &String,
        for_input:   bool) -> Vec<String> {
        
        todo!();
        /*
            Vec<String> names;

            for (JackPortIterator i (client, forInput); i.next();)
                if (i.getClientName() == clientName)
                    names.add (i.getChannelName());

            return names;
        */
    }
    
    pub fn get_output_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return getChannelNames (outputName, true);
        */
    }
    
    pub fn get_input_channel_names(&mut self) -> Vec<String> {
        
        todo!();
        /*
            return getChannelNames (inputName, false);
        */
    }
    
    pub fn get_available_sample_rates(&mut self) -> Vec<f64> {
        
        todo!();
        /*
            Vec<double> rates;

            if (client != nullptr)
                rates.add (jack_get_sample_rate (client));

            return rates;
        */
    }
    
    pub fn get_available_buffer_sizes(&mut self) -> Vec<i32> {
        
        todo!();
        /*
            Vec<int> sizes;

            if (client != nullptr)
                sizes.add (static_cast<int> (jack_get_buffer_size (client)));

            return sizes;
        */
    }
    
    pub fn get_default_buffer_size(&mut self) -> i32 {
        
        todo!();
        /*
            return getCurrentBufferSizeSamples();
        */
    }
    
    pub fn get_current_buffer_size_samples(&mut self) -> i32 {
        
        todo!();
        /*
            return client != nullptr ? static_cast<int> (jack_get_buffer_size (client)) : 0;
        */
    }
    
    pub fn get_current_sample_rate(&mut self) -> f64 {
        
        todo!();
        /*
            return client != nullptr ? static_cast<int> (jack_get_sample_rate (client)) : 0;
        */
    }
    
    pub fn for_each_client_channel<Fn>(&mut self, 
        client_name: &String,
        is_input:    bool,
        fn_:         Fn)  {
    
        todo!();
        /*
            auto index = 0;

            for (JackPortIterator i (client, isInput); i.next();)
            {
                if (i.getClientName() != clientName)
                    continue;

                fn (i.ports.get()[i.index], index);
                index += 1;
            }
        */
    }
    
    pub fn open(&mut self, 
        input_channels:      &BigInteger,
        output_channels:     &BigInteger,
        sample_rate:         f64,
        buffer_size_samples: i32) -> String {
        
        todo!();
        /*
            if (client == nullptr)
            {
                lastError = "No JACK client running";
                return lastError;
            }

            lastError.clear();
            close();

            xruns.store (0, std::memory_order_relaxed);
            jack_set_process_callback (client, processCallback, this);
            jack_set_port_connect_callback (client, portConnectCallback, this);
            jack_on_shutdown (client, shutdownCallback, this);
            jack_on_info_shutdown (client, infoShutdownCallback, this);
            jack_set_xrun_callback (client, xrunCallback, this);
            jack_activate (client);
            deviceIsOpen = true;

            if (! inputChannels.isZero())
            {
                forEachClientChannel (inputName, false, [&] (const char* portName, int index)
                {
                    if (! inputChannels[index])
                        return;

                    jassert (index < inputPorts.size());

                    const auto* source = portName;
                    const auto* inputPort = inputPorts[index];

                    jassert (jack_port_flags (jack_port_by_name (client, source)) & JackPortIsOutput);
                    jassert (jack_port_flags (inputPort) & JackPortIsInput);

                    auto error = jack_connect (client, source, jack_port_name (inputPort));
                    if (error != 0)
                        ALOE_JACK_LOG ("Cannot connect input port " + String (index) + " (" + portName + "), error " + String (error));
                });
            }

            if (! outputChannels.isZero())
            {
                forEachClientChannel (outputName, true, [&] (const char* portName, int index)
                {
                    if (! outputChannels[index])
                        return;

                    jassert (index < outputPorts.size());

                    const auto* outputPort = outputPorts[index];
                    const auto* destination = portName;

                    jassert (jack_port_flags (outputPort) & JackPortIsOutput);
                    jassert (jack_port_flags (jack_port_by_name (client, destination)) & JackPortIsInput);

                    auto error = jack_connect (client, jack_port_name (outputPort), destination);
                    if (error != 0)
                        ALOE_JACK_LOG ("Cannot connect output port " + String (index) + " (" + portName + "), error " + String (error));
                });
            }

            updateActivePorts();

            return lastError;
        */
    }
    
    pub fn close(&mut self)  {
        
        todo!();
        /*
            stop();

            if (client != nullptr)
            {
                const auto result = jack_deactivate (client);
                jassertquiet (result == 0);

                jack_set_xrun_callback (client, xrunCallback, nullptr);
                jack_set_process_callback (client, processCallback, nullptr);
                jack_set_port_connect_callback (client, portConnectCallback, nullptr);
                jack_on_shutdown (client, shutdownCallback, nullptr);
                jack_on_info_shutdown (client, infoShutdownCallback, nullptr);
            }

            deviceIsOpen = false;
        */
    }
    
    pub fn start(&mut self, new_callback: *mut dyn AudioIODeviceCallback)  {
        
        todo!();
        /*
            if (deviceIsOpen && newCallback != callback)
            {
                if (newCallback != nullptr)
                    newCallback->audioDeviceAboutToStart (this);

                AudioIODeviceCallback* const oldCallback = callback;

                {
                    const ScopedLock sl (callbackLock);
                    callback = newCallback;
                }

                if (oldCallback != nullptr)
                    oldCallback->audioDeviceStopped();
            }
        */
    }
    
    pub fn stop(&mut self)  {
        
        todo!();
        /*
            start (nullptr);
        */
    }
    
    pub fn is_open(&mut self) -> bool {
        
        todo!();
        /*
            return deviceIsOpen;
        */
    }
    
    pub fn is_playing(&mut self) -> bool {
        
        todo!();
        /*
            return callback != nullptr;
        */
    }
    
    pub fn get_current_bit_depth(&mut self) -> i32 {
        
        todo!();
        /*
            return 32;
        */
    }
    
    pub fn get_last_error(&mut self) -> String {
        
        todo!();
        /*
            return lastError;
        */
    }
    
    pub fn getx_run_count(&self) -> i32 {
        
        todo!();
        /*
            return xruns.load (std::memory_order_relaxed);
        */
    }
    
    pub fn get_active_output_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeOutputChannels;
        */
    }
    
    pub fn get_active_input_channels(&self) -> BigInteger {
        
        todo!();
        /*
            return activeInputChannels;
        */
    }
    
    pub fn get_output_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            int latency = 0;

            for (int i = 0; i < outputPorts.size(); i++)
                latency = jmax (latency, (int) jack_port_get_total_latency (client, outputPorts[i]));

            return latency;
        */
    }
    
    pub fn get_input_latency_in_samples(&mut self) -> i32 {
        
        todo!();
        /*
            int latency = 0;

            for (int i = 0; i < inputPorts.size(); i++)
                latency = jmax (latency, (int) jack_port_get_total_latency (client, inputPorts[i]));

            return latency;
        */
    }
    
    pub fn process(&mut self, num_samples: i32)  {
        
        todo!();
        /*
            int numActiveInChans = 0, numActiveOutChans = 0;

            for (int i = 0; i < totalNumberOfInputChannels; ++i)
            {
                if (activeInputChannels[i])
                    if (auto* in = (jack_default_audio_sample_t*) jack_port_get_buffer (inputPorts.getUnchecked (i),
                                                                                              static_cast<jack_nframes_t> (numSamples)))
                        inChans[numActiveInChans++] = (float*) in;
            }

            for (int i = 0; i < totalNumberOfOutputChannels; ++i)
            {
                if (activeOutputChannels[i])
                    if (auto* out = (jack_default_audio_sample_t*) jack_port_get_buffer (outputPorts.getUnchecked (i),
                                                                                               static_cast<jack_nframes_t> (numSamples)))
                        outChans[numActiveOutChans++] = (float*) out;
            }

            const ScopedLock sl (callbackLock);

            if (callback != nullptr)
            {
                if ((numActiveInChans + numActiveOutChans) > 0)
                    callback->audioDeviceIOCallback (const_cast<const float**> (inChans.getData()), numActiveInChans,
                                                     outChans, numActiveOutChans, numSamples);
            }
            else
            {
                for (int i = 0; i < numActiveOutChans; ++i)
                    zeromem (outChans[i], static_cast<size_t> (numSamples) * sizeof (float));
            }
        */
    }
    
    pub fn process_callback(
        nframes:           JackNframes,
        callback_argument: *mut c_void) -> i32 {
        
        todo!();
        /*
            if (callbackArgument != nullptr)
                ((JackAudioIODevice*) callbackArgument)->process (static_cast<int> (nframes));

            return 0;
        */
    }
    
    pub fn xrun_callback(callback_argument: *mut c_void) -> i32 {
        
        todo!();
        /*
            if (callbackArgument != nullptr)
                ((JackAudioIODevice*) callbackArgument)->xruns++;

            return 0;
        */
    }
    
    pub fn update_active_ports(&mut self)  {
        
        todo!();
        /*
            BigInteger newOutputChannels, newInputChannels;

            for (int i = 0; i < outputPorts.size(); ++i)
                if (jack_port_connected (outputPorts.getUnchecked (i)))
                    newOutputChannels.setBit (i);

            for (int i = 0; i < inputPorts.size(); ++i)
                if (jack_port_connected (inputPorts.getUnchecked (i)))
                    newInputChannels.setBit (i);

            if (newOutputChannels != activeOutputChannels
                 || newInputChannels != activeInputChannels)
            {
                AudioIODeviceCallback* const oldCallback = callback;

                stop();

                activeOutputChannels = newOutputChannels;
                activeInputChannels  = newInputChannels;

                if (oldCallback != nullptr)
                    start (oldCallback);

                if (notifyChannelsChanged != nullptr)
                    notifyChannelsChanged();
            }
        */
    }
    
    pub fn port_connect_callback(
        _0:  JackPortId,
        _1:  JackPortId,
        _2:  i32,
        arg: *mut c_void)  {
        
        todo!();
        /*
            if (JackAudioIODevice* device = static_cast<JackAudioIODevice*> (arg))
                device->mainThreadDispatcher.updateActivePorts();
        */
    }
    
    pub fn thread_init_callback(callback_argument: *mut c_void)  {
        
        todo!();
        /*
            ALOE_JACK_LOG ("JackAudioIODevice::initialise");
        */
    }
    
    pub fn shutdown_callback(callback_argument: *mut c_void)  {
        
        todo!();
        /*
            ALOE_JACK_LOG ("JackAudioIODevice::shutdown");

            if (JackAudioIODevice* device = (JackAudioIODevice*) callbackArgument)
            {
                device->client = nullptr;
                device->close();
            }
        */
    }
    
    pub fn info_shutdown_callback(
        code:   JackStatus,
        reason: *const u8,
        arg:    *mut c_void)  {
        
        todo!();
        /*
            jassertquiet (code == 0);

            ALOE_JACK_LOG ("Shutting down with message:");
            ALOE_JACK_LOG (reason);
            ignoreUnused (reason);

            shutdownCallback (arg);
        */
    }
    
    pub fn error_callback(msg: *const u8)  {
        
        todo!();
        /*
            ALOE_JACK_LOG ("JackAudioIODevice::errorCallback " + String (msg));
            ignoreUnused (msg);
        */
    }
}

