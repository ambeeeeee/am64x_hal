#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dat_reg3_hpb_data_mem: DatReg3HpbDataMem,
}
impl RegisterBlock {
    #[doc = "0x00 - FSAS bypass data region3"]
    #[inline(always)]
    pub const fn dat_reg3_hpb_data_mem(&self) -> &DatReg3HpbDataMem {
        &self.dat_reg3_hpb_data_mem
    }
}
#[doc = "DAT_REG3_hpb_data_mem (rw) register accessor: FSAS bypass data region3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_reg3_hpb_data_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat_reg3_hpb_data_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat_reg3_hpb_data_mem`]
module"]
#[doc(alias = "DAT_REG3_hpb_data_mem")]
pub type DatReg3HpbDataMem = crate::Reg<dat_reg3_hpb_data_mem::DatReg3HpbDataMemSpec>;
#[doc = "FSAS bypass data region3"]
pub mod dat_reg3_hpb_data_mem;
