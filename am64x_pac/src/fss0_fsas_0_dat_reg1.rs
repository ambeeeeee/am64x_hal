#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dat_reg1_hpb_data_mem: DatReg1HpbDataMem,
}
impl RegisterBlock {
    #[doc = "0x00 - FSAS boot data region1"]
    #[inline(always)]
    pub const fn dat_reg1_hpb_data_mem(&self) -> &DatReg1HpbDataMem {
        &self.dat_reg1_hpb_data_mem
    }
}
#[doc = "DAT_REG1_hpb_data_mem (rw) register accessor: FSAS boot data region1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dat_reg1_hpb_data_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dat_reg1_hpb_data_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dat_reg1_hpb_data_mem`]
module"]
#[doc(alias = "DAT_REG1_hpb_data_mem")]
pub type DatReg1HpbDataMem = crate::Reg<dat_reg1_hpb_data_mem::DatReg1HpbDataMemSpec>;
#[doc = "FSAS boot data region1"]
pub mod dat_reg1_hpb_data_mem;
