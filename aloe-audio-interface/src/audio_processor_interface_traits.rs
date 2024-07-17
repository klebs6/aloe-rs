crate::ix!();

pub trait AudioProcessorInterface
: AudioProcessorAddBus
+ AddAudioProcessorListener
+ AddParameter
+ AddParameterGroup
+ ApplyBusLayouts
+ BusesPropertiesFromLayoutArray
+ CanApplyBusCountChange
+ ChangeProgramName
+ CheckAcceptsMidi
+ CheckBusesLayoutSupported
+ CheckCanAddBus
+ CheckCanApplyBusesLayout
+ CheckCanRemoveBus
+ CheckHasEditor
+ CheckIsBusesLayoutSupported
+ CheckIsMidiEffect
+ CheckProducesMidi
+ CheckSupportsDoublePrecisionProcessing
+ CheckSupportsMpe
+ ContainsLayout
+ CopyXMLToBinary
+ CreateEditor
+ CreateEditorIfNeeded
+ DisableNonMainBuses
+ EditorBeingDeleted
+ EnableAllBuses
+ GetAAXPluginIDForMainBusConfig
+ GetActiveEditor
+ GetAlternateDisplayNames
+ GetBlockSize
+ GetBus
//+ GetBusBuffer
+ GetBusCount
+ GetBusesLayout
+ GetBypassParameter
+ GetCallbackLock
+ GetChannelCountOfBus
+ GetBusChannelIndexInProcessBlockBuffer
+ GetChannelLayoutOfBus
+ GetCurrentProgram
+ GetCurrentProgramStateInformation
+ GetDefaultNumParameterSteps
+ GetLatencySamples
+ GetMainBusNumInputChannels
+ GetMainBusNumOutputChannels

/*
  | Returns the name of this processor.
  |
  */
+ GetName
+ GetNextBestLayoutInLayoutList
+ GetNumPrograms
+ GetOffsetInBusBufferForAbsoluteChannelIndex
+ GetParameterTree
+ GetParameters
+ GetPlayHead
+ GetProcessingPrecision
+ GetProgramName
+ GetResponseCurve
+ GetSampleRate
+ GetStateInformation
+ GetTailLengthSeconds
+ GetTotalNumInputChannels
+ GetTotalNumOutputChannels
+ GetWrapperTypeDescription
+ GetXMLFromBinary
+ IsNonRealtime
+ IsSuspended
+ IsUsingDoublePrecision
+ LayoutListToArray

/*
  | Called by the host to indicate that you
  | should reduce your memory footprint.
  | 
  | You should override this method to free
  | up some memory gracefully, if possible,
  | otherwise the host may forcibly unload
  | your AudioProcessor.
  | 
  | At the moment this method is only called
  | when your AudioProcessor is an AUv3
  | plug-in running on iOS.
  |
  */
+ MemoryWarningReceived
+ NumBusesChanged
+ NumChannelsChanged
+ PrepareToPlay
+ ProcessBlock
+ ProcessBlockBypassed
+ ProcessBlockF64
+ ProcessorLayoutsChanged
+ RefreshParameterList
+ ReleaseResources
+ RemoveBus
+ RemoveAudioProcessorListener
+ Reset
+ SendParamChangeMessageToListeners
+ SetBusesLayout
+ SetBusesLayoutWithoutEnabling
+ SetChannelLayoutOfBus
+ SetCurrentProgram
+ SetCurrentProgramStateInformation
+ SetLatencySamples
+ SetNonRealtime
+ SetParameterTree
+ SetPlayConfigDetails
+ SetPlayHead
+ SetProcessingPrecision
+ SetRateAndBufferSizeDetails
+ SetStateInformation
+ SetTypeOfNextNewPlugin
+ SuspendProcessing
+ UpdateHostDisplay
+ UpdateTrackProperties
{ }
