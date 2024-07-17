crate::ix!();

pub const JSON_FORMATTER_INDENT_SIZE: usize = 2;

pub struct JSONFormatter {

}

impl JSONFormatter {

    pub fn write(
        out:                    &mut dyn Write,
        v:                      &Var,
        indent_level:           i32,
        all_on_one_line:        bool,
        maximum_decimal_places: i32)  {
        
        todo!();
        /*
            if (v.isString())
            {
                out << '"';
                writeString (out, v.toString().getCharPointer());
                out << '"';
            }
            else if (v.isVoid())
            {
                out << "null";
            }
            else if (v.isUndefined())
            {
                out << "undefined";
            }
            else if (v.isBool())
            {
                out << (static_cast<bool> (v) ? "true" : "false");
            }
            else if (v.isDouble())
            {
                auto d = static_cast<double> (v);

                if (aloe_isfinite (d))
                {
                    out << serialiseDouble (d);
                }
                else
                {
                    out << "null";
                }
            }
            else if (v.isArray())
            {
                writeArray (out, *v.getArray(), indentLevel, allOnOneLine, maximumDecimalPlaces);
            }
            else if (v.isObject())
            {
                if (auto* object = v.getDynamicObject())
                    object->writeAsJSON (out, indentLevel, allOnOneLine, maximumDecimalPlaces);
                else
                    jassertfalse; // Only DynamicObjects can be converted to JSON!
            }
            else
            {
                // Can't convert these other types of object to JSON!
                jassert (! (v.isMethod() || v.isBinaryData()));

                out << v.toString();
            }
        */
    }
    
    pub fn write_escaped_char(
        out:   &mut dyn Write,
        value: u16)  {
        
        todo!();
        /*
            out << "\\u" << String::toHexString ((int) value).paddedLeft ('0', 4);
        */
    }
    
    pub fn write_string(
        out: &mut dyn Write,
        t:   CharPointerType)  {
        
        todo!();
        /*
            for (;;)
            {
                auto c = t.getAndAdvance();

                switch (c)
                {
                    case 0:  return;

                    case '\"':  out << "\\\""; break;
                    case '\\':  out << "\\\\"; break;
                    case '\a':  out << "\\a";  break;
                    case '\b':  out << "\\b";  break;
                    case '\f':  out << "\\f";  break;
                    case '\t':  out << "\\t";  break;
                    case '\r':  out << "\\r";  break;
                    case '\n':  out << "\\n";  break;

                    default:
                        if (c >= 32 && c < 127)
                        {
                            out << (char) c;
                        }
                        else
                        {
                            if (CharPointer_UTF16::getBytesRequiredFor (c) > 2)
                            {
                                CharPointer_UTF16::CharType chars[2];
                                CharPointer_UTF16 utf16 (chars);
                                utf16.write (c);

                                for (int i = 0; i < 2; ++i)
                                    writeEscapedChar (out, (unsigned short) chars[i]);
                            }
                            else
                            {
                                writeEscapedChar (out, (unsigned short) c);
                            }
                        }

                        break;
                }
            }
        */
    }
    
    pub fn write_spaces(
        out:        &mut dyn Write,
        num_spaces: i32)  {
        
        todo!();
        /*
            out.writeRepeatedByte (' ', (size_t) numSpaces);
        */
    }
    
    pub fn write_array(
        out:                    &mut dyn Write,
        array:                  &[Var],
        indent_level:           i32,
        all_on_one_line:        bool,
        maximum_decimal_places: i32)  {
        
        todo!();
        /*
            out << '[';

            if (! array.isEmpty())
            {
                if (! allOnOneLine)
                    out << newLine;

                for (int i = 0; i < array.size(); ++i)
                {
                    if (! allOnOneLine)
                        writeSpaces (out, indentLevel + JSON_FORMATTER_INDENT_SIZE);

                    write (out, array.getReference(i), indentLevel + JSON_FORMATTER_INDENT_SIZE, allOnOneLine, maximumDecimalPlaces);

                    if (i < array.size() - 1)
                    {
                        if (allOnOneLine)
                            out << ", ";
                        else
                            out << ',' << newLine;
                    }
                    else if (! allOnOneLine)
                        out << newLine;
                }

                if (! allOnOneLine)
                    writeSpaces (out, indentLevel);
            }

            out << ']';
        */
    }
}
