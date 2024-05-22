#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    regs_splock_id: RegsSplockId,
    _reserved1: [u8; 0x0c],
    regs_splock_sysconfig: RegsSplockSysconfig,
    regs_splock_systatus: RegsSplockSystatus,
    _reserved3: [u8; 0x07e8],
    regs_lock: RegsLock,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
    #[inline(always)]
    pub const fn regs_splock_id(&self) -> &RegsSplockId {
        &self.regs_splock_id
    }
    #[doc = "0x10 - Provides the SOFTRESET register for backwards compatibility with OMAP Spinlock"]
    #[inline(always)]
    pub const fn regs_splock_sysconfig(&self) -> &RegsSplockSysconfig {
        &self.regs_splock_sysconfig
    }
    #[doc = "0x14 - Provides information about the Spinlock module"]
    #[inline(always)]
    pub const fn regs_splock_systatus(&self) -> &RegsSplockSystatus {
        &self.regs_splock_systatus
    }
    #[doc = "0x800 - The Lock\\[a\\]
register is read and written to perform lock and unlock operations on lock 'a'"]
    #[inline(always)]
    pub const fn regs_lock(&self) -> &RegsLock {
        &self.regs_lock
    }
}
#[doc = "REGS_SPLOCK_ID (rw) register accessor: This is the standard TI peripheral ID register that exists at address 0 in the peripheral space\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_splock_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_splock_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_splock_id`]
module"]
#[doc(alias = "REGS_SPLOCK_ID")]
pub type RegsSplockId = crate::Reg<regs_splock_id::RegsSplockIdSpec>;
#[doc = "This is the standard TI peripheral ID register that exists at address 0 in the peripheral space"]
pub mod regs_splock_id;
#[doc = "REGS_SPLOCK_SYSCONFIG (rw) register accessor: Provides the SOFTRESET register for backwards compatibility with OMAP Spinlock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_splock_sysconfig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_splock_sysconfig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_splock_sysconfig`]
module"]
#[doc(alias = "REGS_SPLOCK_SYSCONFIG")]
pub type RegsSplockSysconfig = crate::Reg<regs_splock_sysconfig::RegsSplockSysconfigSpec>;
#[doc = "Provides the SOFTRESET register for backwards compatibility with OMAP Spinlock"]
pub mod regs_splock_sysconfig;
#[doc = "REGS_SPLOCK_SYSTATUS (rw) register accessor: Provides information about the Spinlock module\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_splock_systatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_splock_systatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_splock_systatus`]
module"]
#[doc(alias = "REGS_SPLOCK_SYSTATUS")]
pub type RegsSplockSystatus = crate::Reg<regs_splock_systatus::RegsSplockSystatusSpec>;
#[doc = "Provides information about the Spinlock module"]
pub mod regs_splock_systatus;
#[doc = "REGS_LOCK (rw) register accessor: The Lock\\[a\\]
register is read and written to perform lock and unlock operations on lock 'a'\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regs_lock`]
module"]
#[doc(alias = "REGS_LOCK")]
pub type RegsLock = crate::Reg<regs_lock::RegsLockSpec>;
#[doc = "The Lock\\[a\\]
register is read and written to perform lock and unlock operations on lock 'a'"]
pub mod regs_lock;
