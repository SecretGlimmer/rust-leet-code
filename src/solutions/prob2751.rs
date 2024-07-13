pub struct Solution {}

#[derive(Clone)]
#[derive(Debug)]
struct Robot {
    pos: i32,
    life: i32,
    index: usize,
}

impl Robot {
    fn new(pos: i32, life: i32, index: usize) -> Self {
        Robot { pos, life, index }
    }
}

impl Solution {
    pub fn survived_robots_healths(positions: Vec<i32>, healths: Vec<i32>, directions: String) -> Vec<i32> {
        let arr: Vec<Robot> = positions.iter().zip(healths.iter()).enumerate().map(|(i,(p,h))| Robot::new(*p*2,*h, i)).collect();
        let mut left_arr: Vec<Robot> = arr.iter().zip(directions.chars()).filter(|(_, d)|*d == 'R').map(|(a,_)|a).cloned().collect(); 
        let mut right_arr: Vec<Robot> = arr.iter().zip(directions.chars()).filter(|(_, d)|*d == 'L').map(|(a,_)|a).cloned().collect(); 

        left_arr.sort_by(|a, b| a.pos.cmp(&b.pos));
        right_arr.sort_by(|a, b| a.pos.cmp(&b.pos));

        while let (Some(left), Some(right)) = (left_arr.first(), right_arr.last()) {
            if left.pos > right.pos {
                break;
            }
            left_arr = left_arr.iter().map(|val|{
                Robot::new(val.pos + 1, val.life, val.index)
            }).collect();
            right_arr = right_arr.iter().map(|val|{
                Robot::new(val.pos - 1, val.life, val.index)
            }).collect();
            let mut l: usize = 0;
            let mut r: usize = 0;
            while l < left_arr.len() && r < right_arr.len() {
                let diff = left_arr[l].pos - right_arr[r].pos;
                if diff.abs() <= 2 {
                    if left_arr[l].life == right_arr[r].life {
                        left_arr.remove(l);
                        right_arr.remove(r);
                    } else if left_arr[l].life < right_arr[r].life {
                        left_arr.remove(l);
                        right_arr[r].life -= 1;
                    } else {
                        right_arr.remove(r);
                        left_arr[l].life -= 1;
                    }
                } else if diff < 0 {
                    l += 1;
                } else {
                    r += 1;
                }
            }

            left_arr = left_arr.iter().map(|val|{
                Robot::new(val.pos + 1, val.life, val.index)
            }).collect();
            right_arr = right_arr.iter().map(|val|{
                Robot::new(val.pos - 1, val.life, val.index)
            }).collect();
        }

        left_arr.extend(right_arr);

        left_arr.sort_by(|a, b| a.index.cmp(&b.index));
        
        left_arr.iter().map(|val|val.life).collect()

    }
}
