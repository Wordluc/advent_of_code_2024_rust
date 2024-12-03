use std::{collections::HashMap, fs};
struct to_find {
    to_find:String,
    map:HashMap<String,i32>,
}
fn read_content(path:String)->String{
    return fs::read_to_string(path).expect("Unable to read file");
}

fn find (patter:to_find,str:String)->Vec<i32>{
    fn check_range(i_end:usize)->Option<usize>{
        
    }
    let i_init:usize=0;
    let i_end:usize=a.to_find.len();
    
}

mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let a=read_content("src/day3/input".to_string());
        assert_ne!(a,"");
    }
}
