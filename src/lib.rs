use std::cmp::PartialEq;

trait LinearSearch<T: PartialEq> {
    fn linear_search<'a>(&'a self, value: T) -> Option<&'a T>;
}

impl<T: PartialEq> LinearSearch<T> for Vec<T> {
    fn linear_search<'a>(&'a self, value: T) -> Option<&'a T> {
        for x in self.iter() {
            if x == &value {
                return Some(x);
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let vec = vec![1,2,3,4];
        assert!(
            vec.linear_search(2) == Some(&vec[1])
        );
    }
    
    #[test]
    fn it_works_too() {
        let vec = vec![1,2,3,4];
        assert_eq!(
            vec.linear_search(5),
            None
        );
    }
}