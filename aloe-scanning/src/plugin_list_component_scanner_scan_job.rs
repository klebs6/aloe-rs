crate::ix!();

#[no_copy]
#[leak_detector]
pub struct PluginListComponentScannerScanJob<'a> {
    base:    ThreadPoolJob<'a>,
    scanner: &'a mut PluginListComponentScanner<'a>,
}

impl<'a> PluginListComponentScannerScanJob<'a> {

    pub fn new(s: &mut PluginListComponentScanner) -> Self {
    
        todo!();
        /*
        : thread_pool_job("pluginscan"),
        : scanner(s),

        
        */
    }
    
    pub fn run_job(&mut self) -> ThreadPoolJobStatus {
        
        todo!();
        /*
            while (scanner.doNextScan() && ! shouldExit())
                    {}

                    return jobHasFinished;
        */
    }
}
