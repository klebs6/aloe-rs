crate::ix!();

pub trait AudioProcessorGraphInterface
: AcceptsMidi 
+ AddConnection 
+ AddNode 
+ AnyNodesNeedPreparing 
+ BuildRenderingSequence 
+ CanConnect 
+ ChangeProgramName 
+ Clear 
+ ClearRenderingSequence 
+ CreateAudioProcessorGraphEditor 
+ DisconnectNode 
+ GetConnections 
+ GetCurrentProgram 
+ GetName 
+ GetNode 
+ GetNodeConnections 
+ GetNodeForId 
+ GetNodes 
+ GetNumNodes 
+ GetNumPrograms 
+ GetProgramName 
+ GetStateInformation 
+ GetTailLengthSeconds 
+ HandleAsyncUpdate 
+ HasEditor 
+ IsAnInputTo 
+ IsConnected 
+ IsConnectionLegal 
+ IsLegal 
+ PrepareToPlay 
+ ProcessBlock 
+ ProducesMidi 
+ ReleaseResources 
+ RemoveConnection 
+ RemoveIllegalConnections 
+ RemoveNode 
+ Reset 
+ SetCurrentProgram 
+ SetNonRealtime 
+ SetStateInformation 
+ SupportsDoublePrecisionProcessing 
+ TopologyChanged 
+ Unprepare 
{}
