// TODO: 使用 `spawn` 和 `join` 实现 `sum` 函数的多线程版本。
//  给定一个整数向量，将其拆分为两半，
//  在单独的线程中对每一半求和。

// 注意：我们无法测试函数的*实现方式*，
// 只能验证它是否产生正确的结果。
// 你_可以_直接返回 `v.iter().sum()` 来通过测试，
// 但这违背了本练习的目的。
//
// 提示：你无法让生成的线程直接_借用_向量的切片。
// 你需要为原始向量的每一半分配新的向量。
// 我们将在下一节中看到为什么这是必要的。
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid= v.len()/2;
    let(v1,v2)=v.split_at(mid);
    let v1=v1.to_vec();
    let v2=v2.to_vec();
    let handle1=thread::spawn(move||v1.iter().sum::<i32>());
    let handle2=thread::spawn(move||v2.iter().sum::<i32>());

    handle1.join().unwrap()+handle2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
