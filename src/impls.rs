use core::sync::atomic;

/// Ideally, an atomic pointer is used, with a function pointer being the
/// same size as a data pointer.
const USE_ATOMIC_PTR: bool =
    cfg!(target_has_atomic = "ptr") && size_of::<fn()>() == size_of::<*mut ()>();

/// Uses const generics to statically select an atomic and raw inner type that
/// is layout-compatible with a `fn()`.
///
/// # Safety
/// - The types in `Atomic` and `Raw` must have the same size as `SIZE`.
/// - If `USE_ATOMIC_PTR` is `true`, they must be `AtomicPtr<U>` and `*mut U`
///   respectively.
/// - These types must not have stricter bit validity requirements than `fn`.
pub unsafe trait SelectAtomicFnInner<const USE_ATOMIC_PTR: bool, const SIZE: usize> {
    /// The equivalent `Atomic` type with a size and alignment compatible
    /// with `fn` on this platform.
    type Atomic;

    /// The non-atomic type stored in the [`AtomicFnInner`] atomic type.
    type Raw;
}

#[cfg(target_has_atomic = "ptr")]
unsafe impl SelectAtomicFnInner<true, { size_of::<*mut ()>() }> for fn() {
    type Atomic = atomic::AtomicPtr<()>;
    type Raw = *mut ();
}

#[cfg(target_has_atomic = "16")]
unsafe impl SelectAtomicFnInner<false, { size_of::<u16>() }> for fn() {
    type Atomic = atomic::AtomicU16;
    type Raw = u16;
}

#[cfg(target_has_atomic = "32")]
unsafe impl SelectAtomicFnInner<false, { size_of::<u32>() }> for fn() {
    type Atomic = atomic::AtomicU32;
    type Raw = u32;
}

#[cfg(target_has_atomic = "64")]
unsafe impl SelectAtomicFnInner<false, { size_of::<u64>() }> for fn() {
    type Atomic = atomic::AtomicU64;
    type Raw = u64;
}

/// An `Atomic` type with a size and alignment compatible to store and load any
/// `fn` on this platform.
pub type AtomicFnInner =
    <fn() as SelectAtomicFnInner<USE_ATOMIC_PTR, { size_of::<fn()>() }>>::Atomic;

/// The non-atomic type stored in the [`AtomicFnInner`] atomic type.
pub type AtomicFnInnerRaw =
    <fn() as SelectAtomicFnInner<USE_ATOMIC_PTR, { size_of::<fn()>() }>>::Raw;

const _: () = assert!(
    size_of::<AtomicFnInner>() == size_of::<fn()>()
        && size_of::<AtomicFnInner>() == size_of::<AtomicFnInnerRaw>()
);
const _: fn(&AtomicFnInner) = |inner: &AtomicFnInner| {
    let _check_inner_type: *mut AtomicFnInnerRaw = inner.as_ptr();
};
