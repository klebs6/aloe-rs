crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/xml/aloe_XmlElement.h]

pub struct XmlElementIterator {
    element: <Self as HasValueType>::ValueType, // default = nullptr
}

impl Default for XmlElementIterator {

    fn default() -> Self {
        todo!();
    }
}

pub trait HasDifferenceType { type DifferenceType; }
pub trait HasValueType      { type ValueType; }
pub trait HasPointer        { type Pointer; }
pub trait HasReference      { type Reference; }
pub trait InputIteratorTag  { }

impl HasDifferenceType for XmlElementIterator {
    type DifferenceType   = libc::ptrdiff_t;
}

impl HasValueType for XmlElementIterator {
    type ValueType = *mut XmlElement;
}

impl HasPointer for XmlElementIterator {
    type Pointer = *const <Self as HasValueType>::ValueType;
}

impl HasReference for XmlElementIterator { 
    type Reference = <Self as HasValueType>::ValueType;
}

impl InputIteratorTag for XmlElementIterator {}

impl PartialEq<XmlElementIterator> for XmlElementIterator {
    
    #[inline] fn eq(&self, other: &XmlElementIterator) -> bool {
        todo!();
        /*
            return element == other.element;
        */
    }
}

impl Eq for XmlElementIterator {}

impl Deref for XmlElementIterator {

    type Target = <Self as HasPointer>::Pointer;

    #[inline] fn deref(&self) -> &Self::Target {
        todo!();
        /*
            return &element;
        */
    }
}

impl XmlElementIterator {

    pub fn new<Args>(
        e:    *mut XmlElement,
        args: Args) -> Self {
    
        todo!();
        /*

            : Traits (std::forward<Args> (args)...), element (e)
        */
    }
    
    pub fn begin(&self) -> XmlElementIterator {
        
        todo!();
        /*
            return *this;
        */
    }
    
    pub fn end(&self) -> XmlElementIterator {
        
        todo!();
        /*
            return XmlElementIterator{};
        */
    }
    
    pub fn prefix_increment(&mut self) -> &mut XmlElementIterator {
        
        todo!();
        /*
            element = Traits::getNext (*element);
                return *this;
        */
    }
    
    pub fn postfix_increment(&mut self) -> XmlElementIterator {
        
        todo!();
        /*
            auto copy = *this;
                ++(*this);
                return copy;
        */
    }
}

/** 
  | Used to build a tree of elements representing
  | an XML document.
  |
  | An XML document can be parsed into a tree of
  | XmlElements, each of which represents an XML
  | tag structure, and which may itself contain
  | other nested elements.
  |
  | An XmlElement can also be converted back into
  | a text document, and has lots of useful
  | methods for manipulating its attributes and
  | sub-elements, so XmlElements can actually be
  | used as a handy general-purpose data
  | structure.
  |
  | Here's an example of parsing some elements: @code
  | // check we're looking at the right kind of document..
  |
  |   if (myElement->hasTagName ("ANIMALS"))
  |   {
  |       // now we'll iterate its sub-elements looking for 'giraffe' elements..
  |       for (auto* e : myElement->getChildIterator())
  |       {
  |           if (e->hasTagName ("GIRAFFE"))
  |           {
  |               // found a giraffe, so use some of its attributes..
  |
  |               String giraffeName  = e->getStringAttribute ("name");
  |               int giraffeAge      = e->getIntAttribute ("age");
  |               bool isFriendly     = e->getBoolAttribute ("friendly");
  |           }
  |       }
  |   }
  |   @endcode
  |
  |   And here's an example of how to create an XML
  |   document from scratch: @code
  |
  |   // create an outer node called "ANIMALS"
  |   XmlElement animalsList ("ANIMALS");
  |
  |   for (int i = 0; i < numAnimals; ++i)
  |   {
  |       // create an inner element..
  |       XmlElement* giraffe = new XmlElement ("GIRAFFE");
  |
  |       giraffe->setAttribute ("name", "nigel");
  |       giraffe->setAttribute ("age", 10);
  |       giraffe->setAttribute ("friendly", true);
  |
  |       // ..and add our new element to the parent node
  |       animalsList.addChildElement (giraffe);
  |   }
  |
  |   // now we can turn the whole thing into textual XML
  |   auto xmlString = animalsList.toString();
  |   @endcode
  |
  |   @see parseXML, parseXMLIfTagMatches, XmlDocument
  |
  |   @tags{Core}
  */
#[leak_detector]
pub struct XmlElement {
    next_list_item:      LinkedListPointer<XmlElement>,
    first_child_element: LinkedListPointer<XmlElement>,
    attributes:          LinkedListPointer<xml_element::XmlAttributeNode>,
    tag_name:            String,
}

impl Drop for XmlElement {

    /**
      | Deleting an XmlElement will also delete
      | all of its child elements.
      |
      */
    fn drop(&mut self) {
        /* 
        firstChildElement.deleteAll();
        attributes.deleteAll();
         */
    }
}

pub mod xml_element {

    use super::*;

    /**
      | A struct containing options for formatting
      | the text when representing an XML element
      | as a string.
      |
      */
    pub struct TextFormat
    {

        /**
          | If supplied, this DTD will be added to
          | the document.
          |
          */
        dtd:                String,

        /**
          | If supplied, this header will be used
          | (and customEncoding & addDefaultHeader
          | will be ignored).
          |
          */
        custom_header:      String,

        /**
          | If not empty and addDefaultHeader is
          | true, this will be set as the encoding.
          | Otherwise, a default of "UTF-8" will
          | be used
          |
          */
        custom_encoding:    String,

        /**
          | If true, a default header will be generated;
          | otherwise just bare XML will be emitted.
          |
          */
        add_default_header: bool, // default = true

        /**
          | A maximum line length before wrapping
          | is done. (If newLineChars is nullptr,
          | this is ignored)
          |
          */
        line_wrap_length:   i32, // default = 60

        /**
          | Allows the newline characters to be
          | set. If you set this to nullptr, then
          | the whole XML document will be placed
          | on a single line.
          |
          */
        new_line_chars:     String, // default = "\r\n"
    }

    impl Default for TextFormat {

        fn default() -> Self {

            Self {
                dtd:                String::default(),
                custom_header:      String::default(),
                custom_encoding:    String::default(),
                add_default_header: true,
                line_wrap_length:   60,
                new_line_chars:     String::from("\r\n"),
            }
        }
    }

    impl TextFormat {

        /**
          | returns a copy of this format with newLineChars
          | set to nullptr.
          |
          */
        pub fn single_line(&self) -> TextFormat {
            
            todo!();
            /*
                auto f = *this;
            f.newLineChars = nullptr;
            return f;
            */
        }
        
        /**
          | returns a copy of this format with the
          | addDefaultHeader flag set to false.
          |
          */
        pub fn without_header(&self) -> TextFormat {
            
            todo!();
            /*
                auto f = *this;
            f.addDefaultHeader = false;
            return f;
            */
        }
    }
    
    ///----------------------------
    pub struct GetNextElement { }

    impl GetNextElement {

        pub fn get_next(&self, e: &XmlElement) -> *mut XmlElement {
            
            todo!();
            /*
                return e.getNextElement();
            */
        }
    }

    ///----------------------------
    #[derive(Default)]
    pub struct GetNextElementWithTagName {
        name: String,
    }

    impl GetNextElementWithTagName {

        pub fn new(n: String) -> Self {
        
            todo!();
            /*
            : name(std::move (n)),

            
            */
        }
        
        pub fn get_next(&self, e: &XmlElement) -> *mut XmlElement {
            
            todo!();
            /*
                return e.getNextElementWithTagName (name);
            */
        }
    }

    ///----------------------------

    pub struct XmlAttributeNode {
        next_list_item: LinkedListPointer<XmlAttributeNode>,
        name:           Identifier,
        value:          String,
    }

    impl XmlAttributeNode {

        pub fn new_from_other(other: &XmlAttributeNode) -> Self {
        
            todo!();
            /*
            : name (other.name),
              value (other.value)
            */
        }
        
        pub fn new_from_identifier_and_string(
            n: &Identifier,
            v: &String) -> Self {
        
            todo!();
            /*
            : name (n), value (v)
            jassert (isValidXmlName (name));
            */
        }
        
        pub fn new_from_range(
            name_start: CharPointerType,
            name_end:   CharPointerType) -> Self {
        
            todo!();
            /*
            : name (nameStart, nameEnd)
            jassert (isValidXmlName (name));
            */
        }
    }
}

impl XmlElement {

    /**
      | Returns this element's tag type name.
      | 
      | E.g. for an element such as \<MOOSE legs="4"
      | antlers="2">, this would return "MOOSE".
      | @see hasTagName
      |
      */
    pub fn get_tag_name(&self) -> &String {
        
        todo!();
        /*
            return tagName;
        */
    }

    /* --------- Attribute-handling methods..   --------- */

    /* ----------- Child element methods..   ----------- */

    /**
      | Returns the first of this element's
      | sub-elements. see getNextElement()
      | for an example of how to iterate the sub-elements.
      | 
      | @see getChildIterator
      |
      */
    pub fn get_first_child_element(&self) -> *mut XmlElement {
        
        todo!();
        /*
            return firstChildElement;
        */
    }

    /**
      | Returns the next of this element's siblings.
      | 
      | This can be used for iterating an element's
      | sub-elements, e.g.
      | @code
      | XmlElement* child = myXmlDocument->getFirstChildElement();
      |
      | while (child != nullptr)
      | {
      |     ...do stuff with this child..
      |
      |     child = child->getNextElement();
      | }
      | @endcode
      | 
      | Note that when iterating the child elements,
      | some of them might be text elements as
      | well as XML tags - use isTextElement()
      | to work this out.
      | 
      | Also, it's much easier and neater to
      | use this method indirectly via the getChildIterator()
      | method.
      | 
      | -----------
      | @return
      | 
      | the sibling element that follows this
      | one, or a nullptr if this is the last element
      | in its parent
      | 
      | @see getNextElement, isTextElement,
      | getChildIterator
      |
      */
    #[inline] pub fn get_next_element(&self) -> *mut XmlElement {
        
        todo!();
        /*
            return nextListItem;
        */
    }

    /**
      | Sorts the child elements using a comparator.
      | 
      | This will use a comparator object to
      | sort the elements into order. The object
      | passed must have a method of the form:
      | @code int compareElements (const XmlElement*
      | first, const XmlElement* second);
      | @endcode
      | 
      | ..and this method must return:
      | 
      | - a value of < 0 if the first comes before
      | the second
      | 
      | - a value of 0 if the two objects are equivalent
      | 
      | - a value of > 0 if the second comes before
      | the first
      | 
      | To improve performance, the compareElements()
      | method can be declared as static or const.
      | 
      | -----------
      | @param comparator
      | 
      | the comparator to use for comparing
      | elements.
      | ----------
      | @param retainOrderOfEquivalentItems
      | 
      | if this is true, then items which the
      | comparator says are equivalent will
      | be kept in the order in which they currently
      | appear in the array. This is slower to
      | perform, but may be important in some
      | cases. If it's false, a faster algorithm
      | is used, but equivalent elements may
      | be rearranged.
      |
      */
    pub fn sort_child_elements<ElementComparator>(
        &mut self, 
        comparator:                       &mut ElementComparator,
        retain_order_of_equivalent_items: Option<bool>)  {

        let retain_order_of_equivalent_items: bool =
            retain_order_of_equivalent_items.unwrap_or(false);

        todo!();
        /*
            auto num = getNumChildElements();

            if (num > 1)
            {
                HeapBlock<XmlElement*> elems (num);
                getChildElementsAsArray (elems);
                sortArray (comparator, (XmlElement**) elems, 0, num - 1, retainOrderOfEquivalentItems);
                reorderChildElements (elems, num);
            }
        */
    }

    /**
      | Allows iterating the children of an
      | XmlElement using range-for syntax.
      | 
      | @code void doSomethingWithXmlChildren
      | (const XmlElement& myParentXml) {
      | for (auto* element : myParentXml.getChildIterator())
      | doSomethingWithXmlElement (element);
      | } @endcode
      |
      */
    pub fn get_child_iterator(&self) -> XmlElementIterator /* <xml_element::GetNextElement> */ {
        
        todo!();
        /*
            return XmlElementIterator<GetNextElement> { getFirstChildElement() };
        */
    }

    /**
      | Allows iterating children of an XmlElement
      | with a specific tag using range-for
      | syntax.
      | 
      | @code void doSomethingWithXmlChildren
      | (const XmlElement& myParentXml) {
      | for (auto* element : myParentXml.getChildWithTagNameIterator
      | ("MYTAG")) doSomethingWithXmlElement
      | (element); } @endcode
      |
      */
    pub fn get_child_with_tag_name_iterator(&self, name: &str) -> XmlElementIterator /* <xml_element::GetNextElementWithTagName> */ {
        
        todo!();
        /*
            return XmlElementIterator<GetNextElementWithTagName> { getChildByName (name), name };
        */
    }
}

/** 
  | DEPRECATED: A handy macro to make it easy to
  | iterate all the child elements in an
  | XmlElement.
  |
  |    New code should avoid this macro, and instead
  |    use getChildIterator directly.
  |
  |    The parentXmlElement should be a reference to
  |    the parent XML, and the
  |    childElementVariableName will be the name of
  |    a pointer to each child element.
  |
  |    E.g. @code
  |    XmlElement* myParentXml = createSomeKindOfXmlDocument();
  |
  |    forEachXmlChildElement (*myParentXml, child)
  |    {
  |        if (child->hasTagName ("FOO"))
  |            doSomethingWithXmlElement (child);
  |    }
  |
  |    @endcode
  |
  |    @see forEachXmlChildElementWithTagName
  */
macro_rules! foreachxmlchildelement {
    ($parentXmlElement:ident, $childElementVariableName:ident) => {
        /*
        
            for (auto* (childElementVariableName) : ((parentXmlElement).macroBasedForLoop(), (parentXmlElement).getChildIterator()))
        */
    }
}

/** 
  | DEPRECATED: A macro that makes it easy to
  | iterate all the child elements of an XmlElement
  | which have a specified tag.
  |
  | New code should avoid this macro, and instead
  | use getChildWithTagNameIterator directly.
  |
  | This does the same job as the
  | forEachXmlChildElement macro, but only for
  | those elements that have a particular tag
  | name.
  |
  | The parentXmlElement should be a reference to
  | the parent XML, and the
  | childElementVariableName will be the name of
  | a pointer to each child element. The
  | requiredTagName is the tag name to match.
  |
  |   E.g. @code
  |   XmlElement* myParentXml = createSomeKindOfXmlDocument();
  |
  |   forEachXmlChildElementWithTagName (*myParentXml, child, "MYTAG")
  |   {
  |       // the child object is now guaranteed to be a <MYTAG> element..
  |       doSomethingWithMYTAGElement (child);
  |   }
  |
  |   @endcode
  |
  |   @see forEachXmlChildElement
  */
macro_rules! foreachxmlchildelementwithtagname {
    ($parentXmlElement:ident, $childElementVariableName:ident, $requiredTagName:ident) => {
        /*
        
            for (auto* (childElementVariableName) : ((parentXmlElement).macroBasedForLoop(), (parentXmlElement).getChildWithTagNameIterator ((requiredTagName))))
        */
    }
}

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/xml/aloe_XmlElement.cpp]

pub fn is_valid_xml_name_start_character(character: wchar_t) -> bool {
    
    todo!();
    /*
        return character == ':'
            || character == '_'
            || (character >= 'a'     && character <= 'z')
            || (character >= 'A'     && character <= 'Z')
            || (character >= 0xc0    && character <= 0xd6)
            || (character >= 0xd8    && character <= 0xf6)
            || (character >= 0xf8    && character <= 0x2ff)
            || (character >= 0x370   && character <= 0x37d)
            || (character >= 0x37f   && character <= 0x1fff)
            || (character >= 0x200c  && character <= 0x200d)
            || (character >= 0x2070  && character <= 0x218f)
            || (character >= 0x2c00  && character <= 0x2fef)
            || (character >= 0x3001  && character <= 0xd7ff)
            || (character >= 0xf900  && character <= 0xfdcf)
            || (character >= 0xfdf0  && character <= 0xfffd)
            || (character >= 0x10000 && character <= 0xeffff);
    */
}

pub fn is_valid_xml_name_body_character(character: wchar_t) -> bool {
    
    todo!();
    /*
        return isValidXmlNameStartCharacter (character)
            || character == '-'
            || character == '.'
            || character == 0xb7
            || (character >= '0'    && character <= '9')
            || (character >= 0x300  && character <= 0x036f)
            || (character >= 0x203f && character <= 0x2040);
    */
}

///--------------------------------
impl XmlElement {

    /**
      | Creates an XmlElement with this tag
      | name.
      |
      */
    pub fn new_from_string_ref(tag: &String) -> Self {
    
        todo!();
        /*


            : tagName (StringPool::getGlobalPool().getPooledString (tag))

        jassert (isValidXmlName (tagName));
        */
    }
    
    /**
      | Creates an XmlElement with this tag
      | name.
      |
      */
    pub fn new_from_raw_string(tag: *const u8) -> Self {
    
        todo!();
        /*


            : tagName (StringPool::getGlobalPool().getPooledString (tag))
        jassert (isValidXmlName (tagName));
        */
    }
    
    /**
      | Creates an XmlElement with this tag
      | name.
      |
      */
    pub fn new_from_tag(tag: &str) -> Self {
    
        todo!();
        /*


            : tagName (StringPool::getGlobalPool().getPooledString (tag))
        jassert (isValidXmlName (tagName));
        */
    }
    
    /**
      | Creates an XmlElement with this tag
      | name.
      |
      */
    pub fn new_from_identifier(tag: &Identifier) -> Self {
    
        todo!();
        /*


            : tagName (tag.toString())
        jassert (isValidXmlName (tagName));
        */
    }
    
    /**
      | Creates an XmlElement with this tag
      | name.
      |
      */
    pub fn new_from_tag_range(
        tag_name_start: CharPointerType,
        tag_name_end:   CharPointerType) -> Self {
    
        todo!();
        /*


            : tagName (StringPool::getGlobalPool().getPooledString (tagNameStart, tagNameEnd))
        jassert (isValidXmlName (tagName));
        */
    }
    
    pub fn new_from_i32(dummy: i32) -> Self {
    
        todo!();
        /*


        
        */
    }
    
    /**
      | Creates a (deep) copy of another element.
      |
      */
    pub fn new_from_other_ref(other: &XmlElement) -> Self {
    
        todo!();
        /*


            : tagName (other.tagName)

        copyChildrenAndAttributesFrom (other);
        */
    }
    
    /**
      | Creates a (deep) copy of another element.
      |
      */
    pub fn assign_from_ref(&mut self, other: &XmlElement) -> &mut XmlElement {
        
        todo!();
        /*
            if (this != &other)
        {
            removeAllAttributes();
            deleteAllChildElements();
            tagName = other.tagName;
            copyChildrenAndAttributesFrom (other);
        }

        return *this;
        */
    }
    
    pub fn new_from_other(other: XmlElement) -> Self {
    
        todo!();
        /*


            : nextListItem      (std::move (other.nextListItem)),
          firstChildElement (std::move (other.firstChildElement)),
          attributes        (std::move (other.attributes)),
          tagName           (std::move (other.tagName))
        */
    }
    
    /**
      | Move assignment operator
      |
      */
    pub fn assign_from(&mut self, other: XmlElement) -> &mut XmlElement {
        
        todo!();
        /*
            jassert (this != &other); // hopefully the compiler should make this situation impossible!

        removeAllAttributes();
        deleteAllChildElements();

        nextListItem      = std::move (other.nextListItem);
        firstChildElement = std::move (other.firstChildElement);
        attributes        = std::move (other.attributes);
        tagName           = std::move (other.tagName);

        return *this;
        */
    }
    
    pub fn copy_children_and_attributes_from(&mut self, other: &XmlElement)  {
        
        todo!();
        /*
            jassert (firstChildElement.get() == nullptr);
        firstChildElement.addCopyOfList (other.firstChildElement);

        jassert (attributes.get() == nullptr);
        attributes.addCopyOfList (other.attributes);
        */
    }
}

pub mod xml_output_functions {
    use super::*;

    pub mod  legal_char_lookup_table {
        use super::*;

        pub fn bit<const c: i32>() -> bool {

            todo!();
            /*
            template <int c>
            struct Bit
            {
                enum { v = ((c >= 'a' && c <= 'z')
                         || (c >= 'A' && c <= 'Z')
                         || (c >= '0' && c <= '9')
                         || c == ' ' || c == '.'  || c == ',' || c == ';'
                         || c == ':' || c == '-'  || c == '(' || c == ')'
                         || c == '_' || c == '+'  || c == '=' || c == '?'
                         || c == '!' || c == '$'  || c == '#' || c == '@'
                         || c == '[' || c == ']'  || c == '/' || c == '|'
                         || c == '*' || c == '%'  || c == '~' || c == '{'
                         || c == '}' || c == '\'' || c == '\\')
                            ? (1 << (c & 7)) : 0 };
            };
            */
        }

        pub fn byte<const table_index: i32>() -> bool {

            todo!();
            /*
            enum { v = (int) Bit<tableIndex * 8 + 0>::v | (int) Bit<tableIndex * 8 + 1>::v
                     | (int) Bit<tableIndex * 8 + 2>::v | (int) Bit<tableIndex * 8 + 3>::v
                     | (int) Bit<tableIndex * 8 + 4>::v | (int) Bit<tableIndex * 8 + 5>::v
                     | (int) Bit<tableIndex * 8 + 6>::v | (int) Bit<tableIndex * 8 + 7>::v };
            */
        }

        pub fn is_legal(c: u32) -> bool {
            
            todo!();
            /*
                static const unsigned char legalChars[] = { Byte< 0>::v, Byte< 1>::v, Byte< 2>::v, Byte< 3>::v,
                                                                    Byte< 4>::v, Byte< 5>::v, Byte< 6>::v, Byte< 7>::v,
                                                                    Byte< 8>::v, Byte< 9>::v, Byte<10>::v, Byte<11>::v,
                                                                    Byte<12>::v, Byte<13>::v, Byte<14>::v, Byte<15>::v };

                        return c < sizeof (legalChars) * 8
                                 && (legalChars[c >> 3] & (1 << (c & 7))) != 0;
            */
        }
    }

    pub fn escape_illegal_xml_chars<W: Write>(
        output_stream:    &mut W,
        text:             &String,
        change_new_lines: bool
    ) {
        
        todo!();
        /*
            auto t = text.getCharPointer();

                for (;;)
                {
                    auto character = (uint32) t.getAndAdvance();

                    if (character == 0)
                        break;

                    if (LegalCharLookupTable::isLegal (character))
                    {
                        outputStream << (char) character;
                    }
                    else
                    {
                        switch (character)
                        {
                            case '&':   outputStream << "&amp;"; break;
                            case '"':   outputStream << "&quot;"; break;
                            case '>':   outputStream << "&gt;"; break;
                            case '<':   outputStream << "&lt;"; break;

                            case '\n':
                            case '\r':
                                if (! changeNewLines)
                                {
                                    outputStream << (char) character;
                                    break;
                                }
                                ALOE_FALLTHROUGH
                            default:
                                outputStream << "&#" << ((int) character) << ';';
                                break;
                        }
                    }
                }
        */
    }

    pub fn write_spaces<W: Write>(out: &mut W, num_spaces: usize)  {
        
        todo!();
        /*
            out.writeRepeatedByte (' ', numSpaces);
        */
    }
}

pub fn get_empty_string_ref() -> &'static str {
    
    todo!();
    /*
        static String empty;
        return empty;
    */
}

pub const aloe_xmltextContentAttributeName: &'static str = "text";

impl XmlElement {
    
    pub fn write_element_as_text<W: Write>(
        &self, 
        output_stream:     &mut W,
        indentation_level: i32,
        line_wrap_length:  i32,
        new_line_chars:    *const u8)  {
        
        todo!();
        /*
            if (indentationLevel >= 0)
            XmlOutputFunctions::writeSpaces (outputStream, (size_t) indentationLevel);

        if (! isTextElement())
        {
            outputStream.writeByte ('<');
            outputStream << tagName;

            {
                auto attIndent = (size_t) (indentationLevel + tagName.length() + 1);
                int lineLen = 0;

                for (auto* att = attributes.get(); att != nullptr; att = att->nextListItem)
                {
                    if (lineLen > lineWrapLength && indentationLevel >= 0)
                    {
                        outputStream << newLineChars;
                        XmlOutputFunctions::writeSpaces (outputStream, attIndent);
                        lineLen = 0;
                    }

                    auto startPos = outputStream.getPosition();
                    outputStream.writeByte (' ');
                    outputStream << att->name;
                    outputStream.write ("=\"", 2);
                    XmlOutputFunctions::escapeIllegalXmlChars (outputStream, att->value, true);
                    outputStream.writeByte ('"');
                    lineLen += (int) (outputStream.getPosition() - startPos);
                }
            }

            if (auto* child = firstChildElement.get())
            {
                outputStream.writeByte ('>');
                bool lastWasTextNode = false;

                for (; child != nullptr; child = child->nextListItem)
                {
                    if (child->isTextElement())
                    {
                        XmlOutputFunctions::escapeIllegalXmlChars (outputStream, child->getText(), false);
                        lastWasTextNode = true;
                    }
                    else
                    {
                        if (indentationLevel >= 0 && ! lastWasTextNode)
                            outputStream << newLineChars;

                        child->writeElementAsText (outputStream,
                                                   lastWasTextNode ? 0 : (indentationLevel + (indentationLevel >= 0 ? 2 : 0)), lineWrapLength,
                                                   newLineChars);
                        lastWasTextNode = false;
                    }
                }

                if (indentationLevel >= 0 && ! lastWasTextNode)
                {
                    outputStream << newLineChars;
                    XmlOutputFunctions::writeSpaces (outputStream, (size_t) indentationLevel);
                }

                outputStream.write ("</", 2);
                outputStream << tagName;
                outputStream.writeByte ('>');
            }
            else
            {
                outputStream.write ("/>", 2);
            }
        }
        else
        {
            XmlOutputFunctions::escapeIllegalXmlChars (outputStream, getText(), false);
        }
        */
    }
    
    /**
      | Returns a text version of this XML element.
      | 
      | If your intention is to write the XML
      | to a file or stream, it's probably more
      | efficient to use writeTo() instead
      | of creating an intermediate string.
      | 
      | @see writeTo
      |
      */
    pub fn to_string(&self, options: &xml_element::TextFormat) -> String {
        
        todo!();
        /*
            MemoryOutputStream mem (2048);
        writeTo (mem, options);
        return mem.toUTF8();
        */
    }
    
    /**
      | Writes the document to a stream as UTF-8.
      | @see writeTo, toString
      |
      */
    pub fn write_to_writer<W: Write>(
        &self, 
        output:  &mut W,
        options: &xml_element::TextFormat)  {
        
        todo!();
        /*
            if (options.customHeader.isNotEmpty())
        {
            output << options.customHeader;

            if (options.newLineChars == nullptr)
                output.writeByte (' ');
            else
                output << options.newLineChars
                       << options.newLineChars;
        }
        else if (options.addDefaultHeader)
        {
            output << "<?xml version=\"1.0\" encoding=\"";

            if (options.customEncoding.isNotEmpty())
                output << options.customEncoding;
            else
                output << "UTF-8";

            output << "\"?>";

            if (options.newLineChars == nullptr)
                output.writeByte (' ');
            else
                output << options.newLineChars
                       << options.newLineChars;
        }

        if (options.dtd.isNotEmpty())
        {
            output << options.dtd;

            if (options.newLineChars == nullptr)
                output.writeByte (' ');
            else
                output << options.newLineChars;
        }

        writeElementAsText (output, options.newLineChars == nullptr ? -1 : 0,
                            options.lineWrapLength,
                            options.newLineChars);

        if (options.newLineChars != nullptr)
            output << options.newLineChars;
        */
    }
    
    /**
      | Writes the document to a file as UTF-8.
      | @see writeTo, toString
      |
      */
    pub fn write_to_file(
        &self, 
        destination_file: &std::fs::File,
        options:          &xml_element::TextFormat) -> bool {
        
        todo!();
        /*
            TemporaryFile tempFile (destinationFile);

        {
            FileOutputStream out (tempFile.getFile());

            if (! out.openedOk())
                return false;

            writeTo (out, options);
            out.flush(); // (called explicitly to force an fsync on posix)

            if (out.getStatus().failed())
                return false;
        }

        return tempFile.overwriteTargetFileWithTemporary();
        */
    }

    pub fn write_to(
        &self, 
        file:             &std::fs::File,
        dtd_to_use:       &str,
        encoding_type:    &str,
        line_wrap_length: i32) -> bool {
        
        todo!();
        /*
            TextFormat options;
        options.dtd = dtdToUse;
        options.customEncoding = encodingType;
        options.lineWrapLength = lineWrapLength;

        return writeTo (file, options);
        */
    }
    
    pub fn create_document(
        &self, 
        dtd_to_use:         &str,
        all_on_one_line:    bool,
        include_xml_header: bool,
        encoding_type:      &str,
        line_wrap_length:   i32) -> String {
        
        todo!();
        /*
            TextFormat options;
        options.dtd = dtdToUse;
        options.customEncoding = encodingType;
        options.addDefaultHeader = includeXmlHeader;
        options.lineWrapLength = lineWrapLength;

        if (allOnOneLine)
            options.newLineChars = nullptr;

        return toString (options);
        */
    }
    
    pub fn write_to_stream<W: Write>(
        &self, 
        output:             &mut W,
        dtd_to_use:         &str,
        all_on_one_line:    bool,
        include_xml_header: bool,
        encoding_type:      &str,
        line_wrap_length:   i32)  {
        
        todo!();
        /*
            TextFormat options;
        options.dtd = dtdToUse;
        options.customEncoding = encodingType;
        options.addDefaultHeader = includeXmlHeader;
        options.lineWrapLength = lineWrapLength;

        if (allOnOneLine)
            options.newLineChars = nullptr;

        writeTo (output, options);
        */
    }
    
    /**
      | Tests whether this element has a particular
      | tag name.
      | 
      | -----------
      | @param possibleTagName
      | 
      | the tag name you're comparing it with
      | @see getTagName
      |
      */
    pub fn has_tag_name(&self, possible_tag_name: &str) -> bool {
        
        todo!();
        /*
            const bool matches = tagName.equalsIgnoreCase (possibleTagName);

        // XML tags should be case-sensitive, so although this method allows a
        // case-insensitive match to pass, you should try to avoid this.
        jassert ((! matches) || tagName == possibleTagName);

        return matches;
        */
    }
    
    /**
      | Returns the namespace portion of the
      | tag-name, or an empty string if none
      | is specified.
      |
      */
    pub fn get_namespace(&self) -> String {
        
        todo!();
        /*
            return tagName.upToFirstOccurrenceOf (":", false, false);
        */
    }
    
    /**
      | Returns the part of the tag-name that
      | follows any namespace declaration.
      |
      */
    pub fn get_tag_name_without_namespace(&self) -> String {
        
        todo!();
        /*
            return tagName.fromLastOccurrenceOf (":", false, false);
        */
    }
    
    /**
      | Tests whether this element has a particular
      | tag name, ignoring any XML namespace
      | prefix.
      | 
      | So a test for e.g. "xyz" will return true
      | for "xyz" and also "foo:xyz", "bar::xyz",
      | etc. @see getTagName
      |
      */
    pub fn has_tag_name_ignoring_namespace(&self, possible_tag_name: &str) -> bool {
        
        todo!();
        /*
            return hasTagName (possibleTagName) || getTagNameWithoutNamespace() == possibleTagName;
        */
    }
    
    /**
      | Returns the next of this element's siblings
      | which has the specified tag name.
      | 
      | This is like getNextElement(), but
      | will scan through the list until it finds
      | an element with the given tag name.
      | 
      | @see getNextElement, getChildIterator
      |
      */
    pub fn get_next_element_with_tag_name(&self, required_tag_name: &str) -> *mut XmlElement {
        
        todo!();
        /*
            auto* e = nextListItem.get();

        while (e != nullptr && ! e->hasTagName (requiredTagName))
            e = e->nextListItem;

        return e;
        */
    }
    
    /**
      | Changes this elements tag name. @see
      | getTagName
      |
      */
    pub fn set_tag_name(&mut self, new_tag_name: &str)  {
        
        todo!();
        /*
            jassert (isValidXmlName (newTagName));
        tagName = StringPool::getGlobalPool().getPooledString (newTagName);
        */
    }
    
    /**
      | Returns the number of XML attributes
      | this element contains.
      | 
      | E.g. for an element such as \<MOOSE legs="4"
      | antlers="2">, this would return 2.
      |
      */
    pub fn get_num_attributes(&self) -> i32 {
        
        todo!();
        /*
            return attributes.size();
        */
    }
    
    /**
      | Returns the name of one of the elements
      | attributes.
      | 
      | E.g. for an element such as \<MOOSE legs="4"
      | antlers="2">, then getAttributeName(1)
      | would return "antlers".
      | 
      | @see getAttributeValue, getStringAttribute
      |
      */
    pub fn get_attribute_name(&self, index: i32) -> &String {
        
        todo!();
        /*
            if (auto* att = attributes[index].get())
            return att->name.toString();

        return getEmptyStringRef();
        */
    }
    
    /**
      | Returns the value of one of the elements
      | attributes.
      | 
      | E.g. for an element such as \<MOOSE legs="4"
      | antlers="2">, then getAttributeName(1)
      | would return "2".
      | 
      | @see getAttributeName, getStringAttribute
      |
      */
    pub fn get_attribute_value(&self, index: i32) -> &String {
        
        todo!();
        /*
            if (auto* att = attributes[index].get())
            return att->value;

        return getEmptyStringRef();
        */
    }
    
    pub fn get_attribute(&self, attribute_name: &str) -> *mut xml_element::XmlAttributeNode {
        
        todo!();
        /*
            for (auto* att = attributes.get(); att != nullptr; att = att->nextListItem)
            if (att->name == attributeName)
                return att;

        return nullptr;
        */
    }
    
    /**
      | Checks whether the element contains
      | an attribute with a certain name.
      |
      */
    pub fn has_attribute(&self, attribute_name: &str) -> bool {
        
        todo!();
        /*
            return getAttribute (attributeName) != nullptr;
        */
    }
    
    /**
      | Returns the value of a named attribute.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      |
      */
    pub fn get_string_attribute(&self, attribute_name: &str) -> &String {
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
            return att->value;

        return getEmptyStringRef();
        */
    }
    
    /**
      | Returns the value of a named attribute.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the element doesn't
      | have an attribute with this name
      |
      */
    pub fn get_string_attribute_with_default_return_value(
        &self, 
        attribute_name:       &str,
        default_return_value: &String) -> String {
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
            return att->value;

        return defaultReturnValue;
        */
    }
    
    /**
      | Returns the value of a named attribute
      | as an integer.
      | 
      | This will try to find the attribute and
      | convert it to an integer (using the String::getIntValue()
      | method).
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the element doesn't
      | have an attribute with this name @see
      | setAttribute
      |
      */
    pub fn get_int_attribute(
        &self, 
        attribute_name:       &str,
        default_return_value: Option<i32>) -> i32 {

        let default_return_value: i32 = default_return_value.unwrap_or(0);
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
            return att->value.getIntValue();

        return defaultReturnValue;
        */
    }
    
    /**
      | Returns the value of a named attribute
      | as floating-point.
      | 
      | This will try to find the attribute and
      | convert it to a double (using the String::getDoubleValue()
      | method).
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the element doesn't
      | have an attribute with this name @see
      | setAttribute
      |
      */
    pub fn get_double_attribute(
        &self, 
        attribute_name:       &str,
        default_return_value: Option<f64>) -> f64 {

        let default_return_value: f64 = default_return_value.unwrap_or(0.0);
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
            return att->value.getDoubleValue();

        return defaultReturnValue;
        */
    }
    
    /**
      | Returns the value of a named attribute
      | as a boolean.
      | 
      | This will try to find the attribute and
      | interpret it as a boolean. To do this,
      | it'll return true if the value is "1",
      | "true", "y", etc, or false for other
      | values.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      | ----------
      | @param defaultReturnValue
      | 
      | a value to return if the element doesn't
      | have an attribute with this name
      |
      */
    pub fn get_bool_attribute(
        &self, 
        attribute_name:       &str,
        default_return_value: Option<bool>) -> bool {

        let default_return_value: bool = default_return_value.unwrap_or(false);
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
        {
            auto firstChar = *(att->value.getCharPointer().findEndOfWhitespace());

            return firstChar == '1'
                || firstChar == 't'
                || firstChar == 'y'
                || firstChar == 'T'
                || firstChar == 'Y';
        }

        return defaultReturnValue;
        */
    }
    
    /**
      | Compares the value of a named attribute
      | with a value passed-in.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to look up
      | ----------
      | @param stringToCompareAgainst
      | 
      | the value to compare it with
      | ----------
      | @param ignoreCase
      | 
      | whether the comparison should be case-insensitive
      | 
      | -----------
      | @return
      | 
      | true if the value of the attribute is
      | the same as the string passed-in; false
      | if it's different (or if no such attribute
      | exists)
      |
      */
    pub fn compare_attribute(
        &self, 
        attribute_name:            &str,
        string_to_compare_against: &str,
        ignore_case:               Option<bool>) -> bool {

        let ignore_case: bool = ignore_case.unwrap_or(false);
        
        todo!();
        /*
            if (auto* att = getAttribute (attributeName))
            return ignoreCase ? att->value.equalsIgnoreCase (stringToCompareAgainst)
                              : att->value == stringToCompareAgainst;

        return false;
        */
    }
    
    /**
      | Adds a named attribute to the element.
      | 
      | If the element already contains an attribute
      | with this name, it's value will be updated
      | to the new value. If there's no such attribute
      | yet, a new one will be added.
      | 
      | Note that there are other setAttribute()
      | methods that take integers, doubles,
      | etc. to make it easy to store numbers.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to set
      | ----------
      | @param newValue
      | 
      | the value to set it to @see removeAttribute
      |
      */
    pub fn set_attribute(
        &mut self, 
        attribute_name: &Identifier,
        value:          &String)  {
        
        todo!();
        /*
            if (attributes == nullptr)
        {
            attributes = new XmlAttributeNode (attributeName, value);
        }
        else
        {
            for (auto* att = attributes.get(); ; att = att->nextListItem)
            {
                if (att->name == attributeName)
                {
                    att->value = value;
                    break;
                }

                if (att->nextListItem == nullptr)
                {
                    att->nextListItem = new XmlAttributeNode (attributeName, value);
                    break;
                }
            }
        }
        */
    }
    
    /**
      | Adds a named attribute to the element,
      | setting it to an integer value.
      | 
      | If the element already contains an attribute
      | with this name, it's value will be updated
      | to the new value. If there's no such attribute
      | yet, a new one will be added.
      | 
      | Note that there are other setAttribute()
      | methods that take integers, doubles,
      | etc. to make it easy to store numbers.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to set
      | ----------
      | @param newValue
      | 
      | the value to set it to
      |
      */
    pub fn set_attribute_i32(
        &mut self, 
        attribute_name: &Identifier,
        number:         i32)  {
        
        todo!();
        /*
            setAttribute (attributeName, String (number));
        */
    }
    
    /**
      | Adds a named attribute to the element,
      | setting it to a floating-point value.
      | 
      | If the element already contains an attribute
      | with this name, it's value will be updated
      | to the new value. If there's no such attribute
      | yet, a new one will be added.
      | 
      | -----------
      | @note
      | 
      | there are other setAttribute() methods
      | that take integers, doubles, etc. to
      | make it easy to store numbers.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to set
      | ----------
      | @param newValue
      | 
      | the value to set it to
      |
      */
    pub fn set_attribute_f64(
        &mut self, 
        attribute_name: &Identifier,
        number:         f64)  {
        
        todo!();
        /*
            setAttribute (attributeName, serialiseDouble (number));
        */
    }
    
    /**
      | Removes a named attribute from the element.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to remove @see
      | removeAllAttributes
      |
      */
    pub fn remove_attribute(&mut self, attribute_name: &Identifier)  {
        
        todo!();
        /*
            for (auto* att = &attributes; att->get() != nullptr; att = &(att->get()->nextListItem))
        {
            if (att->get()->name == attributeName)
            {
                delete att->removeNext();
                break;
            }
        }
        */
    }
    
    /**
      | Removes all attributes from this element.
      |
      */
    pub fn remove_all_attributes(&mut self)  {
        
        todo!();
        /*
            attributes.deleteAll();
        */
    }
    
    /**
      | Returns the number of sub-elements
      | in this element. @see getChildElement
      |
      */
    pub fn get_num_child_elements(&self) -> i32 {
        
        todo!();
        /*
            return firstChildElement.size();
        */
    }
    
    /**
      | Returns the sub-element at a certain
      | index.
      | 
      | It's not very efficient to iterate the
      | sub-elements by index - see getNextElement()
      | for an example of how best to iterate.
      | 
      | -----------
      | @return
      | 
      | the n'th child of this element, or nullptr
      | if the index is out-of-range @see getNextElement,
      | isTextElement, getChildByName
      |
      */
    pub fn get_child_element(&self, index: i32) -> *mut XmlElement {
        
        todo!();
        /*
            return firstChildElement[index].get();
        */
    }
    
    /**
      | Returns the first sub-element with
      | a given tag-name.
      | 
      | -----------
      | @param tagNameToLookFor
      | 
      | the tag name of the element you want to
      | find
      | 
      | -----------
      | @return
      | 
      | the first element with this tag name,
      | or nullptr if none is found @see getNextElement,
      | isTextElement, getChildElement,
      | getChildByAttribute
      |
      */
    pub fn get_child_by_name(&self, child_name: &str) -> *mut XmlElement {
        
        todo!();
        /*
            jassert (! childName.isEmpty());

        for (auto* child = firstChildElement.get(); child != nullptr; child = child->nextListItem)
            if (child->hasTagName (childName))
                return child;

        return nullptr;
        */
    }
    
    /**
      | Returns the first sub-element which
      | has an attribute that matches the given
      | value.
      | 
      | -----------
      | @param attributeName
      | 
      | the name of the attribute to check
      | ----------
      | @param attributeValue
      | 
      | the target value of the attribute
      | 
      | -----------
      | @return
      | 
      | the first element with this attribute
      | value, or nullptr if none is found @see
      | getChildByName
      |
      */
    pub fn get_child_by_attribute(
        &self, 
        attribute_name:  &str,
        attribute_value: &str) -> *mut XmlElement {
        
        todo!();
        /*
            jassert (! attributeName.isEmpty());

        for (auto* child = firstChildElement.get(); child != nullptr; child = child->nextListItem)
            if (child->compareAttribute (attributeName, attributeValue))
                return child;

        return nullptr;
        */
    }
    
    /**
      | Appends an element to this element's
      | list of children.
      | 
      | Child elements are deleted automatically
      | when their parent is deleted, so make
      | sure the object that you pass in will
      | not be deleted by anything else, and
      | make sure it's not already the child
      | of another element.
      | 
      | Note that due to the XmlElement using
      | a singly-linked-list, prependChildElement()
      | is an O(1) operation, but addChildElement()
      | is an O(N) operation - so if you're adding
      | large number of elements, you may prefer
      | to do so in reverse order!
      | 
      | @see getFirstChildElement, getNextElement,
      | getNumChildElements, getChildElement,
      | removeChildElement
      |
      */
    pub fn add_child_element(&mut self, new_node: *mut XmlElement)  {
        
        todo!();
        /*
            if (newNode != nullptr)
        {
            // The element being added must not be a child of another node!
            jassert (newNode->nextListItem == nullptr);

            firstChildElement.append (newNode);
        }
        */
    }
    
    /**
      | Inserts an element into this element's
      | list of children.
      | 
      | Child elements are deleted automatically
      | when their parent is deleted, so make
      | sure the object that you pass in will
      | not be deleted by anything else, and
      | make sure it's not already the child
      | of another element.
      | 
      | -----------
      | @param newChildElement
      | 
      | the element to add
      | ----------
      | @param indexToInsertAt
      | 
      | the index at which to insert the new element
      | - if this is below zero, it will be added
      | to the end of the list @see addChildElement,
      | insertChildElement
      |
      */
    pub fn insert_child_element(
        &mut self, 
        new_node:           *mut XmlElement,
        index_to_insert_at: i32)  {
        
        todo!();
        /*
            if (newNode != nullptr)
        {
            // The element being added must not be a child of another node!
            jassert (newNode->nextListItem == nullptr);

            firstChildElement.insertAtIndex (indexToInsertAt, newNode);
        }
        */
    }
    
    /**
      | Inserts an element at the beginning
      | of this element's list of children.
      | 
      | Child elements are deleted automatically
      | when their parent is deleted, so make
      | sure the object that you pass in will
      | not be deleted by anything else, and
      | make sure it's not already the child
      | of another element.
      | 
      | Note that due to the XmlElement using
      | a singly-linked-list, prependChildElement()
      | is an O(1) operation, but addChildElement()
      | is an O(N) operation - so if you're adding
      | large number of elements, you may prefer
      | to do so in reverse order!
      | 
      | @see addChildElement, insertChildElement
      |
      */
    pub fn prepend_child_element(&mut self, new_node: *mut XmlElement)  {
        
        todo!();
        /*
            if (newNode != nullptr)
        {
            // The element being added must not be a child of another node!
            jassert (newNode->nextListItem == nullptr);

            firstChildElement.insertNext (newNode);
        }
        */
    }
    
    /**
      | Creates a new element with the given
      | name and returns it, after adding it
      | as a child element.
      | 
      | This is a handy method that means that
      | instead of writing this: @code XmlElement*
      | newElement = new XmlElement ("foobar");
      | myParentElement->addChildElement
      | (newElement); @endcode
      | 
      | ..you could just write this: @code XmlElement*
      | newElement = myParentElement->createNewChildElement
      | ("foobar"); @endcode
      |
      */
    pub fn create_new_child_element(&mut self, child_tag_name: &str) -> *mut XmlElement {
        
        todo!();
        /*
            auto newElement = new XmlElement (childTagName);
        addChildElement (newElement);
        return newElement;
        */
    }
    
    /**
      | Replaces one of this element's children
      | with another node.
      | 
      | If the current element passed-in isn't
      | actually a child of this element, this
      | will return false and the new one won't
      | be added. Otherwise, the existing element
      | will be deleted, replaced with the new
      | one, and it will return true.
      |
      */
    pub fn replace_child_element(
        &mut self, 
        current_child_element: *mut XmlElement,
        new_node:              *mut XmlElement) -> bool {
        
        todo!();
        /*
            if (newNode != nullptr)
        {
            if (auto* p = firstChildElement.findPointerTo (currentChildElement))
            {
                if (currentChildElement != newNode)
                    delete p->replaceNext (newNode);

                return true;
            }
        }

        return false;
        */
    }
    
    /**
      | Removes a child element.
      | 
      | -----------
      | @param childToRemove
      | 
      | the child to look for and remove
      | ----------
      | @param shouldDeleteTheChild
      | 
      | if true, the child will be deleted, if
      | false it'll just remove it
      |
      */
    pub fn remove_child_element(
        &mut self, 
        child_to_remove:         *mut XmlElement,
        should_delete_the_child: bool)  {
        
        todo!();
        /*
            if (childToRemove != nullptr)
        {
            jassert (containsChildElement (childToRemove));

            firstChildElement.remove (childToRemove);

            if (shouldDeleteTheChild)
                delete childToRemove;
        }
        */
    }
    
    /**
      | Compares two XmlElements to see if they
      | contain the same text and attributes.
      | 
      | The elements are only considered equivalent
      | if they contain the same attributes
      | with the same values, and have the same
      | sub-nodes.
      | 
      | -----------
      | @param other
      | 
      | the other element to compare to
      | ----------
      | @param ignoreOrderOfAttributes
      | 
      | if true, this means that two elements
      | with the same attributes in a different
      | order will be considered the same; if
      | false, the attributes must be in the
      | same order as well
      |
      */
    pub fn is_equivalent_to(
        &self, 
        other:                      *const XmlElement,
        ignore_order_of_attributes: bool) -> bool {
        
        todo!();
        /*
            if (this != other)
        {
            if (other == nullptr || tagName != other->tagName)
                return false;

            if (ignoreOrderOfAttributes)
            {
                int totalAtts = 0;

                for (auto* att = attributes.get(); att != nullptr; att = att->nextListItem)
                {
                    if (! other->compareAttribute (att->name, att->value))
                        return false;

                    ++totalAtts;
                }

                if (totalAtts != other->getNumAttributes())
                    return false;
            }
            else
            {
                auto* thisAtt = attributes.get();
                auto* otherAtt = other->attributes.get();

                for (;;)
                {
                    if (thisAtt == nullptr || otherAtt == nullptr)
                    {
                        if (thisAtt == otherAtt) // both nullptr, so it's a match
                            break;

                        return false;
                    }

                    if (thisAtt->name != otherAtt->name
                         || thisAtt->value != otherAtt->value)
                    {
                        return false;
                    }

                    thisAtt = thisAtt->nextListItem;
                    otherAtt = otherAtt->nextListItem;
                }
            }

            auto* thisChild = firstChildElement.get();
            auto* otherChild = other->firstChildElement.get();

            for (;;)
            {
                if (thisChild == nullptr || otherChild == nullptr)
                {
                    if (thisChild == otherChild) // both 0, so it's a match
                        break;

                    return false;
                }

                if (! thisChild->isEquivalentTo (otherChild, ignoreOrderOfAttributes))
                    return false;

                thisChild = thisChild->nextListItem;
                otherChild = otherChild->nextListItem;
            }
        }

        return true;
        */
    }
    
    /**
      | Deletes all the child elements in the
      | element. @see removeChildElement,
      | deleteAllChildElementsWithTagName
      |
      */
    pub fn delete_all_child_elements(&mut self)  {
        
        todo!();
        /*
            firstChildElement.deleteAll();
        */
    }
    
    /**
      | Deletes all the child elements with
      | a given tag name. @see removeChildElement
      |
      */
    pub fn delete_all_child_elements_with_tag_name(&mut self, name: &str)  {
        
        todo!();
        /*
            for (auto* child = firstChildElement.get(); child != nullptr;)
        {
            auto* nextChild = child->nextListItem.get();

            if (child->hasTagName (name))
                removeChildElement (child, true);

            child = nextChild;
        }
        */
    }
    
    /**
      | Returns true if the given element is
      | a child of this one.
      |
      */
    pub fn contains_child_element(&self, possible_child: *const XmlElement) -> bool {
        
        todo!();
        /*
            return firstChildElement.contains (possibleChild);
        */
    }
    
    /**
      | Recursively searches all sub-elements
      | of this one, looking for an element which
      | is the direct parent of the specified
      | element.
      | 
      | Because elements don't store a pointer
      | to their parent, if you have one and need
      | to find its parent, the only way to do
      | so is to exhaustively search the whole
      | tree for it.
      | 
      | If the given child is found somewhere
      | in this element's hierarchy, then this
      | method will return its parent. If not,
      | it will return nullptr.
      |
      */
    pub fn find_parent_element_of(&mut self, element_to_look_for: *const XmlElement) -> *mut XmlElement {
        
        todo!();
        /*
            if (this == elementToLookFor || elementToLookFor == nullptr)
            return nullptr;

        for (auto* child = firstChildElement.get(); child != nullptr; child = child->nextListItem)
        {
            if (elementToLookFor == child)
                return this;

            if (auto* found = child->findParentElementOf (elementToLookFor))
                return found;
        }

        return nullptr;
        */
    }
    
    pub fn get_child_elements_as_array(&self, elems: *mut *mut XmlElement)  {
        
        todo!();
        /*
            firstChildElement.copyToArray (elems);
        */
    }
    
    pub fn reorder_child_elements(
        &mut self, 
        elems: *mut *mut XmlElement,
        num:   i32)  {
        
        todo!();
        /*
            auto* e = elems[0];
        firstChildElement = e;

        for (int i = 1; i < num; ++i)
        {
            e->nextListItem = elems[i];
            e = e->nextListItem;
        }

        e->nextListItem = nullptr;
        */
    }
    
    /**
      | Returns true if this element is a section
      | of text.
      | 
      | Elements can either be an XML tag element
      | or a section of text, so this is used to
      | find out what kind of element this one
      | is.
      | 
      | @see getAllText, addTextElement,
      | deleteAllTextElements
      |
      */
    pub fn is_text_element(&self) -> bool {
        
        todo!();
        /*
            return tagName.isEmpty();
        */
    }
    
    /**
      | Returns the text for a text element.
      | 
      | Note that if you have an element like
      | this:
      | 
      | @code<xyz>hello</xyz>@endcode
      | 
      | then calling getText on the "xyz" element
      | won't return "hello", because that
      | is actually stored in a special text
      | sub-element inside the xyz element.
      | To get the "hello" string, you could
      | either call getText on the (unnamed)
      | sub-element, or use getAllSubText()
      | to do this automatically.
      | 
      | -----------
      | @note
      | 
      | leading and trailing whitespace will
      | be included in the string - to remove
      | if, just call String::trim() on the
      | result.
      | 
      | @see isTextElement, getAllSubText,
      | getChildElementAllSubText
      |
      */
    pub fn get_text(&self) -> &String {
        
        todo!();
        /*
            jassert (isTextElement());  // you're trying to get the text from an element that
                                    // isn't actually a text element.. If this contains text sub-nodes, you
                                    // probably want to use getAllSubText instead.

        return getStringAttribute (aloe_xmltextContentAttributeName);
        */
    }
    
    /**
      | Sets the text in a text element.
      | 
      | -----------
      | @note
      | 
      | this is only a valid call if this element
      | is a text element. If it's not, then no
      | action will be performed. If you're
      | trying to add text inside a normal element,
      | you probably want to use addTextElement()
      | instead.
      |
      */
    pub fn set_text(&mut self, new_text: &String)  {
        
        todo!();
        /*
            if (isTextElement())
            setAttribute (aloe_xmltextContentAttributeName, newText);
        else
            jassertfalse; // you can only change the text in a text element, not a normal one.
        */
    }
    
    /**
      | Returns all the text from this element's
      | child nodes.
      | 
      | This iterates all the child elements
      | and when it finds text elements, it concatenates
      | their text into a big string which it
      | returns.
      | 
      | E.g. @code<xyz>hello <x>there</x>
      | world</xyz>@endcode if you called
      | getAllSubText on the "xyz" element,
      | it'd return "hello there world".
      | 
      | -----------
      | @note
      | 
      | leading and trailing whitespace will
      | be included in the string - to remove
      | if, just call String::trim() on the
      | result.
      | 
      | @see isTextElement, getChildElementAllSubText,
      | getText, addTextElement
      |
      */
    pub fn get_all_sub_text(&self) -> String {
        
        todo!();
        /*
            if (isTextElement())
            return getText();

        if (getNumChildElements() == 1)
            return firstChildElement.get()->getAllSubText();

        MemoryOutputStream mem (1024);

        for (auto* child = firstChildElement.get(); child != nullptr; child = child->nextListItem)
            mem << child->getAllSubText();

        return mem.toUTF8();
        */
    }
    
    /**
      | Returns all the sub-text of a named child
      | element.
      | 
      | If there is a child element with the given
      | tag name, this will return all of its
      | sub-text (by calling getAllSubText()
      | on it). If there is no such child element,
      | this will return the default string
      | passed-in.
      | 
      | @see getAllSubText
      |
      */
    pub fn get_child_element_all_sub_text(
        &self, 
        child_tag_name:       &str,
        default_return_value: &String) -> String {
        
        todo!();
        /*
            if (auto* child = getChildByName (childTagName))
            return child->getAllSubText();

        return defaultReturnValue;
        */
    }
    
    /**
      | Creates a text element that can be added
      | to a parent element.
      |
      */
    pub fn create_text_element(&mut self, text: &String) -> *mut XmlElement {
        
        todo!();
        /*
            auto e = new XmlElement ((int) 0);
        e->setAttribute (aloe_xmltextContentAttributeName, text);
        return e;
        */
    }
    
    /**
      | Checks if a given string is a valid XML
      | name
      |
      */
    pub fn is_valid_xml_name(&mut self, text: &str) -> bool {
        
        todo!();
        /*
            if (text.isEmpty() || ! isValidXmlNameStartCharacter (text.text.getAndAdvance()))
            return false;

        for (;;)
        {
            if (text.isEmpty())
                return true;

            if (! isValidXmlNameBodyCharacter (text.text.getAndAdvance()))
                return false;
        }
        */
    }
    
    /**
      | Appends a section of text to this element.
      | @see isTextElement, getText, getAllSubText
      |
      */
    pub fn add_text_element(&mut self, text: &String)  {
        
        todo!();
        /*
            addChildElement (createTextElement (text));
        */
    }
    
    /**
      | Removes all the text elements from this
      | element. @see isTextElement, getText,
      | getAllSubText, addTextElement
      |
      */
    pub fn delete_all_text_elements(&mut self)  {
        
        todo!();
        /*
            for (auto* child = firstChildElement.get(); child != nullptr;)
        {
            auto* next = child->nextListItem.get();

            if (child->isTextElement())
                removeChildElement (child, true);

            child = next;
        }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
pub struct XmlElementTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for XmlElementTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("XmlElement", UnitTestCategories::xml
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl XmlElementTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            {
                beginTest ("Float formatting");

                auto element = std::make_unique<XmlElement> ("test");
                Identifier number ("number");

                std::map<double, String> tests;
                tests[1] = "1.0";
                tests[1.1] = "1.1";
                tests[1.01] = "1.01";
                tests[0.76378] = "0.76378";
                tests[-10] = "-10.0";
                tests[10.01] = "10.01";
                tests[0.0123] = "0.0123";
                tests[-3.7e-27] = "-3.7e-27";
                tests[1e+40] = "1.0e40";
                tests[-12345678901234567.0] = "-1.234567890123457e16";
                tests[192000] = "192000.0";
                tests[1234567] = "1.234567e6";
                tests[0.00006] = "0.00006";
                tests[0.000006] = "6.0e-6";

                for (auto& test : tests)
                {
                    element->setAttribute (number, test.first);
                    expectEquals (element->getStringAttribute (number), test.second);
                }
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static XmlElementTests xmlElementTests;
    */
}
