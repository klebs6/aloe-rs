crate::ix!();

pub const minNumberOfStringsForGarbageCollection: i32 = 300;
pub const garbageCollectionInterval:              u32 = 30000;

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringPool.h]

/**
  | A StringPool holds a set of shared strings,
  | which reduces storage overheads and
  | improves comparison speed when dealing
  | with many duplicate strings.
  | 
  | When you add a string to a pool using getPooledString,
  | it'll return a character array containing
  | the same string. This array is owned
  | by the pool, and the same array is returned
  | every time a matching string is asked
  | for. This means that it's trivial to
  | compare two pooled strings for equality,
  | as you can simply compare their pointers.
  | It also cuts down on storage if you're
  | using many copies of the same string.
  | 
  | @tags{Core}
  |
  */
#[no_copy]
pub struct StringPool {
    lock: Mutex<StringPoolInner>, //was aloe::CriticalSection
}

struct StringPoolInner {
    strings:                      Vec<String>,
    last_garbage_collection_time: u32,
}

impl Default for StringPool {

    /**
      | Creates an empty pool.
      |
      */
    fn default() -> Self {

        todo!();
        /*
           : last_garbage_collection_time(0),


           */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_StringPool.cpp]
impl StringPool {

    /**
      | Returns a pointer to a shared copy of
      | the string that is passed in.
      | 
      | The pool will always return the same
      | String object when asked for a string
      | that matches it.
      |
      */
    pub fn get_pooled_string_from_raw(&mut self, new_string: *const u8) -> String {
        
        todo!();
        /*
            if (newString == nullptr || *newString == 0)
            return {};

        const ScopedLock sl (lock);
        garbageCollectIfNeeded();
        return addPooledString (strings, CharPointer_UTF8 (newString));
        */
    }
    
    /**
      | Returns a pointer to a copy of the string
      | that is passed in.
      | 
      | The pool will always return the same
      | String object when asked for a string
      | that matches it.
      |
      */
    pub fn get_pooled_string_from_range(
        &mut self, 
        start: CharPointerType,
        end:   CharPointerType) -> String 
    {
        todo!();

        /*
            if (start.isEmpty() || start == end)
            return {};

        const ScopedLock sl (lock);
        garbageCollectIfNeeded();
        return addPooledString (strings, StartEndString (start, end));
        */
    }
    
    /**
      | Returns a pointer to a shared copy of
      | the string that is passed in.
      | 
      | The pool will always return the same
      | String object when asked for a string
      | that matches it.
      |
      */
    pub fn get_pooled_string_from_str(&mut self, new_string: &str) -> String {
        
        todo!();
        /*
            if (newString.isEmpty())
            return {};

        const ScopedLock sl (lock);
        garbageCollectIfNeeded();
        return addPooledString (strings, newString.text);
        */
    }
    
    /**
      | Returns a pointer to a copy of the string
      | that is passed in.
      | 
      | The pool will always return the same
      | String object when asked for a string
      | that matches it.
      |
      */
    pub fn get_pooled_string_from_string(&mut self, new_string: &String) -> String {
        
        todo!();
        /*
            if (newString.isEmpty())
            return {};

        const ScopedLock sl (lock);
        garbageCollectIfNeeded();
        return addPooledString (strings, newString);
        */
    }
    
    pub fn garbage_collect_if_needed(&mut self)  {
        
        todo!();
        /*
            if (strings.size() > minNumberOfStringsForGarbageCollection
             && Time::getApproximateMillisecondCounter() > lastGarbageCollectionTime + garbageCollectionInterval)
            garbageCollect();
        */
    }
    
    /**
      | Scans the pool, and removes any strings
      | that are unreferenced.
      | 
      | You don't generally need to call this
      | - it'll be called automatically when
      | the pool grows large enough to warrant
      | it.
      |
      */
    pub fn garbage_collect(&mut self)  {
        
        todo!();
        /*
            const ScopedLock sl (lock);

        for (int i = strings.size(); --i >= 0;)
            if (strings.getReference(i).getReferenceCount() == 1)
                strings.remove (i);

        lastGarbageCollectionTime = Time::getApproximateMillisecondCounter();
        */
    }
    
    /**
      | Returns a shared global pool which is
      | used for things like Identifiers, XML
      | parsing.
      |
      */
    pub fn get_global_pool(&mut self) -> &mut StringPool {
        
        todo!();
        /*
            static StringPool pool;
        return pool;
        */
    }
}

pub struct StartEndString {
    start: CharPointerType,
    end:   CharPointerType,
}

impl StartEndString {

    pub fn new(
        s: CharPointerType,
        e: CharPointerType) -> Self {
    
        todo!();
        /*
        : start(s),
        : end(e),

        
        */
    }
}

impl Into<String> for StartEndString {
    
    #[inline] fn into(self) -> String {
        todo!();
        /*
            return String (start, end);
        */
    }
}

pub fn compare_strings(
        string1: &str,
        string2: &str) -> i32 {
    
    todo!();
    /*
        CharPointerType s1 (string1.start), s2 (string2.getCharPointer());

        for (;;)
        {
            const int c1 = s1 < string1.end ? (int) s1.getAndAdvance() : 0;
            const int c2 = (int) s2.getAndAdvance();
            const int diff = c1 - c2;

            if (diff != 0)  return diff < 0 ? -1 : 1;
            if (c1 == 0)    break;
        }

        return 0;
    */
}

pub fn add_pooled_string<NewStringType>(
    strings:    &mut Vec<String>,
    new_string: &NewStringType
) -> String 
{
    todo!();

    /*
        int start = 0;
        int end = strings.size();

        while (start < end)
        {
            const String& startString = strings.getReference (start);
            const int startComp = compareStrings (newString, startString);

            if (startComp == 0)
                return startString;

            const int halfway = (start + end) / 2;

            if (halfway == start)
            {
                if (startComp > 0)
                    ++start;

                break;
            }

            const String& halfwayString = strings.getReference (halfway);
            const int halfwayComp = compareStrings (newString, halfwayString);

            if (halfwayComp == 0)
                return halfwayString;

            if (halfwayComp > 0)
                start = halfway;
            else
                end = halfway;
        }

        strings.insert (start, newString);
        return strings.getReference (start);
    */
}


