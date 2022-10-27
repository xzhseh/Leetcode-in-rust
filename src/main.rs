mod solution;
mod solution1;
mod solution2;
mod solution5;
mod solution21;
mod solution43;
mod solution48;
mod solution49;
mod solution59;
mod solution70;
mod solution119;
mod solution121;
mod solution187;
mod solution205;
mod solution206;
mod solution238;
mod solution240;
mod solution278;
mod solution290;
mod solution334;
mod solution392;
mod solution409;
mod solution415;
mod solution435;
mod solution509;
mod solution560;
mod solution704;
mod solution724;
mod solution733;
mod solution746;
mod solution763;
mod solution876;
mod solution1480;

fn main() {
    let current_vec: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
    solution733::Solution::flood_fill(current_vec.clone(), 1, 1, 2);
}