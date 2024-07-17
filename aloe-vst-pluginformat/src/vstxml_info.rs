crate::ix!();

pub struct VSTXMLInfoBase
{
    parent: *mut VSTXMLInfoGroup, // default = nullptr
}

pub struct VSTXMLInfoParam {
    base:             VSTXMLInfoBase,
    paramid:          i32,
    expr:             String,
    name:             String,
    label:            String,
    short_names:      StringArray,
    ty:               String,
    number_of_states: i32,
    default_value:    f32,
}

pub struct VSTXMLInfoGroup {
    base:       VSTXMLInfoBase,
    name:       String,
    param_tree: Vec<Box<VSTXMLInfoBase>>,
}

///------------------------
#[derive(Default)]
pub struct VSTXMLInfoRange {
    low:            f32, // default = 0
    high:           f32, // default = 0
    inclusive_low:  bool, // default = false
    inclusive_high: bool, // default = false
}

impl VSTXMLInfoRange {

    pub fn new(s: &String) -> Self {
    
        todo!();
        /*


            set (s);
        */
    }
    
    pub fn set(&mut self, s: &String)  {
        
        todo!();
        /*
            inclusiveLow  = s.startsWithChar ('[');
                inclusiveHigh = s.endsWithChar   (']');

                auto str = s.removeCharacters ("[]");

                low  = str.upToFirstOccurrenceOf (",", false, false).getFloatValue();
                high = str.fromLastOccurrenceOf  (",", false, false).getFloatValue();
        */
    }
    
    pub fn contains(&self, f: f32) -> bool {
        
        todo!();
        /*
            return (inclusiveLow  ? (f >= low)  : (f > low))
                    && (inclusiveHigh ? (f <= high) : (f < high));
        */
    }
}

pub struct VSTXMLInfoEntry
{
    name:  String,
    range: VSTXMLInfoRange,
}

pub struct VSTXMLInfoValueType
{
    name:    String,
    label:   String,
    entries: Vec<Box<VSTXMLInfoEntry>>,
}

pub struct VSTXMLInfoTemplate
{
    name:   String,
    params: Vec<Box<VSTXMLInfoParam>>,
}

///-----------------------
pub struct VSTXMLInfo {
    param_tree:        Vec<Box<VSTXMLInfoBase>>,
    value_types:       Vec<Box<VSTXMLInfoValueType>>,
    templates:         Vec<Box<VSTXMLInfoTemplate>>,
    switch_value_type: VSTXMLInfoValueType,
}

impl VSTXMLInfo {
    
    pub fn create_for(xml: &XmlElement) -> *mut VSTXMLInfo {
        
        todo!();
        /*
            if (xml.hasTagName ("VSTParametersStructure"))
                return new VSTXMLInfo (xml);

            if (const auto* x = xml.getChildByName ("VSTParametersStructure"))
                return new VSTXMLInfo (*x);

            return nullptr;
        */
    }
    
    pub fn get_param_forid(&self, 
        paramid: i32,
        grp:     *const VSTXMLInfoGroup) -> *const VSTXMLInfoParam {
        
        todo!();
        /*
            for (auto item : (grp != nullptr ? grp->paramTree : paramTree))
            {
                if (auto param = dynamic_cast<const VSTXMLInfoParam*> (item))
                    if (param->paramID == paramID)
                        return param;

                if (auto group = dynamic_cast<const VSTXMLInfoGroup*> (item))
                    if (auto res = getParamForID (paramID, group))
                        return res;
            }

            return nullptr;
        */
    }
    
    pub fn get_value_type(&self, name: &String) -> *const VSTXMLInfoValueType {
        
        todo!();
        /*
            for (auto v : valueTypes)
                if (v->name == name)
                    return v;

            return nullptr;
        */
    }
    
    pub fn new(xml: &XmlElement) -> Self {
    
        todo!();
        /*


            switchValueType.entries.add (new VSTXMLInfoEntry({ TRANS("Off"), VSTXMLInfoRange ("[0, 0.5[") }));
            switchValueType.entries.add (new VSTXMLInfoEntry({ TRANS("On"),  VSTXMLInfoRange ("[0.5, 1]") }));

            for (auto* item : xml.getChildIterator())
            {
                if (item->hasTagName ("VSTXMLInfoParam"))           parseParam (*item, nullptr, nullptr);
                else if (item->hasTagName ("VSTXMLInfoValueType"))  parseValueType (*item);
                else if (item->hasTagName ("VSTXMLInfoTemplate"))   parseTemplate (*item);
                else if (item->hasTagName ("VSTXMLInfoGroup"))      parseGroup (*item, nullptr);
            }
        */
    }
    
    pub fn parse_param(&mut self, 
        item:  &XmlElement,
        group: *mut VSTXMLInfoGroup,
        temp:  *mut VSTXMLInfoTemplate)  {
        
        todo!();
        /*
            auto param = new VSTXMLInfoParam();

            if (temp != nullptr)
                param->expr = item.getStringAttribute ("id");
            else
                param->paramID = item.getIntAttribute ("id");

            param->name           = item.getStringAttribute ("name");
            param->label          = item.getStringAttribute ("label");
            param->type           = item.getStringAttribute ("type");
            param->numberOfStates = item.getIntAttribute ("numberOfStates");
            param->defaultValue   = (float) item.getDoubleAttribute ("defaultValue");

            param->shortNames.addTokens (item.getStringAttribute ("shortName"), ",", "");
            param->shortNames.trim();
            param->shortNames.removeEmptyStrings();

            if (group != nullptr)
            {
                group->paramTree.add (param);
                param->parent = group;
            }
            else if (temp != nullptr)
            {
                temp->params.add (param);
            }
            else
            {
                paramTree.add (param);
            }
        */
    }
    
    pub fn parse_value_type(&mut self, item: &XmlElement)  {
        
        todo!();
        /*
            auto vt = new VSTXMLInfoValueType();
            valueTypes.add (vt);

            vt->name  = item.getStringAttribute ("name");
            vt->label = item.getStringAttribute ("label");

            int curEntry = 0;
            const int numEntries = item.getNumChildElements();

            for (auto* entryXml : item.getChildWithTagNameIterator ("VSTXMLInfoEntry"))
            {
                auto entry = new VSTXMLInfoEntry();
                entry->name = entryXml->getStringAttribute ("name");

                if (entryXml->hasAttribute ("value"))
                {
                    entry->range.set(entryXml->getStringAttribute ("value"));
                }
                else
                {
                    entry->range.low  = (float) curEntry / (float) numEntries;
                    entry->range.high = (float) (curEntry + 1) / (float) numEntries;

                    entry->range.inclusiveLow  = true;
                    entry->range.inclusiveHigh = (curEntry == numEntries - 1);
                }

                vt->entries.add (entry);
                ++curEntry;
            }
        */
    }
    
    pub fn parse_template(&mut self, item: &XmlElement)  {
        
        todo!();
        /*
            auto temp = new VSTXMLInfoTemplate();
            templates.add (temp);
            temp->name = item.getStringAttribute ("name");

            for (auto* param : item.getChildIterator())
                parseParam (*param, nullptr, temp);
        */
    }
    
    pub fn parse_group(&mut self, 
        item:         &XmlElement,
        parent_group: *mut VSTXMLInfoGroup)  {
        
        todo!();
        /*
            auto group = new VSTXMLInfoGroup();

            if (parentGroup)
            {
                parentGroup->paramTree.add (group);
                group->parent = parentGroup;
            }
            else
            {
                paramTree.add (group);
            }

            group->name = item.getStringAttribute ("name");

            if (item.hasAttribute ("template"))
            {
                StringArray variables;
                variables.addTokens (item.getStringAttribute ("values"), ";", "");
                variables.trim();

                for (auto temp : templates)
                {
                    if (temp->name == item.getStringAttribute ("template"))
                    {
                        for (int i = 0; i < temp->params.size(); ++i)
                        {
                            auto param = new VSTXMLInfoParam();
                            group->paramTree.add (param);

                            param->parent         = group;
                            param->paramID        = evaluate (temp->params[i]->expr, variables);
                            param->defaultValue   = temp->params[i]->defaultValue;
                            param->label          = temp->params[i]->label;
                            param->name           = temp->params[i]->name;
                            param->numberOfStates = temp->params[i]->numberOfStates;
                            param->shortNames     = temp->params[i]->shortNames;
                            param->type           = temp->params[i]->type;
                        }
                    }
                }
            }
            else
            {
                for (auto* subItem : item.getChildIterator())
                {
                    if (subItem->hasTagName ("VSTXMLInfoParam"))       parseParam (*subItem, group, nullptr);
                    else if (subItem->hasTagName ("VSTXMLInfoGroup"))  parseGroup (*subItem, group);
                }
            }
        */
    }
    
    pub fn evaluate(&self, 
        expr:      String,
        variables: &StringArray) -> i32 {
        
        todo!();
        /*
            StringArray names;
            Vec<int> vals;

            for (auto& v : variables)
            {
                if (v.contains ("="))
                {
                    names.add (v.upToFirstOccurrenceOf ("=", false, false));
                    vals.add  (v.fromFirstOccurrenceOf ("=", false, false).getIntValue());
                }
            }

            for (int i = 0; i < names.size(); ++i)
            {
                for (;;)
                {
                    const int idx = expr.indexOfWholeWord (names[i]);
                    if (idx < 0)
                        break;

                    expr = expr.replaceSection (idx, names[i].length(), String (vals[i]));
                }
            }

            expr = expr.retainCharacters ("01234567890-+")
                       .replace ("+", " + ")
                       .replace ("-", " - ");

            StringArray tokens;
            tokens.addTokens (expr, " ", "");

            bool add = true;
            int val = 0;

            for (const auto& s : tokens)
            {
                if (s == "+")
                {
                    add = true;
                }
                else if (s == "-")
                {
                    add = false;
                }
                else
                {
                    if (add)
                        val += s.getIntValue();
                    else
                        val -= s.getIntValue();
                }
            }

            return val;
        */
    }
}
