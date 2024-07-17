crate::ix!();

#[cfg(ALOE_UNIT_TESTS)]
pub struct FileTests {
    base: UnitTest,
}

#[cfg(ALOE_UNIT_TESTS)]
impl Default for FileTests {
    
    fn default() -> Self {
        todo!();
        /*


            : UnitTest ("Files", UnitTestCategories::files
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
impl FileTests {

    pub fn run_test(&mut self)  {
        
        todo!();
        /*
            beginTest ("Reading");

            const File home (File::getSpecialLocation (File::userHomeDirectory));
            const File temp (File::getSpecialLocation (File::tempDirectory));

            expect (! File().exists());
            expect (! File().existsAsFile());
            expect (! File().isDirectory());
           #if ! ALOE_WINDOWS
            expect (File("/").isDirectory());
           #endif
            expect (home.isDirectory());
            expect (home.exists());
            expect (! home.existsAsFile());
            expect (File::getSpecialLocation (File::userApplicationDataDirectory).isDirectory());
            expect (File::getSpecialLocation (File::currentExecutableFile).exists());
            expect (File::getSpecialLocation (File::currentApplicationFile).exists());
            expect (File::getSpecialLocation (File::invokedExecutableFile).exists());
            expect (home.getVolumeTotalSize() > 1024 * 1024);
            expect (home.getBytesFreeOnVolume() > 0);
            expect (! home.isHidden());
            expect (home.isOnHardDisk());
            expect (! home.isOnCDRomDrive());
            expect (File::getCurrentWorkingDirectory().exists());
            expect (home.setAsCurrentWorkingDirectory());

            {
                auto homeParent = home;
                bool noSymlinks = true;

                while (! homeParent.isRoot())
                {
                    if (homeParent.isSymbolicLink())
                    {
                        noSymlinks = false;
                        break;
                    }

                    homeParent = homeParent.getParentDirectory();
                }

                if (noSymlinks)
                    expect (File::getCurrentWorkingDirectory() == home);
            }

            {
                Vec<File> roots;
                File::findFileSystemRoots (roots);
                expect (roots.size() > 0);

                int numRootsExisting = 0;
                for (int i = 0; i < roots.size(); ++i)
                    if (roots[i].exists())
                        ++numRootsExisting;

                // (on windows, some of the drives may not contain media, so as long as at least one is ok..)
                expect (numRootsExisting > 0);
            }

            beginTest ("Writing");

            auto random = getRandom();
            const auto tempFolderName = "Aloe UnitTests Temp Folder "
                                      + String::toHexString (random.nextInt())
                                      + ".folder";
            File demoFolder (temp.getChildFile (tempFolderName));
            expect (demoFolder.deleteRecursively());
            expect (demoFolder.createDirectory());
            expect (demoFolder.isDirectory());
            expect (demoFolder.getParentDirectory() == temp);
            expect (temp.isDirectory());
            expect (temp.findChildFiles (File::findFilesAndDirectories, false, "*").contains (demoFolder));
            expect (temp.findChildFiles (File::findDirectories, true, "*.folder").contains (demoFolder));

            File tempFile (demoFolder.getNonexistentChildFile ("test", ".txt", false));

            expect (tempFile.getFileExtension() == ".txt");
            expect (tempFile.hasFileExtension (".txt"));
            expect (tempFile.hasFileExtension ("txt"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension (".xyz"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension ("abc;xyz;foo"));
            expect (tempFile.withFileExtension ("xyz").hasFileExtension ("xyz;foo"));
            expect (! tempFile.withFileExtension ("h").hasFileExtension ("bar;foo;xx"));
            expect (tempFile.getSiblingFile ("foo").isAChildOf (temp));
            expect (tempFile.hasWriteAccess());

            expect (home.getChildFile (".") == home);
            expect (home.getChildFile ("..") == home.getParentDirectory());
            expect (home.getChildFile (".xyz").getFileName() == ".xyz");
            expect (home.getChildFile ("..xyz").getFileName() == "..xyz");
            expect (home.getChildFile ("...xyz").getFileName() == "...xyz");
            expect (home.getChildFile ("./xyz") == home.getChildFile ("xyz"));
            expect (home.getChildFile ("././xyz") == home.getChildFile ("xyz"));
            expect (home.getChildFile ("../xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile (".././xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile (".././xyz/./abc") == home.getParentDirectory().getChildFile ("xyz/abc"));
            expect (home.getChildFile ("./../xyz") == home.getParentDirectory().getChildFile ("xyz"));
            expect (home.getChildFile ("a1/a2/a3/./../../a4") == home.getChildFile ("a1/a4"));

            {
                FileOutputStream fo (tempFile);
                fo.write ("0123456789", 10);
            }

            expect (tempFile.exists());
            expect (tempFile.getSize() == 10);
            expect (std::abs ((int) (tempFile.getLastModificationTime().toMilliseconds() - Time::getCurrentTime().toMilliseconds())) < 3000);
            expectEquals (tempFile.loadFileAsString(), String ("0123456789"));
            expect (! demoFolder.containsSubDirectories());

            expectEquals (tempFile.getRelativePathFrom (demoFolder.getParentDirectory()), demoFolder.getFileName() + File::getSeparatorString() + tempFile.getFileName());
            expectEquals (demoFolder.getParentDirectory().getRelativePathFrom (tempFile), ".." + File::getSeparatorString() + ".." + File::getSeparatorString() + demoFolder.getParentDirectory().getFileName());

            expect (demoFolder.getNumberOfChildFiles (File::findFiles) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findFilesAndDirectories) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findDirectories) == 0);
            demoFolder.getNonexistentChildFile ("tempFolder", "", false).createDirectory();
            expect (demoFolder.getNumberOfChildFiles (File::findDirectories) == 1);
            expect (demoFolder.getNumberOfChildFiles (File::findFilesAndDirectories) == 2);
            expect (demoFolder.containsSubDirectories());

            expect (tempFile.hasWriteAccess());
            tempFile.setReadOnly (true);
            expect (! tempFile.hasWriteAccess());
            tempFile.setReadOnly (false);
            expect (tempFile.hasWriteAccess());

            Time t (Time::getCurrentTime());
            tempFile.setLastModificationTime (t);
            Time t2 = tempFile.getLastModificationTime();
            expect (std::abs ((int) (t2.toMilliseconds() - t.toMilliseconds())) <= 1000);

            {
                MemoryBlock mb;
                tempFile.loadFileAsData (mb);
                expect (mb.getSize() == 10);
                expect (mb[0] == '0');
            }

            {
                expect (tempFile.getSize() == 10);
                FileOutputStream fo (tempFile);
                expect (fo.openedOk());

                expect (fo.setPosition  (7));
                expect (fo.truncate().wasOk());
                expect (tempFile.getSize() == 7);
                fo.write ("789", 3);
                fo.flush();
                expect (tempFile.getSize() == 10);
            }

            beginTest ("Memory-mapped files");

            {
                MemoryMappedFile mmf (tempFile, MemoryMappedFile::readOnly);
                expect (mmf.getSize() == 10);
                expect (mmf.getData() != nullptr);
                expect (memcmp (mmf.getData(), "0123456789", 10) == 0);
            }

            {
                const File tempFile2 (tempFile.getNonexistentSibling (false));
                expect (tempFile2.create());
                expect (tempFile2.appendData ("xxxxxxxxxx", 10));

                {
                    MemoryMappedFile mmf (tempFile2, MemoryMappedFile::readWrite);
                    expect (mmf.getSize() == 10);
                    expect (mmf.getData() != nullptr);
                    memcpy (mmf.getData(), "abcdefghij", 10);
                }

                {
                    MemoryMappedFile mmf (tempFile2, MemoryMappedFile::readWrite);
                    expect (mmf.getSize() == 10);
                    expect (mmf.getData() != nullptr);
                    expect (memcmp (mmf.getData(), "abcdefghij", 10) == 0);
                }

                expect (tempFile2.deleteFile());
            }

            beginTest ("More writing");

            expect (tempFile.appendData ("abcdefghij", 10));
            expect (tempFile.getSize() == 20);
            expect (tempFile.replaceWithData ("abcdefghij", 10));
            expect (tempFile.getSize() == 10);

            File tempFile2 (tempFile.getNonexistentSibling (false));
            expect (tempFile.copyFileTo (tempFile2));
            expect (tempFile2.exists());
            expect (tempFile2.hasIdenticalContentTo (tempFile));
            expect (tempFile.deleteFile());
            expect (! tempFile.exists());
            expect (tempFile2.moveFileTo (tempFile));
            expect (tempFile.exists());
            expect (! tempFile2.exists());

            expect (demoFolder.deleteRecursively());
            expect (! demoFolder.exists());

            {
                Url url ("https://audio.dev/foo/bar/");
                expectEquals (url.toString (false), String ("https://audio.dev/foo/bar/"));
                expectEquals (url.getChildURL ("x").toString (false), String ("https://audio.dev/foo/bar/x"));
                expectEquals (url.getParentURL().toString (false), String ("https://audio.dev/foo"));
                expectEquals (url.getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/foo/x"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/x"));
            }

            {
                Url url ("https://audio.dev/foo/bar");
                expectEquals (url.toString (false), String ("https://audio.dev/foo/bar"));
                expectEquals (url.getChildURL ("x").toString (false), String ("https://audio.dev/foo/bar/x"));
                expectEquals (url.getParentURL().toString (false), String ("https://audio.dev/foo"));
                expectEquals (url.getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().toString (false), String ("https://audio.dev/"));
                expectEquals (url.getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/foo/x"));
                expectEquals (url.getParentURL().getParentURL().getParentURL().getChildURL ("x").toString (false), String ("https://audio.dev/x"));
            }
        */
    }
}

#[cfg(ALOE_UNIT_TESTS)]
lazy_static!{
    /*
    static FileTests fileUnitTests;
    */
}
