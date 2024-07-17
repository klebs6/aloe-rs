crate::ix!();

pub struct Queue<Element> {
    fifo:    AbstractFifo,
    storage: Vec<Element>,
}

impl<Element> Queue<Element> {

    pub fn new(size: i32) -> Self {
    
        todo!();
        /*


            : fifo (size), storage (static_cast<size_t> (size))
        */
    }
    
    pub fn push(&mut self, element: &mut Element) -> bool {
        
        todo!();
        /*
            if (fifo.getFreeSpace() == 0)
                return false;

            const auto writer = fifo.write (1);

            if (writer.blockSize1 != 0)
                storage[static_cast<size_t> (writer.startIndex1)] = std::move (element);
            else if (writer.blockSize2 != 0)
                storage[static_cast<size_t> (writer.startIndex2)] = std::move (element);

            return true;
        */
    }
    
    pub fn pop<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            popN (1, std::forward<Fn> (fn));
        */
    }
    
    pub fn pop_all<Fn>(&mut self, fn_: Fn)  {
    
        todo!();
        /*
            popN (fifo.getNumReady(), std::forward<Fn> (fn));
        */
    }
    
    pub fn has_pending_messages(&self) -> bool {
        
        todo!();
        /*
            return fifo.getNumReady() > 0;
        */
    }
    
    pub fn popn<Fn>(&mut self, n: i32, fn_: Fn)  {
    
        todo!();
        /*
            fifo.read (n).forEach ([&] (int index)
                                   {
                                       fn (storage[static_cast<size_t> (index)]);
                                   });
        */
    }
}
