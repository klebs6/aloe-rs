crate::ix!();

pub struct BufferTransfer {
    buffer:     BufferWithSampleRate,
    new_buffer: bool, // default = false
    mutex:      SpinLock,
}

impl BufferTransfer {
    
    pub fn set(&mut self, p: BufferWithSampleRate)  {
        
        todo!();
        /*
            const SpinLock::ScopedLockType lock (mutex);
                buffer = std::move (p);
                newBuffer = true;
        */
    }

    /**
      | Call `fn` passing the new buffer, if
      | there's one available
      |
      */
    pub fn get<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            const SpinLock::ScopedTryLockType lock (mutex);

                if (lock.isLocked() && newBuffer)
                {
                    fn (buffer);
                    newBuffer = false;
                }
        */
    }
}
