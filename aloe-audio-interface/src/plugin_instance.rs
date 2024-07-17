crate::ix!();

pub trait AudioPluginInstanceInterface 
: GetPluginDescription
+ GetPlatformSpecificData
+ GetExtensions
+ GetParameterID
+ GetParameter
+ SetParameter
+ GetParameterName
+ GetParameterText
+ GetParameterDefaultValue
+ GetParameterNumSteps
+ IsParameterDiscrete
+ IsParameterAutomatable
+ GetParameterLabel
+ IsParameterOrientationInverted
+ IsMetaParameter
+ GetParameterCategory
+ AssertOnceOnDeprecatedMethodUse
{ }
