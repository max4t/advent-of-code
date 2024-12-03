use std::ops::ControlFlow;

pub fn basic_solve(s: &str) -> i32 {
    let r = s.chars().map(|i| {
        match i {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    }).filter(|&i| i != 0).enumerate().try_fold(0_i64, |acc, (idx, v)| {
        if acc < 0 {
            ControlFlow::Break(idx)
        } else {
            ControlFlow::Continue(acc+v)
        }
    });
    match r {
        ControlFlow::Continue(_) => s.len().try_into().unwrap(),
        ControlFlow::Break(v) => v.try_into().unwrap(),
    }
}
