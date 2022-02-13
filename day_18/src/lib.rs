
pub mod lib {

    use std::ops::Add;

    #[derive(Debug,Eq,PartialEq)]
    pub enum SnailFish {
        Number(i32),
        Pair(Box<SnailFish>, Box<SnailFish>)
    }
    use SnailFish::{Number, Pair};
    
    /// They told me I would regret it if I tried to make Rust into an object-
    /// oriented programming language...
    ///
    /// I. HAVE. NO. REGRETS.
    impl SnailFish {
        
        pub fn new(s: &str) -> Self {
            let mut pos = 0usize;
            Self::new_recurse(s, &mut pos)
        }
    
        pub fn new_recurse(s: &str, pos: &mut usize) -> Self {
            loop {
                match &s[(*pos)..(*pos+1)] {
                    "[" => { *pos += 1;
                        return Pair(
                            Box::new(Self::new_recurse(s, pos)),
                            Box::new(Self::new_recurse(s, pos))
                            );
                    },
                    "," => { *pos += 1; },
                    "]" => { *pos += 1; },
                    _ => { *pos += 1; break Number(s[(*pos-1)..(*pos)].parse().unwrap()) }
                }
            }
        }
        
        pub fn len(&self) -> usize {
            match self {
                Number(_) => 1,
                Pair(left, right) => left.len() + right.len()
            }
        }
    
        pub fn get(&self, ind: i32) -> Option<&Self> {
    
            if ind < 0 {return None;}
            if ind >= self.len() as i32 {return None;}
    
            let mut i = 0;
    
            self.search(ind, &mut i)
        }
    
        pub fn search(&self, ind: i32, i: &mut i32) -> Option<&Self> {
            match self {
                Number(_) => {
                    if ind == *i {
                        Some(&self) 
                    } else { 
                        *i += 1;
                        None 
                    }
                },
                Pair(left, right) => {
                    if let Some(num) = left.search(ind, i) {
                        return Some(num);
                    }
                    right.search(ind, i)
                }
            }
        }
        
        pub fn get_mut(&mut self) -> Option<&mut Self> {
            Some(self)
        }

        pub fn search_mut(&self, ind: i32, i: &mut i32) -> Option<&Self> {
            match self {
                Number(_) => {
                    if ind == *i {
                        Some(&mut self) 
                    } else { 
                        *i += 1;
                        None 
                    }
                },
                Pair(left, right) => {
                    if let Some(num) = left.search_mut(ind, i) {
                        return Some(num);
                    }
                    right.search_mut(ind, i)
                }
            }
        }
    
        pub fn magnitude(&self) -> i32 {
            match self {
                Number(val) => *val,
                Pair(left, right) => 3*left.magnitude() + 2*right.magnitude()
            }
        }
        
        pub fn reduce(&mut self) {
        
        }
        
        pub fn explode(&mut self, ind: i32) {
        
        }
        
        pub fn split(&mut self, ind: i32) {
        
        }
    }
    
    impl Add for SnailFish {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            let mut sum = Pair(Box::new(self), Box::new(rhs));
            sum.reduce();
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::SnailFish;
    use crate::lib::SnailFish::{Pair,Number};

    #[test]
    fn add_snailfish() {
        let num1 = SnailFish::new("[1,2]");
        let num2 = SnailFish::new("[3,4]");
        let num3 = num1 + num2;
        assert_eq!(num3, SnailFish::new("[[1,2],[3,4]]"));
    }

    #[test]
    fn get_snailfish() {
        let pair = SnailFish::new("[1,[2,3]]");
        assert_eq!(pair.get(-1), None);
        assert_eq!(pair.get(0), Some(&Number(1)));
        assert_eq!(pair.get(1), Some(&Number(2)));
        assert_eq!(pair.get(2), Some(&Number(3)));
        assert_eq!(pair.get(3), None);
    }
}
