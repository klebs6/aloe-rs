crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_TextDiff.h]

/**
  | Calculates and applies a sequence of
  | changes to convert one text string into
  | another.
  | 
  | Once created, the TextDiff object contains
  | an array of change objects, where each
  | change can be either an insertion or
  | a deletion. When applied in order to
  | the original string, these changes
  | will convert it to the target string.
  | 
  | @tags{Core}
  |
  */
pub struct TextDiff {

    /**
      | The list of changes required to perform
      | the transformation.
      | 
      | Applying each of these, in order, to
      | the original string will produce the
      | target.
      |
      */
    changes: Vec<text_diff::Change>,
}

pub mod text_diff {

    use super::*;

    /**
      | Describes a change, which can be either
      | an insertion or deletion.
      |
      */
    pub struct Change {

        /**
          | If this change is a deletion, this string
          | will be empty; otherwise, it'll be the text
          | that should be inserted at the index
          | specified by start.
          */
        inserted_text: String,

        /**
          | Specifies the character index in a string
          | at which text should be inserted or
          | deleted.
          */
        start:         i32,

        /**
          | If this change is a deletion, this
          | specifies the number of characters to
          | delete. For an insertion, this is the length
          | of the new text being inserted.
          */
        length:        i32,
    }

    impl Change {

        /**
          | Returns true if this change is a deletion,
          | or false for an insertion.
          |
          */
        pub fn is_deletion(&self) -> bool {
            
            todo!();
            /*
                return insertedText.isEmpty();
            */
        }
        
        /**
          | Returns the result of applying this
          | change to a string.
          |
          */
        pub fn applied_to(&self, text: &String) -> String {
            
            todo!();
            /*
                return text.replaceSection (start, length, insertedText);
            */
        }
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/text/aloe_TextDiff.cpp]
impl TextDiff {

    /**
      | Creates a set of diffs for converting
      | the original string into the target.
      |
      */
    pub fn new(
        original: &String,
        target:   &String) -> Self {
    
        todo!();
        /*
            TextDiffHelpers::diffSkippingCommonStart (*this, original, target);
        */
    }
    
    /**
      | Applies this sequence of changes to
      | the original string, producing the
      | target string that was specified when
      | generating them.
      | 
      | Obviously it only makes sense to call
      | this function with the string that was
      | originally passed to the constructor.
      | Any other input will produce an undefined
      | result.
      |
      */
    pub fn applied_to(&self, text: String) -> String {
        
        todo!();
        /*
            for (auto& c : changes)
            text = c.appliedTo (text);

        return text;
        */
    }
}

pub struct TextDiffHelpers {

}

pub mod text_diff_helpers {

    pub const MIN_LENGTH_TO_MATCH: usize = 3;
    pub const MAX_COMPLEXITY:      usize = 16 * 1024 * 1024;
}

impl TextDiffHelpers {

    pub fn add_insertion(
        td:     &mut TextDiff,
        text:   CharPointerType,
        index:  i32,
        length: i32)  {
        
        todo!();
        /*
            TextDiff::Change c;
            c.insertedText = String (text, (size_t) length);
            c.start = index;
            c.length = 0;
            td.changes.add (c);
        */
    }
    
    pub fn add_deletion(
        td:     &mut TextDiff,
        index:  i32,
        length: i32)  {
        
        todo!();
        /*
            TextDiff::Change c;
            c.start = index;
            c.length = length;
            td.changes.add (c);
        */
    }
    
    pub fn diff_skipping_common_start(
        td: &mut TextDiff,
        a:  StringRegion,
        b:  StringRegion)  {
        
        todo!();
        /*
            for (;;)
            {
                auto ca = *a.text;
                auto cb = *b.text;

                if (ca != cb || ca == 0)
                    break;

                a.incrementStart();
                b.incrementStart();
            }

            diffRecursively (td, a, b);
        */
    }
    
    pub fn diff_recursively(
        td: &mut TextDiff,
        a:  StringRegion,
        b:  StringRegion)  {
        
        todo!();
        /*
            int indexA = 0, indexB = 0;
            auto len = findLongestCommonSubstring (a.text, a.length, indexA,
                                                   b.text, b.length, indexB);

            if (len >= MIN_LENGTH_TO_MATCH)
            {
                if (indexA > 0 && indexB > 0)
                    diffSkippingCommonStart (td, StringRegion (a.text, a.start, indexA),
                                                 StringRegion (b.text, b.start, indexB));
                else if (indexA > 0)
                    addDeletion (td, b.start, indexA);
                else if (indexB > 0)
                    addInsertion (td, b.text, b.start, indexB);

                diffRecursively (td, StringRegion (a.text + (indexA + len), a.start + indexA + len, a.length - indexA - len),
                                     StringRegion (b.text + (indexB + len), b.start + indexB + len, b.length - indexB - len));
            }
            else
            {
                if (a.length > 0)   addDeletion (td, b.start, a.length);
                if (b.length > 0)   addInsertion (td, b.text, b.start, b.length);
            }
        */
    }
    
    pub fn find_longest_common_substring(
        a:         CharPointerType,
        lena:      i32,
        index_ina: &mut i32,
        b:         CharPointerType,
        lenb:      i32,
        index_inb: &mut i32) -> i32 {
        
        todo!();
        /*
            if (lenA == 0 || lenB == 0)
                return 0;

            if (lenA * lenB > MAX_COMPLEXITY)
                return findCommonSuffix (a, lenA, indexInA,
                                         b, lenB, indexInB);

            auto scratchSpace = sizeof (int) * (2 + 2 * (size_t) lenB);

            if (scratchSpace < 4096)
            {
                ALOE_BEGIN_IGNORE_WARNINGS_MSVC (6255)
                auto* scratch = (int*) alloca (scratchSpace);
                ALOE_END_IGNORE_WARNINGS_MSVC

                return findLongestCommonSubstring (a, lenA, indexInA, b, lenB, indexInB, scratchSpace, scratch);
            }

            HeapBlock<int> scratch (scratchSpace);
            return findLongestCommonSubstring (a, lenA, indexInA, b, lenB, indexInB, scratchSpace, scratch);
        */
    }
    
    pub fn find_longest_common_substring_with_scratch(
        a:             CharPointerType,
        lena:          i32,
        index_ina:     &mut i32,
        b:             CharPointerType,
        lenb:          i32,
        index_inb:     &mut i32,
        scratch_space: usize,
        lines:         *mut i32) -> i32 {
        
        todo!();
        /*
            zeromem (lines, scratchSpace);

            auto* l0 = lines;
            auto* l1 = l0 + lenB + 1;

            int loopsWithoutImprovement = 0;
            int bestLength = 0;

            for (int i = 0; i < lenA; ++i)
            {
                auto ca = a.getAndAdvance();
                auto b2 = b;

                for (int j = 0; j < lenB; ++j)
                {
                    if (ca != b2.getAndAdvance())
                    {
                        l1[j + 1] = 0;
                    }
                    else
                    {
                        auto len = l0[j] + 1;
                        l1[j + 1] = len;

                        if (len > bestLength)
                        {
                            loopsWithoutImprovement = 0;
                            bestLength = len;
                            indexInA = i;
                            indexInB = j;
                        }
                    }
                }

                if (++loopsWithoutImprovement > 100)
                    break;

                std::swap (l0, l1);
            }

            indexInA -= bestLength - 1;
            indexInB -= bestLength - 1;
            return bestLength;
        */
    }
    
    pub fn find_common_suffix(
        a:         CharPointerType,
        lena:      i32,
        index_ina: &mut i32,
        b:         CharPointerType,
        lenb:      i32,
        index_inb: &mut i32) -> i32 {
        
        todo!();
        /*
            int length = 0;
            a += lenA - 1;
            b += lenB - 1;

            while (length < lenA && length < lenB && *a == *b)
            {
                --a;
                --b;
                ++length;
            }

            indexInA = lenA - length;
            indexInB = lenB - length;
            return length;
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct DiffTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for DiffTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("TextDiff class", UnitTestCategories::text
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl DiffTests {

    pub fn create_string(r: &mut Random) -> String {
        
        todo!();
        /*
            aloe_wchar buffer[500] = { 0 };

            for (int i = r.nextInt (numElementsInArray (buffer) - 1); --i >= 0;)
            {
                if (r.nextInt (10) == 0)
                {
                    do
                    {
                        buffer[i] = (aloe_wchar) (1 + r.nextInt (0x10ffff - 1));
                    }
                    while (! CharPointer_UTF16::canRepresent (buffer[i]));
                }
                else
                    buffer[i] = (aloe_wchar) ('a' + r.nextInt (3));
            }

            return CharPointer_UTF32 (buffer);
        */
    }
    
    pub fn test_diff(&mut self, 
        a: &String,
        b: &String)  {
        
        todo!();
        /*
            TextDiff diff (a, b);
            auto result = diff.appliedTo (a);
            expectEquals (result, b);
        */
    }
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("TextDiff");

            auto r = getRandom();

            testDiff (String(), String());
            testDiff ("x", String());
            testDiff (String(), "x");
            testDiff ("x", "x");
            testDiff ("x", "y");
            testDiff ("xxx", "x");
            testDiff ("x", "xxx");

            for (int i = 1000; --i >= 0;)
            {
                auto s = createString (r);
                testDiff (s, createString (r));
                testDiff (s + createString (r), s + createString (r));
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static DiffTests diffTests;
    */
}
