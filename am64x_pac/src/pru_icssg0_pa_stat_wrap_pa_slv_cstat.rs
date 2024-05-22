#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pa_stat_wrap__pa_slv__cstat_cram: PaStatWrap_PaSlv_CstatCram,
}
impl RegisterBlock {
    #[doc = "0x00 - query mode RAM"]
    #[inline(always)]
    pub const fn pa_stat_wrap__pa_slv__cstat_cram(&self) -> &PaStatWrap_PaSlv_CstatCram {
        &self.pa_stat_wrap__pa_slv__cstat_cram
    }
}
#[doc = "PA_STAT_WRAP__PA_SLV__CSTAT_CRAM (rw) register accessor: query mode RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_stat_wrap__pa_slv__cstat_cram::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_stat_wrap__pa_slv__cstat_cram::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pa_stat_wrap__pa_slv__cstat_cram`]
module"]
#[doc(alias = "PA_STAT_WRAP__PA_SLV__CSTAT_CRAM")]
pub type PaStatWrap_PaSlv_CstatCram =
    crate::Reg<pa_stat_wrap__pa_slv__cstat_cram::PaStatWrap_PaSlv_CstatCramSpec>;
#[doc = "query mode RAM"]
pub mod pa_stat_wrap__pa_slv__cstat_cram;
