crate::ix!();

pub struct AudioProcessorGraphRenderSequenceFloat  { base: GraphRenderSequence<f32>, }
pub struct AudioProcessorGraphRenderSequenceDouble { base: GraphRenderSequence<f64>, }

/**
  | A special index that represents the
  | midi channel of a node.
  | 
  | This is used as a channel index value
  | if you want to refer to the midi input
  | or output instead of an audio channel.
  |
  */
pub const AUDIO_PROCESSOR_GRAPH_MIDI_CHANNEL_INDEX: usize = 0x1000;
