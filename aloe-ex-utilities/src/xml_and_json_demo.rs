crate::ix!();

/**
  | The type of database to parse.
  |
  */
pub enum XMLAndJsonDemoType
{
    xml,
    json
}

#[no_copy]
#[leak_detector]
pub struct XMLandJSONDemo<'a> {
    base:                    Component<'a>,
    type_box:                ComboBox<'a>,
    combo_box_label:         Label<'a>, // default = { {}, "Database Type:"  }
    code_document:           CodeDocument<'a>,
    code_document_component: CodeEditorComponent<'a>, // default = { codeDocument, nullptr  }
    results_tree:            TreeView<'a>,
    root_item:               Box<TreeViewItem<'a>>,
    parsed_xml:              Box<XmlElement>,
    error_message:           TextEditor<'a>,
}

impl<'a> CodeDocumentListener for XMLandJSONDemo<'a> {

    fn code_document_text_inserted(
        &mut self, 
        _0: &str,
        _1: i32)  {
        
        todo!();
        /*
            rebuildTree();
        */
    }
    
    fn code_document_text_deleted(&mut self, _0: i32, _1: i32)  {
        
        todo!();
        /*
            rebuildTree();
        */
    }
}

impl<'a> Default for XMLandJSONDemo<'a> {
    
    fn default() -> Self {
        todo!();
        /*


            setOpaque (true);

            addAndMakeVisible (typeBox);
            typeBox.addItem ("XML",  1);
            typeBox.addItem ("JSON", 2);

            typeBox.onChange = [this]
            {
                if (typeBox.getSelectedId() == 1)
                    reset (xml);
                else
                    reset (json);
            };

            comboBoxLabel.attachToComponent (&typeBox, true);

            addAndMakeVisible (codeDocumentComponent);
            codeDocument.addListener (this);

            resultsTree.setTitle ("Results");
            addAndMakeVisible (resultsTree);
            resultsTree.setColour (TreeView::backgroundColourId, Colours::white);
            resultsTree.setDefaultOpenness (true);

            addAndMakeVisible (errorMessage);
            errorMessage.setReadOnly (true);
            errorMessage.setMultiLine (true);
            errorMessage.setCaretVisible (false);
            errorMessage.setColour (TextEditor::outlineColourId, Colours::transparentWhite);
            errorMessage.setColour (TextEditor::shadowColourId,  Colours::transparentWhite);

            typeBox.setSelectedId (1);

            setSize (500, 500)
        */
    }
}

impl<'a> Drop for XMLandJSONDemo<'a> {

    fn drop(&mut self) {
        todo!();
        /* 
            resultsTree.setRootItem (nullptr);
         */
    }
}

impl<'a> XMLandJSONDemo<'a> {
    
    pub fn paint(&mut self, g: &mut Graphics)  {
        
        todo!();
        /*
            g.fillAll (getUIColourIfAvailable (LookAndFeel_V4::ColourScheme::UIColour::windowBackground));
        */
    }
    
    pub fn resized(&mut self)  {
        
        todo!();
        /*
            auto area = getLocalBounds();

            typeBox.setBounds (area.removeFromTop (36).removeFromRight (150).reduced (8));
            codeDocumentComponent.setBounds (area.removeFromTop(area.getHeight() / 2).reduced (8));
            resultsTree          .setBounds (area.reduced (8));
            errorMessage         .setBounds (resultsTree.getBounds());
        */
    }
    
    pub fn rebuild_tree(&mut self)  {
        
        todo!();
        /*
            std::unique_ptr<XmlElement> openness;

            if (rootItem.get() != nullptr)
                openness = rootItem->getOpennessState();

            createNewRootNode();

            if (openness.get() != nullptr && rootItem.get() != nullptr)
                rootItem->restoreOpennessState (*openness);
        */
    }
    
    pub fn create_new_root_node(&mut self)  {
        
        todo!();
        /*
            // clear the current tree
            resultsTree.setRootItem (nullptr);
            rootItem.reset();

            // try and parse the editor's contents
            switch (typeBox.getSelectedItemIndex())
            {
                case xml:           rootItem.reset (rebuildXml());        break;
                case json:          rootItem.reset (rebuildJson());       break;
                default:            rootItem.reset();                     break;
            }

            // if we have a valid TreeViewItem hide any old error messages and set our TreeView to use it
            if (rootItem.get() != nullptr)
                errorMessage.clear();

            errorMessage.setVisible (! errorMessage.isEmpty());

            resultsTree.setRootItem (rootItem.get());
        */
    }

    /**
      | Parses the editor's contents as XML.
      |
      */
    pub fn rebuild_xml(&mut self) -> *mut TreeViewItem {
        
        todo!();
        /*
            parsedXml.reset();

            XmlDocument doc (codeDocument.getAllContent());
            parsedXml = doc.getDocumentElement();

            if (parsedXml.get() == nullptr)
            {
                auto error = doc.getLastParseError();

                if (error.isEmpty())
                    error = "Unknown error";

                errorMessage.setText ("Error parsing XML: " + error, dontSendNotification);

                return nullptr;
            }

            return new XmlTreeItem (*parsedXml);
        */
    }

    /**
      | Parses the editor's contents as JSON.
      |
      */
    pub fn rebuild_json(&mut self) -> *mut TreeViewItem {
        
        todo!();
        /*
            var parsedJson;
            auto result = JSON::parse (codeDocument.getAllContent(), parsedJson);

            if (! result.wasOk())
            {
                errorMessage.setText ("Error parsing JSON: " + result.getErrorMessage());
                return nullptr;
            }

            return new JsonTreeItem (Identifier(), parsedJson);
        */
    }

    /**
      | Clears the editor and loads some default
      | text.
      |
      */
    pub fn reset(&mut self, ty: XMLAndJsonDemoType)  {
        
        todo!();
        /*
            switch (type)
            {
                case xml:   codeDocument.replaceAllContent (loadEntireAssetIntoString ("treedemo.xml")); break;
                case json:  codeDocument.replaceAllContent (loadEntireAssetIntoString ("aloe_module_info")); break;
                default:    codeDocument.replaceAllContent ({}); break;
            }
        */
    }
    
}
