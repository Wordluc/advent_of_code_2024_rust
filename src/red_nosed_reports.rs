use std::fs;
#[derive(Debug)]
enum Order {
    Asc = 0,
    Desc = 1,
}
impl Order {
    fn equal(&self, o: &Order)->bool {
        match self {
            Order::Asc => match o {
                Order::Asc => true,
                Order::Desc => false,
            },
            Order::Desc => match o {
                Order::Asc => false,
                Order::Desc => true,
            },
        }
    }
}

fn read_content(file_path: String) -> Vec<Vec<i32>> {
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file.split("\n");
    let mut ret: Vec<Vec<i32>> = Vec::new();
    let mut conv: Result<i32, _>;
    for line in lines {
        let parts = line.split(" ");
        let mut l: Vec<i32> = Vec::new();
        for part in parts {
            conv = part.parse::<i32>();
            if conv.is_err() {
                break;
            }
            l.push(conv.expect("impossible to convert part"));
        }
        if l.len() == 0 {
            break;
        }
        ret.push(l.clone());
        l.clear();
    }
    ret
}
fn check_range(a: i32, b: i32) -> Result<Order, String> {
    let diff = a - b;
    if diff > 0 && diff.abs() <= 3 {
        Ok(Order::Asc)
    } else if diff < 0 && diff.abs() <= 3 {
        Ok(Order::Desc)
    } else {
        Err("Out of range".to_string())
    }
}

fn check_order(line: &Vec<i32>) -> Order {
    let mut n_in: i16 = 0;
    let mut n_de: i16 = 0;
    let mut i_pre: i32 = line[0];
    for elt in &line[1..] {
        if elt >= &i_pre {
            n_in += 1;
        } else if elt < &i_pre {
            n_de += 1;
        }
        i_pre = *elt;
    }
    if n_in > n_de {
        Order::Asc
    } else {
        Order::Desc
    }
}
fn check_order_without_sacrifice(line: &Vec<i32>) -> Result<Order, String> {
    Ok(Order::Asc)
}

fn check_order_with_sacrifice(line: &Vec<i32>) -> Result<Order, String> {
    let order: Order = check_order(line);
    let mut pre = line[0];
    for el in &line[1..] {
        let res = check_range(*el, pre);
        match res {
            Ok(o) => {
                if !o.equal(&order) {
                    return Err("Change sort".to_string());
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
        pre = *el
    }
    Ok(order)
}

fn check_reports(reports: &Vec<Vec<i32>>, check_f: fn(&Vec<i32>) -> Result<Order, String>) -> i32 {
    let mut n_reports: i32 = 0;
    for rep in reports {
        if check_f(rep).is_ok() {
            n_reports = n_reports + 1;
        }
    }
    n_reports
}

mod test {
    use super::*;
    #[test]
    fn test_read_content() {
        let a = read_content(String::from("src/day2/input"));
        if a.len() == 0 {
            if a[0].len() == 0 {
                panic!("impossible");
            }
            panic!("impossible");
        }
    }
    #[test]
    fn test_incr_order() {
        let prova: Vec<i32> = vec![2, 3, 4, 5];
        let r = check_order_without_sacrifice(&prova);
        assert!(r.is_ok());
    }
    #[test]
    fn test_dec_order() {
        let prova: Vec<i32> = vec![5, 4, 3, 2];
        let r = check_order_without_sacrifice(&prova);
        assert!(r.is_ok());
    }
    #[test]
    fn test_wrong_order() {
        let prova: Vec<i32> = vec![5, 4, 1, 2];
        let r = check_order_without_sacrifice(&prova);
        assert!(!r.is_ok());
    }
    #[test]
    fn test_red_nosed_try() {
        let mut a: Vec<Vec<i32>> = Vec::new();
        a.push(vec![7, 6, 4, 2, 1]);
        a.push(vec![1, 2, 7, 8, 9]);
        a.push(vec![9, 7, 6, 2, 1]);
        a.push(vec![1, 3, 2, 4, 5]);
        a.push(vec![8, 6, 4, 4, 1]);
        a.push(vec![1, 3, 6, 7, 9]);
        assert_eq!(check_reports(&a, check_order_without_sacrifice), 2);
    }
    #[test]
    fn test_red_nosed_1() {
        let a = read_content(String::from("src/day2/input"));
        assert_eq!(check_reports(&a, check_order_without_sacrifice), 524);
    }

    #[test]
    fn test_red_nosed_try2() {
        let mut a: Vec<Vec<i32>> = Vec::new();
        a.push(vec![7, 6, 4, 2, 1]);
        a.push(vec![1, 2, 7, 8, 9]);
        a.push(vec![9, 7, 6, 2, 1]);
        a.push(vec![1, 3, 2, 4, 5]);
        a.push(vec![8, 6, 4, 4, 1]);
        a.push(vec![1, 3, 6, 7, 9]);
        assert_eq!(check_reports(&a, check_order_with_sacrifice), 4);
    }
    #[test]
    fn test_wrong_order_with_sacrifice1() {
        let prova: Vec<i32> = vec![2, 1, 3, 5, 8];
        let r = check_order_with_sacrifice(&prova);
        println!("{:?}", r);
        assert!(!r.is_ok())
    }
    #[test]
    fn test_wrong_order_with_sacrifice2() {
        let prova: Vec<i32> = vec![3, 1, 2, 4, 5, 6];
        let r = check_order_with_sacrifice(&prova);
        println!("{:?}", r);
        assert!(!r.is_ok())
    }
    fn test_red_nosed_2() {
        let a = read_content(String::from("src/day2/input"));
        assert_eq!(check_reports(&a, check_order_with_sacrifice), 0);
    }
}
