crate::ix!();

#[leak_detector]
pub trait GraphRenderSequenceRenderingOp<FloatType: num::Float>
{
    fn perform(&mut self, _0: &GraphRenderSequenceContext<FloatType>);
}

pub struct GraphRenderSequenceContext<FloatType: num::Float>
{
    audio_buffers:   *mut *mut FloatType,
    midi_buffers:    *mut MidiBuffer,
    audio_play_head: *mut dyn AudioPlayHeadInterface,
    num_samples:     i32,
}
