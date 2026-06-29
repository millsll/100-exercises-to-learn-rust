// TODO: 为 `min` 添加必要的 trait 约束，使其能够编译成功。
//   请查阅 `std::cmp` 模块的文档，了解你可能需要的 trait 的更多信息。
//
// 注意：有不同的 trait 约束可以让编译器通过，但它们
// 带有不同的语义。我们稍后在讨论有序集合（如 BTreeMap）时会详细介绍这些区别。

/// 返回两个值中的最小值。
pub fn min<T:Ord>(left: T, right: T) -> T {
    if left <= right {
        left
    } else {
        right
    }
}
