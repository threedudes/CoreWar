use crate::types;
use regex::Regex;
use std::collections::HashMap;
pub fn parse(text: &str) -> Vec<types::Instruction> {
    let lower = text.to_lowercase();
    let lines = lower.split("\n");
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
    let mut labels: HashMap<&str, i16> = HashMap::new();
    let mut counter = 0;
    let mut instructions = vec![];
    for line in lines {
        let matches = re.captures_iter(&line);
        for t_match in matches {
            let opstring = match t_match.name("opstring") {
                Some(opstring) => opstring.as_str(),
                None => continue
            };
            match t_match.name("label") {
                Some(label) => {
                    labels.insert(label.as_str(), counter);
                },
                None => ()
            };
            let modifier_string = t_match.name("modifier").map_or("", |m| m.as_str());
            println!("{modifier_string}");
            let modifier = types::Modifier::from_str(modifier_string);
            let params_string = t_match.name("params").map_or("", |m| m.as_str());
            let params_match = params_re.captures_iter(params_string);
            let blank = types::Param{mode: types::AddressingModes::Direct, value: types::Value::Integer(0)};
            let mut params = vec![blank.clone(), blank];
            for (index,param) in params_match.enumerate(){
                let value_string = param.name("value").unwrap().as_str();
                let value_num: Result<i16, _> = value_string.parse();
                let value = match value_num {
                    Ok(num) => types::Value::Integer(num),
                    Err(_) => types::Value::Label(value_string.to_string())
                };
                let mode = types::AddressingModes::from_str(param.name("addressmode").map_or("$", |m| m.as_str()));
                let param = types::Param{mode,value};
                params[index] = param
            }
            let opcode = types::OpCode::from_opstring(opstring).unwrap();
            let instruction = types::Instruction {
                opcode: opcode,
                modifier: modifier,
                params: (params[0].clone(), params[1].clone())
            };
            instructions.push(instruction);
            counter += 1
        }
    }

    return instructions
}