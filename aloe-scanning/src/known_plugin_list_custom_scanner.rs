crate::ix!();

pub trait KnownPluginListCustomScannerInterface: FindPluginTypesFor + ScanFinished { }

/**
  | Class to define a custom plugin scanner
  |
  */
pub struct KnownPluginListCustomScanner {

}

impl KnownPluginListCustomScanner {

    pub fn scan_finished(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    /**
      | Returns true if the current scan should
      | be abandoned.
      | 
      | Any blocking methods should check this
      | value repeatedly and return if if becomes
      | true.
      |
      */
    pub fn should_exit(&self) -> bool {
        
        todo!();
        /*
            if (auto* job = ThreadPoolJob::getCurrentThreadPoolJob())
            return job->shouldExit();

        return false;
        */
    }
}
