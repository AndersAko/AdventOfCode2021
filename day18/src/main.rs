use std::fs;
use std::fmt;

type PairIndex = usize;

#[derive(Debug, Clone)]
struct Pair {
    regular: Option<i8>,
    lhs: Option<PairIndex>,
    rhs: Option<PairIndex>,
    parent: Option<PairIndex>
}

struct SnailFishNumber {
    arena: Vec<Option<Pair>>,
    root: Option<PairIndex>
}

#[derive(Debug)]
enum Dir {
    FromRight,
    FromLeft,
    FromUp
}

impl SnailFishNumber {
    fn new() -> Self {
        SnailFishNumber { arena: Vec::new(), root: None }
    }
    fn add(&mut self,  new_pair: Pair) -> PairIndex {
        let index = self.arena.len();
        self.arena.push(Some(new_pair));
        index
    }
    fn get_mut(&mut self, index: PairIndex) -> &mut Pair {
        if let Some(node) = self.arena.get_mut(index) {
            node.as_mut().unwrap()
        } else {
            panic!()
        }
    }
    fn get(&self, index: PairIndex) -> & Pair {
        self.arena[index].as_ref().unwrap()
    }

    fn fmt_pair(&self, index: PairIndex) -> String {
        let node = self.get(index);
        if let Some(regular) = node.regular {
            format!("{}", regular)
        } else if node.lhs.is_some() && node.rhs.is_some() {
            format!("[{},{}]", self.fmt_pair(node.lhs.unwrap()), self.fmt_pair(node.rhs.unwrap()))
        } else {
            format!("[INVALID]")
        }
    }
// fn reduce(snail_fish: Pair) -> Pair {
//     loop {
//         if nested_4 { explode; continue; }
//         else if Regular>9 { split; }
//     }

// }    
   

    fn find_left(&mut self, index: PairIndex) -> Option<&mut Pair> {
        // println!("Searching for left element of {}", self.fmt_pair(index));

        let parent = self.get(index).parent;
        if parent.is_none() { return None }
        let parent = parent.unwrap();
        let mut coming_from = if self.get(parent).rhs.unwrap() == index { Dir::FromRight } else { Dir::FromLeft };
        let mut node_index = parent;
        loop {
            // println!("Trying {}", self.fmt_pair(node_index));
            let node = self.get(node_index);
            if node.regular.is_some() && node_index != index {
                return Some(self.get_mut(node_index));
            }
            match coming_from {
                Dir::FromLeft => 
                    if let Some(next) = node.parent {
                        coming_from = if self.get(next).rhs.unwrap() == node_index { Dir::FromRight } else { Dir::FromLeft };
                        node_index = next;
                    } else {
                        return None;
                    },
                Dir::FromRight => 
                    if let Some(next) = node.lhs {
                        coming_from = Dir::FromUp;
                        node_index = next;
                    } else {
                        return None;
                    },
                Dir::FromUp => 
                    if let Some(next) = node.rhs {
                        coming_from = Dir::FromUp;
                        node_index = next;
                    } else {
                        return None;
                    }
            }
        }
    }

    fn find_right(&mut self, index: PairIndex) -> Option<&mut Pair> {
        // println!("Searching for right element of {}", self.fmt_pair(index));

        let parent = self.get(index).parent;
        if parent.is_none() { return None }
        let parent = parent.unwrap();
        let mut coming_from = if self.get(parent).rhs.unwrap() == index { Dir::FromRight } else { Dir::FromLeft };
        let mut node_index = parent;
        loop {
            // println!("Trying {}({}) {:?}", self.fmt_pair(node_index), node_index, coming_from);
            let node = self.get(node_index);
            if node.regular.is_some() && node_index != index {
                return Some(self.get_mut(node_index));
            }
            match coming_from {
                Dir::FromRight => {
                    // println!("FromRight: parent = {:?} node_index = {}", node.parent, node_index);
                    if let Some(next) = node.parent {
                        coming_from = if self.get(next).rhs.unwrap() == node_index { Dir::FromRight } else { Dir::FromLeft };
                        node_index = next;
                    } else {
                        return None;
                    }
                },
                Dir::FromLeft => 
                    if let Some(next) = node.rhs {
                        coming_from = Dir::FromUp;
                        node_index = next;
                    } else {
                        return None;
                    },
                Dir::FromUp => 
                    if let Some(next) = node.lhs {
                        coming_from = Dir::FromUp;
                        node_index = next;
                    } else {
                        return None;
                    }
            }
        }
    }

    fn explode(&mut self, index:PairIndex, nesting_level: usize) -> bool {
        if nesting_level == 4 && self.get(index).lhs.is_some() && self.get(index).rhs.is_some() { 
            // println!("Exploding {} {}", nesting_level, self.fmt_pair(index));
            if let Some(lhs) = self.get(index).lhs {
                let lhs_regular = self.get(lhs).regular.unwrap();
                if let Some(left) = self.find_left(index) {
                    left.regular = left.regular.and_then(|x| Some(x + lhs_regular));
                }
            } else {
                println!("Exploding a pair without left link {}!!", self.fmt_pair(index));
                panic!()
            }
            if let Some(rhs) = self.get(index).rhs {
                let rhs_regular = self.get(rhs).regular;
                if let Some(right) = self.find_right(index) {
                    right.regular = right.regular.and_then(|x| Some(x + rhs_regular.unwrap()));
                }
            } else {
                println!("Exploding a pair without right link {}!!", self.fmt_pair(index));
                panic!()
            }
            let node = self.get_mut(index);
            node.regular = Some(0);
            node.lhs = None;
            node.rhs = None;
            true
        } else {
            if let Some(left) = self.get(index).lhs {
                if self.explode(left, nesting_level+1) {
                    return true
                }
            }
            if let Some(right) = self.get(index).rhs {
                if self.explode(right, nesting_level+1) {
                    return true
                }
            }
            false
        }
        
    }

    fn split(&mut self, index: PairIndex) -> bool {
        if let Some(reg) = self.get(index).regular {
            if reg >=10 {
                // println!("Splitting {}", self.fmt_pair(index));
                let new_left = self.add(Pair { 
                    regular: Some(reg / 2), lhs: None, rhs: None, parent: Some(index)
                });
                let new_right = self.add(Pair { 
                    regular: Some(reg - reg / 2), lhs: None, rhs: None, parent: Some(index)
                });
                let node = self.get_mut(index);
                node.lhs = Some(new_left);
                node.rhs = Some(new_right);
                node.regular = None;
                return true;
            }
        } else {
            if let Some(left) = self.get(index).lhs {
                if self.split(left) {
                    return true
                }
            }
            if let Some(right) = self.get(index).rhs {
                if self.split(right) {
                    return true
                }
            }
        }
        return false
    }

    fn reduce(&mut self) {
        if let Some(tree_index) = self.root {
            loop {
                // println!("-- {}", self);
                if self.explode(tree_index, 0) { continue }
                if self.split(tree_index) { continue }
                break;
            }
        }
    }

    fn magnitude(&self, index: PairIndex) -> usize {
        let node = self.get(index);
        if let Some(regular) = node.regular {
            regular as usize
        } else if node.lhs.is_some() && node.rhs.is_some() {
            self.magnitude(node.lhs.unwrap()) * 3 + self.magnitude(node.rhs.unwrap()) * 2
        } else {
            panic!();
        }
    }

}
impl fmt::Display for SnailFishNumber {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        if let Some(tree_index) = self.root {
            write!(f, "{}", self.fmt_pair(tree_index))
        } else {
            write!(f, "Empty")
        }
    }

}

fn read_snailfish(snail: &mut SnailFishNumber, input: &str, index: &mut usize, parent: Option<PairIndex> ) 
    -> PairIndex {
    // println!("read_snailfish {} {}", input, index);

    if &input[*index..*index+1] == "[" {
        *index += 1;
        let this_node = Pair { regular: None, lhs: None, rhs: None, parent: parent };
        let this_index = snail.add(this_node); 

        let lhs = read_snailfish(snail, input, index, Some(this_index));
        assert_eq!(&input[*index..*index+1], ",");
        *index += 1;
        let rhs = read_snailfish(snail, input, index, Some(this_index));
        assert_eq!(&input[*index..*index+1], "]");
        *index += 1;

        let this_node = snail.get_mut(this_index);
        this_node.lhs = Some(lhs);
        this_node.rhs = Some(rhs);
    
        this_index        
    } else {
        *index += 1;
        let this_node = Pair { 
            regular: input[*index-1..*index].parse::<i8>().ok(), lhs:None, rhs: None, parent
        };
        let this_index = snail.add(this_node);
        this_index
    }
} 

fn main() {
    let filecontents = fs::read_to_string("input.txt").expect("Something went wrong?");
    let lines: Vec<&str> = filecontents.split_terminator("\n").collect();

    let mut snail = SnailFishNumber::new();

    println!("Part 1");
    for line in &lines {
        let mut index: usize = 0;
        let number = Some(read_snailfish(&mut snail, line, &mut index, None));
        if snail.root.is_none() {
            snail.root = number;
        } else {
            let old_root = snail.root;
            let new_root = snail.add(Pair { 
                regular: None, parent: None, lhs: snail.root, rhs: number
            });
            snail.root = Some(new_root);
            snail.get_mut(old_root.unwrap()).parent = Some(new_root);
            snail.get_mut(number.unwrap()).parent = Some(new_root);
        }
        println!("Input: {} gives snail = {} ", line, snail);
        snail.reduce();
        println!("which reduces to => {} with magnitude {} ", snail, snail.magnitude(snail.root.unwrap()));
    }
    println!();

    // Part 2
    let mut highest = 0;
    for first in 0..lines.len() {
        for second in 0..lines.len() {
            if first == second { continue }
            let mut snail = SnailFishNumber::new();
            let mut index: usize = 0;
            let number1 = Some(read_snailfish(&mut snail, lines[first], &mut index, None));

            index = 0;
            let number2 = Some(read_snailfish(&mut snail, lines[second], &mut index, None));
            
            let root = snail.add(Pair { 
                regular: None, parent: None, lhs: number1, rhs: number2
            });
            snail.root = Some(root);
            snail.get_mut(number1.unwrap()).parent = Some(root);
            snail.get_mut(number2.unwrap()).parent = Some(root);
            
            snail.reduce();
            let magnitude = snail.magnitude(snail.root.unwrap());
            println!("{} + {} => {} with magnitude {} ", 
                snail.fmt_pair(number1.unwrap()), snail.fmt_pair(number2.unwrap()), 
                snail, magnitude
            );
            if magnitude > highest { highest = magnitude }
        }
    }
    println! ("Part2: Highest magnitude of two numbers = {}", highest);
}
