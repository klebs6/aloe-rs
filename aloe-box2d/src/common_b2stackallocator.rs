crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2StackAllocator.h]

pub const b2_stackSize:       usize = 100 * 1024; // 100k
pub const b2_maxStackEntries: usize = 32;

pub struct b2StackEntry
{
    data:        *mut u8,
    size:        i32,
    used_malloc: bool,
}

/**
  | This is a stack allocator used for fast per
  | step allocations.  You must nest allocate/free
  | pairs. The code will assert if you try to
  | interleave multiple allocate/free pairs.
  */
pub struct b2StackAllocator {
    data:           [u8; b2_stackSize],
    index:          i32,
    allocation:     i32,
    max_allocation: i32,
    entries:        [b2StackEntry; b2_maxStackEntries],
    entry_count:    i32,
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_box2d/box2d/Common/b2StackAllocator.cpp]
impl Drop for b2StackAllocator {
    fn drop(&mut self) {
        todo!();
        /* 
        b2Assert(m_index == 0);
        b2Assert(m_entryCount == 0);
 */
    }
}

impl Default for b2StackAllocator {

    fn default() -> Self {
    
        todo!();
        /*


            m_index = 0;
        m_allocation = 0;
        m_maxAllocation = 0;
        m_entryCount = 0;
        */
    }
}

impl b2StackAllocator {
    
    pub fn allocate(&mut self, size: i32)  {
        
        todo!();
        /*
            b2Assert(m_entryCount < b2_maxStackEntries);

        b2StackEntry* entry = m_entries + m_entryCount;
        entry->size = size;
        if (m_index + size > b2_stackSize)
        {
            entry->data = (char*)b2Alloc(size);
            entry->usedMalloc = true;
        }
        else
        {
            entry->data = m_data + m_index;
            entry->usedMalloc = false;
            m_index += size;
        }

        m_allocation += size;
        m_maxAllocation = b2Max(m_maxAllocation, m_allocation);
        ++m_entryCount;

        return entry->data;
        */
    }
    
    pub fn free(&mut self, p: *mut c_void)  {
        
        todo!();
        /*
            b2Assert(m_entryCount > 0);
        b2StackEntry* entry = m_entries + m_entryCount - 1;
        b2Assert(p == entry->data);
        if (entry->usedMalloc)
        {
            b2Free(p);
        }
        else
        {
            m_index -= entry->size;
        }
        m_allocation -= entry->size;
        --m_entryCount;

        p = NULL;
        */
    }
    
    pub fn get_max_allocation(&self) -> i32 {
        
        todo!();
        /*
            return m_maxAllocation;
        */
    }
}
