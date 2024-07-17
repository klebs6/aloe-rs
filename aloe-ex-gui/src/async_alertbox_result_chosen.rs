crate::ix!();

pub struct AsyncAlertBoxResultChosen<'a> {
    demo: &'a mut DialogsDemo<'a>,
}

impl<'a> AsyncAlertBoxResultChosen<'a> {

    pub fn invoke(&self, result: i32)  {
        
        todo!();
        /*
            auto& aw = *demo.asyncAlertWindow;

                aw.exitModalState (result);
                aw.setVisible (false);

                if (result == 0)
                {
                    AlertBoxResultChosen{} (result);
                    return;
                }

                auto optionIndexChosen = aw.getComboBoxComponent ("option")->getSelectedItemIndex();
                auto text = aw.getTextEditorContents ("text");

                AlertWindow::showAsync (MessageBoxOptions()
                                          .withIconType (MessageBoxIconType::InfoIcon)
                                          .withTitle ("Alert Box")
                                          .withMessage ("Result code: " + String (result) + newLine
                                                        + "Option index chosen: " + String (optionIndexChosen) + newLine
                                                        + "Text: " + text)
                                          .withButton ("OK"),
                                        nullptr);
        */
    }
}
