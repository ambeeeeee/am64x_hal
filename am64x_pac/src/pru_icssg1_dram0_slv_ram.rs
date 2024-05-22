#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dram0__slv__ram_ram_reg: Dram0_Slv_RamRamReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The RAM memory words provide memory mapped random access data storage."]
    #[inline(always)]
    pub const fn dram0__slv__ram_ram_reg(&self) -> &Dram0_Slv_RamRamReg {
        &self.dram0__slv__ram_ram_reg
    }
}
#[doc = "DRAM0__SLV__RAM_RAM_REG (rw) register accessor: The RAM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dram0__slv__ram_ram_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dram0__slv__ram_ram_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dram0__slv__ram_ram_reg`]
module"]
#[doc(alias = "DRAM0__SLV__RAM_RAM_REG")]
pub type Dram0_Slv_RamRamReg = crate::Reg<dram0__slv__ram_ram_reg::Dram0_Slv_RamRamRegSpec>;
#[doc = "The RAM memory words provide memory mapped random access data storage."]
pub mod dram0__slv__ram_ram_reg;
