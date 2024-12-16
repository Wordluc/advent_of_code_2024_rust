use std::fs;
enum StateRule {
    WaitForAfter,
    WaitForBefore,
    Nothing,
    Ok,
    Error,
}
struct Rule {
    before: i16,
    after: i16,
    state:  StateRule,
}

impl Rule {
    fn check(mut self, number: i16) -> StateRule {
        self.state = match self.state {
            StateRule::WaitForAfter => {
                if number == self.after {
                    StateRule::Ok
                } else {
                    if number == self.before {
                        StateRule::Error
                    }else{
                        self.state
                    }
                }
            }
            StateRule::WaitForBefore => {
                if number == self.before {
                    StateRule::Ok
                } else {
                    if number == self.after {
                        StateRule::Error
                    }else{
                        self.state
                    }
                }
            }
            StateRule::Nothing => {
                if number == self.before {
                    StateRule::WaitForAfter
                }else if number == self.after {
                    StateRule::WaitForBefore
                }else{
                    StateRule::Nothing
                }
            }
            _ => StateRule::Error
        };
        self.state
    }
}

struct Updates {
    pages: Vec<i16>,
}
fn convert_string_int(input:Vec<&str>)->Result<Vec<i16>,String> {
    let mut ret:Vec<i16>=Vec::new();
    for mut x in input {
        match x.parse::<i16>() {
            Ok(a) => ret.push(a),
            Err(e) => return Err(format!("Error: {} {}",e,x))//Err(e.to_string()),
        }
    }
    Ok(ret)
}
fn read_content(file_path: String) -> Result<(Vec<Rule>, Vec<Updates>),String> {
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file.split("\n");
    let mut ret_rules: Vec<Rule> = Vec::new();
    let mut ret_updates: Vec<Updates> = Vec::new();
    let mut partsi16: Vec<i16>;
    let mut parts_str: Vec<&str>;
    let mut len_part:usize;
    for mut line in lines {
        line=line.trim();
        if line.len() == 0 {
            continue;
        }
        parts_str = line.split("|").collect::<Vec<&str>>();
        len_part = parts_str.len();
        if len_part == 2 {
            match convert_string_int(parts_str){
                Ok(a) => partsi16 = a,
                Err(e) => return Err(e),
            };
            ret_rules.push(Rule {
                state: StateRule::Nothing,
                before: partsi16[0],
                after: partsi16[1],
            });
            continue;
        }
        parts_str = line.split(",").collect::<Vec<&str>>();
        match convert_string_int(parts_str){
            Ok(a) => partsi16 = a,
            Err(e) => return Err(e),
        };
        ret_updates.push(Updates { pages: partsi16 });
    }
    Ok((ret_rules, ret_updates))
}

fn check_rules(rules: &Vec<Rule>, updates: Vec<Updates>) -> i32 {
    let ruleSession = rules.clone();
    4
}
mod test {
    use super::*;
    #[test]
    fn test_read_content() {
        let (rules, updates) = read_content(String::from("src/day5/input.txt")).unwrap();
        assert_ne!(rules.len(), 0);
        assert_ne!(updates.len(), 0);
    }
}
