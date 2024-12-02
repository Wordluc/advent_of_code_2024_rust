use std::{collections::HashMap, fs};

fn read_content(file_path:String)->[Vec<i32>;2]{
    let file=fs::read_to_string(file_path).expect("Unable to read file");
    let lines=file.split("\n");
    println!("{} lines",lines.clone().count());
    let mut ret: [Vec<i32>;2]=[Vec::new(),Vec::new()];
    let mut i=0;
    lines.for_each(|line|{
        if i!=1000{
            let mut parts=line.split("   ");
            let part2=parts.next().expect("expected part 2");
            let part1=parts.next().expect("expected part 1");
            ret[1].push(part1.parse::<i32>().expect("impossible to convert part 2"));
            ret[0].push(part2.parse::<i32>().expect("impossible to convert part 1"));
            i=i+1;
        }
    });
    ret
}

fn historian_hysteria_part1()->i32{
    let mut content=read_content(String::from("src/input"));
    content[0].sort();
    content[1].sort();
    let mut ret=0;
    for i in 0..content[0].len(){
        ret+=(content[1][i]-content[0][i]).abs();
    }
    ret
}

fn historian_hysteria_part2()->i32{
    let content=read_content(String::from("src/input"));
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
        let ret=read_content(String::from("src/input"));
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
