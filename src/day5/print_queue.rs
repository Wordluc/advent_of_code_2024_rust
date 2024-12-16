use std::fs;
enum StateRule{
    WaitForAfter,
    WaitForBefore,
    Nothing,
    Error
}
struct Rule{
    before: String,
    after: String,
    state: StateRule
}
struct Updates{
    pages: Vec<String>
}
fn read_content(file_path: String) -> (Vec<Rule>, Vec<Updates>) {
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file.split("\n");
    let mut ret_rules: Vec<Rule> = Vec::new();
    let mut ret_updates: Vec<Updates> = Vec::new();
    let mut parts: Vec<String>;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        parts=line.split("|").map(|s| s.to_string()).collect();
        if parts.len() == 2 {
            ret_rules.push(Rule{state: StateRule::Nothing, before: parts[0].clone(), after: parts[1].clone()});
        }
        if parts.len() == 1 {
            parts=line.split(",").map(|s| s.to_string()).collect();
            ret_updates.push(Updates{pages: parts});
        }
    }
    (ret_rules, ret_updates)
}
mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let (rules, updates) = read_content(String::from("src/day5/input.txt"));
        assert_ne!(rules.len(), 0);
        assert_ne!(updates.len(), 0);
    }

}
