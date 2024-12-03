use std::{collections::HashMap, fs};
use regex::Regex;

fn read_content(path:String)->String{
    return fs::read_to_string(path).expect("Unable to read file");
}

fn find (str:String)->Result<Vec<(i32,i32)>,String>{
    //input = ( number,number) 
    fn get_parm(str:String)->Result<(i32,i32),String>{
        let value:String=str[1..str.len()-2].to_string();
        let mut split = value.split(",");
        if split.clone().count() != 2{
            return Err("Invalid input".to_string());
        }
        let a=split.next().unwrap().parse::<i32>().unwrap();
        let b=split.next().unwrap().parse::<i32>().unwrap();
        return Ok((a,b));
    }
    let reg=Regex::new(r"mul\([0-9]+,[0-9]+\)");
    if reg.is_err(){
        return Err(reg.unwrap_err().to_string());
    }
    let reg=reg.unwrap();
    let mut res:Vec<(i32,i32)>=Vec::new();
    let matchs=reg.split(&str);
    let mut numbers:(i32,i32);
    for m in matchs{
        println!("{}",m);
        match (get_parm(m.to_string())){
            Err(e) => return Err(e),
            Ok(m) => {
                res.push(m);
            }
        }
    }
    return Ok(res);
}

mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let a=read_content("src/day3/input".to_string());
        assert_ne!(a,"");
    }
    #[test]
    fn test_match(){
        let a=read_content("src/day3/input".to_string());
        let r=find(a);
        println!("{:?}",r);
        assert_eq!(r.is_ok(),true);
    }
}
