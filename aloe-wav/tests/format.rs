crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
#[no_copy]
pub struct WaveAudioFormatTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for WaveAudioFormatTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Wave audio format tests", UnitTestCategories::audio)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod wave_audio_format_tests {

    use super::*;

    pub const numTestAudioBufferChannels: usize = 2;
    pub const numTestAudioBufferSamples:  usize = 256;
}

#[cfg(ALOE_UNIT_TESTS)]
impl WaveAudioFormatTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Setting up metadata");

            StringPairArray metadataValues = WavAudioFormat::createBWAVMetadata ("description",
                                                                                 "originator",
                                                                                 "originatorRef",
                                                                                 Time::getCurrentTime(),
                                                                                 numTestAudioBufferSamples,
                                                                                 "codingHistory");

            for (int i = numElementsInArray (WavFileHelpers::ListInfoChunk::types); --i >= 0;)
                metadataValues.set (WavFileHelpers::ListInfoChunk::types[i],
                                    WavFileHelpers::ListInfoChunk::types[i]);

            if (metadataValues.size() > 0)
                metadataValues.set ("MetaDataSource", "WAV");

            metadataValues.addArray (createDefaultSMPLMetadata());

            WavAudioFormat format;
            MemoryBlock memoryBlock;

            {
                beginTest ("Creating a basic wave writer");

                std::unique_ptr<AudioFormatWriter> writer (format.createWriterFor (new MemoryOutputStream (memoryBlock, false),
                                                                                   44100.0, numTestAudioBufferChannels,
                                                                                   32, metadataValues, 0));
                expect (writer != nullptr);

                AudioBuffer<float> buffer (numTestAudioBufferChannels, numTestAudioBufferSamples);
                buffer.clear();

                beginTest ("Writing audio data to the basic wave writer");
                expect (writer->writeFromAudioSampleBuffer (buffer, 0, numTestAudioBufferSamples));
            }

            {
                beginTest ("Creating a basic wave reader");

                std::unique_ptr<AudioFormatReader> reader (format.createReaderFor (new MemoryInputStream (memoryBlock, false), false));
                expect (reader != nullptr);
                expect (reader->metadataValues == metadataValues, "Somehow, the metadata is different!");
            }
        */
    }
    
    pub fn create_default_smpl_metadata(&self) -> StringPairArray {
        
        todo!();
        /*
            StringPairArray m;

            m.set ("Manufacturer", "0");
            m.set ("Product", "0");
            m.set ("SamplePeriod", "0");
            m.set ("MidiUnityNote", "60");
            m.set ("MidiPitchFraction", "0");
            m.set ("SmpteFormat", "0");
            m.set ("SmpteOffset", "0");
            m.set ("NumSampleLoops", "0");
            m.set ("SamplerData", "0");

            return m;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static const WaveAudioFormatTests waveAudioFormatTests;
    */
}
