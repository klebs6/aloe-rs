crate::ix!();

/**
  | Extended access to Attributes; supports
  | Attribute retrieval via iteration.
  | 
  | - [host imp]
  | 
  | - [released] C7/N6
  | 
  | \ingroup frameworkHostClasses
  |
  */
pub trait IAttributes2: IAttributes {

    /**
      | Returns the number of existing attributes.
      |
      */
    #[PLUGIN_API]
    fn count_attributes(&self) -> i32;

    /**
      | Returns the attribute's ID for the given
      | index.
      |
      */
    #[PLUGIN_API]
    fn get_attributeid(&self, index: i32) -> IAttrID;
}

lazy_static!{
    /*
    static const FUID iattributes2_iid;
    */
}

declare_class_iid!{
    IAttributes2, 
    0x1382126A, 
    0xFECA4871, 
    0x97D52A45, 
    0xB042AE99
}
