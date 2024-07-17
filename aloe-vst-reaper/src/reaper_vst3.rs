crate::ix!();

/**
  | supported by REAPER v6.24+, queried
  | from plug-in IController
  |
  */
pub trait IReaperUIEmbedInterface: FUnknown {

    /**
      | note: VST2 uses CanDo
      | "hasCockosEmbeddedUI"==0xbeef0000,
      | then opcode=effVendorSpecific,
      | index=effEditDraw,
      | opt=(float)msg,
      | value=parm2,
      | ptr=parm3
      |
      | see reaper_plugin_fx_embed.h
      */
    fn embed_message(
        &mut self, 
        msg:   i32,
        parm2: TPtrInt,
        parm3: TPtrInt

    ) -> TPtrInt;
}

lazy_static!{
    /*
    static const FUID ireaper_ui_embed_interface_iid;
    */
}

declare_class_iid!{
    IReaperUIEmbedInterface, 
    0x049bf9e7, 0xbc74ead0, 0xc4101e86, 0x7f725981
}
