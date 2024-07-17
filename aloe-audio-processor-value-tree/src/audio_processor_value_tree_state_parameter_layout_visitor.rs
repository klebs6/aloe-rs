crate::ix!();

pub trait AudioProcessorValueTreeStateParameterLayoutVisitee {}

pub trait AudioProcessorValueTreeStateParameterLayoutVisitor
{
    /**
      | If you have a compiler error telling
      | you that there is no matching member
      | function to call for 'visit', then you
      | are probably attempting to add
      | a parameter that is not derived from
      | RangedAudioParameter to the
      | AudioProcessorValueTreeState.
      */
    fn visit(&self, _0: Box<dyn AudioProcessorValueTreeStateParameterLayoutVisitee>);
}

impl AudioProcessorValueTreeStateParameterLayoutVisitee for RangedAudioParameter {}
impl AudioProcessorValueTreeStateParameterLayoutVisitee for AudioProcessorParameterGroup {}

