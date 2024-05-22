#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    blazar_iram_ram_reg: BlazarIramRamReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The RAM memory words provide memory mapped random access data storage."]
    #[inline(always)]
    pub const fn blazar_iram_ram_reg(&self) -> &BlazarIramRamReg {
        &self.blazar_iram_ram_reg
    }
}
#[doc = "BLAZAR_IRAM_RAM_REG (rw) register accessor: The RAM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blazar_iram_ram_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blazar_iram_ram_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blazar_iram_ram_reg`]
module"]
#[doc(alias = "BLAZAR_IRAM_RAM_REG")]
pub type BlazarIramRamReg = crate::Reg<blazar_iram_ram_reg::BlazarIramRamRegSpec>;
#[doc = "The RAM memory words provide memory mapped random access data storage."]
pub mod blazar_iram_ram_reg;
