
#[test] fn basic_string_matching() {
    /*
        expect (matchOscPattern ("", ""));
        expect (! matchOscPattern ("", "x"));
        expect (! matchOscPattern ("x", ""));
        expect (matchOscPattern ("foo", "foo"));
        expect (! matchOscPattern ("foo", "bar"));
        expect (! matchOscPattern ("ooooo", "oooooo"));
    */
}

#[test] fn string_matching_with_qmark_wildcard() {

    /*
        expect (matchOscPattern ("?", "x"));
        expect (! matchOscPattern ("?", ""));
        expect (! matchOscPattern ("?", "xx"));
        expect (! matchOscPattern ("b?r", "br"));
        expect (matchOscPattern ("b?r", "bar"));
        expect (! matchOscPattern ("b?r", "baar"));
        expect (matchOscPattern ("f???o", "fabco"));
        expect (! matchOscPattern ("f???o", "fabo"));
    */
}

#[test] fn string_matching_with_star_wildcard() {

    /*
        expect (matchOscPattern ("*", ""));
        expect (matchOscPattern ("*", "x"));
        expect (matchOscPattern ("*", "foo"));
        expect (matchOscPattern ("*c", "aaaaaaabc"));
        expect (matchOscPattern ("*c", "aaaaaaabbbcccc"));
        expect (! matchOscPattern ("*c", "aaaaaaabbbcccca"));
        expect (matchOscPattern ("c*", "ccccbbbbaaaa"));
        expect (! matchOscPattern ("c*", "accccbbbaaaa"));

        expect (matchOscPattern ("f*o", "fo"));
        expect (matchOscPattern ("f*o", "fuo"));
        expect (matchOscPattern ("f*o", "fuvwxyzo"));

        expect (matchOscPattern ("*reallyreallylongstringstringstring", "reallyreallylongstringstringstrNOT"
                                                                        "reallyreallylongstringstringstrNOT"
                                                                        "reallyreallylongstringstringstrNOT"
                                                                        "reallyreallylongstringstringstrNOT"
                                                                        "reallyreallylongstringstringstrNOT"
                                                                        "reallyreallylongstringstringstring"));
    */
}

#[test] fn string_matching_with_regular_pattern1() {

    /*
        expect (matchOscPattern ("{}", ""));
        expect (! matchOscPattern ("{}", "x"));
        expect (matchOscPattern ("{abcde}", "abcde"));
        expect (matchOscPattern ("{abcde,f}", "f"));
        expect (! matchOscPattern ("{abcde,f}", "ff"));
        expect (matchOscPattern ("a{bcd}e", "abcde"));
        expect (matchOscPattern ("a{bcd,bce}e", "abcde"));
        expect (! matchOscPattern ("a{bce,bcf}e", "abcde"));
        expect (! matchOscPattern ("a{bce,bcf}e", "ae"));
        expect (matchOscPattern ("a{bce,,bcf}e", "ae"));
        expect (matchOscPattern ("a{bcd,bcd,bcd}e", "abcde"));
        expect (matchOscPattern ("aaa{bc,def,ghij,klmnopqrstu}eee", "aaaghijeee"));
        expect (matchOscPattern ("{a,b,c}bcde", "abcde"));
        expect (! matchOscPattern ("{abc}bcde", "abcde"));
        expect (matchOscPattern ("bcde{a,b,c}", "bcdea"));
        expect (! matchOscPattern ("bcde{abc}", "bcdea"));
        expect (matchOscPattern ("f{o,}o", "fo"));
        expect (matchOscPattern ("f{,,,,,}o", "fo"));
        expect (matchOscPattern ("foo{b,ba,bar}x", "foobarx"));
        expect (matchOscPattern ("a{bc,de}fg{hij,klm}{n}{}", "adefghijn"));

        // should fail gracefully in case of wrong syntax:
        expect (! matchOscPattern ("not{closing", "notclosing"));
        expect (! matchOscPattern ("not}opening", "notopening"));
        expect (! matchOscPattern ("{{nested}}", "nested"));
        expect (! matchOscPattern ("{a-c}bcde", "abcde"));
        expect (! matchOscPattern ("bcde{a-c}", "abcde"));
    */

}

#[test] fn string_matching_with_regular_pattern2() {

    /*
        // using [] for a set of chars:

        expect (matchOscPattern ("[]", ""));
        expect (! matchOscPattern ("[]", "x"));
        expect (! matchOscPattern ("[abcde]", "abcde"));
        expect (matchOscPattern ("[abcde]", "a"));
        expect (matchOscPattern ("[abcde]", "b"));
        expect (matchOscPattern ("[abcde]", "c"));
        expect (matchOscPattern ("[abcde]", "d"));
        expect (matchOscPattern ("[abcde]", "e"));
        expect (! matchOscPattern ("[abcde]", "f"));

        expect (matchOscPattern ("f[oo]", "fo"));
        expect (! matchOscPattern ("f[oo]", "foo"));

        expect (matchOscPattern ("fooba[rxz]foo", "foobarfoo"));
        expect (matchOscPattern ("fooba[rxz]foo", "foobaxfoo"));
        expect (matchOscPattern ("fooba[rxz]foo", "foobazfoo"));
        expect (! matchOscPattern ("fooba[rxz]foo", "foobasfoo"));

        expect (matchOscPattern ("foo[abc]foo[defgh]foo[i]foo[]foo", "foobfoohfooifoofoo"));

        // using [] for a range of chars:

        expect (matchOscPattern ("fooba[r-z]foo", "foobarfoo"));
        expect (matchOscPattern ("fooba[r-z]foo", "foobaxfoo"));
        expect (matchOscPattern ("fooba[r-z]foo", "foobazfoo"));
        expect (matchOscPattern ("fooba[r-z]foo", "foobasfoo"));
        expect (! matchOscPattern ("fooba[r-z]foo", "foobaRfoo"));

        expect (! matchOscPattern ("foo[1-8]bar", "foo0bar"));
        expect (matchOscPattern ("foo[1-8]bar", "foo1bar"));
        expect (matchOscPattern ("foo[1-8]bar", "foo6bar"));
        expect (matchOscPattern ("foo[1-8]bar", "foo8bar"));
        expect (! matchOscPattern ("foo[1-8]bar", "foo9bar"));

        // special case: '-' does not have a special meaning if it is at the end of the set.

        expect (matchOscPattern ("foo[abc-]bar", "fooabar"));
        expect (matchOscPattern ("foo[abc-]bar", "foo-bar"));
        expect (matchOscPattern ("foo[-]bar", "foo-bar"));

        // mixing both set and range:

        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]b[a-b]r", "fooabar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]b[a-a]r", "foobbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]b[aaaa-aaaa-aaaa]r", "foodbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooebar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foogbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooibar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foojbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fookbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foolbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooobar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foopbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooubar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooybar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foozbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foo0bar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foo1bar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foo5bar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foo8bar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "foo9bar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooCbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooDbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooEbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooFbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooGbar"));
        expect (matchOscPattern ("foo[ae-iko-uz1-8D-FX]bar", "fooXbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]ba[Rr]", "fooZbar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]ba[Rr]", "foobar"));
        expect (! matchOscPattern ("foo[ae-iko-uz1-8D-FX]ba[Rr]", "fooFXbar"));

        // using [!...] for a negated set or range of chars:

        expect (! matchOscPattern ("fooba[!rxz]foo", "foobarfoo"));
        expect (! matchOscPattern ("fooba[!rxz]foo", "foobaxfoo"));
        expect (! matchOscPattern ("fooba[!rxz]foo", "foobazfoo"));
        expect (matchOscPattern ("fooba[!rxz]foo", "foobasfoo"));

        expect (! matchOscPattern ("fooba[!r-z]foo", "foobarfoo"));
        expect (! matchOscPattern ("fooba[!r-z]foo", "foobaxfoo"));
        expect (! matchOscPattern ("fooba[!r-z]foo", "foobazfoo"));
        expect (! matchOscPattern ("fooba[!r-z]foo", "foobasfoo"));
        expect (matchOscPattern ("fooba[!r-z]foo", "foobaRfoo"));

        // special case: '!' does not have a special meaning if it is not the first char in the set.

        expect (matchOscPattern ("foo[ab!c]bar", "fooabar"));
        expect (matchOscPattern ("foo[ab!c]bar", "foo!bar"));
        expect (! matchOscPattern ("foo[ab!c]bar", "fooxbar"));
        expect (! matchOscPattern ("foo[!!]bar", "foo!bar"));
        expect (matchOscPattern ("foo[!!]bar", "fooxbar"));
        expect (! matchOscPattern ("foo[!!]bar", "foobar"));

        // should fail gracefully in case of wrong syntax:

        expect (! matchOscPattern ("notclosin[g", "notclosing"));
        expect (! matchOscPattern ("n]otopening", "notopening"));
        expect (! matchOscPattern ("[[nested]]", "nested"));
        expect (! matchOscPattern ("norangestar[-t]", "norangestart"));
        expect (! matchOscPattern ("norangestar[-t]", "norangestar-"));

    */
}

#[test] fn string_matching_combining_patterns() {
    /*
        expect (matchOscPattern ("*ea*ll[y-z0-9X-Zvwx]??m[o-q]l[e]x{fat,mat,pat}te{}r*?", "reallycomplexpattern"));
    */
}
