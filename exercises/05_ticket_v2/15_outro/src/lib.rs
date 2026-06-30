// TODO: 你需要在这个 crate 的每个模块中完成一些工作！
mod description;
mod status;
mod title;

// Rust 中的一个常见模式是将代码分割成多个（私有）模块，
// 然后在 crate 的根目录重新导出这些模块的公共部分。
//
// 这样可以向用户隐藏 crate 的内部结构，同时仍然允许你按照自己的喜好组织代码。
pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
// 我们不再需要将字段设为私有了！
// 因为每个字段都封装了自己的验证逻辑，所以 `Ticket` 的用户
// 不会有以破坏结构体不变量的方式修改字段的风险。
//
// 但要注意：如果你的不变量跨越多个字段，你需要确保这些不变量仍然被维护，
// 并将字段恢复为私有。
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
