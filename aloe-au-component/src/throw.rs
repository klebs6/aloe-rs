crate::ix!();

#[cfg(not(COMPONENT_THROW))]
#[cfg(VERBOSE_COMPONENT_THROW)]
macro_rules! component_throw {
    ($throw_err:ident) => {
        /*
        
                    do { DebugMessage(#throw_err); throw static_cast<OSStatus>(throw_err); } while (0)
        */
    }
}

#[cfg(not(COMPONENT_THROW))]
#[cfg(not(VERBOSE_COMPONENT_THROW))]
macro_rules! component_throw {
    ($throw_err:ident) => {
        /*
        
                    throw static_cast<OSStatus>(throw_err)
        */
    }
}

macro_rules! component_catch {
    () => {
        /*
        
            catch (const CAXException &ex) { result = ex.mError; } 
            catch (std::bad_alloc &) { result = kAudio_MemFullError; } 
            catch (OSStatus catch_err) { result = catch_err; } 
            catch (OSErr catch_err) { result = catch_err; } 
            catch (...) { result = -1; }
        */
    }
}

