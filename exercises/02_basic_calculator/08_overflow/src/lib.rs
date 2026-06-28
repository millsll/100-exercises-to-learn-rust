// 自定义 `dev` 配置文件，使其在溢出时进行回绕（wrap around）。
// 请查阅 Cargo 文档以找到正确的语法：
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// 出于我们稍后会解释的原因，自定义需要在仓库根目录的 `Cargo.toml` 中进行，
// 而不是在该练习的 `Cargo.toml` 中。

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! 等于 2432902008176640000，这个数太大了，无法放入 u32
        // 在默认的 dev 配置文件下，运行 `cargo test` 时会发生 panic
        // 我们希望它改为回绕（wrap around）
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // 使用下划线提高大数字字面量的可读性！
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
