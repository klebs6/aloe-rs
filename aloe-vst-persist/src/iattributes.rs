crate::ix!();

pub type IAttrID = FIDString;

/**
  | Object Data Archive Interface.
  | 
  | - [host imp]
  | 
  | - store data/objects/binary/subattributes
  | in the archive
  | 
  | - read stored data from the archive
  | 
  | All data stored to the archive are identified
  | by a string (IAttrID), which must be
  | unique on each
  | 
  | IAttribute level.
  | 
  | The basic set/get methods make use of
  | the FVariant class defined in 'funknown.h'.
  | 
  | For a more convenient usage of this interface,
  | you should use the functions defined
  | in namespace PAttributes (public.sdk/source/common/pattributes.h+cpp)
  | !!
  | 
  | \ingroup frameworkHostClasses
  |
  */
pub trait IAttributes: FUnknown {

    /**
      | Store any data in the archive. It is even
      | possible to store sub-attributes by
      | creating a new IAttributes instance
      | via the IHostClasses interface and
      | pass it to the parent in the
      | 
      | FVariant. In this case the archive must
      | take the ownership of the newly created
      | object, which is true for all objects
      | that have been created only for storing.
      | You tell the archive to take ownership
      | by adding the FVariant::kOwner flag
      | to the FVariant::type member (data.type
      | |= FVariant::kOwner).
      | 
      | When using the PAttributes functions,
      | this is done through a function parameter.
      |
      */
    #[PLUGIN_API]
    fn set(&mut self, 
            attrid: IAttrID,
            data:   &FVariant) -> tresult;

    /**
      | Store a list of data in the archive. Please
      | note that the type of data is not mixable!
      | So you can only store a list of integers
      | or a list of doubles/strings/etc. You
      | can also store a list of subattributes
      | or other objects that implement the
      | IPersistent interface.
      |
      */
    #[PLUGIN_API]
    fn queue(&mut self, 
            listid: IAttrID,
            data:   &FVariant) -> tresult;

    /**
      | Store binary data in the archive. Parameter
      | 'copyBytes' specifies if the passed
      | data should be copied.
      | 
      | The archive cannot take the ownership
      | of binary data. Either it just references
      | a buffer in order to write it to a file
      | (copyBytes = false) or it copies the
      | data to its own buffers (copyBytes =
      | true).
      | 
      | When binary data should be stored in
      | the default pool for example, you must
      | always copy it!
      |
      */
    #[PLUGIN_API]
    fn set_binary_data(&mut self, 
            attrid:     IAttrID,
            data:       *mut c_void,
            bytes:      u32,
            copy_bytes: bool) -> tresult;

    /**
      | Get data previously stored to the archive.
      |
      */
    #[PLUGIN_API]
    fn get(&mut self, 
            attrid: IAttrID,
            data:   &mut FVariant) -> tresult;

    /**
      | Get list of data previously stored to
      | the archive. As long as there are queue
      | members the method will return kResultTrue.
      | When the queue is empty, the methods
      | returns kResultFalse. All lists except
      | from object lists can be reset which
      | means that the items can be read once
      | again. \see IAttributes::resetQueue
      |
      */
    #[PLUGIN_API]
    fn unqueue(&mut self, 
            listid: IAttrID,
            data:   &mut FVariant) -> tresult;

    /**
      | Get the amount of items in a queue.
      |
      */
    #[PLUGIN_API]
    fn get_queue_item_count(&mut self, _0: IAttrID) -> i32;

    /**
      | Reset a queue. If you need to restart
      | reading a queue, you have to reset it.
      | You can reset a queue at any time.
      |
      */
    #[PLUGIN_API]
    fn reset_queue(&mut self, attrid: IAttrID) -> tresult;

    /**
      | Reset all queues in the archive.
      |
      */
    #[PLUGIN_API]
    fn reset_all_queues(&mut self) -> tresult;

    /**
      | Read binary data from the archive. The
      | data is copied into the passed buffer.
      | The size of that buffer must fit the size
      | of data stored in the archive which can
      | be queried via IAttributes::getBinaryDataSize
      |
      */
    #[PLUGIN_API]
    fn get_binary_data(&mut self, 
            attrid: IAttrID,
            data:   *mut c_void,
            bytes:  u32) -> tresult;

    /**
      | Get the size in bytes of binary data in
      | the archive.
      |
      */
    #[PLUGIN_API]
    fn get_binary_data_size(&mut self, attrid: IAttrID) -> u32;
}

lazy_static!{
    /*
    static const FUID iattributes_iid;
    */
}

declare_class_iid!{
    IAttributes, 
    0xFA1E32F9, 
    0xCA6D46F5, 
    0xA982F956, 
    0xB1191B58
}
