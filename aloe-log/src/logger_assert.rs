crate::ix!();

#[cfg(any(ALOE_LOG_ASSERTIONS,ALOE_DEBUG))]
pub fn log_assertion(
        filename: *const u8,
        line_num: i32)  {
    
    todo!();
    /*
        String m ("Aloe Assertion failure in ");
        m << File::createFileWithoutCheckingPath (CharPointer_UTF8 (filename)).getFileName() << ':' << lineNum;

       #if ALOE_LOG_ASSERTIONS
        Logger::writeToLog (m);
       #else
        DBG (m);
       #endif
    */
}
