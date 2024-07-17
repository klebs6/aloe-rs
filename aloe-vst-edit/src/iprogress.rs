crate::ix!();

/**
  | Extended host callback interface for
  | an edit controller: Vst::IProgress
  | \ingroup vstIHost vst370
  | 
  | - [host imp]
  | 
  | - [extends IComponentHandler]
  | 
  | - [released: 3.7.0]
  | 
  | - [optional]
  | 
  | Allows the plug-in to request the host
  | to create a progress for some specific
  | tasks which take some time. The host
  | can visualize the progress as read-only
  | UI elements. For example, after loading
  | a project where a plug-in needs to load
  | extra data (e.g. samples) in a background
  | thread, this enables the host to get
  | and visualize the current status of
  | the loading progress and to inform the
  | user when the loading is finished.
  | 
  | -----------
  | @note
  | 
  | During the progress, the host can unload
  | the plug-in at any time. Make sure that
  | the plug-in supports this use case.
  | 
  | \section IProgressExample Example
  | 
  | -----------
  | @code
  | 
  | {.cpp}
  | 
  | // we are in the editcontroller:
  | // as member: IProgress::IProgressID mProgressID;
  | 
  | FUnknownPtr<IProgress> progress (componentHandler);
  | if (progress)
  |     progress->start (IProgress::IProgressProgressType::UIBackgroundTask, STR ("Load Samples..."), mProgressID);
  | 
  | // ...
  | myProgressValue += incProgressStep;
  | FUnknownPtr<IProgress> progress (componentHandler);
  | if (progress)
  |     progress->update (mProgressID, myProgressValue);
  | 
  | // ...
  | FUnknownPtr<IProgress> progress (componentHandler);
  | if (progress)
  |     progress->finish (mProgressID);
  | 
  | \see \ref IComponentHandler
  |
  */
pub trait IProgress: FUnknown {

    /**
      | Start a new progress of a given type and
      | optional Description. outID is as IProgressID
      | created by the host to identify this
      | newly created progress (for update
      | and finish method)
      |
      */
    #[PLUGIN_API]
    fn start(&mut self, 
            ty:                   IProgressProgressType,
            optional_description: *const u16,
            outid:                &mut IProgressID) -> tresult;

    /**
      | Update the progress value (normValue
      | between [0, 1]) associated to the given
      | id
      |
      */
    #[PLUGIN_API]
    fn update(&mut self, 
            id:         IProgressID,
            norm_value: ParamValue) -> tresult;

    /**
      | Finish the progress associated to the
      | given id
      |
      */
    #[PLUGIN_API]
    fn finish(&mut self, id: IProgressID) -> tresult;
}

#[repr(u32)]
pub enum IProgressProgressType 
{
    /**
      | plug-in state is restored async (in
      | a background Thread)
      |
      */
    AsyncStateRestoration = 0,  

    /**
      | a plug-in task triggered by a UI action
      |
      */
    UIBackgroundTask            
}

pub type IProgressID = u64;

lazy_static!{
    /*
    static const FUID iprogress_iid;
    */
}

declare_class_iid!{
    IProgress, 
    0x00C9DC5B, 
    0x9D904254, 
    0x91A388C8, 
    0xB4E91B69
}
