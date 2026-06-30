// 这是一个 `main.rs` 文件，因此 `cargo` 将其解释为二进制目标（binary target）的根。

// TODO: 修复这个损坏的 import。在 `src` 目录中创建一个新的 library target。
//   library target 应该暴露一个名为 `hello_world` 的公共函数，
//   该函数不接受任何参数，也不返回任何内容。
use packages::hello_world;

// 这是二进制文件的入口点。
fn main() {
    hello_world();
}
