#[cfg(feature = "mp3")] //see lib.rs for disclaimer
crate::ix!();

#[no_copy]
#[leak_detector]
#[cfg(feature = "lame")]
pub struct LameEncoderAudioFormatWriter<'a, W: Write> {
    base:        AudioFormatWriter<'a, W>,
    vbr_level:   i32,
    cbr_bitrate: i32,
    temp_wav:    TemporaryFile, // default = { ".wav"  }
    writer:      Box<AudioFormatWriter<'a, W>>,
    args:        StringArray,
}

#[cfg(feature = "lame")]
impl<'a, W: Write> Drop for LameEncoderAudioFormatWriter<'a, W> {
    fn drop(&mut self) {
        todo!();
        /*
            if (writer != nullptr)
            {
                writer = nullptr;

                if (! convertToMP3())
                    convertToMP3(); // try again
            }
        */
    }
}

#[cfg(feature = "lame")]
impl<'a, W: Write> LameEncoderAudioFormatWriter<'a, W> {

    pub fn new(
        dest_stream:        *mut dyn Write,
        format_name:        &String,
        app_file:           &File,
        vbr:                i32,
        cbr:                i32,
        sample_rate_in:     f64,
        number_of_channels: u32,
        bits_per_sample_in: i32,
        metadata:           &StringPairArray) -> Self {
    
        todo!();
        /*


            : AudioFormatWriter (destStream, formatName, sampleRateIn,
                                 numberOfChannels, (unsigned int) bitsPerSampleIn),
              vbrLevel (vbr), cbrBitrate (cbr)

            WavAudioFormat wavFormat;

            if (auto out = tempWav.getFile().createOutputStream())
            {
                writer.reset (wavFormat.createWriterFor (out.release(), sampleRateIn, numChannels,
                                                         bitsPerSampleIn, metadata, 0));

                args.add (appFile.getFullPathName());

                args.add ("--quiet");

                if (cbrBitrate == 0)
                {
                    args.add ("--vbr-new");
                    args.add ("-V");
                    args.add (String (vbrLevel));
                }
                else
                {
                    args.add ("--cbr");
                    args.add ("-b");
                    args.add (String (cbrBitrate));
                }

                addMetadataArg (metadata, "id3title",       "--tt");
                addMetadataArg (metadata, "id3artist",      "--ta");
                addMetadataArg (metadata, "id3album",       "--tl");
                addMetadataArg (metadata, "id3comment",     "--tc");
                addMetadataArg (metadata, "id3date",        "--ty");
                addMetadataArg (metadata, "id3genre",       "--tg");
                addMetadataArg (metadata, "id3trackNumber", "--tn");
            }
        */
    }
    
    pub fn add_metadata_arg(&mut self, 
        metadata:  &StringPairArray,
        key:       *const u8,
        lame_flag: *const u8)  {
        
        todo!();
        /*
            auto value = metadata.getValue (key, {});

            if (value.isNotEmpty())
            {
                args.add (lameFlag);
                args.add (value);
            }
        */
    }
    
    pub fn write(&mut self, 
        samples_to_write: *const *const i32,
        num_samples:      i32) -> bool {
        
        todo!();
        /*
            return writer != nullptr && writer->write (samplesToWrite, numSamples);
        */
    }
    
    pub fn run_lame_child_process(&self, 
        tempmp3:      &TemporaryFile,
        process_args: &StringArray) -> bool {
        
        todo!();
        /*
            ChildProcess cp;

            if (cp.start (processArgs))
            {
                auto childOutput = cp.readAllProcessOutput();
                DBG (childOutput); ignoreUnused (childOutput);

                cp.waitForProcessToFinish (10000);
                return tempMP3.getFile().getSize() > 0;
            }

            return false;
        */
    }
    
    pub fn convert_tomp3(&self) -> bool {
        
        todo!();
        /*
            TemporaryFile tempMP3 (".mp3");

            StringArray args2 (args);
            args2.add (tempWav.getFile().getFullPathName());
            args2.add (tempMP3.getFile().getFullPathName());

            DBG (args2.joinIntoString (" "));

            if (runLameChildProcess (tempMP3, args2))
            {
                FileInputStream fis (tempMP3.getFile());

                if (fis.openedOk() && output->writeFromInputStream (fis, -1) > 0)
                {
                    output->flush();
                    return true;
                }
            }

            return false;
        */
    }
}
