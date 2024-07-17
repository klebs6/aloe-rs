crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct CodeDocumentTest {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for CodeDocumentTest {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("CodeDocument", UnitTestCategories::text)
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl CodeDocumentTest {
    
    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            const String jabberwocky ("'Twas brillig, and the slithy toves\n"
                                            "Did gyre and gimble in the wabe;\n"
                                            "All mimsy were the borogoves,\n"
                                            "And the mome raths outgrabe.\n\n"

                                            "'Beware the Jabberwock, my son!\n"
                                            "The jaws that bite, the claws that catch!\n"
                                            "Beware the Jubjub bird, and shun\n"
                                            "The frumious Bandersnatch!'");

            {
                beginTest ("Basic checks");
                CodeDocument d;
                d.replaceAllContent (jabberwocky);

                expectEquals (d.getNumLines(), 9);
                expect (d.getLine (0).startsWith ("'Twas brillig"));
                expect (d.getLine (2).startsWith ("All mimsy"));
                expectEquals (d.getLine (4), String ("\n"));
            }

            {
                beginTest ("Insert/replace/delete");

                CodeDocument d;
                d.replaceAllContent (jabberwocky);

                d.insertText (CodeDocument::CodeDocumentPosition (d, 0, 6), "very ");
                expect (d.getLine (0).startsWith ("'Twas very brillig"),
                        "Insert text within a line");

                d.replaceSection (74, 83, "Quite hungry");
                expectEquals (d.getLine (2), String ("Quite hungry were the borogoves,\n"),
                              "Replace section at start of line");

                d.replaceSection (11, 18, "cold");
                expectEquals (d.getLine (0), String ("'Twas very cold, and the slithy toves\n"),
                              "Replace section within a line");

                d.deleteSection (CodeDocument::CodeDocumentPosition (d, 2, 0), CodeDocument::CodeDocumentPosition (d, 2, 6));
                expectEquals (d.getLine (2), String ("hungry were the borogoves,\n"),
                              "Delete section within a line");

                d.deleteSection (CodeDocument::CodeDocumentPosition (d, 2, 6), CodeDocument::CodeDocumentPosition (d, 5, 11));
                expectEquals (d.getLine (2), String ("hungry Jabberwock, my son!\n"),
                              "Delete section across multiple line");
            }

            {
                beginTest ("Line splitting and joining");

                CodeDocument d;
                d.replaceAllContent (jabberwocky);
                expectEquals (d.getNumLines(), 9);

                const String splitComment ("Adding a newline should split a line into two.");
                d.insertText (49, "\n");

                expectEquals (d.getNumLines(), 10, splitComment);
                expectEquals (d.getLine (1), String ("Did gyre and \n"), splitComment);
                expectEquals (d.getLine (2), String ("gimble in the wabe;\n"), splitComment);

                const String joinComment ("Removing a newline should join two lines.");
                d.deleteSection (CodeDocument::CodeDocumentPosition (d, 0, 35),
                                 CodeDocument::CodeDocumentPosition (d, 1, 0));

                expectEquals (d.getNumLines(), 9, joinComment);
                expectEquals (d.getLine (0), String ("'Twas brillig, and the slithy tovesDid gyre and \n"), joinComment);
                expectEquals (d.getLine (1), String ("gimble in the wabe;\n"), joinComment);
            }

            {
                beginTest ("Undo/redo");

                CodeDocument d;
                d.replaceAllContent (jabberwocky);
                d.newTransaction();
                d.insertText (30, "INSERT1");
                d.newTransaction();
                d.insertText (70, "INSERT2");
                d.undo();

                expect (d.getAllContent().contains ("INSERT1"), "1st edit should remain.");
                expect (! d.getAllContent().contains ("INSERT2"), "2nd edit should be undone.");

                d.redo();
                expect (d.getAllContent().contains ("INSERT2"), "2nd edit should be redone.");

                d.newTransaction();
                d.deleteSection (25, 90);
                expect (! d.getAllContent().contains ("INSERT1"), "1st edit should be deleted.");
                expect (! d.getAllContent().contains ("INSERT2"), "2nd edit should be deleted.");
                d.undo();
                expect (d.getAllContent().contains ("INSERT1"), "1st edit should be restored.");
                expect (d.getAllContent().contains ("INSERT2"), "1st edit should be restored.");

                d.undo();
                d.undo();
                expectEquals (d.getAllContent(), jabberwocky, "Original document should be restored.");
            }

            {
                beginTest ("Positions");

                CodeDocument d;
                d.replaceAllContent (jabberwocky);

                {
                    const String comment ("Keeps negative positions inside document.");
                    CodeDocument::CodeDocumentPosition p1 (d, 0, -3);
                    CodeDocument::CodeDocumentPosition p2 (d, -8);
                    expectEquals (p1.getLineNumber(), 0, comment);
                    expectEquals (p1.getIndexInLine(), 0, comment);
                    expectEquals (p1.getCharacter(), aloe_wchar ('\''), comment);
                    expectEquals (p2.getLineNumber(), 0, comment);
                    expectEquals (p2.getIndexInLine(), 0, comment);
                    expectEquals (p2.getCharacter(), aloe_wchar ('\''), comment);
                }

                {
                    const String comment ("Moving by character handles newlines correctly.");
                    CodeDocument::CodeDocumentPosition p1 (d, 0, 35);
                    p1.moveBy (1);
                    expectEquals (p1.getLineNumber(), 1, comment);
                    expectEquals (p1.getIndexInLine(), 0, comment);
                    p1.moveBy (75);
                    expectEquals (p1.getLineNumber(), 3, comment);
                }

                {
                    const String comment1 ("setPositionMaintained tracks position.");
                    const String comment2 ("setPositionMaintained tracks position following undos.");

                    CodeDocument::CodeDocumentPosition p1 (d, 3, 0);
                    p1.setPositionMaintained (true);
                    expectEquals (p1.getCharacter(), aloe_wchar ('A'), comment1);

                    d.newTransaction();
                    d.insertText (p1, "INSERT1");

                    expectEquals (p1.getCharacter(), aloe_wchar ('A'), comment1);
                    expectEquals (p1.getLineNumber(), 3, comment1);
                    expectEquals (p1.getIndexInLine(), 7, comment1);
                    d.undo();
                    expectEquals (p1.getIndexInLine(), 0, comment2);

                    d.newTransaction();
                    d.insertText (15, "\n");

                    expectEquals (p1.getLineNumber(), 4, comment1);
                    d.undo();
                    expectEquals (p1.getLineNumber(), 3, comment2);
                }
            }

            {
                beginTest ("Iterators");

                CodeDocument d;
                d.replaceAllContent (jabberwocky);

                {
                    const String comment1 ("Basic iteration.");
                    const String comment2 ("Reverse iteration.");
                    const String comment3 ("Reverse iteration stops at doc start.");
                    const String comment4 ("Check iteration across line boundaries.");

                    CodeDocument::CodeDocumentIterator it (d);
                    expectEquals (it.peekNextChar(), aloe_wchar ('\''), comment1);
                    expectEquals (it.nextChar(), aloe_wchar ('\''), comment1);
                    expectEquals (it.nextChar(), aloe_wchar ('T'), comment1);
                    expectEquals (it.nextChar(), aloe_wchar ('w'), comment1);
                    expectEquals (it.peekNextChar(), aloe_wchar ('a'), comment2);
                    expectEquals (it.previousChar(), aloe_wchar ('w'), comment2);
                    expectEquals (it.previousChar(), aloe_wchar ('T'), comment2);
                    expectEquals (it.previousChar(), aloe_wchar ('\''), comment2);
                    expectEquals (it.previousChar(), aloe_wchar (0), comment3);
                    expect (it.isSOF(), comment3);

                    while (it.peekNextChar() != aloe_wchar ('D')) // "Did gyre..."
                        it.nextChar();

                    expectEquals (it.nextChar(), aloe_wchar ('D'), comment3);
                    expectEquals (it.peekNextChar(), aloe_wchar ('i'), comment3);
                    expectEquals (it.previousChar(), aloe_wchar ('D'), comment3);
                    expectEquals (it.previousChar(), aloe_wchar ('\n'), comment3);
                    expectEquals (it.previousChar(), aloe_wchar ('s'), comment3);
                }

                {
                    const String comment1 ("CodeDocumentIterator created from CodeDocument::CodeDocumentPosition objects.");
                    const String comment2 ("CodeDocument::CodeDocumentPosition created from CodeDocumentIterator objects.");
                    const String comment3 ("CodeDocument::CodeDocumentPosition created from EOF CodeDocumentIterator objects.");

                    CodeDocument::CodeDocumentPosition p (d, 6, 0); // "The jaws..."
                    CodeDocument::CodeDocumentIterator it (p);

                    expectEquals (it.nextChar(), aloe_wchar ('T'), comment1);
                    expectEquals (it.nextChar(), aloe_wchar ('h'), comment1);
                    expectEquals (it.previousChar(), aloe_wchar ('h'), comment1);
                    expectEquals (it.previousChar(), aloe_wchar ('T'), comment1);
                    expectEquals (it.previousChar(), aloe_wchar ('\n'), comment1);
                    expectEquals (it.previousChar(), aloe_wchar ('!'), comment1);

                    const auto p2 = it.toPosition();
                    expectEquals (p2.getLineNumber(), 5, comment2);
                    expectEquals (p2.getIndexInLine(), 30, comment2);

                    while (! it.isEOF())
                        it.nextChar();

                    const auto p3 = it.toPosition();
                    expectEquals (p3.getLineNumber(), d.getNumLines() - 1, comment3);
                    expectEquals (p3.getIndexInLine(), d.getLine (d.getNumLines() - 1).length(), comment3);
                }
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static CodeDocumentTest codeDocumentTests;
    */
}
