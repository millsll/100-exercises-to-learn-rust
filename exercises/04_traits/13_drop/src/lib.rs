// TODO: 实现一个所谓的"Drop bomb"：一个在被丢弃时会 panic 的类型，
//  除非已经对它执行了某个特定操作。
//  你可以在下面的测试中看到预期的 API。

pub struct DropBomb {
    is_defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        Self {
            is_defused: false,
        }
    }

    pub fn defuse(&mut self) {
        self.is_defused = true;
    }
}

impl Drop for DropBomb{
    fn drop(&mut self) {
        if !self.is_defused {
            panic!("Bomb is not defused");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // 炸弹在被丢弃时应该 panic
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // 炸弹在被丢弃时不应该 panic
        // 因为它已经被解除了
    }
}
