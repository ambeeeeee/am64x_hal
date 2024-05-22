#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ram__slv__ram_ram_reg: Ram_Slv_RamRamReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The RAM memory words provide memory mapped random access data storage."]
    #[inline(always)]
    pub const fn ram__slv__ram_ram_reg(&self) -> &Ram_Slv_RamRamReg {
        &self.ram__slv__ram_ram_reg
    }
}
#[doc = "RAM__SLV__RAM_RAM_REG (rw) register accessor: The RAM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ram__slv__ram_ram_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ram__slv__ram_ram_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram__slv__ram_ram_reg`]
module"]
#[doc(alias = "RAM__SLV__RAM_RAM_REG")]
pub type Ram_Slv_RamRamReg = crate::Reg<ram__slv__ram_ram_reg::Ram_Slv_RamRamRegSpec>;
#[doc = "The RAM memory words provide memory mapped random access data storage."]
pub mod ram__slv__ram_ram_reg;
