crate::ix!();

/**
  | Host callback for extended unit support:
  | Vst::IUnitHandler2 \ingroup vstIHost
  | vst365
  | 
  | - [host imp]
  | 
  | - [extends IUnitHandler]
  | 
  | - [released: 3.6.5]
  | 
  | - [optional]
  | 
  | Host callback interface, used with
  | IUnitInfo.
  | 
  | Retrieve via queryInterface from IComponentHandler.
  | 
  | The plug-in has the possibility to inform
  | the host with notifyUnitByBusChange
  | that something has changed in the bus
  | - unit assignment, the host then has
  | to recall IUnitInfo::getUnitByBus
  | in order to get the new relations between
  | busses and unit.
  | 
  | \see \ref vst3Units, IUnitHandler
  |
  */
pub trait IUnitHandler2: FUnknown {

    /**
      | Tell host that assignment Unit-Bus
      | defined by IUnitInfo::getUnitByBus
      | has changed.
      |
      */
    #[PLUGIN_API]
    fn notify_unit_by_bus_change(&mut self) -> tresult;
}

lazy_static!{
    /*
    static const FUID iunit_handler2_iid;
    */
}

declare_class_iid!{
    IUnitHandler2, 
    0xF89F8CDF, 
    0x699E4BA5, 
    0x96AAC9A4, 
    0x81452B01
}
