crate::ix!();

pub fn get_device_sample_rates(
        handle: *mut SndPcm,
        rates:  &mut Vec<f64>)  {
    
    todo!();
    /*
        const int ratesToTry[] = { 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000, 0 };

        snd_pcm_hw_params_t* hwParams;
        snd_pcm_hw_params_alloca (&hwParams);

        for (int i = 0; ratesToTry[i] != 0; ++i)
        {
            if (snd_pcm_hw_params_any (handle, hwParams) >= 0
                 && snd_pcm_hw_params_test_rate (handle, hwParams, (unsigned int) ratesToTry[i], 0) == 0)
            {
                rates.addIfNotAlreadyThere ((double) ratesToTry[i]);
            }
        }
    */
}

pub fn get_device_num_channels(
        handle:    *mut SndPcm,
        min_chans: *mut u32,
        max_chans: *mut u32)  {
    
    todo!();
    /*
        snd_pcm_hw_params_t *params;
        snd_pcm_hw_params_alloca (&params);

        if (snd_pcm_hw_params_any (handle, params) >= 0)
        {
            snd_pcm_hw_params_get_channels_min (params, minChans);
            snd_pcm_hw_params_get_channels_max (params, maxChans);

            ALOE_ALSA_LOG ("getDeviceNumChannels: " << (int) *minChans << " " << (int) *maxChans);

            // some virtual devices (dmix for example) report 10000 channels , we have to clamp these values
            *maxChans = jmin (*maxChans, 256u);
            *minChans = jmin (*minChans, *maxChans);
        }
        else
        {
            ALOE_ALSA_LOG ("getDeviceNumChannels failed");
        }
    */
}

pub fn get_device_properties(
        deviceid:      &String,
        min_chans_out: &mut u32,
        max_chans_out: &mut u32,
        min_chans_in:  &mut u32,
        max_chans_in:  &mut u32,
        rates:         &mut Vec<f64>,
        test_output:   bool,
        test_input:    bool)  {
    
    todo!();
    /*
        minChansOut = maxChansOut = minChansIn = maxChansIn = 0;

        if (deviceID.isEmpty())
            return;

        ALOE_ALSA_LOG ("getDeviceProperties(" << deviceID.toUTF8().getAddress() << ")");

        snd_pcm_info_t* info;
        snd_pcm_info_alloca (&info);

        if (testOutput)
        {
            snd_pcm_t* pcmHandle;

            if (ALOE_CHECKED_RESULT (snd_pcm_open (&pcmHandle, deviceID.toUTF8().getAddress(), SND_PCM_STREAM_PLAYBACK, SND_PCM_NONBLOCK)) >= 0)
            {
                getDeviceNumChannels (pcmHandle, &minChansOut, &maxChansOut);
                getDeviceSampleRates (pcmHandle, rates);

                snd_pcm_close (pcmHandle);
            }
        }

        if (testInput)
        {
            snd_pcm_t* pcmHandle;

            if (ALOE_CHECKED_RESULT (snd_pcm_open (&pcmHandle, deviceID.toUTF8(), SND_PCM_STREAM_CAPTURE, SND_PCM_NONBLOCK) >= 0))
            {
                getDeviceNumChannels (pcmHandle, &minChansIn, &maxChansIn);

                if (rates.size() == 0)
                    getDeviceSampleRates (pcmHandle, rates);

                snd_pcm_close (pcmHandle);
            }
        }
    */
}

