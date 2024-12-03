use std::{collections::HashMap, fs};

fn read_content(file_path:String)->[Vec<i32>;2]{
    let file=fs::read_to_string(file_path).expect("Unable to read file");
    let lines=file.split("\n");
    let mut ret: [Vec<i32>;2]=[Vec::new(),Vec::new()];
    let mut conv:Result<i32,_>;
    let mut i:usize=0;
    for line in lines {
        let parts = line.split("   ");
        if parts.clone().count() != 2 {
            break;
        }
        for mut part in parts {
            part=part.trim();
            conv = part.parse::<i32>();
            if conv.is_err() {
                println!("Error parsing part: {:?}", part);
                break;
            }
            ret[i].push(conv.expect("impossible to convert part").clone());
            i+=1
        }
        i=0;
    }
    ret
}

fn historian_hysteria_part1()->i32{
    let mut content=read_content(String::from("src/day1/input.txt"));
    content[0].sort();
    content[1].sort();
    let mut ret=0;
    for i in 0..content[0].len(){
        ret+=(content[1][i]-content[0][i]).abs();
    }
    ret
}

fn historian_hysteria_part2()->i32{
    let content=read_content(String::from("src/day1/input.txt"));
    let iter_second=content[1].iter();
    let mut doppioni:HashMap<i32,i32>=HashMap::new();
    let mut ret=0;
    let mut iteration=0;
    for i in 0..content[0].len(){
        if doppioni.get_key_value(&content[0][i]).is_some(){
            ret+=doppioni[&content[0][i]];
            continue;
        }
        iter_second.clone().for_each(|j|{
            if *j==content[0][i]{
                iteration+=1;
            }
        });
        if iteration==0{
            continue;
        }
        doppioni.insert(content[0][i],iteration*content[0][i]);
        ret+=iteration*content[0][i];
        iteration=0;
    }
    ret
}

mod test{
    use super::*;
    #[test]
    fn test_read_content(){
        let ret=read_content(String::from("src/day1/input.txt"));
        assert_eq!(ret[0].len(),1000);
        assert_eq!(ret[1].len(),1000);
    }
    #[test]
    fn historian_hysteria1_test(){
        let ret=historian_hysteria_part1();
        assert_eq!(ret,1666427);
    }

    #[test]
    fn historian_hysteria2_test(){
        let ret=historian_hysteria_part2();
        assert_eq!(ret,24316233);
    }
}
