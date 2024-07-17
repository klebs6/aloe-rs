crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2GrowableStack.h]

/**
  | This is a growable LIFO stack with an initial
  | capacity of N.  If the stack size exceeds the
  | initial capacity, the heap is used to increase
  | the size of the stack.
  */
pub struct b2GrowableStack<T,const N: usize> {
    stack:    *mut T,
    array:    [T; N],
    count:    i32,
    capacity: i32,
}

impl<T,const N: usize> Default for b2GrowableStack<T,N> {
    
    fn default() -> Self {
        todo!();
        /*


            m_stack = m_array;
            m_count = 0;
            m_capacity = N
        */
    }
}

impl<T,const N: usize> Drop for b2GrowableStack<T,N> {
    fn drop(&mut self) {
        todo!();
        /* 
            if (m_stack != m_array)
            {
                b2Free(m_stack);
                m_stack = NULL;
            }
         */
    }
}

impl<T,const N: usize> b2GrowableStack<T,N> {

    pub fn push(&mut self, element: &T)  {
        
        todo!();
        /*
            if (m_count == m_capacity)
            {
                T* old = m_stack;
                m_capacity *= 2;
                m_stack = (T*)b2Alloc(m_capacity * sizeof(T));
                std::memcpy(m_stack, old, m_count * sizeof(T));
                if (old != m_array)
                {
                    b2Free(old);
                }
            }

            m_stack[m_count] = element;
            ++m_count;
        */
    }
    
    pub fn pop(&mut self) -> T {
        
        todo!();
        /*
            b2Assert(m_count > 0);
            --m_count;
            return m_stack[m_count];
        */
    }
    
    pub fn get_count(&mut self) -> i32 {
        
        todo!();
        /*
            return m_count;
        */
    }
}
