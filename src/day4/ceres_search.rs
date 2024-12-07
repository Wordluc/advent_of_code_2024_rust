use std::{collections::{hash_map::Entry, HashMap}, fs};
fn read_content(file_path: String) -> Vec<String> {
    let file = fs::read_to_string(file_path).expect("Unable to read file");
    let lines = file.split("\n");
    let mut ret: Vec<String> = Vec::new();
    for line in lines {
        if line.len() == 0 {
            break;
        }
        ret.push(line.to_string());
    }
    ret
}
#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}
impl Coordinates {
    fn get_string(&self) -> String {
        return format!("[{},{}]", self.x, self.y);
    }
}
fn search_letter(letter: char, text: &Vec<String>) -> Vec<Coordinates> {
    let mut coordinates: Vec<Coordinates> = Vec::new();
    for y in 0..text.len() {
        for x in 0..text[y].len() {
            match text[y].chars().nth(x) {
                Some(c) => {
                    if c == letter {
                        coordinates.push(Coordinates {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                }
                None => {}
            }
        }
    }
    return coordinates;
}

fn search_word_horizontal(word: &String, text: &Vec<String>, pos: &Coordinates) -> Option<()> {
    let lenght = word.len() as i32;
    let slice: &str;
    if pos.x + lenght <= (text[pos.y as usize].len() as i32) {
        slice = &text[pos.y as usize][pos.x as usize..(pos.x + lenght) as usize];
        if slice == word {
            return Some(());
        }
    }
    return None;
}

fn search_word_vertical(word: &String, texts: &Vec<String>, pos: &Coordinates) -> Option<()> {
    let lenght = word.len() as i32;
    let mut i_word = 0;
    for text in &texts[(pos.y as usize)..] {
        if text.chars().nth(pos.x as usize).unwrap() != word.chars().nth(i_word).unwrap() {
            break;
        }
        i_word += 1;
        if i_word as i32 >= lenght {
            return Some(());
        }
    }
    return None;
}

fn search_word_45(word: &String, texts: &Vec<String>, pos: &Coordinates) -> Option<()> {
    let lenght = word.len() as i32;
    let mut i_word = 0;
    if pos.y - (lenght - 1) < 0 {
        return None;
    }
    let texts = &texts[((pos.y - (lenght - 1)) as usize)..=pos.y as usize];
    for text in texts.iter() {
        if pos.x + i_word >= text.len() as i32 {
            break;
        }
        if (pos.x + lenght - 1 - i_word) >= (text.len() as i32) {
            break;
        }
        if text
            .chars()
                .nth((pos.x + lenght - 1 - i_word) as usize)
                .unwrap()
                != word.chars().nth((lenght - 1 - i_word) as usize).unwrap()
        {
            break;
        }
        i_word += 1;
        if i_word as i32 >= lenght {
            return Some(());
        }
    }
    None
}
fn search_word_275(word: &String, texts: &Vec<String>, pos: &Coordinates) -> Option<()> {
    let lenght = word.len() as i32;
    let mut i_word = 0;
    for text in &texts[(pos.y as usize)..] {
        if pos.x + i_word >= text.len() as i32 {
            break;
        }
        if text.chars().nth((pos.x + i_word) as usize).unwrap()
            != word.chars().nth((i_word) as usize).unwrap()
        {
            break;
        }
        i_word += 1;
        if i_word as i32 >= lenght {
            return Some(());
        }
    }
    None
}
fn get_n_match(work: String, text: &Vec<String>) -> Result<i32, String> {
    let first_letter = match work.chars().nth(0) {
        Some(c) => c,
        None => return Err("No first letter".to_string()),
    };
    let coordinates_first_letter: Vec<Coordinates> = search_letter(first_letter, &text);
    let mut founded: i32 = 0;
    for pos in coordinates_first_letter {
        if search_word_vertical(&work, &text, &pos).is_some() {
            founded += 1;
        }
        if search_word_horizontal(&work, &text, &pos).is_some() {
            founded += 1;
        }
        if search_word_45(&work, &text, &pos).is_some() {
            founded += 1;
        }
        if search_word_275(&work, &text, &pos).is_some() {
            founded += 1;
        }
    }
    Ok(founded)
}
#[derive(Debug)]
enum Orientation {
    _45,
    _275,
}
fn get_n_x_match(word:String, text: &Vec<String>) -> Result<i32, String> {
    let first_letter = match word.chars().nth(0) {
        Some(c) => c,
        None => return Err("No first letter".to_string()),
    };
    let mut dic_x:HashMap<String,Vec<Orientation>>=HashMap::new();
    let coordinates_first_letter: Vec<Coordinates> = search_letter(first_letter, &text);
    let mut center:Coordinates;
    for pos in coordinates_first_letter {
        if search_word_45(&word, &text, &pos).is_some() {
            center=Coordinates{x:pos.x+1,y:pos.y-1};
            match dic_x.entry(center.get_string()){
                Entry::Vacant(e) => {
                    e.insert(vec![Orientation::_45]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(Orientation::_45);
                }
            }
        }
        if search_word_275(&word, &text, &pos).is_some() {
            center=Coordinates{x:pos.x+1,y:pos.y+1};
            match dic_x.entry(center.get_string()){
                Entry::Vacant(e) => {
                    e.insert(vec![Orientation::_275]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(Orientation::_275);
                }
            }
        }
    }
    let word:String=word.chars().rev().collect();
    let first_letter = match word.chars().nth(0) {
        Some(c) => c,
        None => return Err("No first letter".to_string()),
    };
    let coordinates_first_letter: Vec<Coordinates> = search_letter(first_letter, &text);
    for pos in coordinates_first_letter {
        if search_word_45(&word, &text, &pos).is_some() {
            center=Coordinates{x:pos.x+1,y:pos.y-1};
            match dic_x.entry(center.get_string()){
                Entry::Vacant(e) => {
                    e.insert(vec![Orientation::_45]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(Orientation::_45);
                }
            }
        }
        if search_word_275(&word, &text, &pos).is_some() {
            center=Coordinates{x:pos.x+1,y:pos.y+1};
            match dic_x.entry(center.get_string()){
                Entry::Vacant(e) => {
                    e.insert(vec![Orientation::_275]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(Orientation::_275);
                }
            }
        }
    }
    let mut found=0;
    for (d,v) in dic_x{
        println!("{:?}",d);
        if v.len()==2{
            found+=1;
        }
    }
    Ok(found)
}
mod test {
    use super::*;
    #[test]
    fn test_horizzontal_search() {
        let text = vec![
            "00000Abcd00Abc000".to_string(),
            "Abc0".to_string(),
            "0A".to_string(),
        ];
        let positions = search_letter('A', &text);
        assert_eq!(positions.len(), 4);
        let mut positions = positions.iter();
        let mut result: Option<()>;

        result = search_word_horizontal(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());

        result = search_word_horizontal(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());

        result = search_word_horizontal(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());

        result = search_word_horizontal(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
    }
    #[test]
    fn test_vertical_search() {
        let text = vec![
            "AbcA".to_string(),
            "bAbb".to_string(),
            "ccAc".to_string(),
            "acvc".to_string(),
        ];

        let positions = search_letter('A', &text);
        assert_eq!(positions.len(), 4);
        let mut positions = positions.iter();
        let mut result: Option<()>;

        result = search_word_vertical(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());

        result = search_word_vertical(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());

        result = search_word_vertical(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
    }
    #[test]
    fn test_45_search() {
        let text = vec![
            "qqcA".to_string(),
            "qbcb".to_string(),
            "AbAc".to_string(),
            "Acvc".to_string(),
        ];

        let positions = search_letter('A', &text);
        assert_eq!(positions.len(), 4);
        let mut positions = positions.iter();
        let mut result: Option<()>;

        result = search_word_45(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
        result = search_word_45(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());
        println!("dggf");
        result = search_word_45(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
        result = search_word_45(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());
    }
    #[test]
    fn test_275_search() {
        let text = vec![
            "AccA".to_string(),
            "Abbb".to_string(),
            "AbcA".to_string(),
            "cccc".to_string(),
        ];

        let positions = search_letter('A', &text);
        assert_eq!(positions.len(), 5);
        let mut positions = positions.iter();
        let mut result: Option<()>;

        result = search_word_275(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());
        result = search_word_275(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
        result = search_word_275(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_some());
        result = search_word_275(&"Abc".to_string(), &text, &(positions.next().unwrap()));
        assert!(result.is_none());
    }
    #[test]
    fn test_advent_of_code_example() {
        let text = vec![
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
        let a = get_n_match("XMAS".to_string(), &text);
        let b = get_n_match("SAMX".to_string(), &text);
        assert_eq!(a.unwrap() + b.unwrap(), 18);
    }
    #[test]
    fn test_advent_of_code_x_example() {
        let text = vec![
            "MMSS".to_string(),
            "MAAM".to_string(),
            "MMSS".to_string(),
        ];
        let a = get_n_x_match("MAS".to_string(), &text);
        assert_eq!(a.unwrap(), 2);
    }
    #[test]
    fn test_advent_of_code_2_example() {
        let text = vec![
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
        let a = get_n_x_match("MAS".to_string(), &text);
        assert_eq!(a.unwrap() , 9);
    }
    #[test]
    fn test_advent_of_code() {
        let input = read_content("src/day4/input.txt".to_string());
        let a = get_n_match("XMAS".to_string(), &input);
        let b = get_n_match("SAMX".to_string(), &input);
        assert_eq!(a.unwrap() + b.unwrap(), 2591);
    }
    #[test]
    fn test_advent_of_code_x_2() {
        let input = read_content("src/day4/input.txt".to_string());
        let a = get_n_x_match("MAS".to_string(), &input);
        assert_eq!(a.unwrap(), 2);
    }
}
