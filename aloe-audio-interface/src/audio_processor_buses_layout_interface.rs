crate::ix!();

pub trait AudioProcessorBusesLayoutInterface 
: GetNumChannels 
+ GetChannelSet 
+ GetMainInputChannelSet 
+ GetMainOutputChannelSet 
+ GetMainInputChannels 
+ GetMainOutputChannels 
{ }
