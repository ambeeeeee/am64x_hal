#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intr_router_cfg_pid: IntrRouterCfgPid,
    intr_router_cfg_intr_muxcntl: IntrRouterCfgIntrMuxcntl,
}
impl RegisterBlock {
    #[doc = "0x00 - Identification register"]
    #[inline(always)]
    pub const fn intr_router_cfg_pid(&self) -> &IntrRouterCfgPid {
        &self.intr_router_cfg_pid
    }
    #[doc = "0x04 - Interrupt mux control register"]
    #[inline(always)]
    pub const fn intr_router_cfg_intr_muxcntl(&self) -> &IntrRouterCfgIntrMuxcntl {
        &self.intr_router_cfg_intr_muxcntl
    }
}
#[doc = "INTR_ROUTER_CFG_PID (rw) register accessor: Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_router_cfg_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_router_cfg_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_router_cfg_pid`]
module"]
#[doc(alias = "INTR_ROUTER_CFG_PID")]
pub type IntrRouterCfgPid = crate::Reg<intr_router_cfg_pid::IntrRouterCfgPidSpec>;
#[doc = "Identification register"]
pub mod intr_router_cfg_pid;
#[doc = "INTR_ROUTER_CFG_INTR_MUXCNTL (rw) register accessor: Interrupt mux control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_router_cfg_intr_muxcntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_router_cfg_intr_muxcntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_router_cfg_intr_muxcntl`]
module"]
#[doc(alias = "INTR_ROUTER_CFG_INTR_MUXCNTL")]
pub type IntrRouterCfgIntrMuxcntl =
    crate::Reg<intr_router_cfg_intr_muxcntl::IntrRouterCfgIntrMuxcntlSpec>;
#[doc = "Interrupt mux control register"]
pub mod intr_router_cfg_intr_muxcntl;
