use crate::types;
use regex::Regex;
use std::collections::HashMap;
pub fn parse(text: &str) -> types::Warrior {
    let lower = text.to_lowercase();
    let lines = lower.split("\n");
    //TODO: Replace this parser with a beautiful one using pest crate.
    
    let re = Regex::new(r"[ \t]*((?P<label>[a-z]*):?[ \t]+)?((?P<opstring>[a-z]+)(.(?P<modifier>[a-z]+))?)[ \t]+(?P<params>[^;]*)[ \t]*(;.*)?").unwrap();
    /*
    This regex match this pattern:
    label: instruction.modifier params

    Everything have to be in lowercase.
    */
    let params_re = Regex::new(r"(?P<addressmode>[#@*<>{}])?(?P<value>[a-z0-9-]+)").unwrap();
    /*
    This regex match parameters like:
    #3, 1
    @2, {-1
    modifier+value, modifier+value,..
    */
    let mut labels: HashMap<String, i16> = HashMap::new();
    let mut counter = 0;
    let mut instructions = vec![];
    for line in lines {
        let matches = re.captures_iter(&line);
        for t_match in matches {
            let opstring = match t_match.name("opstring") {
                Some(opstring) => opstring.as_str(),
                None => continue
            };
            let label = t_match.name("label");
            match label {
                Some(label) => {
                    labels.insert(label.as_str().to_string(), counter);
                },
                None => ()
            };
            let modifier = t_match.name("modifier").map_or(None, |m| Some(types::Modifier::from_str(m.as_str()).unwrap()));
            let params_string = t_match.name("params").map_or("", |m| m.as_str());
            let params_match = params_re.captures_iter(params_string);
            let blank = types::Param{mode: types::AddressingMode::Direct, value: types::Value::Integer(0)};
            let mut params = vec![blank.clone(), blank];
            for (index,param) in params_match.enumerate(){
                let value_string = param.name("value").unwrap().as_str();
                let value_num: Result<i16, _> = value_string.parse();
                let value = match value_num {
                    Ok(num) => types::Value::Integer(num),
                    Err(_) => types::Value::Label(value_string.to_string())
                };
                let mode = types::AddressingMode::from_str(param.name("addressmode").map_or("$", |m| m.as_str())).unwrap();
                let param = types::Param{mode,value};
                params[index] = param
            }
            let opcode = types::OpCode::from_str(opstring).unwrap();
            let instruction = types::Instruction {
                opcode,
                modifier,
                //TODO:  .clone should not be used !
                params: (params[0].clone(), params[1].clone()),
            };
            instructions.push(instruction);
            counter += 1
        }
    }

    let mut warrior = types::Warrior{instructions};
    warrior.process_labels(labels);
    return warrior
}
