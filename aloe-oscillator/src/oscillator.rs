crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_dsp/widgets/aloe_Oscillator.h]

pub type OscillatorGenerator<T> = fn(_0: T) -> T;

/**
  | Generates a signal based on a user-supplied
  | function.
  | 
  | @tags{DSP}
  |
  | Default Creates an uninitialised oscillator. 
  | Call initialise before first use.
  */
pub struct Oscillator<SampleType: FloatSample> {
    generator:    OscillatorGenerator<NumericType<Self>>,
    lookup_table: Box<LookupTableTransform<NumericType<Self>>>,
    ramp_buffer:  Vec<NumericType<Self>>,
    frequency:    SmoothedValue<NumericType<Self>>, // { static_cast<NumericType> (440.0) };
    sample_rate:  NumericType<Self>,                // default = 48000.0
    phase:        Phase<NumericType<Self>>,
}

impl<SampleType: FloatSample> Default for Oscillator<SampleType> {
    fn default() -> Self {
        todo!();
    }
}

impl<SampleType: FloatSample> HasElementType for Oscillator<SampleType> {
    type Type = SampleType;
}

impl<SampleType: FloatSample> Oscillator<SampleType> {

    /**
      | Creates an oscillator with a periodic
      | input function (-pi..pi).
      | 
      | If lookup table is not zero, then the
      | function will be approximated with
      | a lookup table.
      |
      */
    pub fn new(
        function:                &fn(_0: NumericType<Self>) -> NumericType<Self>,
        lookup_table_num_points: Option<usize>

    ) -> Self {

        let lookup_table_num_points: usize =
            lookup_table_num_points.unwrap_or(0);

        todo!();
        /*
            initialise (function, lookupTableNumPoints);
        */
    }

    /**
      | Returns true if the Oscillator has been
      | initialised.
      |
      */
    pub fn is_initialised(&self) -> bool {
        
        todo!();
        /*
            return static_cast<bool> (generator);
        */
    }

    /**
      | Initialises the oscillator with a waveform.
      |
      */
    pub fn initialise(
        &mut self, 
        function:                &OscillatorGenerator<NumericType<Self>>,
        lookup_table_num_points: Option<usize>

    ) {

        let lookup_table_num_points: usize = lookup_table_num_points.unwrap_or(0);

        todo!();
        /*
            if (lookupTableNumPoints != 0)
            {
                auto* table = new LookupTableTransform<NumericType> (function,
                                                                     -MathConstants<NumericType>::pi,
                                                                     MathConstants<NumericType>::pi,
                                                                     lookupTableNumPoints);

                lookupTable.reset (table);
                generator = [table] (NumericType x) { return (*table) (x); };
            }
            else
            {
                generator = function;
            }
        */
    }
    
    /**
      | Sets the frequency of the oscillator.
      |
      */
    pub fn set_frequency(
        &mut self, 
        new_frequency: NumericType<Self>,
        force:         Option<bool>
    ) {

        let force: bool = force.unwrap_or(false);

        todo!();
        /*
            if (force)
            {
                frequency.setCurrentAndTargetValue (newFrequency);
                return;
            }

            frequency.setTargetValue (newFrequency);
        */
    }

    /**
      | Returns the current frequency of the
      | oscillator.
      |
      */
    pub fn get_frequency(&self) -> NumericType<Self> {
        
        todo!();
        /*
            return frequency.getTargetValue();
        */
    }
    
    /**
      | Called before processing starts.
      |
      */
    pub fn prepare(&mut self, spec: &ProcessSpec)  {
        
        todo!();
        /*
            sampleRate = static_cast<NumericType> (spec.sampleRate);
            rampBuffer.resize ((int) spec.maximumBlockSize);

            reset();
        */
    }

    /**
      | Resets the internal state of the oscillator
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            phase.reset();

            if (sampleRate > 0)
                frequency.reset (sampleRate, 0.05);
        */
    }

    /**
      | Returns the result of processing a single
      | sample.
      |
      */
    pub fn process_sample(&mut self, input: SampleType) -> SampleType {
        
        todo!();
        /*
            jassert (isInitialised());
            auto increment = MathConstants<NumericType>::twoPi * frequency.getNextValue() / sampleRate;
            return input + generator (phase.advance (increment) - MathConstants<NumericType>::pi);
        */
    }

    /**
      | Processes the input and output buffers
      | supplied in the processing context.
      |
      */
    pub fn process<ProcessContext>(&mut self, context: &ProcessContext)  {
    
        todo!();
        /*
            jassert (isInitialised());
            auto&& outBlock = context.getOutputBlock();
            auto&& inBlock  = context.getInputBlock();

            // this is an output-only processor
            jassert (outBlock.getNumSamples() <= static_cast<size_t> (rampBuffer.size()));

            auto len           = outBlock.getNumSamples();
            auto numChannels   = outBlock.getNumChannels();
            auto inputChannels = inBlock.getNumChannels();
            auto baseIncrement = MathConstants<NumericType>::twoPi / sampleRate;

            if (context.isBypassed)
                context.getOutputBlock().clear();

            if (frequency.isSmoothing())
            {
                auto* buffer = rampBuffer.getRawDataPointer();

                for (size_t i = 0; i < len; ++i)
                    buffer[i] = phase.advance (baseIncrement * frequency.getNextValue())
                                  - MathConstants<NumericType>::pi;

                if (! context.isBypassed)
                {
                    size_t ch;

                    if (context.usesSeparateInputAndOutputBlocks())
                    {
                        for (ch = 0; ch < jmin (numChannels, inputChannels); ++ch)
                        {
                            auto* dst = outBlock.getChannelPointer (ch);
                            auto* src = inBlock.getChannelPointer (ch);

                            for (size_t i = 0; i < len; ++i)
                                dst[i] = src[i] + generator (buffer[i]);
                        }
                    }
                    else
                    {
                        for (ch = 0; ch < jmin (numChannels, inputChannels); ++ch)
                        {
                            auto* dst = outBlock.getChannelPointer (ch);

                            for (size_t i = 0; i < len; ++i)
                                dst[i] += generator (buffer[i]);
                        }
                    }

                    for (; ch < numChannels; ++ch)
                    {
                        auto* dst = outBlock.getChannelPointer (ch);

                        for (size_t i = 0; i < len; ++i)
                            dst[i] = generator (buffer[i]);
                    }
                }
            }
            else
            {
                auto freq = baseIncrement * frequency.getNextValue();
                auto p = phase;

                if (context.isBypassed)
                {
                    frequency.skip (static_cast<int> (len));
                    p.advance (freq * static_cast<NumericType> (len));
                }
                else
                {
                    size_t ch;

                    if (context.usesSeparateInputAndOutputBlocks())
                    {
                        for (ch = 0; ch < jmin (numChannels, inputChannels); ++ch)
                        {
                            p = phase;
                            auto* dst = outBlock.getChannelPointer (ch);
                            auto* src = inBlock.getChannelPointer (ch);

                            for (size_t i = 0; i < len; ++i)
                                dst[i] = src[i] + generator (p.advance (freq) - MathConstants<NumericType>::pi);
                        }
                    }
                    else
                    {
                        for (ch = 0; ch < jmin (numChannels, inputChannels); ++ch)
                        {
                            p = phase;
                            auto* dst = outBlock.getChannelPointer (ch);

                            for (size_t i = 0; i < len; ++i)
                                dst[i] += generator (p.advance (freq) - MathConstants<NumericType>::pi);
                        }
                    }

                    for (; ch < numChannels; ++ch)
                    {
                        p = phase;
                        auto* dst = outBlock.getChannelPointer (ch);

                        for (size_t i = 0; i < len; ++i)
                            dst[i] = generator (p.advance (freq) - MathConstants<NumericType>::pi);
                    }
                }

                phase = p;
            }
        */
    }
}
