crate::ix!();

pub trait MemoryMappedAudioFormatReaderInterface 
: GetFile
+ MapEntireFile
+ GetMappedSection
+ TouchSample
+ GetNumBytesUsed
+ SampleToFilePos
+ FilePosToSample
+ SampleToPointer
+ ScanMinAndMaxInterleaved
{ }

