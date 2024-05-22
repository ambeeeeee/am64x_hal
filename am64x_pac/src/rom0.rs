#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rom_rom_reg: RomRomReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The ROM memory words provide memory mapped random access data storage."]
    #[inline(always)]
    pub const fn rom_rom_reg(&self) -> &RomRomReg {
        &self.rom_rom_reg
    }
}
#[doc = "ROM_ROM_REG (rw) register accessor: The ROM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_rom_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_rom_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_rom_reg`]
module"]
#[doc(alias = "ROM_ROM_REG")]
pub type RomRomReg = crate::Reg<rom_rom_reg::RomRomRegSpec>;
#[doc = "The ROM memory words provide memory mapped random access data storage."]
pub mod rom_rom_reg;
