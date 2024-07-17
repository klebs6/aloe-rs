crate::ix!();

pub struct AllComponentRepainter {
    base:  Timer,
    base2: DeletedAtShutdown,
}

impl Drop for AllComponentRepainter {

    fn drop(&mut self) {
        todo!();
        /*
            clearSingletonInstance();
        */
    }
}

aloe_declare_singleton!{
    AllComponentRepainter, false
}

aloe_implement_singleton!{
    AllComponentRepainter
}

impl AllComponentRepainter {
    
    pub fn trigger(&mut self)  {
        
        todo!();
        /*
            if (! isTimerRunning())
                startTimer (100);
        */
    }
    
    pub fn timer_callback(&mut self)  {
        
        todo!();
        /*
            stopTimer();

            Vec<Component*> alreadyDone;

            for (int i = TopLevelWindow::getNumTopLevelWindows(); --i >= 0;)
                if (auto* c = TopLevelWindow::getTopLevelWindow(i))
                    repaintAndResizeAllComps (c, alreadyDone);

            auto& desktop = Desktop::getInstance();

            for (int i = desktop.getNumComponents(); --i >= 0;)
                if (auto* c = desktop.getComponent(i))
                    repaintAndResizeAllComps (c, alreadyDone);
        */
    }
    
    pub fn repaint_and_resize_all_comps(
        c:            ComponentSafePointer<Component>,
        already_done: &mut Vec<*mut Component>
    ) {
        
        todo!();
        /*
            if (c->isVisible() && ! alreadyDone.contains (c))
            {
                c->repaint();
                c->resized();

                for (int i = c->getNumChildComponents(); --i >= 0;)
                {
                    if (auto* child = c->getChildComponent(i))
                    {
                        repaintAndResizeAllComps (child, alreadyDone);
                        alreadyDone.add (child);
                    }

                    if (c == nullptr)
                        break;
                }
            }
        */
    }
}
