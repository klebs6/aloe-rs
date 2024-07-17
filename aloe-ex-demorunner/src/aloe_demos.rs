crate::ix!();

//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Demos/ALOEDemos.h]
//-------------------------------------------[.cpp/Aloe/examples/DemoRunner/Source/Demos/ALOEDemos.cpp]

pub struct ALOEDemosFileAndCallback<'a>
{
    demo_file:      File,
    callback:       fn() -> Component<'a>,
    is_heavyweight: bool,
}

pub struct ALOEDemosDemoCategory<'a>
{
    name:  String,
    demos: Vec<ALOEDemosFileAndCallback<'a>>,
}

pub struct ALOEDemos {

}

impl ALOEDemos {
    
    pub fn get_categories<'a>(&mut self) -> &'a mut Vec<ALOEDemosDemoCategory> {
        
        todo!();
        /*
            static std::vector<ALOEDemosDemoCategory> categories;
        return categories;
        */
    }
    
    pub fn get_category<'a>(&mut self, name: &String) -> &'a mut ALOEDemosDemoCategory {
        
        todo!();
        /*
            auto& categories = getCategories();

        for (auto& c : categories)
            if (c.name == name)
                return c;

        std::vector<ALOEDemosFileAndCallback> fc;
        categories.push_back ({ name, fc });

        return categories.back();
        */
    }
    
    pub fn register_demo<'a>(
        &mut self, 
        constructor_callback: fn() -> *mut Component<'a>,
        file_path:            &String,
        category:             &String,
        is_heavyweight:       bool
    ) {
        
        todo!();
        /*
            #if ALOE_MAC
        auto f = File::getSpecialLocation (File::currentExecutableFile)
                      .getParentDirectory().getParentDirectory().getChildFile ("Resources");
       #else
        auto f = findExamplesDirectoryFromExecutable (File::getSpecialLocation (File::currentApplicationFile));
       #endif

        #if ! (ALOE_ANDROID || ALOE_IOS)
        if (f == File())
        {
            jassertfalse;
            return;
        }
        #endif

        getCategory (category).demos.push_back ({ f.getChildFile (filePath), constructorCallback, isHeavyweight });
        */
    }
    
    pub fn find_examples_directory_from_executable(&mut self, exec: File) -> File {
        
        todo!();
        /*
            int numTries = 15;
        auto exampleDir = exec.getParentDirectory().getChildFile ("examples");

        if (exampleDir.exists())
            return exampleDir;

        while (exec.getFileName() != "examples" && numTries-- > 0)
            exec = exec.getParentDirectory();
        if (exec.getFileName() == "examples")
            return exec;
        return {};
        */
    }
}
