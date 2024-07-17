
#[test] fn construction_and_parsing() {
    /*
            expectThrowsType (OSCAddress (""), OSCFormatError);
            expectThrowsType (OSCAddress ("noleadingslash"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar "), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar#"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar*"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar,"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar?"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar["), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar]"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar{"), OSCFormatError);
            expectThrowsType (OSCAddress ("/notallowedchar}andsomemorechars"), OSCFormatError);
            expectThrowsType (OSCAddress (String::fromUTF8 ("/nonasciicharacter\xc3\xbc""blabla")), OSCFormatError);
            expectThrowsType (OSCAddress ("/nonprintableasciicharacter\t"), OSCFormatError);

            expectDoesNotThrow (OSCAddress ("/"));
            expectDoesNotThrow (OSCAddress ("/a"));
            expectDoesNotThrow (OSCAddress ("/a/"));
            expectDoesNotThrow (OSCAddress ("/a/bcd/"));
            expectDoesNotThrow (OSCAddress ("/abcd/efgh/ijkLMNOPq/666r/s"));
            expectDoesNotThrow (OSCAddress ("/allowedprintablecharacters!$%&()+-.^_`|~"));
            expectDoesNotThrow (OSCAddress ("/additonalslashes//will///be////ignored"));
    */
}

#[test] fn conversion_to_from_string() {
    /*
       OSCAddress address ("/this/is/a/very/long/address/");
       expectEquals (address.toString(), String ("/this/is/a/very/long/address"));
    */
}
