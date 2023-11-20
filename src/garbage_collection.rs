pub struct Solution {

}

struct Tracks {
    // M, P, G,
    time: Vec<usize>,
    stack: Vec<usize>,
}

impl Tracks {
    pub fn new() -> Self {
        Self { time: vec![0; 3], stack: vec![0; 3] } 
    }

    pub fn pick(&mut self, ch: char) {
        let idx = match ch {
            'M' => 0,
            'P' => 1,
            'G' => 2,
            _ => 3
        };
        assert!(idx < 3, "Undefined value for Pick operations");
        self.time[idx] += 1 + self.stack[idx];
        println!("Pick: [{ch}] + 1 + {} = {}", self.stack[idx], self.time[idx]);
        self.stack[idx] = 0;
    }

    pub fn trevale(&mut self, time: usize) {
        self.stack[0] += time;
        self.stack[1] += time;
        self.stack[2] += time;
    }

    pub fn total(&self) -> usize {
        self.time.iter().sum()
    }
}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let mut tracks = Tracks::new();
        for (h, trash) in garbage.into_iter().enumerate() {
            for (idx, item) in trash.char_indices() {
                tracks.pick(item);
            }
            if h < travel.len() {
                tracks.trevale(travel[h] as usize);
            }
        }
        tracks.total() as i32
    }
}