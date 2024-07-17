#[test] fn construction_and_parsing() {

    /*
            expectThrowsType (OSCAddressPattern (""), OSCFormatError);
            expectThrowsType (OSCAddressPattern ("noleadingslash"), OSCFormatError);
            expectThrowsType (OSCAddressPattern ("/notallowedchar "), OSCFormatError);
            expectThrowsType (OSCAddressPattern ("/notallowedchar#andsomemorechars"), OSCFormatError);
            expectThrowsType (OSCAddressPattern (String::fromUTF8 ("/nonasciicharacter\xc3\xbc""blabla")), OSCFormatError);
            expectThrowsType (OSCAddressPattern ("/nonprintableasciicharacter\t"), OSCFormatError);

            expectDoesNotThrow (OSCAddressPattern ("/"));
            expectDoesNotThrow (OSCAddressPattern ("/a"));
            expectDoesNotThrow (OSCAddressPattern ("/a/"));
            expectDoesNotThrow (OSCAddressPattern ("/a/bcd/"));
            expectDoesNotThrow (OSCAddressPattern ("/abcd/efgh/ijkLMNOPq/666r/s"));
            expectDoesNotThrow (OSCAddressPattern ("/allowedprintablecharacters!$%&()+-.:;<=>@^_`|~"));
            expectDoesNotThrow (OSCAddressPattern ("/additonalslashes//will///be////ignored"));
    */
}

#[test] fn construction_and_parsing_with_wildcards() {

    /*
       expectDoesNotThrow (OSCAddressPattern ("/foo/b?r/"));
       expectDoesNotThrow (OSCAddressPattern ("/?????"));
       expectDoesNotThrow (OSCAddressPattern ("/foo/b\*r"));
       expectDoesNotThrow (OSCAddressPattern ("/\*\*"));
       expectDoesNotThrow (OSCAddressPattern ("/?/b/\*c"));
       */
}

#[test] fn construction_and_parsing_with_match_expressions() {

    /*
            expectDoesNotThrow (OSCAddressPattern ("/{}"));
            expectDoesNotThrow (OSCAddressPattern ("/{foo}"));
            expectDoesNotThrow (OSCAddressPattern ("/{foo,bar,baz}"));
            expectDoesNotThrow (OSCAddressPattern ("/[]"));
            expectDoesNotThrow (OSCAddressPattern ("/[abcde]"));
            expectDoesNotThrow (OSCAddressPattern ("/[a-e]"));
            expectDoesNotThrow (OSCAddressPattern ("/foo/[a-z]x{foo,bar}/\*BAZ42/"));

            /* Note: If malformed expressions are used, e.g. "bracenotclosed{" or "{a-e}" or "[-foo]",
               this should not throw at construction time. Instead it should simply fail any pattern match later.
               So there is no need to test for those.
               The reason is that we do not actually parse the expressions now, but only during matching.
            */
    */
}

#[test] fn equality_comparison() {

    /*
            {
                OSCAddressPattern lhs ("/test/1");
                OSCAddressPattern rhs ("/test/1");
                expect (lhs == rhs);
                expect (! (lhs != rhs));
            }
            {
                OSCAddressPattern lhs ("/test/1");
                OSCAddressPattern rhs ("/test/1/");
                expect (lhs == rhs);
                expect (! (lhs != rhs));
            }
            {
                OSCAddressPattern lhs ("/test/1");
                OSCAddressPattern rhs ("/test/2");
                expect (! (lhs == rhs));
                expect (lhs != rhs);
            }
    */
}

#[test] fn basic_string_matching() {
    /*
     
    /* Note: The actual expression matching is
    tested in OSCPatternMatcher, so
    here we just do some basic tests
    and check if the matching works
    with multi-part addresses.  */
    {
            OSCAddressPattern pattern ("/foo/bar");
            expect (! pattern.containsWildcards());

            OSCAddress address ("/foo/bar");
            expect (pattern.matches (address));
        }
        {
            OSCAddressPattern pattern ("/foo/bar/");
            expect (! pattern.containsWildcards());

            OSCAddress address ("/foo/bar");
            expect (pattern.matches (address));
        }
        {
            OSCAddressPattern pattern ("/");
            expect (! pattern.containsWildcards());

            OSCAddress address ("/");
            expect (pattern.matches (address));
        }
        {
            OSCAddressPattern pattern ("/foo/bar");
            expect (! pattern.containsWildcards());

            expect (! pattern.matches (OSCAddress ("/foo/baz")));
            expect (! pattern.matches (OSCAddress ("/foo/bar/baz")));
            expect (! pattern.matches (OSCAddress ("/foo")));
        }


    */
}

#[test] fn basic_string_matching_with_wildcards() {

    /*
            OSCAddressPattern pattern ("\/\*\/\*put/slider[0-9]");
            expect (pattern.containsWildcards());

            expect (pattern.matches (OSCAddress ("/mypatch/input/slider0")));
            expect (pattern.matches (OSCAddress ("/myotherpatch/output/slider9")));
            expect (! pattern.matches (OSCAddress ("/myotherpatch/output/slider10")));
            expect (! pattern.matches (OSCAddress ("/output/slider9")));
            expect (! pattern.matches (OSCAddress ("/myotherpatch/output/slider9/position")));
    */
}

#[test] fn conversion_to_from_string() {

    /*
        {
            OSCAddressPattern ap ("/this/is/a/very/long/address/");
            expectEquals (ap.toString(), String ("/this/is/a/very/long/address"));
        }
        {
            OSCAddressPattern ap ("\/\*\/\*put/{fader,slider,knob}[0-9]/ba?/");
            expectEquals (ap.toString(), String ("\/\*\/\*put/{fader,slider,knob}[0-9]/ba?"));
        }
    */
}
