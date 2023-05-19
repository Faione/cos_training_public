#![no_std]
#![feature(asm_const)]

use riscv::register::satp;

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

const PHYS_VIRT_OFFSET: usize = 0xffff_ffc0_0000_0000;

#[cfg(feature = "sv39")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV39: [u64; 512] = [0; 512];

#[cfg(feature = "sv39")]
pub unsafe fn pre_mmu() {
    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[2] = (0x80000 << 10) | 0xef;

    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x102] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    BOOT_PT_SV39[0x1ff] = (0x80000 << 10) | 0xef;
}

#[cfg(feature = "sv39")]
pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV39.as_ptr() as usize;
    satp::set(satp::Mode::Sv39, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

#[cfg(feature = "sv39")]
pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}

#[cfg(feature = "sv48")]
#[link_section = ".data.boot_page_table"]
static mut BOOT_PT_SV48: [[u64; 512]; 3] = [[0; 512]; 3];

#[cfg(feature = "sv48")]
pub unsafe fn pre_mmu() {
    // BOOT_PT_SV48 4K 对齐, 因而BOOT_PT_SV48[0/1/2] 也都是4k对齐的
    // 注意需将 RWX 置0, V 置 1 使得页表生效
    BOOT_PT_SV48[0][0] = ((BOOT_PT_SV48[1].as_ptr() as u64) >> 2) | 0x1;
    BOOT_PT_SV48[0][0x1ff] = ((BOOT_PT_SV48[2].as_ptr() as u64) >> 2) | 0x1;

    // 0x8000_0000..0xc000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV48[1][2] = (0x80000 << 10) | 0xef;

    // 0xffff_ffc0_8000_0000..0xffff_ffc0_c000_0000, VRWX_GAD, 1G block
    BOOT_PT_SV48[2][0x102] = (0x80000 << 10) | 0xef;

    // 0xffff_ffff_c000_0000..highest, VRWX_GAD, 1G block
    BOOT_PT_SV48[2][0x1ff] = (0x80000 << 10) | 0xef;
}

#[cfg(feature = "sv48")]
pub unsafe fn enable_mmu() {
    let page_table_root = BOOT_PT_SV48.as_ptr() as usize;
    satp::set(satp::Mode::Sv48, 0, page_table_root >> 12);
    riscv::asm::sfence_vma_all();
}

#[cfg(feature = "sv48")]
pub unsafe fn post_mmu() {
    core::arch::asm!("
        li      t0, {phys_virt_offset}  // fix up virtual high address
        add     sp, sp, t0
        add     ra, ra, t0
        ret     ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
    )
}
