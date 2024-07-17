crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_audio_processors/format_types/Vst3_SDK/pluginterfaces/vst/ivstautomationstate.h]

/**
  | Extended plug-in interface IEditController:
  | Vst::IAutomationState \ingroup vstIPlug
  | vst365
  | 
  | - [plug imp]
  | 
  | - [extends IEditController]
  | 
  | - [released: 3.6.5]
  | 
  | - [optional]
  | 
  | Hosts can inform the plug-in about its
  | current automation state (Read/Write/Nothing).
  |
  */
pub trait IAutomationState: FUnknown {

    /**
      | Sets the current Automation state.
      |
      */
    #[PLUGIN_API]
    fn set_automation_state(&mut self, state: i32) -> tresult;
}

lazy_static!{
    /*
    static const FUID IAUTOMATION_STATE_IID;
    */
}

bitflags!{

    pub struct AutomationStates: u32
    {
        /**
          | Not Read and not Write
          |
          */
        const kNoAutomation   = 0;

        /**
          | Read state
          |
          */
        const kReadState      = 1 << 0;

        /**
          | Write state
          |
          */
        const kWriteState     = 1 << 1;

        /**
          | Read and Write enable
          |
          */
        const kReadWriteState = Self::kReadState.bits() | Self::kWriteState.bits();
    }
}

declare_class_iid!{
    IAutomationState, 
    0xB4E8287F, 
    0x1BB346AA, 
    0x83A46667, 
    0x68937BAB
}
