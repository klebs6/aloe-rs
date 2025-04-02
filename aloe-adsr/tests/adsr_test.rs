use aloe_adsr::*;
use aloe_buffers::*;
use aloe_test::*;
use aloe_3p::*;

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_basics/utilities/aloe_ADSR_test.cpp]

pub struct ADSRTests {
    base: UnitTest,
}

impl Default for ADSRTests {
    
    fn default() -> Self {
        todo!();
        /*
        : unit_test("ADSR", UnitTestCategories::audio),

        
        */
    }
}

impl ADSRTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            constexpr double sampleRate = 44100.0;
            const ADSR::Parameters parameters { 0.1f, 0.1f, 0.5f, 0.1f };

            ADSR adsr;
            adsr.setSampleRate (sampleRate);
            adsr.setParameters (parameters);

            beginTest ("Idle");
            {
                adsr.reset();

                expect (! adsr.isActive());
                expectEquals (adsr.getNextSample(), 0.0f);
            }

            beginTest ("Attack");
            {
                adsr.reset();

                adsr.noteOn();
                expect (adsr.isActive());

                auto buffer = getTestBuffer (sampleRate, parameters.attack);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isIncreasing (buffer));
            }

            beginTest ("Decay");
            {
                adsr.reset();

                adsr.noteOn();
                advanceADSR (adsr, roundToInt (parameters.attack * sampleRate));

                auto buffer = getTestBuffer (sampleRate, parameters.decay);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isDecreasing (buffer));
            }

            beginTest ("Sustain");
            {
                adsr.reset();

                adsr.noteOn();
                advanceADSR (adsr, roundToInt ((parameters.attack + parameters.decay + 0.01) * sampleRate));

                auto random = getRandom();

                for (int numTests = 0; numTests < 100; ++numTests)
                {
                    const auto sustainLevel = random.nextFloat();
                    const auto sustainLength = jmax (0.1f, random.nextFloat());

                    adsr.setParameters ({ parameters.attack, parameters.decay, sustainLevel, parameters.release });

                    auto buffer = getTestBuffer (sampleRate, sustainLength);
                    adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                    expect (isSustained (buffer, sustainLevel));
                }
            }

            beginTest ("Release");
            {
                adsr.reset();

                adsr.noteOn();
                advanceADSR (adsr, roundToInt ((parameters.attack + parameters.decay) * sampleRate));
                adsr.noteOff();

                auto buffer = getTestBuffer (sampleRate, parameters.release);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isDecreasing (buffer));
            }

            beginTest ("Zero-length attack jumps to decay");
            {
                adsr.reset();
                adsr.setParameters ({ 0.0f, parameters.decay, parameters.sustain, parameters.release });

                adsr.noteOn();

                auto buffer = getTestBuffer (sampleRate, parameters.decay);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isDecreasing (buffer));
            }

            beginTest ("Zero-length decay jumps to sustain");
            {
                adsr.reset();
                adsr.setParameters ({ parameters.attack, 0.0f, parameters.sustain, parameters.release });

                adsr.noteOn();
                advanceADSR (adsr, roundToInt (parameters.attack * sampleRate));
                adsr.getNextSample();

                expectEquals (adsr.getNextSample(), parameters.sustain);

                auto buffer = getTestBuffer (sampleRate, 1);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isSustained (buffer, parameters.sustain));
            }

            beginTest ("Zero-length attack and decay jumps to sustain");
            {
                adsr.reset();
                adsr.setParameters ({ 0.0f, 0.0f, parameters.sustain, parameters.release });

                adsr.noteOn();

                expectEquals (adsr.getNextSample(), parameters.sustain);

                auto buffer = getTestBuffer (sampleRate, 1);
                adsr.applyEnvelopeToBuffer (buffer, 0, buffer.getNumSamples());

                expect (isSustained (buffer, parameters.sustain));
            }

            beginTest ("Zero-length release resets to idle");
            {
                adsr.reset();
                adsr.setParameters ({ parameters.attack, parameters.decay, parameters.sustain, 0.0f });

                adsr.noteOn();
                advanceADSR (adsr, roundToInt ((parameters.attack + parameters.decay) * sampleRate));
                adsr.noteOff();

                expect (! adsr.isActive());
            }
        */
    }
    
    pub fn advanceadsr(
        adsr:                   &mut ADSR,
        num_samples_to_advance: i32)  {
        
        todo!();
        /*
            while (--numSamplesToAdvance >= 0)
                adsr.getNextSample();
        */
    }
    
    pub fn get_test_buffer(
        sample_rate:       f64,
        length_in_seconds: f32) -> AudioBuffer<f32> {
        
        todo!();
        /*
            AudioBuffer<float> buffer { 2, roundToInt (lengthInSeconds * sampleRate) };

            for (int channel = 0; channel < buffer.getNumChannels(); ++channel)
                for (int sample = 0; sample < buffer.getNumSamples(); ++sample)
                    buffer.setSample (channel, sample, 1.0f);

            return buffer;
        */
    }
    
    pub fn is_increasing(b: &AudioBuffer<f32>) -> bool {
        
        todo!();
        /*
            jassert (b.getNumChannels() > 0 && b.getNumSamples() > 0);

            for (int channel = 0; channel < b.getNumChannels(); ++channel)
            {
                float previousSample = -1.0f;

                for (int sample = 0; sample < b.getNumSamples(); ++sample)
                {
                    const auto currentSample = b.getSample (channel, sample);

                    if (currentSample <= previousSample)
                        return false;

                    previousSample = currentSample;
                }
            }

            return true;
        */
    }
    
    pub fn is_decreasing(b: &AudioBuffer<f32>) -> bool {
        
        todo!();
        /*
            jassert (b.getNumChannels() > 0 && b.getNumSamples() > 0);

            for (int channel = 0; channel < b.getNumChannels(); ++channel)
            {
                float previousSample = std::numeric_limits<float>::max();

                for (int sample = 0; sample < b.getNumSamples(); ++sample)
                {
                    const auto currentSample = b.getSample (channel, sample);

                    if (currentSample >= previousSample)
                        return false;

                    previousSample = currentSample;
                }
            }

            return true;
        */
    }
    
    pub fn is_sustained(
        b:             &AudioBuffer<f32>,
        sustain_level: f32) -> bool {
        
        todo!();
        /*
            jassert (b.getNumChannels() > 0 && b.getNumSamples() > 0);

            for (int channel = 0; channel < b.getNumChannels(); ++channel)
                if (b.findMinMax (channel, 0, b.getNumSamples()) != Range<float> { sustainLevel, sustainLevel })
                    return false;

            return true;
        */
    }
}

lazy_static!{
    /*
    static ADSRTests adsrTests;
    */
}
