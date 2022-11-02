pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let n = rooms.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut num: i32 = 0;
        dfs(rooms, &mut visited, 0, &mut num);

        num == n as i32
    }
}

pub fn dfs(rooms: Vec<Vec<i32>>, visited: &mut Vec<bool>, x: i32, num: &mut i32) {
    visited[x as usize] = true;
    *num += 1;
    for room in rooms[x as usize].clone() {
        if !visited[room as usize] {
            dfs(rooms.clone(), visited, room, num);
        }
    }
}
