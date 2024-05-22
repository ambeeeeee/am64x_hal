#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_dccgctrl: CfgDccgctrl,
    cfg_dccrev: CfgDccrev,
    cfg_dcccntseed0: CfgDcccntseed0,
    cfg_dccvalidseed0: CfgDccvalidseed0,
    cfg_dcccntseed1: CfgDcccntseed1,
    cfg_dccstat: CfgDccstat,
    cfg_dcccnt0: CfgDcccnt0,
    cfg_dccvalid0: CfgDccvalid0,
    cfg_dcccnt1: CfgDcccnt1,
    cfg_dccclksrc1: CfgDccclksrc1,
    cfg_dccclksrc0: CfgDccclksrc0,
    cfg_dccgctrl2: CfgDccgctrl2,
    cfg_dccstatus2: CfgDccstatus2,
    cfg_dccerrcnt: CfgDccerrcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Starts / stops the counters. Clears the error signal."]
    #[inline(always)]
    pub const fn cfg_dccgctrl(&self) -> &CfgDccgctrl {
        &self.cfg_dccgctrl
    }
    #[doc = "0x04 - Specifies the module version."]
    #[inline(always)]
    pub const fn cfg_dccrev(&self) -> &CfgDccrev {
        &self.cfg_dccrev
    }
    #[doc = "0x08 - Seed value for the counter attached to clock source 0"]
    #[inline(always)]
    pub const fn cfg_dcccntseed0(&self) -> &CfgDcccntseed0 {
        &self.cfg_dcccntseed0
    }
    #[doc = "0x0c - Seed value for the timeout counter attached to clock source 0."]
    #[inline(always)]
    pub const fn cfg_dccvalidseed0(&self) -> &CfgDccvalidseed0 {
        &self.cfg_dccvalidseed0
    }
    #[doc = "0x10 - Seed value for the counter attached to clock source 1."]
    #[inline(always)]
    pub const fn cfg_dcccntseed1(&self) -> &CfgDcccntseed1 {
        &self.cfg_dcccntseed1
    }
    #[doc = "0x14 - Specifies the status of the DCC Module."]
    #[inline(always)]
    pub const fn cfg_dccstat(&self) -> &CfgDccstat {
        &self.cfg_dccstat
    }
    #[doc = "0x18 - Value of the counter attached to clock source 0."]
    #[inline(always)]
    pub const fn cfg_dcccnt0(&self) -> &CfgDcccnt0 {
        &self.cfg_dcccnt0
    }
    #[doc = "0x1c - Value of the valid counter attached to clock source 0."]
    #[inline(always)]
    pub const fn cfg_dccvalid0(&self) -> &CfgDccvalid0 {
        &self.cfg_dccvalid0
    }
    #[doc = "0x20 - Value of the counter attached to clock source 1."]
    #[inline(always)]
    pub const fn cfg_dcccnt1(&self) -> &CfgDcccnt1 {
        &self.cfg_dcccnt1
    }
    #[doc = "0x24 - Selects the clock source for counter 1."]
    #[inline(always)]
    pub const fn cfg_dccclksrc1(&self) -> &CfgDccclksrc1 {
        &self.cfg_dccclksrc1
    }
    #[doc = "0x28 - Selects the clock source for counter 0."]
    #[inline(always)]
    pub const fn cfg_dccclksrc0(&self) -> &CfgDccclksrc0 {
        &self.cfg_dccclksrc0
    }
    #[doc = "0x2c - Allows configuring different modes of operation for DCC."]
    #[inline(always)]
    pub const fn cfg_dccgctrl2(&self) -> &CfgDccgctrl2 {
        &self.cfg_dccgctrl2
    }
    #[doc = "0x30 - Specifies the status of the DCC FIFOs."]
    #[inline(always)]
    pub const fn cfg_dccstatus2(&self) -> &CfgDccstatus2 {
        &self.cfg_dccstatus2
    }
    #[doc = "0x34 - Counts number of errors since last clear."]
    #[inline(always)]
    pub const fn cfg_dccerrcnt(&self) -> &CfgDccerrcnt {
        &self.cfg_dccerrcnt
    }
}
#[doc = "CFG_DCCGCTRL (rw) register accessor: Starts / stops the counters. Clears the error signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccgctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccgctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccgctrl`]
module"]
#[doc(alias = "CFG_DCCGCTRL")]
pub type CfgDccgctrl = crate::Reg<cfg_dccgctrl::CfgDccgctrlSpec>;
#[doc = "Starts / stops the counters. Clears the error signal."]
pub mod cfg_dccgctrl;
#[doc = "CFG_DCCREV (rw) register accessor: Specifies the module version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccrev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccrev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccrev`]
module"]
#[doc(alias = "CFG_DCCREV")]
pub type CfgDccrev = crate::Reg<cfg_dccrev::CfgDccrevSpec>;
#[doc = "Specifies the module version."]
pub mod cfg_dccrev;
#[doc = "CFG_DCCCNTSEED0 (rw) register accessor: Seed value for the counter attached to clock source 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccntseed0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccntseed0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dcccntseed0`]
module"]
#[doc(alias = "CFG_DCCCNTSEED0")]
pub type CfgDcccntseed0 = crate::Reg<cfg_dcccntseed0::CfgDcccntseed0Spec>;
#[doc = "Seed value for the counter attached to clock source 0"]
pub mod cfg_dcccntseed0;
#[doc = "CFG_DCCVALIDSEED0 (rw) register accessor: Seed value for the timeout counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccvalidseed0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccvalidseed0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccvalidseed0`]
module"]
#[doc(alias = "CFG_DCCVALIDSEED0")]
pub type CfgDccvalidseed0 = crate::Reg<cfg_dccvalidseed0::CfgDccvalidseed0Spec>;
#[doc = "Seed value for the timeout counter attached to clock source 0."]
pub mod cfg_dccvalidseed0;
#[doc = "CFG_DCCCNTSEED1 (rw) register accessor: Seed value for the counter attached to clock source 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccntseed1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccntseed1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dcccntseed1`]
module"]
#[doc(alias = "CFG_DCCCNTSEED1")]
pub type CfgDcccntseed1 = crate::Reg<cfg_dcccntseed1::CfgDcccntseed1Spec>;
#[doc = "Seed value for the counter attached to clock source 1."]
pub mod cfg_dcccntseed1;
#[doc = "CFG_DCCSTAT (rw) register accessor: Specifies the status of the DCC Module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccstat`]
module"]
#[doc(alias = "CFG_DCCSTAT")]
pub type CfgDccstat = crate::Reg<cfg_dccstat::CfgDccstatSpec>;
#[doc = "Specifies the status of the DCC Module."]
pub mod cfg_dccstat;
#[doc = "CFG_DCCCNT0 (rw) register accessor: Value of the counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccnt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccnt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dcccnt0`]
module"]
#[doc(alias = "CFG_DCCCNT0")]
pub type CfgDcccnt0 = crate::Reg<cfg_dcccnt0::CfgDcccnt0Spec>;
#[doc = "Value of the counter attached to clock source 0."]
pub mod cfg_dcccnt0;
#[doc = "CFG_DCCVALID0 (rw) register accessor: Value of the valid counter attached to clock source 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccvalid0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccvalid0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccvalid0`]
module"]
#[doc(alias = "CFG_DCCVALID0")]
pub type CfgDccvalid0 = crate::Reg<cfg_dccvalid0::CfgDccvalid0Spec>;
#[doc = "Value of the valid counter attached to clock source 0."]
pub mod cfg_dccvalid0;
#[doc = "CFG_DCCCNT1 (rw) register accessor: Value of the counter attached to clock source 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dcccnt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dcccnt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dcccnt1`]
module"]
#[doc(alias = "CFG_DCCCNT1")]
pub type CfgDcccnt1 = crate::Reg<cfg_dcccnt1::CfgDcccnt1Spec>;
#[doc = "Value of the counter attached to clock source 1."]
pub mod cfg_dcccnt1;
#[doc = "CFG_DCCCLKSRC1 (rw) register accessor: Selects the clock source for counter 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccclksrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccclksrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccclksrc1`]
module"]
#[doc(alias = "CFG_DCCCLKSRC1")]
pub type CfgDccclksrc1 = crate::Reg<cfg_dccclksrc1::CfgDccclksrc1Spec>;
#[doc = "Selects the clock source for counter 1."]
pub mod cfg_dccclksrc1;
#[doc = "CFG_DCCCLKSRC0 (rw) register accessor: Selects the clock source for counter 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccclksrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccclksrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccclksrc0`]
module"]
#[doc(alias = "CFG_DCCCLKSRC0")]
pub type CfgDccclksrc0 = crate::Reg<cfg_dccclksrc0::CfgDccclksrc0Spec>;
#[doc = "Selects the clock source for counter 0."]
pub mod cfg_dccclksrc0;
#[doc = "CFG_DCCGCTRL2 (rw) register accessor: Allows configuring different modes of operation for DCC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccgctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccgctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccgctrl2`]
module"]
#[doc(alias = "CFG_DCCGCTRL2")]
pub type CfgDccgctrl2 = crate::Reg<cfg_dccgctrl2::CfgDccgctrl2Spec>;
#[doc = "Allows configuring different modes of operation for DCC."]
pub mod cfg_dccgctrl2;
#[doc = "CFG_DCCSTATUS2 (rw) register accessor: Specifies the status of the DCC FIFOs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccstatus2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccstatus2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccstatus2`]
module"]
#[doc(alias = "CFG_DCCSTATUS2")]
pub type CfgDccstatus2 = crate::Reg<cfg_dccstatus2::CfgDccstatus2Spec>;
#[doc = "Specifies the status of the DCC FIFOs."]
pub mod cfg_dccstatus2;
#[doc = "CFG_DCCERRCNT (rw) register accessor: Counts number of errors since last clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccerrcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccerrcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dccerrcnt`]
module"]
#[doc(alias = "CFG_DCCERRCNT")]
pub type CfgDccerrcnt = crate::Reg<cfg_dccerrcnt::CfgDccerrcntSpec>;
#[doc = "Counts number of errors since last clear."]
pub mod cfg_dccerrcnt;
