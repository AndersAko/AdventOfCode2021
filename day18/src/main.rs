use std::fs;
use std::fmt;
use std::collections::HashSet;

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
        enum Dir {
            FromRight,
            FromLeft,
            FromUp
        }

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
        enum Dir {
            FromRight,
            FromLeft,
            FromUp
        }

        // println!("Searching for right element of {}", self.fmt_pair(index));

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
                Dir::FromRight => 
                    if let Some(next) = node.parent {
                        coming_from = if self.get(next).rhs.unwrap() == node_index { Dir::FromRight } else { Dir::FromLeft };
                        node_index = next;
                    } else {
                        return None;
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

    fn explode (&mut self, index:PairIndex, nesting_level: usize) -> bool {
        if nesting_level == 4 && self.get(index).lhs.is_some() && self.get(index).rhs.is_some() { 
            println!("Exploding {} {}", nesting_level, self.fmt_pair(index));
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

    fn split (&mut self, index: PairIndex) -> bool {
        if let Some(reg) = self.get(index).regular {
            if reg >=10 {
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

    fn reduce (&mut self) {
        if let Some(tree_index) = self.root {
            loop {
                if self.explode(tree_index, 0) { continue }
                if self.split(tree_index) { continue }
                break;
            }
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
    let filecontents = fs::read_to_string("input_test_explode.txt").expect("Something went wrong?");
    let lines: Vec<&str> = filecontents.split_terminator("\n").collect();

    let input = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]";
    let mut index: usize = 0;
    let mut snail = SnailFishNumber::new();
    snail.root = Some(read_snailfish(&mut snail, input, &mut index, None));
    println!("Input: {} => {} ", input, snail);
    // for i in 0..snail.arena.len() {
    //     println!("Left of index {}({}) = {:?}", i, snail.fmt_pair(i),  snail.find_left(i));
    // }
    // for i in 0..snail.arena.len() {
    //     println!("Right of index {}({}) = {:?}", i, snail.fmt_pair(i),  snail.find_right(i));
    // }

    for line in lines {
        let mut index: usize = 0;
        let mut snail = SnailFishNumber::new();
        snail.root = Some(read_snailfish(&mut snail, line, &mut index, None));
        snail.reduce();
        println!("Input: {} reduces to => {} ", line, snail);
    }
}
