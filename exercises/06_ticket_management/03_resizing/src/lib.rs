#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // 已达到最大容量
        assert_eq!(v.capacity(), 2);

        v.push(3); // 超出容量，需要重新分配

        // 你能猜出新的容量是多少吗？
        // 注意，标准库不保证用于重新分配向量的算法，
        // 所以这可能会在未来发生变化。
        assert_eq!(v.capacity(), 4);
    }
}
