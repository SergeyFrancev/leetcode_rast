impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let nums = nums.into_iter().collect::<BucketSorter>();
        (0..(n>>1))
        .scan(
            nums.into_iter(), 
            |it, _| { 
                it.next()
                .map(|a| it.next_back().map(|b| a+b))
                .flatten()
            }
        ).max().unwrap()
    }
}

struct BucketSorter {
    buckets: [u16; 100000],
}

impl BucketSorter {
    pub fn new() -> Self {
        Self {
            buckets: [0; 100000]
        }
    }

    pub fn add(&mut self, num: i32) {
        self.buckets[(num-1) as usize] += 1;
    }

    pub fn into_iter(self) -> BucketSorterIter {
        BucketSorterIter {
            buckets: self.buckets,
            lo: 0,
            hi: 99999,
        }
    }
}

impl std::iter::FromIterator<i32> for BucketSorter {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let mut buckets = BucketSorter::new();
        for num in iter {
            buckets.add(num);
        }
        buckets
    }
}

struct BucketSorterIter {
    buckets: [u16; 100000],
    lo: usize,
    hi: usize,
}

impl Iterator for BucketSorterIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        while self.lo <= self.hi && self.buckets[self.lo] == 0 {
            self.lo += 1;
        }

        if self.buckets[self.lo] > 0 {
            self.buckets[self.lo] -= 1;
            Some((self.lo+1) as i32)
        } else {
            None
        }
    }
}

impl std::iter::DoubleEndedIterator for BucketSorterIter {
    fn next_back(&mut self) -> Option<i32> {
        while self.hi > 0 && self.lo <= self.hi && self.buckets[self.hi] == 0 {
            self.hi -= 1;
        }

        if self.buckets[self.hi] > 0 {
            self.buckets[self.hi] -= 1;
            Some((self.hi+1) as i32)
        } else {
            None
        }
    }
}