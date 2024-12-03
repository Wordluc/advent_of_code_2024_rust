use std::fs;
fn read_content(path:String)->String{
    return fs::read_to_string(path).expect("Unable to read file");
}

mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let a=read_content("src/day3/input".to_string());
        assert_ne!(a,"");
    }
}
