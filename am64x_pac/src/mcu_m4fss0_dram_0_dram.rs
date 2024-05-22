#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idram__slv__ram_ram_reg: Idram_Slv_RamRamReg,
}
impl RegisterBlock {
    #[doc = "0x00 - The RAM memory words provide memory mapped random access data storage."]
    #[inline(always)]
    pub const fn idram__slv__ram_ram_reg(&self) -> &Idram_Slv_RamRamReg {
        &self.idram__slv__ram_ram_reg
    }
}
#[doc = "IDRAM__SLV__RAM_RAM_REG (rw) register accessor: The RAM memory words provide memory mapped random access data storage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idram__slv__ram_ram_reg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idram__slv__ram_ram_reg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idram__slv__ram_ram_reg`]
module"]
#[doc(alias = "IDRAM__SLV__RAM_RAM_REG")]
pub type Idram_Slv_RamRamReg = crate::Reg<idram__slv__ram_ram_reg::Idram_Slv_RamRamRegSpec>;
#[doc = "The RAM memory words provide memory mapped random access data storage."]
pub mod idram__slv__ram_ram_reg;
