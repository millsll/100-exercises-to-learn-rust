// TODO: 定义一个新的 `SaturatingU16` 类型。
//   它应该持有一个 `u16` 值。
//   它应该提供从 `u16`、`u8`、`&u16` 和 `&u8` 的转换。 // 定义from转换
//   它应该支持与右侧类型为 `SaturatingU16`、`u16`、`&u16` 和 `&SaturatingU16` 的加法运算。
//   加法应该在 `u16` 的最大值处饱和。
//   它应该可以与另一个 `SaturatingU16` 或 `u16` 进行比较。
//   它应该可以打印其调试表示。
//
// 测试位于 `tests` 文件夹中——请注意你的类型和方法的可见性。

#[derive(Debug,Clone,Copy)]
pub struct SaturatingU16{
    value: u16,
}

impl From<u16> for SaturatingU16{
    fn from(value: u16) -> Self {
        Self {
            value:value,
        }
    }
}

impl From<u8> for SaturatingU16{
    fn from(value: u8) -> Self {
        Self {
            value: value as u16,
        }
    }
}

impl From<&u8> for SaturatingU16{
    fn from(value: &u8) -> Self {
        Self { 
            value: *value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16{
    fn from(value: &u16) -> Self{
        Self {
            value: *value,
        }
    }
}

use std::ops::Add;
// 支持加法运算 支持右值的加法运算
impl Add<u16> for SaturatingU16{
    type Output = Self;
    fn add(self, rhs: u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl Add<&u16> for SaturatingU16{
    type Output = Self;
    fn add(self, rhs: &u16) -> Self::Output {
        Self {
            value: self.value.saturating_add(*rhs),
        }
    }
}

impl Add<SaturatingU16> for SaturatingU16{
    type Output = Self;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl Add<&SaturatingU16> for SaturatingU16{
    type Output = Self;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        Self {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

// 支持与另一个SaturatingU16和U16进行比较
impl PartialEq for SaturatingU16{
    fn eq(&self, other: &Self)-> bool{
        self.value == other.value
    }
}

impl PartialEq<u16> for SaturatingU16{
    fn eq(&self, other: &u16)-> bool{
        self.value == *other
    }
}