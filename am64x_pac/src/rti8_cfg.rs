#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg_gctrl: CfgGctrl,
    cfg_tbctrl: CfgTbctrl,
    cfg_capctrl: CfgCapctrl,
    cfg_compctrl: CfgCompctrl,
    cfg_frc0: CfgFrc0,
    cfg_uc0: CfgUc0,
    cfg_cpuc0: CfgCpuc0,
    _reserved7: [u8; 0x04],
    cfg_cafrc0: CfgCafrc0,
    cfg_cauc0: CfgCauc0,
    _reserved9: [u8; 0x08],
    cfg_frc1: CfgFrc1,
    cfg_uc1: CfgUc1,
    cfg_cpuc1: CfgCpuc1,
    _reserved12: [u8; 0x04],
    cfg_cafrc1: CfgCafrc1,
    cfg_cauc1: CfgCauc1,
    _reserved14: [u8; 0x08],
    cfg_comp0: CfgComp0,
    cfg_udcp0: CfgUdcp0,
    cfg_comp1: CfgComp1,
    cfg_udcp1: CfgUdcp1,
    cfg_comp2: CfgComp2,
    cfg_udcp2: CfgUdcp2,
    cfg_comp3: CfgComp3,
    cfg_udcp3: CfgUdcp3,
    cfg_tblcomp: CfgTblcomp,
    cfg_tbhcomp: CfgTbhcomp,
    _reserved24: [u8; 0x08],
    cfg_setint: CfgSetint,
    cfg_clearint: CfgClearint,
    cfg_intflag: CfgIntflag,
    _reserved27: [u8; 0x04],
    cfg_dwdctrl: CfgDwdctrl,
    cfg_dwdprld: CfgDwdprld,
    cfg_wdstatus: CfgWdstatus,
    cfg_wdkey: CfgWdkey,
    cfg_dwdcntr: CfgDwdcntr,
    cfg_wwdrxnctrl: CfgWwdrxnctrl,
    cfg_wwdsizectrl: CfgWwdsizectrl,
    cfg_intclrenable: CfgIntclrenable,
    cfg_comp0clr: CfgComp0clr,
    cfg_comp1clr: CfgComp1clr,
    cfg_comp2clr: CfgComp2clr,
    cfg_comp3clr: CfgComp3clr,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG_GCTRL"]
    #[inline(always)]
    pub const fn cfg_gctrl(&self) -> &CfgGctrl {
        &self.cfg_gctrl
    }
    #[doc = "0x04 - CFG_TBCTRL"]
    #[inline(always)]
    pub const fn cfg_tbctrl(&self) -> &CfgTbctrl {
        &self.cfg_tbctrl
    }
    #[doc = "0x08 - CFG_CAPCTRL"]
    #[inline(always)]
    pub const fn cfg_capctrl(&self) -> &CfgCapctrl {
        &self.cfg_capctrl
    }
    #[doc = "0x0c - CFG_COMPCTRL"]
    #[inline(always)]
    pub const fn cfg_compctrl(&self) -> &CfgCompctrl {
        &self.cfg_compctrl
    }
    #[doc = "0x10 - CFG_FRC0"]
    #[inline(always)]
    pub const fn cfg_frc0(&self) -> &CfgFrc0 {
        &self.cfg_frc0
    }
    #[doc = "0x14 - CFG_UC0"]
    #[inline(always)]
    pub const fn cfg_uc0(&self) -> &CfgUc0 {
        &self.cfg_uc0
    }
    #[doc = "0x18 - CFG_CPUC0"]
    #[inline(always)]
    pub const fn cfg_cpuc0(&self) -> &CfgCpuc0 {
        &self.cfg_cpuc0
    }
    #[doc = "0x20 - CFG_CAFRC0"]
    #[inline(always)]
    pub const fn cfg_cafrc0(&self) -> &CfgCafrc0 {
        &self.cfg_cafrc0
    }
    #[doc = "0x24 - CFG_CAUC0"]
    #[inline(always)]
    pub const fn cfg_cauc0(&self) -> &CfgCauc0 {
        &self.cfg_cauc0
    }
    #[doc = "0x30 - CFG_FRC1"]
    #[inline(always)]
    pub const fn cfg_frc1(&self) -> &CfgFrc1 {
        &self.cfg_frc1
    }
    #[doc = "0x34 - CFG_UC1"]
    #[inline(always)]
    pub const fn cfg_uc1(&self) -> &CfgUc1 {
        &self.cfg_uc1
    }
    #[doc = "0x38 - CFG_CPUC1"]
    #[inline(always)]
    pub const fn cfg_cpuc1(&self) -> &CfgCpuc1 {
        &self.cfg_cpuc1
    }
    #[doc = "0x40 - CFG_CAFRC1"]
    #[inline(always)]
    pub const fn cfg_cafrc1(&self) -> &CfgCafrc1 {
        &self.cfg_cafrc1
    }
    #[doc = "0x44 - CFG_CAUC1"]
    #[inline(always)]
    pub const fn cfg_cauc1(&self) -> &CfgCauc1 {
        &self.cfg_cauc1
    }
    #[doc = "0x50 - CFG_COMP0"]
    #[inline(always)]
    pub const fn cfg_comp0(&self) -> &CfgComp0 {
        &self.cfg_comp0
    }
    #[doc = "0x54 - CFG_UDCP0"]
    #[inline(always)]
    pub const fn cfg_udcp0(&self) -> &CfgUdcp0 {
        &self.cfg_udcp0
    }
    #[doc = "0x58 - CFG_COMP1"]
    #[inline(always)]
    pub const fn cfg_comp1(&self) -> &CfgComp1 {
        &self.cfg_comp1
    }
    #[doc = "0x5c - CFG_UDCP1"]
    #[inline(always)]
    pub const fn cfg_udcp1(&self) -> &CfgUdcp1 {
        &self.cfg_udcp1
    }
    #[doc = "0x60 - CFG_COMP2"]
    #[inline(always)]
    pub const fn cfg_comp2(&self) -> &CfgComp2 {
        &self.cfg_comp2
    }
    #[doc = "0x64 - CFG_UDCP2"]
    #[inline(always)]
    pub const fn cfg_udcp2(&self) -> &CfgUdcp2 {
        &self.cfg_udcp2
    }
    #[doc = "0x68 - CFG_COMP3"]
    #[inline(always)]
    pub const fn cfg_comp3(&self) -> &CfgComp3 {
        &self.cfg_comp3
    }
    #[doc = "0x6c - CFG_UDCP3"]
    #[inline(always)]
    pub const fn cfg_udcp3(&self) -> &CfgUdcp3 {
        &self.cfg_udcp3
    }
    #[doc = "0x70 - CFG_TBLCOMP"]
    #[inline(always)]
    pub const fn cfg_tblcomp(&self) -> &CfgTblcomp {
        &self.cfg_tblcomp
    }
    #[doc = "0x74 - CFG_TBHCOMP"]
    #[inline(always)]
    pub const fn cfg_tbhcomp(&self) -> &CfgTbhcomp {
        &self.cfg_tbhcomp
    }
    #[doc = "0x80 - CFG_SETINT"]
    #[inline(always)]
    pub const fn cfg_setint(&self) -> &CfgSetint {
        &self.cfg_setint
    }
    #[doc = "0x84 - CFG_CLEARINT"]
    #[inline(always)]
    pub const fn cfg_clearint(&self) -> &CfgClearint {
        &self.cfg_clearint
    }
    #[doc = "0x88 - CFG_INTFLAG"]
    #[inline(always)]
    pub const fn cfg_intflag(&self) -> &CfgIntflag {
        &self.cfg_intflag
    }
    #[doc = "0x90 - CFG_DWDCTRL"]
    #[inline(always)]
    pub const fn cfg_dwdctrl(&self) -> &CfgDwdctrl {
        &self.cfg_dwdctrl
    }
    #[doc = "0x94 - CFG_DWDPRLD"]
    #[inline(always)]
    pub const fn cfg_dwdprld(&self) -> &CfgDwdprld {
        &self.cfg_dwdprld
    }
    #[doc = "0x98 - CFG_WDSTATUS"]
    #[inline(always)]
    pub const fn cfg_wdstatus(&self) -> &CfgWdstatus {
        &self.cfg_wdstatus
    }
    #[doc = "0x9c - CFG_WDKEY"]
    #[inline(always)]
    pub const fn cfg_wdkey(&self) -> &CfgWdkey {
        &self.cfg_wdkey
    }
    #[doc = "0xa0 - CFG_DWDCNTR"]
    #[inline(always)]
    pub const fn cfg_dwdcntr(&self) -> &CfgDwdcntr {
        &self.cfg_dwdcntr
    }
    #[doc = "0xa4 - CFG_WWDRXNCTRL"]
    #[inline(always)]
    pub const fn cfg_wwdrxnctrl(&self) -> &CfgWwdrxnctrl {
        &self.cfg_wwdrxnctrl
    }
    #[doc = "0xa8 - CFG_WWDSIZECTRL"]
    #[inline(always)]
    pub const fn cfg_wwdsizectrl(&self) -> &CfgWwdsizectrl {
        &self.cfg_wwdsizectrl
    }
    #[doc = "0xac - CFG_INTCLRENABLE"]
    #[inline(always)]
    pub const fn cfg_intclrenable(&self) -> &CfgIntclrenable {
        &self.cfg_intclrenable
    }
    #[doc = "0xb0 - CFG_COMP0CLR"]
    #[inline(always)]
    pub const fn cfg_comp0clr(&self) -> &CfgComp0clr {
        &self.cfg_comp0clr
    }
    #[doc = "0xb4 - CFG_COMP1CLR"]
    #[inline(always)]
    pub const fn cfg_comp1clr(&self) -> &CfgComp1clr {
        &self.cfg_comp1clr
    }
    #[doc = "0xb8 - CFG_COMP2CLR"]
    #[inline(always)]
    pub const fn cfg_comp2clr(&self) -> &CfgComp2clr {
        &self.cfg_comp2clr
    }
    #[doc = "0xbc - CFG_COMP3CLR"]
    #[inline(always)]
    pub const fn cfg_comp3clr(&self) -> &CfgComp3clr {
        &self.cfg_comp3clr
    }
}
#[doc = "CFG_GCTRL (rw) register accessor: CFG_GCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_gctrl`]
module"]
#[doc(alias = "CFG_GCTRL")]
pub type CfgGctrl = crate::Reg<cfg_gctrl::CfgGctrlSpec>;
#[doc = "CFG_GCTRL"]
pub mod cfg_gctrl;
#[doc = "CFG_TBCTRL (rw) register accessor: CFG_TBCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tbctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tbctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tbctrl`]
module"]
#[doc(alias = "CFG_TBCTRL")]
pub type CfgTbctrl = crate::Reg<cfg_tbctrl::CfgTbctrlSpec>;
#[doc = "CFG_TBCTRL"]
pub mod cfg_tbctrl;
#[doc = "CFG_CAPCTRL (rw) register accessor: CFG_CAPCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_capctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_capctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_capctrl`]
module"]
#[doc(alias = "CFG_CAPCTRL")]
pub type CfgCapctrl = crate::Reg<cfg_capctrl::CfgCapctrlSpec>;
#[doc = "CFG_CAPCTRL"]
pub mod cfg_capctrl;
#[doc = "CFG_COMPCTRL (rw) register accessor: CFG_COMPCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_compctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_compctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_compctrl`]
module"]
#[doc(alias = "CFG_COMPCTRL")]
pub type CfgCompctrl = crate::Reg<cfg_compctrl::CfgCompctrlSpec>;
#[doc = "CFG_COMPCTRL"]
pub mod cfg_compctrl;
#[doc = "CFG_FRC0 (rw) register accessor: CFG_FRC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_frc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_frc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_frc0`]
module"]
#[doc(alias = "CFG_FRC0")]
pub type CfgFrc0 = crate::Reg<cfg_frc0::CfgFrc0Spec>;
#[doc = "CFG_FRC0"]
pub mod cfg_frc0;
#[doc = "CFG_UC0 (rw) register accessor: CFG_UC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_uc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_uc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_uc0`]
module"]
#[doc(alias = "CFG_UC0")]
pub type CfgUc0 = crate::Reg<cfg_uc0::CfgUc0Spec>;
#[doc = "CFG_UC0"]
pub mod cfg_uc0;
#[doc = "CFG_CPUC0 (rw) register accessor: CFG_CPUC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cpuc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cpuc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cpuc0`]
module"]
#[doc(alias = "CFG_CPUC0")]
pub type CfgCpuc0 = crate::Reg<cfg_cpuc0::CfgCpuc0Spec>;
#[doc = "CFG_CPUC0"]
pub mod cfg_cpuc0;
#[doc = "CFG_CAFRC0 (rw) register accessor: CFG_CAFRC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cafrc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cafrc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cafrc0`]
module"]
#[doc(alias = "CFG_CAFRC0")]
pub type CfgCafrc0 = crate::Reg<cfg_cafrc0::CfgCafrc0Spec>;
#[doc = "CFG_CAFRC0"]
pub mod cfg_cafrc0;
#[doc = "CFG_CAUC0 (rw) register accessor: CFG_CAUC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cauc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cauc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cauc0`]
module"]
#[doc(alias = "CFG_CAUC0")]
pub type CfgCauc0 = crate::Reg<cfg_cauc0::CfgCauc0Spec>;
#[doc = "CFG_CAUC0"]
pub mod cfg_cauc0;
#[doc = "CFG_FRC1 (rw) register accessor: CFG_FRC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_frc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_frc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_frc1`]
module"]
#[doc(alias = "CFG_FRC1")]
pub type CfgFrc1 = crate::Reg<cfg_frc1::CfgFrc1Spec>;
#[doc = "CFG_FRC1"]
pub mod cfg_frc1;
#[doc = "CFG_UC1 (rw) register accessor: CFG_UC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_uc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_uc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_uc1`]
module"]
#[doc(alias = "CFG_UC1")]
pub type CfgUc1 = crate::Reg<cfg_uc1::CfgUc1Spec>;
#[doc = "CFG_UC1"]
pub mod cfg_uc1;
#[doc = "CFG_CPUC1 (rw) register accessor: CFG_CPUC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cpuc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cpuc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cpuc1`]
module"]
#[doc(alias = "CFG_CPUC1")]
pub type CfgCpuc1 = crate::Reg<cfg_cpuc1::CfgCpuc1Spec>;
#[doc = "CFG_CPUC1"]
pub mod cfg_cpuc1;
#[doc = "CFG_CAFRC1 (rw) register accessor: CFG_CAFRC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cafrc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cafrc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cafrc1`]
module"]
#[doc(alias = "CFG_CAFRC1")]
pub type CfgCafrc1 = crate::Reg<cfg_cafrc1::CfgCafrc1Spec>;
#[doc = "CFG_CAFRC1"]
pub mod cfg_cafrc1;
#[doc = "CFG_CAUC1 (rw) register accessor: CFG_CAUC1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_cauc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_cauc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_cauc1`]
module"]
#[doc(alias = "CFG_CAUC1")]
pub type CfgCauc1 = crate::Reg<cfg_cauc1::CfgCauc1Spec>;
#[doc = "CFG_CAUC1"]
pub mod cfg_cauc1;
#[doc = "CFG_COMP0 (rw) register accessor: CFG_COMP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp0`]
module"]
#[doc(alias = "CFG_COMP0")]
pub type CfgComp0 = crate::Reg<cfg_comp0::CfgComp0Spec>;
#[doc = "CFG_COMP0"]
pub mod cfg_comp0;
#[doc = "CFG_UDCP0 (rw) register accessor: CFG_UDCP0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_udcp0`]
module"]
#[doc(alias = "CFG_UDCP0")]
pub type CfgUdcp0 = crate::Reg<cfg_udcp0::CfgUdcp0Spec>;
#[doc = "CFG_UDCP0"]
pub mod cfg_udcp0;
#[doc = "CFG_COMP1 (rw) register accessor: CFG_COMP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp1`]
module"]
#[doc(alias = "CFG_COMP1")]
pub type CfgComp1 = crate::Reg<cfg_comp1::CfgComp1Spec>;
#[doc = "CFG_COMP1"]
pub mod cfg_comp1;
#[doc = "CFG_UDCP1 (rw) register accessor: CFG_UDCP1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_udcp1`]
module"]
#[doc(alias = "CFG_UDCP1")]
pub type CfgUdcp1 = crate::Reg<cfg_udcp1::CfgUdcp1Spec>;
#[doc = "CFG_UDCP1"]
pub mod cfg_udcp1;
#[doc = "CFG_COMP2 (rw) register accessor: CFG_COMP2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp2`]
module"]
#[doc(alias = "CFG_COMP2")]
pub type CfgComp2 = crate::Reg<cfg_comp2::CfgComp2Spec>;
#[doc = "CFG_COMP2"]
pub mod cfg_comp2;
#[doc = "CFG_UDCP2 (rw) register accessor: CFG_UDCP2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_udcp2`]
module"]
#[doc(alias = "CFG_UDCP2")]
pub type CfgUdcp2 = crate::Reg<cfg_udcp2::CfgUdcp2Spec>;
#[doc = "CFG_UDCP2"]
pub mod cfg_udcp2;
#[doc = "CFG_COMP3 (rw) register accessor: CFG_COMP3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp3`]
module"]
#[doc(alias = "CFG_COMP3")]
pub type CfgComp3 = crate::Reg<cfg_comp3::CfgComp3Spec>;
#[doc = "CFG_COMP3"]
pub mod cfg_comp3;
#[doc = "CFG_UDCP3 (rw) register accessor: CFG_UDCP3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_udcp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_udcp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_udcp3`]
module"]
#[doc(alias = "CFG_UDCP3")]
pub type CfgUdcp3 = crate::Reg<cfg_udcp3::CfgUdcp3Spec>;
#[doc = "CFG_UDCP3"]
pub mod cfg_udcp3;
#[doc = "CFG_TBLCOMP (rw) register accessor: CFG_TBLCOMP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tblcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tblcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tblcomp`]
module"]
#[doc(alias = "CFG_TBLCOMP")]
pub type CfgTblcomp = crate::Reg<cfg_tblcomp::CfgTblcompSpec>;
#[doc = "CFG_TBLCOMP"]
pub mod cfg_tblcomp;
#[doc = "CFG_TBHCOMP (rw) register accessor: CFG_TBHCOMP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tbhcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tbhcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_tbhcomp`]
module"]
#[doc(alias = "CFG_TBHCOMP")]
pub type CfgTbhcomp = crate::Reg<cfg_tbhcomp::CfgTbhcompSpec>;
#[doc = "CFG_TBHCOMP"]
pub mod cfg_tbhcomp;
#[doc = "CFG_SETINT (rw) register accessor: CFG_SETINT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_setint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_setint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_setint`]
module"]
#[doc(alias = "CFG_SETINT")]
pub type CfgSetint = crate::Reg<cfg_setint::CfgSetintSpec>;
#[doc = "CFG_SETINT"]
pub mod cfg_setint;
#[doc = "CFG_CLEARINT (rw) register accessor: CFG_CLEARINT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_clearint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_clearint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_clearint`]
module"]
#[doc(alias = "CFG_CLEARINT")]
pub type CfgClearint = crate::Reg<cfg_clearint::CfgClearintSpec>;
#[doc = "CFG_CLEARINT"]
pub mod cfg_clearint;
#[doc = "CFG_INTFLAG (rw) register accessor: CFG_INTFLAG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intflag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intflag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_intflag`]
module"]
#[doc(alias = "CFG_INTFLAG")]
pub type CfgIntflag = crate::Reg<cfg_intflag::CfgIntflagSpec>;
#[doc = "CFG_INTFLAG"]
pub mod cfg_intflag;
#[doc = "CFG_DWDCTRL (rw) register accessor: CFG_DWDCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dwdctrl`]
module"]
#[doc(alias = "CFG_DWDCTRL")]
pub type CfgDwdctrl = crate::Reg<cfg_dwdctrl::CfgDwdctrlSpec>;
#[doc = "CFG_DWDCTRL"]
pub mod cfg_dwdctrl;
#[doc = "CFG_DWDPRLD (rw) register accessor: CFG_DWDPRLD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdprld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdprld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dwdprld`]
module"]
#[doc(alias = "CFG_DWDPRLD")]
pub type CfgDwdprld = crate::Reg<cfg_dwdprld::CfgDwdprldSpec>;
#[doc = "CFG_DWDPRLD"]
pub mod cfg_dwdprld;
#[doc = "CFG_WDSTATUS (rw) register accessor: CFG_WDSTATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wdstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wdstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_wdstatus`]
module"]
#[doc(alias = "CFG_WDSTATUS")]
pub type CfgWdstatus = crate::Reg<cfg_wdstatus::CfgWdstatusSpec>;
#[doc = "CFG_WDSTATUS"]
pub mod cfg_wdstatus;
#[doc = "CFG_WDKEY (rw) register accessor: CFG_WDKEY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wdkey::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wdkey::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_wdkey`]
module"]
#[doc(alias = "CFG_WDKEY")]
pub type CfgWdkey = crate::Reg<cfg_wdkey::CfgWdkeySpec>;
#[doc = "CFG_WDKEY"]
pub mod cfg_wdkey;
#[doc = "CFG_DWDCNTR (rw) register accessor: CFG_DWDCNTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdcntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdcntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_dwdcntr`]
module"]
#[doc(alias = "CFG_DWDCNTR")]
pub type CfgDwdcntr = crate::Reg<cfg_dwdcntr::CfgDwdcntrSpec>;
#[doc = "CFG_DWDCNTR"]
pub mod cfg_dwdcntr;
#[doc = "CFG_WWDRXNCTRL (rw) register accessor: CFG_WWDRXNCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wwdrxnctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wwdrxnctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_wwdrxnctrl`]
module"]
#[doc(alias = "CFG_WWDRXNCTRL")]
pub type CfgWwdrxnctrl = crate::Reg<cfg_wwdrxnctrl::CfgWwdrxnctrlSpec>;
#[doc = "CFG_WWDRXNCTRL"]
pub mod cfg_wwdrxnctrl;
#[doc = "CFG_WWDSIZECTRL (rw) register accessor: CFG_WWDSIZECTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_wwdsizectrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_wwdsizectrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_wwdsizectrl`]
module"]
#[doc(alias = "CFG_WWDSIZECTRL")]
pub type CfgWwdsizectrl = crate::Reg<cfg_wwdsizectrl::CfgWwdsizectrlSpec>;
#[doc = "CFG_WWDSIZECTRL"]
pub mod cfg_wwdsizectrl;
#[doc = "CFG_INTCLRENABLE (rw) register accessor: CFG_INTCLRENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intclrenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intclrenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_intclrenable`]
module"]
#[doc(alias = "CFG_INTCLRENABLE")]
pub type CfgIntclrenable = crate::Reg<cfg_intclrenable::CfgIntclrenableSpec>;
#[doc = "CFG_INTCLRENABLE"]
pub mod cfg_intclrenable;
#[doc = "CFG_COMP0CLR (rw) register accessor: CFG_COMP0CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp0clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp0clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp0clr`]
module"]
#[doc(alias = "CFG_COMP0CLR")]
pub type CfgComp0clr = crate::Reg<cfg_comp0clr::CfgComp0clrSpec>;
#[doc = "CFG_COMP0CLR"]
pub mod cfg_comp0clr;
#[doc = "CFG_COMP1CLR (rw) register accessor: CFG_COMP1CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp1clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp1clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp1clr`]
module"]
#[doc(alias = "CFG_COMP1CLR")]
pub type CfgComp1clr = crate::Reg<cfg_comp1clr::CfgComp1clrSpec>;
#[doc = "CFG_COMP1CLR"]
pub mod cfg_comp1clr;
#[doc = "CFG_COMP2CLR (rw) register accessor: CFG_COMP2CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp2clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp2clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp2clr`]
module"]
#[doc(alias = "CFG_COMP2CLR")]
pub type CfgComp2clr = crate::Reg<cfg_comp2clr::CfgComp2clrSpec>;
#[doc = "CFG_COMP2CLR"]
pub mod cfg_comp2clr;
#[doc = "CFG_COMP3CLR (rw) register accessor: CFG_COMP3CLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_comp3clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_comp3clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_comp3clr`]
module"]
#[doc(alias = "CFG_COMP3CLR")]
pub type CfgComp3clr = crate::Reg<cfg_comp3clr::CfgComp3clrSpec>;
#[doc = "CFG_COMP3CLR"]
pub mod cfg_comp3clr;
