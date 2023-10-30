mod sort_by_bits;
use sort_by_bits::Solution;

fn main() {
    // let res = Solution::sort_by_bits(Vec::from([5]));
    let data = Vec::from([0,1,2,3,4,5,6,7,8]);
    let res = Solution::sort_by_bits(data);
    println!("-> res: {:?}", res);

}
