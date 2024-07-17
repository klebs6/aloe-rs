crate::ix!();

pub fn get_all_param_ids(controller: &mut dyn IEditController) -> Vec<ParamID> {
    
    todo!();
        /*
            std::vector<VstParamID> result;

        auto count = controller.getParameterCount();

        for (decltype (count) i = 0; i < count; ++i)
        {
            VstParameterInfo info{};
            controller.getParameterInfo (i, info);
            result.push_back (info.id);
        }

        return result;
        */
}
