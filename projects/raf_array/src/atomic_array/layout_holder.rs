#![allow(clippy::items_after_statements)]

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{align_of, size_of},
    sync::atomic::{
        AtomicU32,
        Ordering}
};

use const_format::formatcp;

use super::macros::{readonly, readonly_by_value};

pub(super) const MAX_PREFIX: usize = 8192;

readonly_by_value!(
    pub(super) struct AlignmentType {
        value: usize,
    }
);

readonly_by_value!(
    pub(super) struct OffsetType {
        value: usize,
    }
);

pub(super) struct AtomicType {
    value: AtomicU32,
}

impl AtomicType {
    const _STATIC_CHECK: () = {
        assert!(size_of::<AtomicType>() == size_of::<AtomicU32>());
        assert!(align_of::<AtomicType>() == align_of::<AtomicU32>());
        assert!(size_of::<AtomicType>() == size_of::<u32>());
        assert!(size_of::<AtomicType>() == 4);
        assert!(align_of::<AtomicType>() == 4);
    };

    #[inline(always)]
    pub(super) fn new(value: u32) -> Self {
        Self { value: AtomicU32::new(value) }
    }

    #[inline(always)]
    pub(super) fn atomic_load(&self) -> u32 {
        self.value.load(Ordering::SeqCst)
    }

    #[inline(always)]
    pub(super) fn atomic_add(&mut self, val: u32) -> u32 {
        self.value.fetch_add(val, Ordering::SeqCst)
    }

    #[inline(always)]
    pub(super) fn atomic_sub(&mut self, val: u32) -> u32 {
        self.value.fetch_sub(val, Ordering::SeqCst)
    }

    #[inline(always)]
    pub(super) fn compare_exchange_weak(&mut self, old_value: u32, new_value: u32)
        -> Result<u32, u32>
    {
        self.value.compare_exchange_weak(
            old_value,
            new_value,
            Ordering::SeqCst,
            Ordering::SeqCst)
    }
}

readonly!(
    pub(super) struct InternalArrayLayout
    {
        total_alignment: AlignmentType,
        strong_offset: OffsetType,
        weak_offset: OffsetType,
        data_offset: OffsetType,
    }
);

const fn _max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

#[inline(always)]
const fn _round_to_power_of(size: usize, align: usize) -> usize {
    size.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
}

const fn _extend_layout(layout: &Layout, next: &Layout) -> (Layout, usize) {
    let new_align = _max(layout.align(), next.align());
    let padded_size = _round_to_power_of(layout.size(), new_align);
    let new_size = padded_size + next.size();

    match Layout::from_size_align(new_size, new_align)
    {
        Ok(new_layout) => { return (new_layout, padded_size); }
        Err(_) => { panic!("Couldn't extend layout." )}
    }
}


#[allow(clippy::cast_possible_wrap)]
const fn _add_field_to_layout<T>(base_layout: &Layout)
    -> (Layout, OffsetType) 
{
    let total_size = size_of::<T>();
    let alignment = align_of::<T>();
    match Layout::from_size_align(total_size, alignment)
    {
        Ok(layout) => {
            let (new_layout, offset) 
                = _extend_layout(base_layout, &layout);
            assert!(offset <= i16::MAX as usize, "Offset doesn't fit in i16");
            return (new_layout, OffsetType::new(offset));
        }
        Err(_) => {
            panic!("Couldn't get layout.");
        }
    }
}

pub(super) struct LayoutHolder<T>(PhantomData<T>);

impl<T> LayoutHolder<T>
{
    pub const LAYOUT: InternalArrayLayout = {
        let size = size_of::<T>();
        let align = align_of::<T>();
        assert!(size > 0, "Size of T has to be positive.");
        assert!(align <= size, "Alignment has to be smaller than size of T.");
        assert!(align > 0, "Alignment has to be positive.");
        assert!(align.is_power_of_two(), "Alignment has to be a power of 2");
        assert!(size % align == 0, "Alignment has to divide size.");

        let mut layout: Layout = match Layout::from_size_align(0, 1) {
            Ok(result_layout) => { result_layout }
            Err(_) => { panic!("Couldn't initalize layout.") }
        };

        let total_alignment: AlignmentType;
        let strong_offset: OffsetType;
        let weak_offset: OffsetType;
        let data_offset: OffsetType;

        {
            let (new_layout, offset) 
                = _add_field_to_layout::<AtomicType>(&layout);
            layout = new_layout;
            strong_offset = offset;
        }

        {
            let (new_layout, offset) 
                = _add_field_to_layout::<AtomicType>(&layout);
            layout = new_layout;
            weak_offset = offset;
        }

        {
            let (_, offset)
                = _add_field_to_layout::<T>(&layout);
            data_offset = offset;

            const MSG: &str = formatcp!("Data offset above {MAX_PREFIX}? Something definitely went wrong.");
            assert!(data_offset.value <= MAX_PREFIX, "{}", MSG);
        }

        {
            let mut numeric_align = align_of::<AtomicType>();
            numeric_align = _max(numeric_align, align_of::<T>());
            const MSG: &str = formatcp!("Alignment above {MAX_PREFIX}? Something definitely went wrong.");
            assert!(numeric_align <= MAX_PREFIX, "{}", MSG);
            assert!(numeric_align.is_power_of_two(), "Alignment has to be a power of 2.");
            total_alignment = AlignmentType::new(numeric_align);
        }

        InternalArrayLayout::new(total_alignment, strong_offset, weak_offset, data_offset)
    };

    #[inline(always)]
    pub fn get_layout_for_length(length: usize) -> Layout {
        let real_alignment = Self::LAYOUT.total_alignment().value();
        let real_length = {
            let total_size = Self::LAYOUT.data_offset().value() + (length * size_of::<T>());
            _round_to_power_of(total_size, real_alignment)
        };
        unsafe { Layout::from_size_align_unchecked(real_length, real_alignment) }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct InternalArrayLayoutExplicit
    {
        pub total_alignment: usize,
        pub strong_offset: usize,
        pub weak_offset: usize,
        pub data_offset: usize,
    }

    const fn explicit_layout<T>() -> InternalArrayLayoutExplicit {
        let layout = LayoutHolder::<T>::LAYOUT;
        InternalArrayLayoutExplicit {
            total_alignment: layout.total_alignment().value(),
            strong_offset: layout.strong_offset().value(),
            weak_offset: layout.weak_offset().value(),
            data_offset: layout.data_offset().value(),
        }
    }

    #[test]
    fn test_i32_layout() {
        assert_eq!(size_of::<i32>(), 4);
        assert_eq!(align_of::<i32>(), 4);
        let layout_info = explicit_layout::<i32>();
        assert_eq!(layout_info.total_alignment, 4);
        assert_eq!(layout_info.strong_offset, 0);
        assert_eq!(layout_info.weak_offset, 4);
        assert_eq!(layout_info.data_offset, 8);
        let layout = LayoutHolder::<i32>::get_layout_for_length(3);
        assert_eq!(layout.size(), 20);
        assert_eq!(layout.align(), 4);

        let layout = LayoutHolder::<i32>::get_layout_for_length(0);
        assert_eq!(layout.size(), 8);
        assert_eq!(layout.align(), 4);
    }

    #[test]
    fn test_u64_layout() {
        assert!(size_of::<u64>() >= size_of::<u32>());
        assert!(align_of::<u64>() >= align_of::<u32>());
        let layout = explicit_layout::<u64>();
        assert_eq!(layout.total_alignment, align_of::<u64>());
        assert_eq!(layout.strong_offset, 0);
        assert_eq!(layout.weak_offset, 4);
        let data_offset = _round_to_power_of(8, align_of::<u64>());
        assert_eq!(layout.data_offset, data_offset);
        let layout = LayoutHolder::<u64>::get_layout_for_length(5);
        assert_eq!(layout.size(), data_offset + 5*size_of::<u64>());
        assert_eq!(layout.align(), align_of::<u64>());

        let layout = LayoutHolder::<u64>::get_layout_for_length(0);
        assert_eq!(layout.size(), data_offset);
        assert_eq!(layout.align(), align_of::<u64>());
    }

    #[test]
    fn test_bigg_layout() {
        #[repr(align(16))]
        struct Bigg {
            _data: [u8; 160]
        }

        assert!(size_of::<Bigg>() == 160);
        assert!(align_of::<Bigg>() == 16);
        let layout = explicit_layout::<Bigg>();
        assert_eq!(layout.total_alignment, 16);
        assert_eq!(layout.strong_offset, 0);
        assert_eq!(layout.weak_offset, 4);
        assert_eq!(layout.data_offset, 16);
        let layout = LayoutHolder::<Bigg>::get_layout_for_length(7);
        assert_eq!(layout.size(), 1136);
        assert_eq!(layout.align(), 16);

        let layout = LayoutHolder::<Bigg>::get_layout_for_length(0);
        assert_eq!(layout.size(), 16);
        assert_eq!(layout.align(), 16);
    }

    #[test]
    fn test_biggger_layout() {
        #[repr(align(64))]
        struct Biggger {
            _data: [u8; 640]
        }

        assert!(size_of::<Biggger>() == 640);
        assert!(align_of::<Biggger>() == 64);
        let layout = explicit_layout::<Biggger>();
        assert_eq!(layout.total_alignment, 64);
        assert_eq!(layout.strong_offset, 0);
        assert_eq!(layout.weak_offset, 4);
        assert_eq!(layout.data_offset, 64);
        let layout = LayoutHolder::<Biggger>::get_layout_for_length(11);
        assert_eq!(layout.size(), 7104);
        assert_eq!(layout.align(), 64);

        let layout = LayoutHolder::<Biggger>::get_layout_for_length(0);
        assert_eq!(layout.size(), 64);
        assert_eq!(layout.align(), 64);
    }

    #[test]
    fn test_zipp_layout() {
        #[repr(align(1))]
        struct Zipp {
            _data: [u8; 331]
        }

        assert!(size_of::<Zipp>() == 331);
        assert!(align_of::<Zipp>() == 1);
        let layout = explicit_layout::<Zipp>();
        assert_eq!(layout.total_alignment, 4);
        assert_eq!(layout.strong_offset, 0);
        assert_eq!(layout.weak_offset, 4);
        assert_eq!(layout.data_offset, 8);

        let layout = LayoutHolder::<Zipp>::get_layout_for_length(11);
        assert_eq!(layout.size(), 3652);
        assert_eq!(layout.align(), 4);

        let layout = LayoutHolder::<Zipp>::get_layout_for_length(0);
        assert_eq!(layout.size(), 8);
        assert_eq!(layout.align(), 4);
    }
}
