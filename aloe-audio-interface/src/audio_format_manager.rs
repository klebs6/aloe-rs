crate::ix!();

pub trait AudioFormatManagerInterface
: RegisterFormat
+ RegisterBasicFormats
+ ClearFormats
+ GetNumKnownFormats
+ GetKnownFormat
+ GetDefaultFormat
+ FindFormatForFileExtension
+ GetWildcardForAllFormats
+ CreateReaderFor
{ }

