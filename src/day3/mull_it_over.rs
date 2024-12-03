use regex::Regex;
use std::{collections::HashMap, fs};

fn read_content(path: String) -> String {
    return fs::read_to_string(path).expect("Unable to read file");
}

fn find(str: String) -> Result<Vec<(i32, i32)>, String> {
    //input = ( number,number)
    fn get_parm(str: String) -> Result<(i32, i32), String> {
        let value: String = str[0..str.len()].to_string();
        let mut split = value.split(",");
        if split.clone().count() != 2 {
            return Err("Invalid input".to_string());
        }
        let a = split.next().unwrap().parse::<i32>();
        let b = split.next().unwrap().parse::<i32>();
        if a.is_err() || b.is_err() {
            return Err(format!("Invalid input {}",value));
        }
        let a = a.unwrap();
        let b = b.unwrap();
        return Ok((a, b));
    }
    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)");
    if reg.is_err() {
        return Err(reg.unwrap_err().to_string());
    }
    let reg = reg.unwrap();
    let mut res: Vec<(i32, i32)> = Vec::new();
    for x in reg.find_iter(&str) {
        let x = x.as_str()[4..x.len() - 1].to_string();
        match get_parm(x) {
            Ok(a) => {res.push(a)}
            Err(e) => {return Err(e)}
        }
    }
    return Ok(res);
}

mod test {
    use super::*;
    #[test]
    fn test_read_content() {
        let a = read_content("src/day3/input".to_string());
        assert_ne!(a, "");
    }
    #[test]
    fn test_match() {
        let a = read_content("src/day3/input".to_string());
        let r = find(a);
        match &r {
            Ok(_) => {},
            Err(e) => {println!("{}", e);assert!(false)}
        }
        assert_eq!(r.is_ok(), true);
        let r =r.unwrap();
        let mut sum = 0;
        r.iter().for_each(|x| {
            sum += x.0 * x.1
        });
        assert_eq!(sum, 161289189);
    }
}
