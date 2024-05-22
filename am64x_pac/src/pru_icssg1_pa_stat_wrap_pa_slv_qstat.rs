#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_stat_wrap__pa_slv__qstat_qram: PaStatWrap_PaSlv_QstatQram,
}
impl RegisterBlock {
    #[doc = "0x00 - query mode RAM"]
    #[inline(always)]
    pub const fn pa_stat_wrap__pa_slv__qstat_qram(&self) -> &PaStatWrap_PaSlv_QstatQram {
        &self.pa_stat_wrap__pa_slv__qstat_qram
    }
}
#[doc = "PA_STAT_WRAP__PA_SLV__QSTAT_QRAM (rw) register accessor: query mode RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_stat_wrap__pa_slv__qstat_qram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_stat_wrap__pa_slv__qstat_qram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_stat_wrap__pa_slv__qstat_qram`]
module"]
#[doc(alias = "PA_STAT_WRAP__PA_SLV__QSTAT_QRAM")]
pub type PaStatWrap_PaSlv_QstatQram =
    crate::Reg<pa_stat_wrap__pa_slv__qstat_qram::PaStatWrap_PaSlv_QstatQramSpec>;
#[doc = "query mode RAM"]
pub mod pa_stat_wrap__pa_slv__qstat_qram;
