
pub mod lib {

    use std::ops::Add;

    //                    type, pair_ind, num_ind
    #[derive(Debug)]
    pub struct ReduceAction(SnailFishType, i32, i32);

    #[derive(Debug,Eq,PartialEq)]
    pub enum SnailFishType {
        N,
        P
    }

    #[derive(Debug,Eq,PartialEq,Clone)]
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

        pub fn pairs(&self) -> usize {
            match self {
                Number(_) => 0,
                Pair(left, right) => 1 + left.pairs() + right.pairs()
            }
        }

        pub fn get_type(&self) -> SnailFishType {
            match self {
                Number(_) => SnailFishType::N,
                Pair(_,_) => SnailFishType::P
            }
        }
    
        pub fn get(&self, ind: i32) -> Option<&Self> {
    
            if ind < 0 {return None;}
            if ind >= self.len() as i32 {return None;}
    
            let mut i = 0;
    
            self.find(ind, &mut i)
        }
    
        pub fn find(&self, ind: i32, i: &mut i32) -> Option<&Self> {
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
                    if let Some(num) = left.find(ind, i) {
                        return Some(num);
                    }
                    right.find(ind, i)
                }
            }
        }
        
        pub fn get_mut(&mut self, ind: i32) -> Option<&mut Self> {

            if ind < 0 {return None;}
            if ind >= self.len() as i32 {return None;}

            let mut i = 0;

            self.find_mut(ind, &mut i)
        }

        pub fn find_mut(&mut self, ind: i32, i: &mut i32) -> Option<&mut Self> {
            match self {
                Number(_) => {
                    if ind == *i {
                        Some(self) 
                    } else { 
                        *i += 1;
                        None 
                    }
                },
                Pair(left, right) => {
                    if let Some(num) = left.find_mut(ind, i) {
                        return Some(num);
                    }
                    right.find_mut(ind, i)
                }
            }
        }

        pub fn get_pair(&self, ind: i32) -> Option<&Self> {
    
            if ind < 0 {return None;}
            if ind >= self.pairs() as i32 {return None;}
    
            let mut i = 0;
    
            self.find_pair(ind, &mut i)
        }
    
        pub fn find_pair(&self, ind: i32, i: &mut i32) -> Option<&Self> {
            match self {
                Number(_) => { None },
                Pair(left, right) => {
                    if ind == *i {
                        return Some(self);
                    }
                    *i += 1;
                    if let Some(pair) = left.find_pair(ind, i) {
                        return Some(pair);
                    }
                    right.find_pair(ind, i)
                }
            }
        }
    
        pub fn get_pair_mut(&mut self, ind: i32) -> Option<&mut Self> {
    
            if ind < 0 {return None;}
            if ind >= self.pairs() as i32 {return None;}
    
            let mut i = 0;
    
            self.find_pair_mut(ind, &mut i)
        }
    
        pub fn find_pair_mut(&mut self, ind: i32, i: &mut i32) -> Option<&mut Self> {

            if ind == *i && self.get_type() == SnailFishType::P {
                return Some(self);
            }

            match self {
                Number(_) => { None },
                Pair(left, right) => {
                    *i += 1;
                    if let Some(pair) = left.find_pair_mut(ind, i) {
                        return Some(pair);
                    }
                    right.find_pair_mut(ind, i)
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
            loop {
                let mut pair_i = 0;
                let mut num_i = 0;
                match self.check_explode(&mut pair_i, &mut num_i, 0) { // restart the traversal, depth = 0
                    Ok(_) => (),
                    Err(action) => {
                        match action {
                            ReduceAction(_, pair_ind, num_ind) => {
                                self.explode(pair_ind, num_ind);
                                continue;
                            }
                        }
                    }
                }
                pair_i = 0;
                num_i = 0;
                match self.check_split(&mut pair_i, &mut num_i) { // restart the traversal, depth = 0
                    Ok(_) => break, 
                    Err(action) => {
                        match action {
                            ReduceAction(_a, _b, num_ind) => {
                                self.split(num_ind);
                                continue;
                            }
                        }
                    }
                }
            }
            ()
        }

        fn check_explode(&self, pair_i: &mut i32, num_i: &mut i32, depth: i32) -> Result<(), ReduceAction> {
            match self {
                Pair(left, right) => { // We check for explosions first

                    if depth >= 4 {
                        return Err(ReduceAction(SnailFishType::P, *pair_i, *num_i));
                    }

                    *pair_i += 1; // By now we have succeeded against the checks
                    left.check_explode(pair_i, num_i, depth + 1)?;
                    right.check_explode(pair_i, num_i, depth + 1)?;
                },
                Number(_val) => {
                    *num_i += 1;
                }
            }
            Ok(())
        }
        
        fn check_split(&self, pair_i: &mut i32, num_i: &mut i32) -> Result<(), ReduceAction> {
            match self {
                Pair(left, right) => { // We check for explosions first
                    *pair_i += 1;
                    left.check_split(pair_i, num_i)?;
                    right.check_split(pair_i, num_i)?;
                },
                Number(val) => {
                    if *val > 9 {
                        return Err(ReduceAction(SnailFishType::N, *pair_i, *num_i));
                    }
                    *num_i += 1;
                }
            }
            Ok(())
        }
        
        /// Explodes a pair inside the root SnailFish number
        /// num_ind indicates the index of the left_most Number in the SnailFish number
        pub fn explode(&mut self, pair_ind: i32, num_ind: i32) {

            let pair = self.get_pair_mut(pair_ind).unwrap();
            match pair.clone() {
                Number(_) => (),
                Pair(left, right) => {
                    // For whatever reason this is okay
                    *pair = Number(0); // This reduces all indices to the right by 1

                    match self.get_mut(num_ind - 1) {
                        None => (),
                        Some(num) => {
                            *num = Number(num.magnitude() + left.magnitude());
                        }
                    }
                    match self.get_mut(num_ind + 1) {
                        None => (),
                        Some(num) => {
                            *num = Number(num.magnitude() + right.magnitude());
                        }
                    }
                }
            }
            
        }
        
        pub fn split(&mut self, ind: i32) {
            match self.get_mut(ind) {
                Some(num) if num.magnitude() <= 9 => {
                    // println!("{:?}", num.magnitude());
                    panic!("Tried splitting a number that was too small");
                },
                Some(num) => {
                    let val = num.magnitude();
                    let (left, right) = (val / 2, val / 2 + val % 2);
                    *num = Pair(Box::new(Number(left)),Box::new(Number(right)));
                },
                None => {
                    panic!("Tried splitting outside of SnailFish bounds");
                }
            }
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
    fn count_numbers() {
        let test = SnailFish::new("[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]");
        assert_eq!(test.len(), 13);
    }

    #[test]
    fn count_pairs() {
        let test = SnailFish::new("[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]");
        assert_eq!(test.pairs(), 12);
    }

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

    #[test]
    fn get_mut_snailfish() {
        let mut pair = SnailFish::new("[1,[2,3]]");
        let num = pair.get_mut(0).unwrap();
        *num = Pair(Box::new(Number(4)),Box::new(Number(5)));
        assert_eq!(pair, SnailFish::new("[[4,5],[2,3]]"));
    }

    #[test]
    fn get_pair_snailfish() {
        let test = SnailFish::new("[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]");
        let res = test.get_pair(4);
        assert_eq!(res, Some(&SnailFish::new("[[5,7],2]")));
    }

    #[test]
    fn get_pair_mut_snailfish() {
        let mut test = SnailFish::new("[[[[0,7],5],[[5,7],2]],[[2,[9,5]],[[7,7],[5,0]]]]");
        let victim = test.get_pair_mut(6).unwrap();
        *victim = Number(0);
        assert_eq!(test, SnailFish::new("[[[[0,7],5],[[5,7],2]],0]"));
    }

    #[test]
    fn explode_snailfish() {
        let mut test = SnailFish::new("[[5,[[3,[1,2]],4]],6]");
        test.explode(4, 2);
        assert_eq!(test, SnailFish::new("[[5,[[4,0],6]],6]"));
    }

    #[test]
    fn split_snailfish() {
        // I built the SnailFish differently b/c I only designed the new
        // function to recognize single digit numbers :/
        let mut pair = Pair(Box::new(Number(15)),Box::new(Number(4)));
        pair.split(0);
        assert_eq!(pair, SnailFish::new("[[7,8],4]"));
    }

    #[test]
    fn reduce_snailfish() {
        let left = SnailFish::new("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = SnailFish::new("[1,1]");
        assert_eq!(left + right, SnailFish::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    }

    #[test]
    fn sum_snailfishes() {
        let input = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]"
        ];
        let pairs: Vec<SnailFish> = input.into_iter().map(|s| SnailFish::new(s)).collect();
        assert_eq!(
            pairs.into_iter().reduce(|left, right| left + right).unwrap(),
            SnailFish::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
            );
    }
}
