use std::collections::HashSet;


struct Monad {
    // w, z
    visited : HashSet<(isize, isize, usize)>,
    params : [(isize,isize,isize);14]
}

impl Monad {
    fn new() -> Self {
        Monad {
            visited: HashSet::new(),
            params: [ 
                (1,12,7),       // digit 1
                (1, 13, 8),
                (1, 13, 10),
                (26, -2, 4),
                (26, -10, 4),   // digit 5
                (1, 13, 6),
                (26, -14, 11),
                (26, -5, 13),
                (1, 15, 1),
                (1, 15, 8),     // digit 10
                (26, -14, 4),
                (1, 10, 13),
                (26, -14, 4),
                (26, -5, 14)    // digit 14
            ]
        }
    }
    fn highest_valid_serial(&mut self,  w: isize, z: isize, digits: &String ) -> Option<String> {
        if self.visited.contains(&(w,z,digits.len())) {
            return None;
        } 
        if digits.len() == 7 { println!("is_valid: {} {} {}", w, z, digits); }
        
        self.visited.insert((w,z,digits.len()));
        // Calculate this digit
        // x = if !(z % 26 + 12) == w { 1 } else { 0 }         a = 1 b = 12 c = 7
        // z = ( z / 1 ) * (25 * x + 1) + (w + 7) * x
        // = z/a * x ? 25 : 1 + x ? (w + c) : 0 
        let (a,b,c) = self.params[digits.len()];
        // println!("a: {} b: {} c: {}", a, b, c);
        let x = if (z % 26 + b) == w { 0 } else { 1 };
        let z = z / a * ( 25 * x + 1) + (w + c) * x;

        // println!("x: {} z: {}", x, z);
        let digits = format!("{}{}", digits, w);
        if digits.len() == 14 {
            // println!("Reached end of line with {} {}", digits, z);
            if z == 0 {
                return Some(digits);
            } else { 
                return None;
            }
        }
        // Then rest of digits
        for n in (1..=9).rev() {
            if let Some(highest_serial) = self.highest_valid_serial(n,z,&digits) {
                return Some(highest_serial);
            } 
        }
        // println!("Tried all 9 numbers with {}", digits);
        return None;
    } 
    
    fn lowest_valid_serial(&mut self,  w: isize, z: isize, digits: &String ) -> Option<String> {
        if self.visited.contains(&(w,z,digits.len())) {
            return None;
        } 
        if digits.len() == 7 { println!("is_valid: {} {} {}", w, z, digits); }
        
        self.visited.insert((w,z,digits.len()));
        let (a,b,c) = self.params[digits.len()];
        // println!("a: {} b: {} c: {}", a, b, c);
        let x = if (z % 26 + b) == w { 0 } else { 1 };
        let z = z / a * ( 25 * x + 1) + (w + c) * x;

        // println!("x: {} z: {}", x, z);
        let digits = format!("{}{}", digits, w);
        if digits.len() == 14 {
            // println!("Reached end of line with {} {}", digits, z);
            if z == 0 {
                return Some(digits);
            } else { 
                return None;
            }
        }
        // Then rest of digits
        for n in (1..=9) {
            if let Some(lowest_serial) = self.lowest_valid_serial(n,z,&digits) {
                return Some(lowest_serial);
            } 
        }
        // println!("Tried all 9 numbers with {}", digits);
        return None;
    } 

}



fn main() {
    let mut monad = Monad::new();
    for n in (1..=9).rev() {
        if let Some(highest_serial) = monad.highest_valid_serial(n,0,&"".to_string()) {
            println!("Part 1: Found a high valid serial number {}", highest_serial);
            break; 
       } 
    }

    for n in 1..=9 {
        if let Some(lowest_serial) = monad.lowest_valid_serial(n,0,&"".to_string()) {
            println!("Part 2: Found a low valid serial number {}", lowest_serial);
            break; 
       } 
    }

}
