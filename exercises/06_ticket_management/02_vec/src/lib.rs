// 给定一个数字 `n`，返回 Fibonacci 序列中的第 `n+1` 个数字。
//
// Fibonacci 序列的定义如下：
//
// - 序列的第一个数字是 0。
// - 序列的第二个数字是 1。
// - 每个后续数字都是前两个数字的和。
//
// 所以序列是：0, 1, 1, 2, 3, 5, 8, 13, 21, 以此类推。
//
// 我们期望 `fibonacci(0)` 返回 `0`，`fibonacci(1)` 返回 `1`，
// `fibonacci(2)` 返回 `1`，以此类推。
pub fn fibonacci(n: u32) -> u32 {
    if n==0{
        return 0;
    }
    if n==1{
        return 1;
    }
    let n=n as usize;
    let mut fibs=vec![0u32,1u32];
    for i in 2..=n{
        fibs.push(fibs[i-1]+fibs[i-2]);
    }
    fibs[n]

}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
