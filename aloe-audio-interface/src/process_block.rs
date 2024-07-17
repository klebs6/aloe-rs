crate::ix!();

pub trait ProcessBlock {

    /**
      | Renders the next block.
      | 
      | When this method is called, the buffer
      | contains a number of channels which
      | is at least as great as the maximum number
      | of input and output channels that this
      | processor is using. It will be filled
      | with the processor's input data and
      | should be replaced with the processor's
      | output.
      | 
      | So for example if your processor has
      | a total of 2 input channels and 4 output
      | channels, then the buffer will contain
      | 4 channels, the first two being filled
      | with the input data. Your processor
      | should read these, do its processing,
      | and replace the contents of all 4 channels
      | with its output.
      | 
      | Or if your processor has a total of 5 inputs
      | and 2 outputs, the buffer will have 5
      | channels, all filled with data, and
      | your processor should overwrite the
      | first 2 of these with its output. But
      | be VERY careful not to write anything
      | to the last 3 channels, as these might
      | be mapped to memory that the host assumes
      | is read-only!
      | 
      | If your plug-in has more than one input
      | or output buses then the buffer passed
      | to the processBlock methods will contain
      | a bundle of all channels of each bus.
      | 
      | Use getBusBuffer to obtain an audio
      | buffer for a particular bus.
      | 
      | -----------
      | @note
      | 
      | if you have more outputs than inputs,
      | then only those channels that correspond
      | to an input channel are guaranteed to
      | contain sensible data - e.g. in the case
      | of 2 inputs and 4 outputs, the first two
      | channels contain the input, but the
      | last two channels may contain garbage,
      | so you should be careful not to let this
      | pass through without being overwritten
      | or cleared.
      | 
      | Also note that the buffer may have more
      | channels than are strictly necessary,
      | but you should only read/write from
      | the ones that your processor is supposed
      | to be using.
      | 
      | The number of samples in these buffers
      | is NOT guaranteed to be the same for every
      | callback, and may be more or less than
      | the estimated value given to prepareToPlay().
      | 
      | Your code must be able to cope with variable-sized
      | blocks, or you're going to get clicks
      | and crashes!
      | 
      | Also note that some hosts will occasionally
      | decide to pass a buffer containing zero
      | samples, so make sure that your algorithm
      | can deal with that!
      | 
      | If the processor is receiving a MIDI
      | input, then the midiMessages array
      | will be filled with the MIDI messages
      | for this block. Each message's timestamp
      | will indicate the message's time, as
      | a number of samples from the start of
      | the block.
      | 
      | Any messages left in the MIDI buffer
      | when this method has finished are assumed
      | to be the processor's MIDI output. This
      | means that your processor should be
      | careful to clear any incoming messages
      | from the array if it doesn't want them
      | to be passed-on.
      | 
      | If you have implemented the getBypassParameter
      | method, then you need to check the value
      | of this parameter in this callback and
      | bypass your processing if the parameter
      | has a non-zero value.
      | ----------
      | @note
      | 
      | when calling this method as a host, the
      | result may still be bypassed as the parameter
      | that controls the bypass may be non-zero.
      | 
      | Be very careful about what you do in this
      | callback - it's going to be called by
      | the audio thread, so any kind of interaction
      | with the UI is absolutely out of the question.
      | If you change a parameter in here and
      | need to tell your UI to update itself,
      | the best way is probably to inherit from
      | a ChangeBroadcaster, let the UI components
      | register as listeners, and then call
      | sendChangeMessage() inside the processBlock()
      | method to send out an asynchronous message.
      | You could also use the AsyncUpdater
      | class in a similar way.
      | 
      | @see getBusBuffer
      |
      */
      fn process_block(&mut self, 
          buffer:        &mut AudioBuffer<f32>,
          midi_messages: &mut dyn MidiBufferInterface);

}

pub trait ProcessBlockF64 {

    /**
      | Renders the next block.
      | 
      | When this method is called, the buffer
      | contains a number of channels which
      | is at least as great as the maximum number
      | of input and output channels that this
      | processor is using. It will be filled
      | with the processor's input data and
      | should be replaced with the processor's
      | output.
      | 
      | So for example if your processor has
      | a combined total of 2 input channels
      | and 4 output channels, then the buffer
      | will contain 4 channels, the first two
      | being filled with the input data. Your
      | processor should read these, do its
      | processing, and replace the contents
      | of all 4 channels with its output.
      | 
      | Or if your processor has 5 inputs and
      | 2 outputs, the buffer will have 5 channels,
      | all filled with data, and your processor
      | should overwrite the first 2 of these
      | with its output. But be VERY careful
      | not to write anything to the last 3 channels,
      | as these might be mapped to memory that
      | the host assumes is read-only!
      | 
      | If your plug-in has more than one input
      | or output buses then the buffer passed
      | to the processBlock methods will contain
      | a bundle of all channels of each bus.
      | Use getBusBuffer to obtain a audio buffer
      | for a particular bus.
      | 
      | -----------
      | @note
      | 
      | if you have more outputs than inputs,
      | then only those channels that correspond
      | to an input channel are guaranteed to
      | contain sensible data - e.g. in the case
      | of 2 inputs and 4 outputs, the first two
      | channels contain the input, but the
      | last two channels may contain garbage,
      | so you should be careful not to let this
      | pass through without being overwritten
      | or cleared.
      | 
      | Also note that the buffer may have more
      | channels than are strictly necessary,
      | but you should only read/write from
      | the ones that your processor is supposed
      | to be using.
      | 
      | If your plugin uses buses, then you should
      | use getBusBuffer() or getChannelIndexInProcessBlockBuffer()
      | to find out which of the input and output
      | channels correspond to which of the
      | buses.
      | 
      | The number of samples in these buffers
      | is NOT guaranteed to be the same for every
      | callback, and may be more or less than
      | the estimated value given to prepareToPlay().
      | 
      | Your code must be able to cope with variable-sized
      | blocks, or you're going to get clicks
      | and crashes!
      | 
      | Also note that some hosts will occasionally
      | decide to pass a buffer containing zero
      | samples, so make sure that your algorithm
      | can deal with that!
      | 
      | If the processor is receiving a MIDI
      | input, then the midiMessages array
      | will be filled with the MIDI messages
      | for this block. Each message's timestamp
      | will indicate the message's time, as
      | a number of samples from the start of
      | the block.
      | 
      | Any messages left in the MIDI buffer
      | when this method has finished are assumed
      | to be the processor's MIDI output. This
      | means that your processor should be
      | careful to clear any incoming messages
      | from the array if it doesn't want them
      | to be passed-on.
      | 
      | If you have implemented the getBypassParameter
      | method, then you need to check the value
      | of this parameter in this callback and
      | bypass your processing if the parameter
      | has a non-zero value.
      | ----------
      | @note
      | 
      | when calling this method as a host, the
      | result may still be bypassed as the parameter
      | that controls the bypass may be non-zero.
      | 
      | Be very careful about what you do in this
      | callback - it's going to be called by
      | the audio thread, so any kind of interaction
      | with the UI is absolutely out of the question.
      | If you change a parameter in here and
      | need to tell your UI to update itself,
      | the best way is probably to inherit from
      | a ChangeBroadcaster, let the UI components
      | register as listeners, and then call
      | sendChangeMessage() inside the processBlock()
      | method to send out an asynchronous message.
      | You could also use the AsyncUpdater
      | class in a similar way.
      | 
      | @see getBusBuffer
      |
      */
      fn process_block(
          &mut self, 
          buffer:        &mut AudioBuffer<f64>,
          midi_messages: &mut dyn MidiBufferInterface
      );
}

pub trait ProcessBlockBypassed {

    /**
      | Renders the next block when the processor
      | is being bypassed.
      | 
      | The default implementation of this
      | method will pass-through any incoming
      | audio, but you may override this method
      | e.g. to add latency compensation to
      | the data to match the processor's latency
      | characteristics. This will avoid situations
      | where bypassing will shift the signal
      | forward in time, possibly creating
      | pre-echo effects and odd timings.
      | 
      | Another use for this method would be
      | to cross-fade or morph between the wet
      | (not bypassed) and dry (bypassed) signals.
      |
      */
    fn process_block_bypassed(
        &mut self, 
        buffer:        &mut AudioBuffer<f32>,
        midi_messages: &mut dyn MidiBufferInterface
    );

}

pub trait ProcessBlockF64Bypassed {

    /**
      | Renders the next block when the processor
      | is being bypassed.
      | 
      | The default implementation of this
      | method will pass-through any incoming
      | audio, but you may override this method
      | e.g. to add latency compensation to
      | the data to match the processor's latency
      | characteristics. This will avoid situations
      | where bypassing will shift the signal
      | forward in time, possibly creating
      | pre-echo effects and odd timings.
      | 
      | Another use for this method would be
      | to cross-fade or morph between the wet
      | (not bypassed) and dry (bypassed) signals.
      |
      */
    fn process_block_bypassed(&mut self, 
            buffer:        &mut AudioBuffer<f64>,
            midi_messages: &mut dyn MidiBufferInterface);
}
