crate::ix!();

pub trait AudioChannelSetInterface 
: IsDisabled
+ ChannelSetIntersect
+ GetChannelTypeName
+ GetAbbreviatedChannelTypeName
+ GetChannelTypeFromAbbreviation
+ GetSpeakerArrangementAsString
+ FromAbbreviatedString

/*
   | Returns the description of the current
   | layout. For example, this method may
   | return "Quadraphonic". Note that the
   | returned string may not be unique.
   */
+ GetDescription
+ IsDiscreteLayout
+ GetSize
+ GetTypeOfChannel
+ GetChannelIndexForType
+ GetChannelTypes
+ AddChannel
+ RemoveChannel
+ ChannelSetNull
+ Mono
+ Stereo
+ CreateLCR
+ CreateLRS
+ CreateLCRS
+ Create5Point0
+ Create5Point1
+ Create6Point0
+ Create6Point1
+ Create6Point0Music
+ Create6Point1Music
+ Create7Point0
+ Create7Point0Sdds
+ Create7Point1
+ Create7Point1Sdds
+ Quadraphonic
+ Pentagonal
+ Hexagonal
+ Octagonal
+ Create7Point0Point2
+ Create7Point1Point2
+ Create7Point0Point4
+ Create7Point1Point4
+ Ambisonic
+ GetAmbisonicOrder
+ DiscreteChannels
+ CanonicalChannelSet
+ NamedChannelSet
+ ChannelSetsWithNumberOfChannels
+ ChannelSetWithChannels
+ FromWaveChannelMask
+ GetWaveChannelMask
+ GetAmbisonicOrderForNumChannels
{ }

pub trait ChannelSetIntersect {

    /**
      | Intersect two channel layouts.
      |
      */
    fn intersect(&mut self, other: &dyn AudioChannelSetInterface);
}

pub trait CanonicalChannelSet {

    /**
      | Create a canonical channel set for a given
      | number of channels. For example,
      | numChannels = 1 will return mono,
      | numChannels = 2 will return stereo, etc.
      */
    fn canonical_channel_set(&mut self, num_channels: i32) -> dyn AudioChannelSetInterface;
}

pub trait NamedChannelSet {

    /** 
      | Create a channel set for a given number of
      | channels which is non-discrete.
      | If numChannels is larger than the number
      | of channels of the surround format with
      | the maximum amount of channels (currently
      | 7.1 Surround), then this function returns
      | an empty set.
      */
    fn named_channel_set(&mut self, num_channels: i32) -> dyn AudioChannelSetInterface;
}

pub trait ChannelSetsWithNumberOfChannels {

    /**
      | Return an array of channel sets which
      | have a given number of channels
      |
      */
    fn channel_sets_with_number_of_channels(&mut self, num_channels: i32) -> Vec<Rc<RefCell<dyn AudioChannelSetInterface>>>;
}

pub trait ChannelSetWithChannels {

    /**
      | Creates a channel set for a list of channel
      | types. This function will assert if
      | you supply a duplicate channel. Note
      | that this method ignores the order in
      | which the channels are given, i.e. two
      | arrays with the same elements but in
      | a different order will still result
      | in the same channel set.
      |
      */
    fn channel_set_with_channels(&mut self, channel_array: &[AudioChannelType]) -> dyn AudioChannelSetInterface;
}
