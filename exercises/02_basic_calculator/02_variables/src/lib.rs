// 👇 下面以 `///` 开头的行被称为**文档注释**。
//    它们将文档附加到紧随其后的项上。在这个例子中，是 `speed` 函数。
//    如果你从这个练习的目录运行 `cargo doc --open`，Rust 会从这些注释生成
//    HTML 文档并在你的浏览器中打开。

/// 给定一段旅程的起点和终点，以及完成这段旅程所花费的时间，
/// 计算平均速度。
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 定义一个名为 `distance` 的变量，赋以正确的值使测试通过
    //  你需要为 `distance` 添加类型注解吗？为什么需要或为什么不需要？
    let distance = end - start;
    // 不要修改下面这一行
    distance / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        assert_eq!(speed(10, 31, 10), 2);
    }
}
