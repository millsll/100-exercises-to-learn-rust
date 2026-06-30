// TODO: 定义一个名为 `sum` 的函数，它接受一个 `u32` 切片的引用，并返回切片中所有
//  元素的总和。
pub fn sum(slice:&[u32])->u32{
    let n:usize=slice.len();
    let mut s:u32=0;
    for i in 0..n{
        s+=slice[i];
    }
    s
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
