crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_AbstractFifo.h]

/**
  | Encapsulates the logic required to implement
  | a lock-free FIFO.
  |
  | This class handles the logic needed when
  | building a single-reader, single-writer FIFO.
  |
  | It doesn't actually hold any data itself, but
  | your FIFO class can use one of these to manage
  | its position and status when reading or
  | writing to it.
  |
  | To use it, you can call prepareToWrite() to
  | determine the position within your own buffer
  | that an incoming block of data should be
  | stored, and prepareToRead() to find out when
  | the next outgoing block should be read from.
  |
  | e.g.
  | @code
  | struct MyFifo
  | {
  |     void addToFifo (const int* someData, int numItems)
  |     {
  |         int start1, size1, start2, size2;
  |         abstractFifo.prepareToWrite (numItems, start1, size1, start2, size2);
  |
  |         if (size1 > 0)
  |             copySomeData (myBuffer + start1, someData, size1);
  |
  |         if (size2 > 0)
  |             copySomeData (myBuffer + start2, someData + size1, size2);
  |
  |         abstractFifo.finishedWrite (size1 + size2);
  |     }
  |
  |     void readFromFifo (int* someData, int numItems)
  |     {
  |         int start1, size1, start2, size2;
  |         abstractFifo.prepareToRead (numItems, start1, size1, start2, size2);
  |
  |         if (size1 > 0)
  |             copySomeData (someData, myBuffer + start1, size1);
  |
  |         if (size2 > 0)
  |             copySomeData (someData + size1, myBuffer + start2, size2);
  |
  |         abstractFifo.finishedRead (size1 + size2);
  |     }
  |
  |     AbstractFifo abstractFifo { 1024 };
  |     int myBuffer[1024];
  | };
  | @endcode
  |
  | @tags{Core}
  */
#[no_copy]
#[leak_detector]
pub struct AbstractFifo {
    buffer_size: i32,
    valid_start: Atomic<i32>,
    valid_end:   Atomic<i32>,
}

pub mod abstract_fifo {

    use super::*;

    #[derive(ConstParamTy,PartialEq,Eq)]
    pub enum ReadOrWrite
    {
        read,
        write
    }

    /** Class for a scoped reader/writer */
    pub struct ScopedReadWrite<const mode: ReadOrWrite> {
        start_index1: i32,
        block_size1:  i32,
        start_index2: i32,
        block_size2:  i32,
        fifo:         *mut AbstractFifo, // default = nullptr
    }

    impl<const MODE: ReadOrWrite> Default for ScopedReadWrite<MODE> {

        /// Construct an unassigned reader/writer. Doesn't do anything upon destruction. 
        fn default() -> Self {

            Self {
                start_index1: 0,
                block_size1:  0,
                start_index2: 0,
                block_size2:  0,
                fifo:         null_mut(),
            }
        }
    }

    impl<const MODE: ReadOrWrite> Drop for ScopedReadWrite<MODE> {

        /**
          | Calls finishedRead or finishedWrite
          | if this is a non-null scoped reader/writer.
          |
          */
        fn drop(&mut self) {
            todo!();
            /* 
                    if (fifo != nullptr)
                        finish (*fifo, blockSize1 + blockSize2);
                 */
        }
    }

    impl<const MODE: ReadOrWrite> ScopedReadWrite<MODE> {
        
        /**
          | Construct a reader/writer and immediately
          | call prepareRead/prepareWrite on
          | the abstractFifo which was passed in.
          | 
          | This object will hold a pointer back
          | to the fifo, so make sure that the fifo
          | outlives this object.
          |
          */
        pub fn new_from_abstract_fifo(
            f:   &mut AbstractFifo,
            num: i32) -> Self {
        
            todo!();
            /*
            : fifo(&f),

                prepare (*fifo, num);
            */
        }

        /**
          | Calls the passed function with each
          | index that was deemed valid for the current
          | read/write operation.
          |
          */
        pub fn for_each<FunctionToApply>(&self, func: FunctionToApply)  {
        
            todo!();
            /*
                for (auto i = startIndex1, e = startIndex1 + blockSize1; i != e; ++i)  func (i);
                    for (auto i = startIndex2, e = startIndex2 + blockSize2; i != e; ++i)  func (i);
            */
        }

        pub fn new_from_other(other: ScopedReadWrite<MODE>) -> Self {
        
            todo!();
            /*
            : start_index1(other.startIndex1),
            : block_size1(other.blockSize1),
            : start_index2(other.startIndex2),
            : block_size2(other.blockSize2),

                swap (other);
            */
        }
        
        pub fn assign_from(&mut self, other: ScopedReadWrite<MODE>) -> &mut abstract_fifo::ScopedReadWrite<MODE> {
            
            todo!();
            /*
                swap (other);
            return *this;
            */
        }
        
        pub fn swap(&mut self, other: &mut ScopedReadWrite<MODE>)  {
            
            todo!();
            /*
                std::swap (other.fifo, fifo);
            std::swap (other.startIndex1, startIndex1);
            std::swap (other.blockSize1, blockSize1);
            std::swap (other.startIndex2, startIndex2);
            std::swap (other.blockSize2, blockSize2);
            */
        }
    }

    impl ScopedReadWrite<{ReadOrWrite::read}> {

        #[inline] pub fn finish(&mut self, 
            f:   &mut AbstractFifo,
            num: i32)  {
            
            todo!();
            /*
                f.finishedRead (num);
            */
        }
        
        #[inline] pub fn prepare(&mut self, 
            f:   &mut AbstractFifo,
            num: i32)  {
            
            todo!();
            /*
                f.prepareToRead (num, startIndex1, blockSize1, startIndex2, blockSize2);
            */
        }
    }

    impl ScopedReadWrite<{ReadOrWrite::write}> {

        #[inline] pub fn finish(&mut self, 
            f:   &mut AbstractFifo,
            num: i32)  {
            
            todo!();
            /*
                f.finishedWrite (num);
            */
        }
        
        #[inline] pub fn prepare(&mut self, 
            f:   &mut AbstractFifo,
            num: i32)  {
            
            todo!();
            /*
                f.prepareToWrite (num, startIndex1, blockSize1, startIndex2, blockSize2);
            */
        }
    }

    pub type ScopedRead  = ScopedReadWrite<{ReadOrWrite::read}>;
    pub type ScopedWrite = ScopedReadWrite<{ReadOrWrite::write}>;
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/containers/aloe_AbstractFifo.cpp]
impl AbstractFifo {
    
    /**
      | Creates a FIFO to manage a buffer with
      | the specified capacity.
      |
      */
    pub fn new_with_capacity(capacity: i32) -> Self {
    
        todo!();
        /*
        : buffer_size(capacity),

            jassert (bufferSize > 0);
        */
    }
    
    /**
      | Returns the total size of the buffer
      | being managed.
      |
      */
    pub fn get_total_size(&self) -> i32 {
        
        todo!();
        /*
            return bufferSize;
        */
    }
    
    /**
      | Returns the number of items that can
      | currently be added to the buffer without
      | it overflowing.
      |
      */
    pub fn get_free_space(&self) -> i32 {
        
        todo!();
        /*
            return bufferSize - getNumReady() - 1;
        */
    }
    
    /**
      | Returns the number of items that can
      | currently be read from the buffer.
      |
      */
    pub fn get_num_ready(&self) -> i32 {
        
        todo!();
        /*
            auto vs = validStart.get();
        auto ve = validEnd.get();
        return ve >= vs ? (ve - vs) : (bufferSize - (vs - ve));
        */
    }
    
    /**
      | Clears the buffer positions, so that
      | it appears empty.
      |
      */
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            validEnd = 0;
        validStart = 0;
        */
    }
    
    /**
      | Changes the buffer's total size.
      | 
      | -----------
      | @note
      | 
      | that this isn't thread-safe, so don't
      | call it if there's any danger that it
      | might overlap with a call to any other
      | method in this class!
      |
      */
    pub fn set_total_size(&mut self, new_size: i32)  {
        
        todo!();
        /*
            jassert (newSize > 0);
        reset();
        bufferSize = newSize;
        */
    }

    /**
      | Returns the location within the buffer
      | at which an incoming block of data should
      | be written.
      | 
      | Because the section of data that you
      | want to add to the buffer may overlap
      | the end and wrap around to the start,
      | two blocks within your buffer are returned,
      | and you should copy your data into the
      | first one, with any remaining data spilling
      | over into the second.
      | 
      | If the number of items you ask for is too
      | large to fit within the buffer's free
      | space, then blockSize1 + blockSize2
      | may add up to a lower value than numToWrite.
      | If this happens, you may decide to keep
      | waiting and re-trying the method until
      | there's enough space available.
      | 
      | After calling this method, if you choose
      | to write your data into the blocks returned,
      | you must call finishedWrite() to tell
      | the FIFO how much data you actually added.
      | 
      | e.g.
      | 
      | 
      | @code
      | void addToFifo (const int* someData, int numItems)
      | {
      |     int start1, size1, start2, size2;
      |     prepareToWrite (numItems, start1, size1, start2, size2);
      |
      |     if (size1 > 0)
      |         copySomeData (myBuffer + start1, someData, size1);
      |
      |     if (size2 > 0)
      |         copySomeData (myBuffer + start2, someData + size1, size2);
      |
      |     finishedWrite (size1 + size2);
      | }
      | @endcode
      | 
      | -----------
      | @param numToWrite
      | 
      | indicates how many items you'd like
      | to add to the buffer
      | ----------
      | @param startIndex1
      | 
      | on exit, this will contain the start
      | index in your buffer at which your data
      | should be written
      | ----------
      | @param blockSize1
      | 
      | on exit, this indicates how many items
      | can be written to the block starting
      | at startIndex1
      | ----------
      | @param startIndex2
      | 
      | on exit, this will contain the start
      | index in your buffer at which any data
      | that didn't fit into the first block
      | should be written
      | ----------
      | @param blockSize2
      | 
      | on exit, this indicates how many items
      | can be written to the block starting
      | at startIndex2
      | 
      | @see finishedWrite
      |
      */
    pub fn prepare_to_write(&self, 
        num_to_write: i32,
        start_index1: &mut i32,
        block_size1:  &mut i32,
        start_index2: &mut i32,
        block_size2:  &mut i32)  {
        
        todo!();
        /*
            auto vs = validStart.get();
        auto ve = validEnd.get();

        auto freeSpace = ve >= vs ? (bufferSize - (ve - vs)) : (vs - ve);
        numToWrite = jmin (numToWrite, freeSpace - 1);

        if (numToWrite <= 0)
        {
            startIndex1 = 0;
            startIndex2 = 0;
            blockSize1 = 0;
            blockSize2 = 0;
        }
        else
        {
            startIndex1 = ve;
            startIndex2 = 0;
            blockSize1 = jmin (bufferSize - ve, numToWrite);
            numToWrite -= blockSize1;
            blockSize2 = numToWrite <= 0 ? 0 : jmin (numToWrite, vs);
        }
        */
    }
    
    /**
      | Called after writing from the FIFO,
      | to indicate that this many items have
      | been added. @see prepareToWrite
      |
      */
    pub fn finished_write(&mut self, num_written: i32)  {
        
        todo!();
        /*
            jassert (numWritten >= 0 && numWritten < bufferSize);

        auto newEnd = validEnd.get() + numWritten;

        if (newEnd >= bufferSize)
            newEnd -= bufferSize;

        validEnd = newEnd;
        */
    }
        /**
      | Returns the location within the buffer
      | from which the next block of data should
      | be read.
      | 
      | Because the section of data that you
      | want to read from the buffer may overlap
      | the end and wrap around to the start,
      | two blocks within your buffer are returned,
      | and you should read from both of them.
      | 
      | If the number of items you ask for is greater
      | than the amount of data available, then
      | blockSize1 + blockSize2 may add up to
      | a lower value than numWanted. If this
      | happens, you may decide to keep waiting
      | and re-trying the method until there's
      | enough data available.
      | 
      | After calling this method, if you choose
      | to read the data, you must call finishedRead()
      | to tell the FIFO how much data you have
      | consumed.
      | 
      | e.g.
      | 
      | @code
      | void readFromFifo (int* someData, int numItems)
      | {
      |     int start1, size1, start2, size2;
      |     prepareToRead (numSamples, start1, size1, start2, size2);
      |
      |     if (size1 > 0)
      |         copySomeData (someData, myBuffer + start1, size1);
      |
      |     if (size2 > 0)
      |         copySomeData (someData + size1, myBuffer + start2, size2);
      |
      |     finishedRead (size1 + size2);
      | }
      | @endcode
      | 
      | -----------
      | @param numWanted
      | 
      | indicates how many items you'd like
      | to add to the buffer
      | ----------
      | @param startIndex1
      | 
      | on exit, this will contain the start
      | index in your buffer at which your data
      | should be written
      | ----------
      | @param blockSize1
      | 
      | on exit, this indicates how many items
      | can be written to the block starting
      | at startIndex1
      | ----------
      | @param startIndex2
      | 
      | on exit, this will contain the start
      | index in your buffer at which any data
      | that didn't fit into the first block
      | should be written
      | ----------
      | @param blockSize2
      | 
      | on exit, this indicates how many items
      | can be written to the block starting
      | at startIndex2
      | 
      | @see finishedRead
      |
      */

    pub fn prepare_to_read(&self, 
        num_wanted:   i32,
        start_index1: &mut i32,
        block_size1:  &mut i32,
        start_index2: &mut i32,
        block_size2:  &mut i32)  {
        
        todo!();
        /*
            auto vs = validStart.get();
        auto ve = validEnd.get();

        auto numReady = ve >= vs ? (ve - vs) : (bufferSize - (vs - ve));
        numWanted = jmin (numWanted, numReady);

        if (numWanted <= 0)
        {
            startIndex1 = 0;
            startIndex2 = 0;
            blockSize1 = 0;
            blockSize2 = 0;
        }
        else
        {
            startIndex1 = vs;
            startIndex2 = 0;
            blockSize1 = jmin (bufferSize - vs, numWanted);
            numWanted -= blockSize1;
            blockSize2 = numWanted <= 0 ? 0 : jmin (numWanted, ve);
        }
        */
    }
    
    /**
      | Called after reading from the FIFO,
      | to indicate that this many items have
      | now been consumed. @see prepareToRead
      |
      */
    pub fn finished_read(&mut self, num_read: i32)  {
        
        todo!();
        /*
            jassert (numRead >= 0 && numRead <= bufferSize);

        auto newStart = validStart.get() + numRead;

        if (newStart >= bufferSize)
            newStart -= bufferSize;

        validStart = newStart;
        */
    }
    
    /** 
      | Replaces prepareToRead/finishedRead with
      | a single function.
      |
      | This function returns an object which
      | contains the start indices and block
      | sizes, and also automatically finishes the
      | read operation when it goes out of scope.
      |
      |   @code
      |   {
      |       auto readHandle = fifo.read (4);
      |
      |       for (auto i = 0; i != readHandle.blockSize1; ++i)
      |       {
      |           // read the item at index readHandle.startIndex1 + i
      |       }
      |
      |       for (auto i = 0; i != readHandle.blockSize2; ++i)
      |       {
      |           // read the item at index readHandle.startIndex2 + i
      |       }
      |   } // readHandle goes out of scope here, finishing the read operation
      |   @endcode
      */
    pub fn read(&mut self, num_to_read: i32) -> abstract_fifo::ScopedRead {
        
        todo!();
        /*
            return { *this, numToRead };
        */
    }
    
    /** 
      | Replaces prepareToWrite/finishedWrite with
      | a single function.
      |
      | This function returns an object which
      | contains the start indices and block
      | sizes, and also automatically finishes the
      | write operation when it goes out of scope.
      |
      |   @code
      |   {
      |       auto writeHandle = fifo.write (5);
      |
      |       for (auto i = 0; i != writeHandle.blockSize1; ++i)
      |       {
      |           // write the item at index writeHandle.startIndex1 + i
      |       }
      |
      |       for (auto i = 0; i != writeHandle.blockSize2; ++i)
      |       {
      |           // write the item at index writeHandle.startIndex2 + i
      |       }
      |   } // writeHandle goes out of scope here, finishing the write operation
      |   @endcode
      */
    pub fn write(&mut self, num_to_write: i32) -> abstract_fifo::ScopedWrite {
        
        todo!();
        /*
            return { *this, numToWrite };
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct AbstractFifoTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
pub mod abstract_fifo_tests {
    use super::*;

    pub struct WriteThread {
        base:   Thread,
        fifo:   &mut AbstractFifo,
        buffer: *mut i32,
        random: Random,
    }

    impl Drop for WriteThread {
        fn drop(&mut self) {
            todo!();
            /* 
                    stopThread (5000);
                 */
        }
    }

    impl WriteThread {

        pub fn new_from_abstract_fifo(
            f:   &mut AbstractFifo,
            b:   *mut i32,
            rng: Random) -> Self {
        
            todo!();
            /*


                : Thread ("fifo writer"), fifo (f), buffer (b), random (rng)
                    startThread();
            */
        }
        
        pub fn run(&mut self)  {
            
            todo!();
            /*
                int n = 0;

                    while (! threadShouldExit())
                    {
                        int num = random.nextInt (2000) + 1;

                        auto writer = fifo.write (num);

                        jassert (writer.blockSize1 >= 0 && writer.blockSize2 >= 0);
                        jassert (writer.blockSize1 == 0
                                 || (writer.startIndex1 >= 0 && writer.startIndex1 < fifo.getTotalSize()));
                        jassert (writer.blockSize2 == 0
                                 || (writer.startIndex2 >= 0 && writer.startIndex2 < fifo.getTotalSize()));

                        writer.forEach ([this, &n] (int index)  { this->buffer[index] = n++; });
                    }
            */
        }
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for AbstractFifoTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Abstract Fifo", UnitTestCategories::containers
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl AbstractFifoTests {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("AbstractFifo");

            int buffer[5000];
            AbstractFifo fifo (numElementsInArray (buffer));

            WriteThread writer (fifo, buffer, getRandom());

            int n = 0;
            Random r = getRandom();
            r.combineSeed (12345);

            for (int count = 100000; --count >= 0;)
            {
                int num = r.nextInt (6000) + 1;

                auto reader = fifo.read (num);

                if (! (reader.blockSize1 >= 0 && reader.blockSize2 >= 0)
                        && (reader.blockSize1 == 0
                            || (reader.startIndex1 >= 0 && reader.startIndex1 < fifo.getTotalSize()))
                        && (reader.blockSize2 == 0
                            || (reader.startIndex2 >= 0 && reader.startIndex2 < fifo.getTotalSize())))
                {
                    expect (false, "prepareToRead returned -ve values");
                    break;
                }

                bool failed = false;

                reader.forEach ([&failed, &buffer, &n] (int index)
                {
                    failed = (buffer[index] != n++) || failed;
                });

                if (failed)
                {
                    expect (false, "read values were incorrect");
                    break;
                }
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static AbstractFifoTests fifoUnitTests;
    */
}
