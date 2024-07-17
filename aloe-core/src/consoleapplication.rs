crate::ix!();

//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_ConsoleApplication.h]

/**
  | Holds a list of command-line arguments,
  | and provides useful methods for searching
  | and operating on them.
  | 
  | You can create an ArgumentList manually,
  | or give it some argv/argc values from
  | a main() function to parse.
  | 
  | @see ConsoleApplication
  | 
  | @tags{Core}
  |
  */
pub struct ArgumentList {

    /**
      The name or path of the executable that was invoked,
      as it was specified on the command-line.
      */
    executable_name: String,

    /**
      The list of arguments (not including the name of
      the executable that was invoked).
      */
    arguments:       Vec<argument_list::Argument>,
}

pub mod argument_list {

    use super::*;
    
    /**
      | One of the arguments in an ArgumentList.
      | 
      | @tags{Core}
      |
      */
    pub struct Argument {

        /**
          The original text of this argument.
          */
        text: String,
    }

    impl PartialEq<&str> for Argument {
        
        /**
          | Compares this argument against a string.
          | 
          | The string may be a pipe-separated list
          | of options, e.g. "--help|-h"
          |
          */
        #[inline] fn eq(&self, other: &&str) -> bool {
            todo!();
            /*
            
            */
        }
    }

    impl Argument {

        /**
          | Resolves this argument as an absolute
          | File, using the current working directory
          | as a base for resolving relative paths,
          | and stripping quotes, etc.
          |
          */
        pub fn resolve_as_file(&self) -> File {
            
            todo!();
            /*
                return resolveFilename (text);
            */
        }
        
        /**
          | Resolves this argument as an absolute
          | File, using the current working directory
          | as a base for resolving relative paths,
          | and also doing a check to make sure the
          | file exists.
          | 
          | If the file doesn't exist, this will
          | call fail() with a suitable error. @see
          | resolveAsFile, resolveAsExistingFolder
          |
          */
        pub fn resolve_as_existing_file(&self) -> File {
            
            todo!();
            /*
                return checkFileExists (resolveAsFile());
            */
        }
        
        /**
          | Resolves a user-supplied folder name
          | into an absolute File, using the current
          | working directory as a base for resolving
          | relative paths, and also doing a check
          | to make sure the folder exists.
          | 
          | If the folder doesn't exist, this will
          | call fail() with a suitable error. @see
          | resolveAsFile, resolveAsExistingFile
          |
          */
        pub fn resolve_as_existing_folder(&self) -> File {
            
            todo!();
            /*
                auto f = resolveAsFile();

            if (! f.isDirectory())
                ConsoleApplication::fail ("Could not find folder: " + f.getFullPathName());

            return f;
            */
        }
        
        /**
          | Returns true if this argument starts
          | with a double dash.
          |
          */
        pub fn is_long_option(&self) -> bool {
            
            todo!();
            /*
                return isLongOptionFormat (text);
            */
        }
        
        /**
          | Returns true if this argument starts
          | with a single dash.
          |
          */
        pub fn is_short_option(&self) -> bool {
            
            todo!();
            /*
                return isShortOptionFormat (text);
            */
        }
        
        /**
          | Returns true if this argument starts
          | with one or more dashes.
          |
          */
        pub fn is_option(&self) -> bool {
            
            todo!();
            /*
                return isOptionFormat (text);
            */
        }
        
        /**
          | Returns true if this argument starts
          | with a double dash, followed by the given
          | string.
          |
          */
        pub fn is_long_option_with_arg(&self, option: &String) -> bool {
            
            todo!();
            /*
                if (! isLongOptionFormat (option))
            {
                jassert (! isShortOptionFormat (option)); // this will always fail to match
                return isLongOption ("--" + option);
            }

            return text.upToFirstOccurrenceOf ("=", false, false) == option;
            */
        }
        
        /**
          | If this argument is a long option with
          | a value, this returns the value. e.g.
          | for "--foo=bar", this would return
          | 'bar'.
          |
          */
        pub fn get_long_option_value(&self) -> String {
            
            todo!();
            /*
                if (isLongOption())
            {
                auto equalsIndex = text.indexOfChar ('=');

                if (equalsIndex > 0)
                    return text.substring (equalsIndex + 1);
            }

            return {};
            */
        }
        
        /**
          | Returns true if this argument starts
          | with a single dash and then contains
          | the given character somewhere inside
          | it.
          |
          */
        pub fn is_short_option_with_u8(&self, option: u8) -> bool {
            
            todo!();
            /*
                jassert (option != '-'); // this is probably not what you intended to pass in

            return isShortOption() && text.containsChar (String (option)[0]);
            */
        }
        
        #[inline] fn eq(&self, other: &&str) -> bool {
            todo!();
            /*
                for (auto& o : StringArray::fromTokens (wildcard, "|", {}))
            {
                if (text == o)
                    return true;

                if (isShortOptionFormat (o) && o.length() == 2 && isShortOption ((char) o[1]))
                    return true;

                if (isLongOptionFormat (o) && isLongOption (o))
                    return true;
            }

            return false;
            */
        }
    }
}

impl Index<i32> for ArgumentList {

    type Output = argument_list::Argument;
    
    /**
      | Returns one of the arguments
      |
      */
    #[inline] fn index(&self, index: i32) -> &Self::Output {
        todo!();
        /*
            return arguments[index];
        */
    }
}

/**
  | Represents a the set of commands that
  | a console app can perform, and provides
  | helper functions for performing them.
  | 
  | When using these helper classes to implement
  | a console app, you probably want to do
  | something along these lines:
  | 
  | -----------
  | @code
  | 
  | int main (int argc, char* argv[])
  | {
  |     ConsoleApplication app;
  | 
  |     app.addHelpCommand ("--help|-h", "Usage:", true);
  |     app.addVersionCommand ("--version|-v", "MyApp version 1.2.3");
  | 
  |     app.addCommand ({ "--foo",
  |                       "--foo filename",
  |                       "Performs a foo operation on the given file",
  |                       [] (const auto& args) { doFoo (args); }});
  | 
  |     return app.findAndRunCommand (argc, argv);
  | }
  | 
  | @see ArgumentList
  | 
  | @tags{Core}
  |
  */
pub struct ConsoleApplication {
    commands:                        Vec<ConsoleApplicationCommand>,
    command_if_no_others_recognised: i32, // default = -1
}

pub struct ConsoleAppFailureCode
{
    error_message: String,
    return_code:   i32,
}

/**
  | Represents a command that can be executed
  | if its command-line arguments are matched.
  | 
  | @see ConsoleApplication::addCommand(),
  | ConsoleApplication::findAndRunCommand()
  | 
  | @tags{Core}
  |
  */
pub struct ConsoleApplicationCommand
{

    /**
      | The option string that must appear in the argument
      | list for this command to be invoked. This can also
      | be a list of different versions separated by pipes,
      | e.g. "--help|-h"
      */
    command_option:       String,

    /**
      | A description of the command-line arguments needed
      | for this command, which will be printed as part
      | of the help text.
      */
    argument_description: String,

    /**
      A short (one line) description of this command,
      which can be printed by ConsoleApplication::printCommandList().
      */
    short_description:    String,

    /**
      A longer description of this command, for use in
      extended help.
      */
    long_description:     String,

    /**
      The actual command that should be invoked to perform
      this action.
      */
    command:              fn(_0: &ArgumentList) -> (),
}


//-------------------------------------------[.cpp/Aloe/modules/aloe_core/misc/aloe_ConsoleApplication.cpp]
impl ConsoleApplication {

    /**
      | Throws a failure exception to cause
      | a command-line app to terminate.
      | 
      | This is intended to be called from code
      | in a Command, so that the exception will
      | be automatically caught and turned
      | into a printed error message and a return
      | code which will be returned from main().
      | @see ConsoleApplication::invokeCatchingFailures()
      |
      */
    pub fn fail(
        &mut self, 
        error_message: String,
        return_code:   Option<i32>

    ) {

        let return_code: i32 = return_code.unwrap_or(1);
        
        todo!();
        /*
            throw ConsoleAppFailureCode { std::move (errorMessage), returnCode };
        */
    }
    
    /**
      | Invokes a function, catching any fail()
      | calls that it might trigger, and handling
      | them by printing their error message
      | and returning their error code. @see
      | ConsoleApplication::fail()
      |
      */
    pub fn invoke_catching_failures(&mut self, f: fn() -> i32) -> i32 {
        
        todo!();
        /*
            int returnCode = 0;

        try
        {
            returnCode = f();
        }
        catch (const ConsoleAppFailureCode& error)
        {
            std::cerr << error.errorMessage << std::endl;
            returnCode = error.returnCode;
        }

        return returnCode;
        */
    }
    
    /**
      | Looks for the first command in the list
      | which matches the given arguments.
      | 
      | If none is found, this returns either
      | the default command (if one is set) or
      | nullptr.
      | 
      | If optionMustBeFirstArg is true, then
      | only the first argument will be looked
      | at when searching the available commands
      | - this lets you do 'git' style commands
      | where the executable name is followed
      | by a verb.
      |
      */
    pub fn find_command(&self, 
        args:                     &ArgumentList,
        option_must_be_first_arg: bool) -> *const ConsoleApplicationCommand {
        
        todo!();
        /*
            for (auto& c : commands)
        {
            auto index = args.indexOfOption (c.commandOption);

            if (optionMustBeFirstArg ? (index == 0) : (index >= 0))
                return &c;
        }

        if (commandIfNoOthersRecognised >= 0)
            return &commands[(size_t) commandIfNoOthersRecognised];

        return {};
        */
    }
    
    /**
      | Looks for the first command in the list
      | which matches the given arguments,
      | and tries to invoke it.
      | 
      | If no command is found, and if there is
      | no default command to run, it fails with
      | a suitable error message. If the command
      | calls the fail() function, this will
      | throw an exception that gets automatically
      | caught and handled, and this method
      | will return the error code that was passed
      | into the fail() call.
      | 
      | If optionMustBeFirstArg is true, then
      | only the first argument will be looked
      | at when searching the available commands
      | - this lets you do 'git' style commands
      | where the executable name is followed
      | by a verb.
      |
      */
    pub fn find_and_run_command_with_arglist(
        &self, 
        args:                     &ArgumentList,
        option_must_be_first_arg: Option<bool>

    ) -> i32 {

        let option_must_be_first_arg: bool = option_must_be_first_arg.unwrap_or(false);
        
        todo!();
        /*
            return invokeCatchingFailures ([&args, optionMustBeFirstArg, this]
        {
            if (auto c = findCommand (args, optionMustBeFirstArg))
                c->command (args);
            else
                fail ("Unrecognised arguments");

            return 0;
        });
        */
    }
    
    /**
      | Creates an ArgumentList object from
      | the argc and argv variablrs, and invokes
      | findAndRunCommand() using it.
      |
      */
    pub fn find_and_run_command(&self, 
        argc: i32,
        argv: *mut &[u8]) -> i32 {
        
        todo!();
        /*
            return findAndRunCommand (ArgumentList (argc, argv));
        */
    }
    
    /**
      | Adds a command to the list.
      |
      */
    pub fn add_command(&mut self, c: ConsoleApplicationCommand)  {
        
        todo!();
        /*
            commands.emplace_back (std::move (c));
        */
    }
    
    /**
      | Adds a command to the list, and marks
      | it as one which is invoked if no other
      | command matches.
      |
      */
    pub fn add_default_command(&mut self, c: ConsoleApplicationCommand)  {
        
        todo!();
        /*
            commandIfNoOthersRecognised = (int) commands.size();
        addCommand (std::move (c));
        */
    }
    
    /**
      | Adds a help command to the list.
      | 
      | This command will print the user-supplied
      | message that's passed in here as an argument,
      | followed by a list of all the registered
      | commands.
      |
      */
    pub fn add_help_command(&mut self, 
        arg:                  String,
        help_message:         String,
        make_default_command: bool)  {
        
        todo!();
        /*
            ConsoleApplicationCommand c { arg, arg, "Prints the list of commands", {},
                    [this, helpMessage] (const ArgumentList& args)
                    {
                        std::cout << helpMessage << std::endl;
                        printCommandList (args);
                    }};

        if (makeDefaultCommand)
            addDefaultCommand (std::move (c));
        else
            addCommand (std::move (c));
        */
    }
    
    /**
      | Adds a command that will print the given
      | text in response to the "--version"
      | option.
      |
      */
    pub fn add_version_command(&mut self, 
        arg:          String,
        version_text: String)  {
        
        todo!();
        /*
            addCommand ({ arg, arg, "Prints the current version number", {},
                      [versionText] (const ArgumentList&)
                      {
                          std::cout << versionText << std::endl;
                      }});
        */
    }
    
    /**
      | Gives read-only access to the list of
      | registered commands.
      |
      */
    pub fn get_commands(&self) -> &Vec<ConsoleApplicationCommand> {
        
        todo!();
        /*
            return commands;
        */
    }
    
    pub fn get_exe_name_and_args(
        args:    &ArgumentList,
        command: &ConsoleApplicationCommand) -> String {
        
        todo!();
        /*
            auto exeName = args.executableName.fromLastOccurrenceOf ("/", false, false)
                                          .fromLastOccurrenceOf ("\\", false, false);

        return " " + exeName + " " + command.argumentDescription;
        */
    }
    
    pub fn print_command_description(
        args:               &ArgumentList,
        command:            &ConsoleApplicationCommand,
        description_indent: i32)  {
        
        todo!();
        /*
            auto nameAndArgs = getExeNameAndArgs (args, command);

        if (nameAndArgs.length() > descriptionIndent)
            std::cout << nameAndArgs << std::endl << String().paddedRight (' ', descriptionIndent);
        else
            std::cout << nameAndArgs.paddedRight (' ', descriptionIndent);

        std::cout << command.shortDescription << std::endl;
        */
    }
    
    /**
      | Prints out the list of commands and their
      | short descriptions in a format that's
      | suitable for use as help.
      |
      */
    pub fn print_command_list(&self, args: &ArgumentList)  {
        
        todo!();
        /*
            int descriptionIndent = 0;

        for (auto& c : commands)
            descriptionIndent = std::max (descriptionIndent, getExeNameAndArgs (args, c).length());

        descriptionIndent = std::min (descriptionIndent + 2, 40);

        for (auto& c : commands)
            printCommandDescription (args, c, descriptionIndent);

        std::cout << std::endl;
        */
    }
    
    /**
      | Prints out a longer description of a
      | particular command, based on its longDescription
      | member.
      |
      */
    pub fn print_command_details(&self, 
        args:    &ArgumentList,
        command: &ConsoleApplicationCommand)  {
        
        todo!();
        /*
            auto len = getExeNameAndArgs (args, command).length();

        printCommandDescription (args, command, std::min (len + 3, 40));

        if (command.longDescription.isNotEmpty())
            std::cout << std::endl << command.longDescription << std::endl;
        */
    }
}

pub fn resolve_filename(name: &String) -> File {
    
    todo!();
    /*
        return File::getCurrentWorkingDirectory().getChildFile (name.unquoted());
    */
}

pub fn check_file_exists(f: &File) -> File {
    
    todo!();
    /*
        if (! f.exists())
            ConsoleApplication::fail ("Could not find file: " + f.getFullPathName());

        return f;
    */
}

pub fn check_folder_exists(f: &File) -> File {
    
    todo!();
    /*
        if (! f.isDirectory())
            ConsoleApplication::fail ("Could not find folder: " + f.getFullPathName());

        return f;
    */
}

pub fn resolve_filename_for_option(
        args:     &ArgumentList,
        option:   &str,
        filename: &String) -> File {
    
    todo!();
    /*
        if (filename.isEmpty())
        {
            args.failIfOptionIsMissing (option);
            ConsoleApplication::fail ("Expected a filename after the " + option + " option");
        }

        return resolveFilename (filename);
    */
}

pub fn is_short_option_format(s: &str) -> bool {
    
    todo!();
    /*
        return s[0] == '-' && s[1] != '-';
    */
}

pub fn is_long_option_format(s: &str) -> bool {
    
    todo!();
    /*
        return s[0] == '-' && s[1] == '-' && s[2] != '-';
    */
}

pub fn is_option_format(s: &str) -> bool {
    
    todo!();
    /*
        return s[0] == '-';
    */
}

impl ArgumentList {
    
    /**
      | Creates an argument list for a given
      | executable.
      |
      */
    pub fn new_with_exe_name(
        exe_name: String,
        args:     StringArray) -> Self {
    
        todo!();
        /*
        : executable_name(std::move (exeName)),

            args.trim();
        args.removeEmptyStrings();

        for (auto& a : args)
            arguments.add ({ a.unquoted() });
        */
    }
    
    /**
      | Parses a standard argv/argc pair to
      | create an argument list.
      |
      */
    pub fn new_with_argcv(
        argc: i32,
        argv: *mut &[u8]) -> Self {
    
        todo!();
        /*


            : ArgumentList (argv[0], StringArray (argv + 1, argc - 1))
        */
    }
    
    /**
      | Tokenises a string containing all the
      | arguments to create an argument list.
      |
      */
    pub fn new_with_exe_name_and_argstring(
        exe_name: &String,
        args:     &String) -> Self {
    
        todo!();
        /*


            : ArgumentList (exeName, StringArray::fromTokens (args, true))
        */
    }
    
    /**
      | Returns the number of arguments in the
      | list.
      |
      */
    pub fn size(&self) -> i32 {
        
        todo!();
        /*
            return arguments.size();
        */
    }
    
    /**
      | Throws an error unless there are at least
      | the given number of arguments.
      |
      */
    pub fn check_min_num_arguments(&self, expected_min_number_of_args: i32)  {
        
        todo!();
        /*
            if (size() < expectedMinNumberOfArgs)
            ConsoleApplication::fail ("Not enough arguments!");
        */
    }
    
    /**
      | Returns the index of the given string
      | if it matches one of the arguments, or
      | -1 if it doesn't.
      | 
      | The option can also be a list of different
      | versions separated by pipes, e.g. "--help|-h"
      |
      */
    pub fn index_of_option(&self, option: &str) -> i32 {
        
        todo!();
        /*
            jassert (option == String (option).trim()); // passing non-trimmed strings will always fail to find a match!

        for (int i = 0; i < arguments.size(); ++i)
            if (arguments.getReference (i) == option)
                return i;

        return -1;
        */
    }
    
    /**
      | Returns true if the given string matches
      | one of the arguments.
      | 
      | The option can also be a list of different
      | versions separated by pipes, e.g. "--help|-h"
      | @see removeOptionIfFound
      |
      */
    pub fn contains_option(&self, option: &str) -> bool {
        
        todo!();
        /*
            return indexOfOption (option) >= 0;
        */
    }
    
    /**
      | Returns true if the given string matches
      | one of the arguments, and also removes
      | the argument from the list if found.
      | 
      | The option can also be a list of different
      | versions separated by pipes, e.g. "--help|-h"
      | @see containsOption
      |
      */
    pub fn remove_option_if_found(&mut self, option: &str) -> bool {
        
        todo!();
        /*
            auto i = indexOfOption (option);

        if (i >= 0)
            arguments.remove (i);

        return i >= 0;
        */
    }
    
    /**
      | Throws an error unless the given option
      | is found in the argument list.
      |
      */
    pub fn fail_if_option_is_missing(&self, option: &str)  {
        
        todo!();
        /*
            if (indexOfOption (option) < 0)
            ConsoleApplication::fail ("Expected the option " + option);
        */
    }
    
    /**
      | Looks for a given argument and returns
      | either its assigned value (for long
      | options) or the string that follows
      | it (for short options).
      | 
      | The option can also be a list of different
      | versions separated by pipes, e.g. "--help|-h"
      | 
      | If it finds a long option, it will look
      | for an assignment with a '=' sign, e.g.
      | "--file=foo.txt", and will return
      | the string following the '='. If there's
      | no '=', it will return an empty string.
      | 
      | If it finds a short option, it will attempt
      | to return the argument that follows
      | it, unless it's another option.
      | 
      | If the argument isn't found, this returns
      | an empty string.
      |
      */
    pub fn get_value_for_option(&self, option: &str) -> String {
        
        todo!();
        /*
            jassert (isOptionFormat (option)); // the thing you're searching for must be an option

        for (int i = 0; i < arguments.size(); ++i)
        {
            auto& arg = arguments.getReference(i);

            if (arg == option)
            {
                if (arg.isShortOption())
                {
                    if (i < arguments.size() - 1 && ! arguments.getReference (i + 1).isOption())
                        return arguments.getReference (i + 1).text;

                    return {};
                }

                if (arg.isLongOption())
                    return arg.getLongOptionValue();
            }
        }

        return {};
        */
    }
    
    /**
      | Looks for a given argument and returns
      | either its assigned value (for long
      | options) or the string that follows
      | it (for short options).
      | 
      | This works like getValueForOption()
      | but also removes the option argument
      | (and any value arguments) from the list
      | if they are found.
      |
      */
    pub fn remove_value_for_option(&mut self, option: &str) -> String {
        
        todo!();
        /*
            jassert (isOptionFormat (option)); // the thing you're searching for must be an option

        for (int i = 0; i < arguments.size(); ++i)
        {
            auto& arg = arguments.getReference(i);

            if (arg == option)
            {
                if (arg.isShortOption())
                {
                    if (i < arguments.size() - 1 && ! arguments.getReference (i + 1).isOption())
                    {
                        auto result = arguments.getReference (i + 1).text;
                        arguments.removeRange (i, 2);
                        return result;
                    }

                    arguments.remove (i);
                    return {};
                }

                if (arg.isLongOption())
                {
                    auto result = arg.getLongOptionValue();
                    arguments.remove (i);
                    return result;
                }
            }
        }

        return {};
        */
    }
    
    /**
      | Looks for the value of argument using
      | getValueForOption() and tries to parse
      | that value as a file.
      | 
      | If the option isn't found, or if the value
      | can't be parsed as a filename, it will
      | throw an error.
      |
      */
    pub fn get_file_for_option(&self, option: &str) -> File {
        
        todo!();
        /*
            return resolveFilenameForOption (*this, option, getValueForOption (option));
        */
    }
    
    /**
      | Looks for the value of argument using
      | getValueForOption() and tries to parse
      | that value as a file.
      | 
      | This works like getFileForOption()
      | but also removes the option argument
      | (and any value arguments) from the list
      | if they are found.
      |
      */
    pub fn get_file_for_option_and_remove(&mut self, option: &str) -> File {
        
        todo!();
        /*
            return resolveFilenameForOption (*this, option, removeValueForOption (option));
        */
    }
    
    /**
      | Looks for a file argument using getFileForOption()
      | and fails with a suitable error if the
      | file doesn't exist.
      |
      */
    pub fn get_existing_file_for_option(&self, option: &str) -> File {
        
        todo!();
        /*
            return checkFileExists (getFileForOption (option));
        */
    }
    
    /**
      | Looks for a file argument using getFileForOption()
      | and fails with a suitable error if the
      | file doesn't exist.
      | 
      | This works like getExistingFileForOption()
      | but also removes the option argument
      | (and any value arguments) from the list
      | if they are found.
      |
      */
    pub fn get_existing_file_for_option_and_remove(&mut self, option: &str) -> File {
        
        todo!();
        /*
            return checkFileExists (getFileForOptionAndRemove (option));
        */
    }
    
    /**
      | Looks for a filename argument using
      | getFileForOption() and fails with
      | a suitable error if the file isn't a folder
      | that exists.
      |
      */
    pub fn get_existing_folder_for_option(&self, option: &str) -> File {
        
        todo!();
        /*
            return checkFolderExists (getFileForOption (option));
        */
    }
    
    /**
      | Looks for a filename argument using
      | getFileForOption() and fails with
      | a suitable error if the file isn't a folder
      | that exists.
      | 
      | This works like getExistingFolderForOption()
      | but also removes the option argument
      | (and any value arguments) from the list
      | if they are found.
      |
      */
    pub fn get_existing_folder_for_option_and_remove(&mut self, option: &str) -> File {
        
        todo!();
        /*
            return checkFolderExists (getFileForOptionAndRemove (option));
        */
    }
}
