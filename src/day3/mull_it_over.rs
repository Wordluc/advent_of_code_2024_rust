use std::{collections::HashMap, fs};
struct to_find {
    to_find:String,
    map:HashMap<String,i32>,
}
fn read_content(path:String)->String{
    return fs::read_to_string(path).expect("Unable to read file");
}
//check string using pattern, return None if pattern is founded, if some return index of first
//different char
fn check_range(i_start:usize,patter:&to_find,str:&String)->Option<usize>{
    let mut checked=patter.to_find.len()-1;
    for i in (i_start..i_start+patter.to_find.len()).rev(){
        if str.chars().nth(i)!=patter.to_find.chars().nth(checked){
            return Some(i);
        }
        if checked==0{
            return None;
        }
        checked-=1;

    }
    return None;
}

fn find (patter:to_find,str:String)->Vec<i32>{
    let i_init:usize=0;
    let i_end:usize=patter.to_find.len();
    Vec::new()
    
}

mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let a=read_content("src/day3/input".to_string());
        assert_ne!(a,"");
    }
}
