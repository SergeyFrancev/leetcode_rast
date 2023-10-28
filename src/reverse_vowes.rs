pub struct Solution {

}

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut out: Vec<char> = s.chars().into_iter().collect();
        let mut point = (0, s.len() - 1);
        let vowes = vec!['a', 'e', 'i', 'o', 'u', 'y'];
        let mut l: Option<usize> = None;
        let mut r: Option<usize> = None;
        loop {
            while l.is_none() && point.0 < point.1 {
                if vowes.contains(&out[point.0].to_ascii_lowercase()) {
                    l = Some(point.0);
                } else {
                    point.0 += 1;
                }
            }

            while r.is_none() && point.0 < point.1 {
                if vowes.contains(&out[point.1].to_ascii_lowercase()) {
                    r = Some(point.1);
                } else {
                    point.1 -= 1;
                }
            }

            // println!("-> {:?} - {:?}", l, r);
            if l.is_some() && r.is_some() {
                out.swap(l.unwrap(), r.unwrap());
                l = None;
                r = None;
                point.0 += 1;
                point.1 -= 1;
            } else {
                break;
            }
        }
        out.into_iter().collect()
    }
}

#[cfg(test)]
mod reverse_vowels {
    use super::Solution;

    #[test]
    fn case1() {
        let sum = Solution::reverse_vowels(String::from("ase"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("esa"));
    }

    #[test]
    fn case2() {
        let sum = Solution::reverse_vowels(String::from("Ase"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("esA"));
    }

    #[test]
    fn case3() {
        let sum = Solution::reverse_vowels(String::from("ddasE"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("ddEsa"));
    }

    #[test]
    fn case4() {
        let sum = Solution::reverse_vowels(String::from("yrrrrrrr"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("yrrrrrrr"));
    }

    #[test]
    fn case5() {
        let sum = Solution::reverse_vowels(String::from("ey"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("ye"));
    }

    #[test]
    fn case6() {
        let sum = Solution::reverse_vowels(String::from(" "));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from(" "));
    }

    #[test]
    fn case7() {
        let sum = Solution::reverse_vowels(String::from("o"));
        // assert!(res.is_ok(), "Valid path is not OK");
        assert_eq!(sum, String::from("o"));
    }
}