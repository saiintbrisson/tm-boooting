//! Definition of the Global Descriptor Table.
//!
//! See: https://wiki.osdev.org/Global_Descriptor_Table.
#![allow(dead_code)]

use core::{
    fmt::Debug,
    mem::{size_of, size_of_val_raw},
    ptr::addr_of,
    slice::from_raw_parts,
};

#[no_mangle]
pub static GDT: [GdtEntry; 3] = [GdtEntry::null(), CODE_SEGMENT_ENTRY, DATA_SEGMENT_ENTRY];
#[no_mangle]
pub static GDTR: GdtDescriptor = unsafe { GdtDescriptor::new(&GDT) };
#[no_mangle]
pub static RUST_CODE_SEG_OFFSET: u16 =
    unsafe { addr_of!(GDT[1]).byte_offset_from(GDT.as_ptr()) as u16 };

static CODE_SEGMENT_ENTRY: GdtEntry = GdtEntry::new(
    0,
    0xFFFFF,
    AccessByte::P
        .union(AccessByte::S)
        .union(AccessByte::E)
        .union(AccessByte::RW),
    Flags::G.union(Flags::DB),
);
static DATA_SEGMENT_ENTRY: GdtEntry = GdtEntry::new(
    0,
    0xFFFFF,
    AccessByte::P.union(AccessByte::S).union(AccessByte::RW),
    Flags::G.union(Flags::DB),
);

// db 0b10011010           ; Access byte flags
bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct AccessByte: u8 {
        /// Present bit. Must be 1 if the segment is valid.
        const P = 0b10000000;
        /// Descriptor privilege level field.
        /// See: https://wiki.osdev.org/Security#Rings.
        const DPL = 0b01100000;
        /// Descriptor type bit.
        /// Clear if this is a system segment.
        /// Set if it defines a code or data segment.
        const S = 0b00010000;
        /// Executable bit.
        /// Clear if this is a data segment.
        /// Set if it is a code segment.
        const E = 0b00001000;
        /// Direction/conforming bit.
        /// For data segments: direction. Clear if segment grows up.
        /// For code segments: conforming. Set if segment can be executed from an equal or
        /// lower privilege level.
        const DC = 0b00000100;
        /// Readable/writable bit.
        /// For data segments: writable. Clear if write access is not allowed.
        /// For code segments: readable. Clear if read access is not allowed.
        const RW = 0b00000010;
        /// Accessed bit. 0 if the segment hasn't been accessed yet.
        const A = 0b00000001;
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Flags: u8 {
        /// Granularity bit. Indicates how the Limit value scales.
        /// 0 if segment scales in 1 byte blocks.
        /// 1 if segment scales by 4 KiB blocks.
        const G = 0b1000;
        /// Size flag.
        /// Clear if the descriptor defines a 16 bit segment.
        /// Set if the descriptor defines a 32 bit segment.
        const DB = 0b0100;
        /// Long-mode code flag.
        /// Set if the descriptor defines a 64 bit code segment.
        const L = 0b0010;
    }
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct GdtDescriptor {
    pub size: u16,
    pub gdt: *const GdtEntry,
}

impl GdtDescriptor {
    pub const unsafe fn new(gdt: &[GdtEntry]) -> Self {
        Self {
            size: size_of_val_raw(gdt) as u16 - 1,
            gdt: gdt.as_ptr(),
        }
    }

    pub unsafe fn entries(&self) -> &[GdtEntry] {
        let length = (self.size as usize + 1) / size_of::<GdtEntry>();
        from_raw_parts(self.gdt, length)
    }
}

unsafe impl Sync for GdtDescriptor {}
unsafe impl Send for GdtDescriptor {}

#[repr(packed)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GdtEntry {
    limit1: u16,
    base1: u16,
    base2: u8,
    access_byte: AccessByte,
    flags: FlagsAndLimit,
    base3: u8,
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access_byte: AccessByte, flags: Flags) -> Self {
        if limit & 0xFFF00000 != 0 {
            panic!("limit must be at max 20 bits wide");
        }

        Self {
            limit1: limit as u16,
            base1: base as u16,
            base2: (base >> 16) as u8,
            access_byte,
            flags: FlagsAndLimit::new((limit >> 16) as u8, flags),
            base3: (base >> 24) as u8,
        }
    }

    pub const fn null() -> Self {
        GdtEntry::new(0, 0, AccessByte::empty(), Flags::empty())
    }

    pub const fn base(&self) -> u32 {
        self.base1 as u32 | (self.base2 as u32) << 16 | (self.base3 as u32) << 24
    }

    pub const fn limit(&self) -> u32 {
        self.limit1 as u32 | (self.flags.limit() as u32) << 16
    }

    pub const fn access_byte(&self) -> AccessByte {
        self.access_byte
    }

    pub const fn flags(&self) -> Flags {
        self.flags.flags()
    }
}

#[repr(packed)]
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct FlagsAndLimit(u8);

impl FlagsAndLimit {
    pub const fn new(limit: u8, flags: Flags) -> Self {
        Self(flags.bits() << 4 | limit & 0x0F)
    }

    const fn limit(&self) -> u8 {
        self.0 & 0x0F
    }

    const fn flags(&self) -> Flags {
        Flags::from_bits_retain(self.0 >> 4)
    }
}

impl Debug for FlagsAndLimit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GdtEntryFlagsAndLimit")
            .field("flags", &self.flags())
            .field("limit", &self.limit())
            .finish()
    }
}
