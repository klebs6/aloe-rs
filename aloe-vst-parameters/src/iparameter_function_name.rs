crate::ix!();

/**
  | Edit controller component interface
  | extension: Vst::IParameterFunctionName
  | \ingroup vstIPlug vst370
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.7.0]
  | 
  | - [optional]
  | 
  | This interface allows the host to get
  | a parameter associated to a specific
  | meaning (a functionName) for a given
  | unit.
  | 
  | The host can use this information, for
  | example, for drawing a Gain Reduction
  | meter in its own UI.
  | 
  | In order to get the plain value of this
  | parameter, the host should use the IEditController::normalizedParamToPlain.
  | 
  | The host can automatically map parameters
  | to dedicated UI controls, such as the
  | wet-dry mix knob or Randomize button.
  | 
  | \section IParameterFunctionNameExample
  | Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | // here an example of how a Vst3 plug-in could support this IParameterFunctionName interface.
  | // we need to define somewhere the iids:
  | 
  | in MyController class declaration
  | class MyController : public Vst::EditController, public Vst::IParameterFunctionName
  | {
  |     ...
  |     tresult PLUGIN_API getParameterIDFromFunctionName (UnitID unitID, FIDString functionName,
  |                                                     Vst::ParamID& paramID) override;
  |     ...
  | 
  |     OBJ_METHODS (MyController, Vst::EditController)
  |     DEFINE_INTERFACES
  |         ...
  |         DEF_INTERFACE (Vst::IParameterFunctionName)
  |     END_DEFINE_INTERFACES (Vst::EditController)
  |     ...
  | }
  | 
  | #include "ivstparameterfunctionname.h"
  | namespace Steinberg {
  |     namespace Vst {
  |         DEF_CLASS_IID (IParameterFunctionName)
  |     }
  | }
  | 
  | tresult PLUGIN_API MyController::getParameterIDFromFunctionName (UnitID unitID, FIDString functionName,
  |                                                                  Vst::ParamID& paramID)
  | {
  |     using namespace Vst;
  | 
  |     paramID = kNoParamId;
  | 
  |     if (unitID == kRootUnitId && FIDStringsEqual (functionName, kCompGainReduction))
  |         paramID = kMyGainReductionId;
  | 
  |     return (paramID != kNoParamId) ? kResultOk : kResultFalse;
  | }
  | 
  | //--- a host implementation example: --------------------
  | ...
  | FUnknownPtr<Vst::IParameterFunctionName> functionName (mEditController->getIEditController ());
  | if (functionName)
  | {
  |     Vst::ParamID paramID;
  |     if (functionName->getParameterIDFromFunctionName (Vst::FunctionNameType::kCompGainReduction, paramID) == kResultTrue)
  |     {
  |         // paramID could be cached for performance issue
  |         ParamValue norm = mEditController->getIEditController ()->getParamNormalized (paramID);
  |         ParamValue plain = mEditController->getIEditController ()->normalizedParamToPlain (paramID, norm);
  |         // plain is something like -6 (-6dB)
  |     }
  | }
  |
  */
pub trait IParameterFunctionName: FUnknown {

    /**
      | Gets for the given unitID the associated
      | paramID to a function Name.
      | 
      | Returns kResultFalse when no found
      | parameter (paramID is set to kNoParamId
      | in this case).
      |
      */
    #[PLUGIN_API]
    fn get_parameter_id_from_function_name(&mut self, 
            unitid:        UnitID,
            function_name: FIDString,
            paramid:       &mut ParamID) -> tresult;

}

lazy_static!{
    /*
    static const FUID iparameter_function_name_iid;
    */
}

declare_class_iid!{
    IParameterFunctionName, 
    0x6D21E1DC, 
    0x91199D4B, 
    0xA2A02FEF, 
    0x6C1AE55C
}
