// TODO: 定义一个新的 trait `Power`，它有一个方法 `power`，将 `self`
//  提升到 `n` 次方。
//  trait 定义及其实现应该足以让测试编译并通过。
//
// 建议：你可能想写一个泛型实现来一次性处理所有情况。然而，这相当复杂，
// 需要使用额外的 crate（如 `num-traits`）。
// 即使那样，可能更倾向于使用简单的宏来避免高度泛型实现的复杂性。
// 如果你想了解更多，可以查看 "Little book of Rust macros"
// (https://veykril.github.io/tlborm/)。
// 不过你不必这样做：手动编写三个独立的实现也是完全可以的。
// 只有在好奇的时候才继续深入。
trait Power<T>{
    fn power(&self, n: T) -> u32;
}

impl Power<u32> for u32{
    fn power(&self, n: u32)->u32{
        self.pow(n)
    }
}

impl Power<u16> for u32{
    fn power(&self,n:u16)->u32{
        self.pow(n as u32)
    }
}

impl Power<&u32> for u32{
    fn power(&self,n:&u32)->u32{
        self.pow(*n)
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
