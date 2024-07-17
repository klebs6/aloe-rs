crate::ix!();

pub trait AudioFormatInterface
: CanHandleFile

/*
   | Returns the name of this format. e.g.
   | "WAV file" or "AIFF file"
   |
   */
+ GetFormatName
+ GetFileExtensions
+ IsCompressed
+ GetQualityOptions
+ CreateMemoryMappedReader
+ IsChannelLayoutSupported
+ CreateWriterFor
{}
