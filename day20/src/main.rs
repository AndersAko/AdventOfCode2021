use std::fs;
use std::collections::HashSet;

fn main() {
    let file_contents = fs::read_to_string("input.txt").expect("Failed!");

    let mut lines = file_contents.split("\n");

    let iea: Iea = lines.next().unwrap().chars().collect();
    assert_eq!(iea.len(), 512);
    lines.next();

    let mut input_image = HashSet::new();
    let mut row = 0;
    while let Some(line) = lines.next() {
        let mut col = 0;
        for c in line.chars() {
            if c == '#' {
                input_image.insert( (col, row));
            }
            col += 1;
        }
        row += 1;
    }
    println!("Input: ");
    print(&input_image);
    println!("Iea: {:?}", iea);

    println!();
    let mut invert = false;
    let image = enhance(&input_image, &iea, invert);
    print(&image);
    println!("Enhanced 1 time to {} pixels", image.len());

    if iea[0]=='#' { invert = !invert; }
    let image = enhance(&image, &iea, invert);
    print(&image);
    println!("Part 1: Enhanced 2 times to {} pixels", image.len());

    let mut invert = false;
    let mut image = input_image;
    for i in 0..50 {
        image = enhance(&image, &iea, invert);
        if iea[0]=='#' { invert = !invert; }
        println!("Pass: {} = {} pixels ({})", i, image.len(), invert);
    }
    println!("Part 2: After 50 passes, there are {} pixels", image.len());
}
type Image = HashSet<(i32, i32)>;
type Iea = Vec<char>;

fn enhance(image: &Image, iea: &Iea, invert: bool) -> Image {
    let min_x = image.iter().min_by(|&x,&y| x.0.cmp(&y.0)).unwrap().0;
    let min_y = image.iter().min_by(|&x,&y| x.1.cmp(&y.1)).unwrap().1;
    let max_x = image.iter().max_by(|&x,&y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = image.iter().max_by(|&x,&y| x.1.cmp(&y.1)).unwrap().1;

    let mut result = HashSet::new();
    for x in min_x-1..=max_x+1 {
        for y in min_y-1..=max_y+1 {
            let mut index_str = String::new();
            for j in 0..3 {
                for i in 0..3 {
                    index_str.push( 
                        if image.contains(&(x+i-1, y+j-1)) || 
                            invert && (x+i-1 < min_x || x+i-1 > max_x || y+j-1 < min_y || y+j-1 > max_y) {
                                 '1' 
                            } else {
                                 '0' 
                            }
                    );
                }
            }
            assert_eq!(index_str.len(), 9);
            let index = usize::from_str_radix(&index_str, 2).unwrap();
            assert_eq!(format!("{:09b}",index), index_str);
            // println!("Index: {} {:09b} = {}", index_str, index, iea[index]);
            if iea[index] == '#' { result.insert((x,y)); } 
            else if iea[index] != '.' { panic!(); }
        }
    }
    result
}

fn print(image: &Image) {
    let min_x = image.iter().min_by(|&x,&y| x.0.cmp(&y.0)).unwrap().0;
    let min_y = image.iter().min_by(|&x,&y| x.1.cmp(&y.1)).unwrap().1;
    let max_x = image.iter().max_by(|&x,&y| x.0.cmp(&y.0)).unwrap().0;
    let max_y = image.iter().max_by(|&x,&y| x.1.cmp(&y.1)).unwrap().1;

    for y in min_y-1..=max_y+1 {
        for x in min_x-1..=max_x+1 {
            if image.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}