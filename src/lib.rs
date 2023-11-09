#![feature(strict_provenance)]

//!
//! Null主要用在其他数据结构中，让值本身支持判断是否空。可以提升内存性能，减少使用Option。
//!
use std::any::TypeId;
use std::mem::transmute;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::rc::Rc;
use std::sync::atomic::*;
use std::sync::Arc;

pub trait Null {
    /// 判断当前值是否空
    fn null() -> Self;
    /// 判断当前值是否空
    fn is_null(&self) -> bool;
}

impl<T> Null for Option<T> {
    #[inline(always)]
    fn null() -> Self {
        None
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        match self {
            Some(_) => false,
            None => true,
        }
    }
}

impl Null for usize {
    #[inline(always)]
    fn null() -> Self {
        usize::max_value()
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == usize::max_value()
    }
}
impl Null for isize {
    #[inline(always)]
    fn null() -> Self {
        isize::min_value()
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == isize::min_value()
    }
}
impl Null for bool {
    #[inline(always)]
    fn null() -> Self {
        false
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self
    }
}
impl Null for u8 {
    #[inline(always)]
    fn null() -> Self {
        u8::MAX
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == u8::MAX
    }
}
impl Null for i8 {
    #[inline(always)]
    fn null() -> Self {
        i8::MIN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == i8::MIN
    }
}
impl Null for u16 {
    #[inline(always)]
    fn null() -> Self {
        u16::MAX
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == u16::MAX
    }
}
impl Null for i16 {
    #[inline(always)]
    fn null() -> Self {
        i16::MIN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == i16::MIN
    }
}
impl Null for u32 {
    #[inline(always)]
    fn null() -> Self {
        u32::MAX
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == u32::MAX
    }
}
impl Null for i32 {
    #[inline(always)]
    fn null() -> Self {
        i32::MIN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == i32::MIN
    }
}
impl Null for u64 {
    #[inline(always)]
    fn null() -> Self {
        u64::MAX
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == u64::MAX
    }
}
impl Null for i64 {
    #[inline(always)]
    fn null() -> Self {
        i64::MIN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == i64::MIN
    }
}
impl Null for u128 {
    #[inline(always)]
    fn null() -> Self {
        u128::MAX
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == u128::MAX
    }
}
impl Null for i128 {
    #[inline(always)]
    fn null() -> Self {
        i128::MIN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        *self == i128::MIN
    }
}
impl Null for f32 {
    #[inline(always)]
    fn null() -> Self {
        f32::NAN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.is_nan()
    }
}
impl Null for f64 {
    #[inline(always)]
    fn null() -> Self {
        f64::NAN
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.is_nan()
    }
}

impl Null for AtomicUsize {
    #[inline(always)]
    fn null() -> Self {
        AtomicUsize::new(usize::max_value())
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == usize::max_value()
    }
}
impl Null for AtomicIsize {
    #[inline(always)]
    fn null() -> Self {
        AtomicIsize::new(isize::min_value())
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == isize::min_value()
    }
}
impl Null for AtomicBool {
    #[inline(always)]
    fn null() -> Self {
        AtomicBool::new(false)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}
impl Null for AtomicU8 {
    #[inline(always)]
    fn null() -> Self {
        AtomicU8::new(u8::MAX)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == u8::MAX
    }
}
impl Null for AtomicI8 {
    #[inline(always)]
    fn null() -> Self {
        AtomicI8::new(i8::MIN)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == i8::MIN
    }
}
impl Null for AtomicU16 {
    #[inline(always)]
    fn null() -> Self {
        AtomicU16::new(u16::MAX)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == u16::MAX
    }
}
impl Null for AtomicI16 {
    #[inline(always)]
    fn null() -> Self {
        AtomicI16::new(i16::MIN)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == i16::MIN
    }
}
impl Null for AtomicU32 {
    #[inline(always)]
    fn null() -> Self {
        AtomicU32::new(u32::MAX)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == u32::MAX
    }
}
impl Null for AtomicI32 {
    #[inline(always)]
    fn null() -> Self {
        AtomicI32::new(i32::MIN)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == i32::MIN
    }
}
impl Null for AtomicU64 {
    #[inline(always)]
    fn null() -> Self {
        AtomicU64::new(u64::MAX)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == u64::MAX
    }
}
impl Null for AtomicI64 {
    #[inline(always)]
    fn null() -> Self {
        AtomicI64::new(i64::MIN)
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.load(Ordering::Relaxed) == i64::MIN
    }
}

impl Null for &str {
    #[inline(always)]
    fn null() -> Self {
        ""
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.is_empty()
    }
}
impl Null for String {
    #[inline(always)]
    fn null() -> Self {
        String::new()
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.is_empty()
    }
}
impl<T: Null> Null for Box<T> {
    #[inline(always)]
    fn null() -> Self {
        Box::new(T::null())
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_ref().is_null()
    }
}
impl<T: Null> Null for Rc<T> {
    #[inline(always)]
    fn null() -> Self {
        Rc::new(T::null())
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_ref().is_null()
    }
}
impl<T: Null> Null for Arc<T> {
    #[inline(always)]
    fn null() -> Self {
        Arc::new(T::null())
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.as_ref().is_null()
    }
}
impl<T> Null for Vec<T> {
    #[inline(always)]
    fn null() -> Self {
        Vec::new()
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.is_empty()
    }
}
impl<T> Null for *const T {
    #[inline(always)]
    fn null() -> Self {
        unsafe { transmute(usize::null()) }
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.addr().is_null()
    }
}
impl<T> Null for *mut T {
    #[inline(always)]
    fn null() -> Self {
        unsafe { transmute(usize::null()) }
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.addr().is_null()
    }
}
impl<T: Null> Null for Range<T> {
    #[inline(always)]
    fn null() -> Self {
        Range {
            start: T::null(),
            end: T::null(),
        }
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self.start.is_null() && self.end.is_null()
    }
}
impl Null for TypeId {
    #[inline(always)]
    fn null() -> Self {
        TypeId::of::<()>()
        // unsafe {mem::transmute::<u128, TypeId>(u128::null()) }
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        self == &TypeId::of::<()>()
        // unsafe {mem::transmute::<&TypeId, &u128>(self) }.is_null()
    }
}
impl<T> Null for MaybeUninit<T> {
    #[inline(always)]
    fn null() -> Self {
        MaybeUninit::uninit()
    }
    #[inline(always)]
    fn is_null(&self) -> bool {
        true
    }
}

#[test]
fn test() {
    let s = Some(1);
    assert_eq!(s.is_null(), false);
    assert_eq!(1.is_null(), false);
    assert_eq!(i8::MIN.is_null(), true);
    assert_eq!(2.0f32.is_null(), false);
    assert_eq!("".is_null(), true);
    assert_eq!("2".is_null(), false);
    println!("TYpeId3:{:?}", TypeId::of::<()>());
}
