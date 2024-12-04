use std::fs;
fn read_content(file_path: String) -> Vec<String> {
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file.split("\n");
    let mut ret: Vec<String> = Vec::new();
    for line in lines {
        ret.push(line.to_string());
    }
    ret
}

struct Point {
    x: usize,
    y: usize,
}
enum Direction {
    Vertical,
    Horizontal,
    Oblique45,
    Oblique275,
}
fn get_part_string(
    to_search: &str,
    in_string: &Vec<String>,
    occurence_first_char: &Point,
    dir: &Direction,
) -> Result<Vec<String>, String> {
    let mut ret: Vec<String> = Vec::new();
    let lenght = to_search.chars().count();
    match dir {
        Direction::Vertical => {
            let mut str: String = "".to_string();
            if occurence_first_char.y + lenght < in_string.len() {
                for y in occurence_first_char.y..occurence_first_char.y + lenght {
                    let char = in_string[y].to_string();
                    let char = char.chars().nth(occurence_first_char.x-1);
                    if char.is_none() {
                        break;
                    }
                    let char = char.unwrap();
                    str.push(char);
                }
            }

            if str.len() != 0 {
                ret.push(str);
            }
            let mut str: String = "".to_string();
            if occurence_first_char.y as i32 - lenght as i32 >= 0 {
                for y in occurence_first_char.y - lenght +1..occurence_first_char.y+1  {
                    let char = in_string[y].to_string();
                    let char = char.chars().nth(occurence_first_char.x-1);
                    if char.is_none() {
                        break;
                    }
                    let char = char.unwrap();
                    str.push(char);
                }
            }
            if str.len() != 0 {
                ret.push(str);
            }
        }
        Direction::Horizontal => {
            if occurence_first_char.x + lenght-1 < in_string[0].len()+1 {
                ret.push(
                    in_string[occurence_first_char.y]
                    [occurence_first_char.x-1..occurence_first_char.x + lenght-1]
                    .to_string(),
                );
            }
            if (occurence_first_char.x as i32 - lenght as i32) >= 0 as i32 {
                ret.push(
                    in_string[occurence_first_char.y]
                    [occurence_first_char.x - lenght..occurence_first_char.x]
                    .to_string(),
                );
            }
        }
        Direction::Oblique45 => {
            let mut str: String = "".to_string();
            let starting_point: Point = Point {
                x: occurence_first_char.x,
                y: occurence_first_char.y,
            };
            if starting_point.y as i32 - lenght as i32 >= 0 {
                for i in 0..lenght {
                    let char = in_string[starting_point.y - i].to_string();
                    let char = char.chars().nth(starting_point.x + i-1);
                    if char.is_none() {
                        str = "".to_string();
                        break;
                    }
                    let char = char.unwrap();
                    str.push(char);
                }
            }
            if str.len() != 0 {
                ret.push(str);
            }
            if (occurence_first_char.x as i32 - lenght as i32) >= 0 {
                if (occurence_first_char.y as i32 + lenght as i32) < in_string.len() as i32 {
                    let starting_point: Point = Point {
                        x: occurence_first_char.x - lenght,
                        y: occurence_first_char.y + lenght - 1,
                    };
                    let mut str: String = "".to_string();
                    for i in 0..lenght {
                        let char = in_string[starting_point.y - i].to_string();
                        let char = char.chars().nth(starting_point.x + i);
                        if char.is_none() {
                            str = "".to_string();
                            break;
                        }
                        let char = char.unwrap();
                        str.push(char);
                    }
                    if str.len() != 0 {
                        ret.push(str);
                    }
                }
            }
        }
        Direction::Oblique275 => {
            let mut str: String = "".to_string();
            let starting_point: Point = Point {
                x: occurence_first_char.x,
                y: occurence_first_char.y,
            };
            if (occurence_first_char.x as i32 + lenght as i32-1) < in_string.len() as i32 {
                if (occurence_first_char.y as i32 + lenght as i32) < in_string.len() as i32 {
                    println!("{}{}", occurence_first_char.x, occurence_first_char.y);
                    for i in 0..lenght {
                        let char = in_string[starting_point.y + i].to_string();
                        let char = char.chars().nth(starting_point.x + i);
                        if char.is_none() {
                            str = "".to_string();
                            break;
                        }
                        let char = char.unwrap();
                        str.push(char);
                    }
                    if str.len() != 0 {
                        ret.push(str);
                    }
                }
            }
            if (occurence_first_char.x as i32 - lenght as i32) >= 0 {

                if (occurence_first_char.y as i32 - lenght as i32) >= 0 {
                    let starting_point: Point = Point {
                        x: occurence_first_char.x - lenght,
                        y: occurence_first_char.y - lenght,
                    };
                    let mut str: String = "".to_string();
                    for i in 0..lenght {
                        let char = in_string[starting_point.y + i].to_string();
                        let char = char.chars().nth(starting_point.x + i);
                        if char.is_none() {
                            str = "".to_string();
                            break;
                        }
                        let char = char.unwrap();
                        str.push(char);
                    }
                    if str.len() != 0 {
                        ret.push(str);
                    }
                }
            }
        }
    }
    return Ok(ret);
}
fn get_occuranges(first_char: char, in_string: &Vec<String>) -> Vec<Point> {
    let mut occurences_first_char: Vec<Point> = Vec::new();
    for y in 0..in_string.len() {
        for x in 0..in_string[y].len() {
            if in_string[y].chars().nth(x).unwrap() == first_char {
                occurences_first_char.push(Point { x:(x+1), y });
            }
        }
    }
    occurences_first_char
}
fn number_of_occurences(search: &str, in_string: Vec<String>) -> Result<i32, String> {
    let mut occurences_first_char: Vec<Point> = Vec::new();
    let first_char: char;
    let mut founded: i32 = 0;
    match search.chars().nth(0) {
        Some(x) => first_char = x,
        None => return Err("Invalid input".to_string()),
    }
    let directions = vec![
        Direction::Horizontal,
        Direction::Vertical,
        Direction::Oblique275,
        Direction::Oblique45,
    ];
    occurences_first_char = get_occuranges(first_char, &in_string);
    for occ in occurences_first_char {
        for dir in &directions {
            let res = get_part_string(search, &in_string, &occ, dir);
            match res {
                Ok(a) => {
                    if a.contains(&search.to_string()) {
                        founded += 1;
                    } else {
                        for mut e in a {
                            e = e.chars().rev().collect::<String>();
                            if e.contains(&search.to_string()) {
                                founded += 1;
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
    }
    return Ok(founded);
}
mod test {
    use super::*;
    fn test_example() {

        let input = vec![
            "MMMSXXMASM".to_string(),
            "MSAMXMSMSA".to_string(),
            "AMXSXMAAMM".to_string(),
            "MSAMASMSMX".to_string(),
            "XMASAMXAMM".to_string(),
            "XXAMMXXAMA".to_string(),
            "SMSMSASXSS".to_string(),
            "SAXAMASAAA".to_string(),
            "MAMMMXMMMM".to_string(),
            "MXMXAXMASX".to_string(),
        ];
        let a = number_of_occurences("XMAS", input);
        if a.is_err() {
            println!("{}", a.unwrap_err());
            assert!(false);
            return;
        }
        assert_eq!(a.unwrap(), 18);
    }
    fn test_example_2() {
        let input = vec![
            "XMASXX".to_string(),
            "MMSMMO".to_string(),
            "ACAAMM".to_string(),
            "SSMSSS".to_string(),
            "CXXXOO".to_string(),
        ];
        let a = number_of_occurences("XMAS", input);
        if a.is_err() {
            println!("{}", a.unwrap_err());
            assert!(false);
            return;
        }
        assert_eq!(a.unwrap(), 7);
    }
    #[test]
    fn test_get_part_horizontal() {
        let input = vec![
            "XMASXX".to_string(),
            "MMSMMO".to_string(),
            "ACAAMM".to_string(),
            "SSMSSS".to_string(),
            "CXXXOO".to_string(),
        ];
        let occurences_first_char = get_occuranges('X', &input);
        let mut expected = vec![
            vec!["XMAS"],
            vec!["MASX"],
            vec!["ASXX"],
            vec!["XXXO"],
            vec!["XXOO"],
            vec!["CXXX"]
        ].into_iter();
        for occ in occurences_first_char {
            let a = get_part_string("XMAS", &input, &occ, &Direction::Horizontal);
            if a.is_err() {
                println!("{}", a.unwrap_err());
                assert!(false);
                return;
            }
            if a.unwrap() != expected.next().unwrap() {
                assert!(false);
                return;
            }
        }
    }
    #[test]
    fn test_get_part_vertical() {
        let input = vec![
            "XMASXX".to_string(),
            "MMSMMO".to_string(),
            "ACAAMM".to_string(),
            "SSMSSS".to_string(),
            "CXXXOO".to_string(),
        ];
        let occurences_first_char = get_occuranges('X', &input);
        let mut expected = vec![
            vec!["XMAS"],
            vec!["XMMS"],
            vec!["XOMS"],
            vec!["MCSX"],
            vec!["SAMX"],
            vec!["MASX"]
        ].into_iter();
        for occ in occurences_first_char {
            let a = get_part_string("XMAS", &input, &occ, &Direction::Vertical);
            println!("{:?}", a);
            if a.is_err() {
                println!("{}", a.unwrap_err());
                assert!(false);
                return;
            }
            if a.unwrap() != expected.next().unwrap() {
                assert!(false);
                return;
            }
        }
    }
    #[test]
    fn test_get_part_oblique45() {
        let input = vec![
            "XMASXX".to_string(),
            "MMSMMO".to_string(),
            "ACAAMM".to_string(),
            "SSMSSS".to_string(),
            "CXXXOO".to_string(),
        ];
        let occurences_first_char = get_occuranges('X', &input);
        let mut expected = vec![
            vec![],
            vec!["SAMX"],
            vec!["MAMX"],
            vec!["XMAM"],
            vec!["XSMO"],
            vec![],
        ].into_iter();
        for occ in occurences_first_char {
            let a = get_part_string("XMAS", &input, &occ, &Direction::Oblique45);
            println!("{:?}", a);
            if a.is_err() {
                println!("{}", a.unwrap_err());
                assert!(false);
                return;
            }
            if a.unwrap() != expected.next().unwrap() {
                assert!(false);
                return;
            }
        }
    }
    #[test]
    fn test_get_part_oblique275() {
        let input = vec![
            "XMASXX".to_string(),
            "MMSMMO".to_string(),
            "ACAAMM".to_string(),
            "SSMSSS".to_string(),
            "CXXXOO".to_string(),
        ];
        let occurences_first_char = get_occuranges('X', &input);
        let mut expected = vec![
            vec![],
            vec!["SAMX"],
            vec!["MAMX"],
            vec!["XMAM"],
            vec!["XSMO"],
            vec![],
        ].into_iter();
        for occ in occurences_first_char {
            let a = get_part_string("XMAS", &input, &occ, &Direction::Oblique275);
            println!("{:?}", a);
            if a.is_err() {
                println!("{}", a.unwrap_err());
                assert!(false);
                return;
            }
           // if a.unwrap() != expected.next().unwrap() {
           //     assert!(false);
           //     return;
           // }
        }
        assert!(false);
    }
    #[test]
    fn test_ceres_search() {
        let input = read_content("src/day4/input.txt".to_string());
        let a = number_of_occurences("XMAS", input);
        if a.is_err() {
            println!("{}", a.unwrap_err());
            assert!(false);
            return;
        }
        assert_eq!(a.unwrap(), 18);
    }
}
