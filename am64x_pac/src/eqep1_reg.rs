#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg_qposcnt: RegQposcnt,
    reg_qposinit: RegQposinit,
    reg_qposmax: RegQposmax,
    reg_qposcmp: RegQposcmp,
    reg_qposilat: RegQposilat,
    reg_qposslat: RegQposslat,
    reg_qposlat: RegQposlat,
    reg_qutmr: RegQutmr,
    reg_quprd: RegQuprd,
    reg_qwdtmr: RegQwdtmr,
    reg_qwdprd: RegQwdprd,
    reg_qdecctl_type2: RegQdecctlType2,
    reg_qepctl: RegQepctl,
    reg_qcapctl: RegQcapctl,
    reg_qposctl: RegQposctl,
    reg_qeint_type1: RegQeintType1,
    reg_qflg_type1: RegQflgType1,
    reg_qclr_type1: RegQclrType1,
    reg_qfrc_type1: RegQfrcType1,
    reg_qepsts_type1: RegQepstsType1,
    reg_qctmr: RegQctmr,
    reg_qcprd: RegQcprd,
    reg_qctmrlat: RegQctmrlat,
    reg_qcprdlat: RegQcprdlat,
    reg_reserved_1: RegReserved1,
    _reserved25: [u8; 0x1c],
    reg_rev_type2: RegRevType2,
    reg_qepstrobesel: RegQepstrobesel,
    reg_qmactrl: RegQmactrl,
    reg_qepsrcsel: RegQepsrcsel,
    reg_reserved_2: RegReserved2,
}
impl RegisterBlock {
    #[doc = "0x00 - Position Counter"]
    #[inline(always)]
    pub const fn reg_qposcnt(&self) -> &RegQposcnt {
        &self.reg_qposcnt
    }
    #[doc = "0x04 - Position Counter Init"]
    #[inline(always)]
    pub const fn reg_qposinit(&self) -> &RegQposinit {
        &self.reg_qposinit
    }
    #[doc = "0x08 - Maximum Position Count"]
    #[inline(always)]
    pub const fn reg_qposmax(&self) -> &RegQposmax {
        &self.reg_qposmax
    }
    #[doc = "0x0c - Position Compare"]
    #[inline(always)]
    pub const fn reg_qposcmp(&self) -> &RegQposcmp {
        &self.reg_qposcmp
    }
    #[doc = "0x10 - Index Position Latch"]
    #[inline(always)]
    pub const fn reg_qposilat(&self) -> &RegQposilat {
        &self.reg_qposilat
    }
    #[doc = "0x14 - Strobe Position Latch"]
    #[inline(always)]
    pub const fn reg_qposslat(&self) -> &RegQposslat {
        &self.reg_qposslat
    }
    #[doc = "0x18 - Position Latch"]
    #[inline(always)]
    pub const fn reg_qposlat(&self) -> &RegQposlat {
        &self.reg_qposlat
    }
    #[doc = "0x1c - QEP Unit Timer"]
    #[inline(always)]
    pub const fn reg_qutmr(&self) -> &RegQutmr {
        &self.reg_qutmr
    }
    #[doc = "0x20 - QEP Unit Period"]
    #[inline(always)]
    pub const fn reg_quprd(&self) -> &RegQuprd {
        &self.reg_quprd
    }
    #[doc = "0x24 - QEP Watchdog Timer"]
    #[inline(always)]
    pub const fn reg_qwdtmr(&self) -> &RegQwdtmr {
        &self.reg_qwdtmr
    }
    #[doc = "0x26 - QEP Watchdog Period"]
    #[inline(always)]
    pub const fn reg_qwdprd(&self) -> &RegQwdprd {
        &self.reg_qwdprd
    }
    #[doc = "0x28 - Quadrature Decoder Control"]
    #[inline(always)]
    pub const fn reg_qdecctl_type2(&self) -> &RegQdecctlType2 {
        &self.reg_qdecctl_type2
    }
    #[doc = "0x2a - QEP Control"]
    #[inline(always)]
    pub const fn reg_qepctl(&self) -> &RegQepctl {
        &self.reg_qepctl
    }
    #[doc = "0x2c - Qaudrature Capture Control"]
    #[inline(always)]
    pub const fn reg_qcapctl(&self) -> &RegQcapctl {
        &self.reg_qcapctl
    }
    #[doc = "0x2e - Position Compare Control"]
    #[inline(always)]
    pub const fn reg_qposctl(&self) -> &RegQposctl {
        &self.reg_qposctl
    }
    #[doc = "0x30 - QEP Interrupt Control"]
    #[inline(always)]
    pub const fn reg_qeint_type1(&self) -> &RegQeintType1 {
        &self.reg_qeint_type1
    }
    #[doc = "0x32 - QEP Interrupt Flag"]
    #[inline(always)]
    pub const fn reg_qflg_type1(&self) -> &RegQflgType1 {
        &self.reg_qflg_type1
    }
    #[doc = "0x34 - QEP Interrupt Clear"]
    #[inline(always)]
    pub const fn reg_qclr_type1(&self) -> &RegQclrType1 {
        &self.reg_qclr_type1
    }
    #[doc = "0x36 - QEP Interrupt Force"]
    #[inline(always)]
    pub const fn reg_qfrc_type1(&self) -> &RegQfrcType1 {
        &self.reg_qfrc_type1
    }
    #[doc = "0x38 - QEP Status"]
    #[inline(always)]
    pub const fn reg_qepsts_type1(&self) -> &RegQepstsType1 {
        &self.reg_qepsts_type1
    }
    #[doc = "0x3a - QEP Capture Timer"]
    #[inline(always)]
    pub const fn reg_qctmr(&self) -> &RegQctmr {
        &self.reg_qctmr
    }
    #[doc = "0x3c - QEP Capture Period"]
    #[inline(always)]
    pub const fn reg_qcprd(&self) -> &RegQcprd {
        &self.reg_qcprd
    }
    #[doc = "0x3e - QEP Capture Latch"]
    #[inline(always)]
    pub const fn reg_qctmrlat(&self) -> &RegQctmrlat {
        &self.reg_qctmrlat
    }
    #[doc = "0x40 - QEP Capture Period Latch"]
    #[inline(always)]
    pub const fn reg_qcprdlat(&self) -> &RegQcprdlat {
        &self.reg_qcprdlat
    }
    #[doc = "0x42 - REG_Reserved_1"]
    #[inline(always)]
    pub const fn reg_reserved_1(&self) -> &RegReserved1 {
        &self.reg_reserved_1
    }
    #[doc = "0x60 - QEP Revision Number"]
    #[inline(always)]
    pub const fn reg_rev_type2(&self) -> &RegRevType2 {
        &self.reg_rev_type2
    }
    #[doc = "0x64 - QEP Strobe select register"]
    #[inline(always)]
    pub const fn reg_qepstrobesel(&self) -> &RegQepstrobesel {
        &self.reg_qepstrobesel
    }
    #[doc = "0x68 - QMA Control register"]
    #[inline(always)]
    pub const fn reg_qmactrl(&self) -> &RegQmactrl {
        &self.reg_qmactrl
    }
    #[doc = "0x6c - QEP Source Select Register"]
    #[inline(always)]
    pub const fn reg_qepsrcsel(&self) -> &RegQepsrcsel {
        &self.reg_qepsrcsel
    }
    #[doc = "0x70 - REG_Reserved_2"]
    #[inline(always)]
    pub const fn reg_reserved_2(&self) -> &RegReserved2 {
        &self.reg_reserved_2
    }
}
#[doc = "REG_QPOSCNT (rw) register accessor: Position Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposcnt`]
module"]
#[doc(alias = "REG_QPOSCNT")]
pub type RegQposcnt = crate::Reg<reg_qposcnt::RegQposcntSpec>;
#[doc = "Position Counter"]
pub mod reg_qposcnt;
#[doc = "REG_QPOSINIT (rw) register accessor: Position Counter Init\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposinit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposinit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposinit`]
module"]
#[doc(alias = "REG_QPOSINIT")]
pub type RegQposinit = crate::Reg<reg_qposinit::RegQposinitSpec>;
#[doc = "Position Counter Init"]
pub mod reg_qposinit;
#[doc = "REG_QPOSMAX (rw) register accessor: Maximum Position Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposmax::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposmax::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposmax`]
module"]
#[doc(alias = "REG_QPOSMAX")]
pub type RegQposmax = crate::Reg<reg_qposmax::RegQposmaxSpec>;
#[doc = "Maximum Position Count"]
pub mod reg_qposmax;
#[doc = "REG_QPOSCMP (rw) register accessor: Position Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposcmp`]
module"]
#[doc(alias = "REG_QPOSCMP")]
pub type RegQposcmp = crate::Reg<reg_qposcmp::RegQposcmpSpec>;
#[doc = "Position Compare"]
pub mod reg_qposcmp;
#[doc = "REG_QPOSILAT (rw) register accessor: Index Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposilat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposilat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposilat`]
module"]
#[doc(alias = "REG_QPOSILAT")]
pub type RegQposilat = crate::Reg<reg_qposilat::RegQposilatSpec>;
#[doc = "Index Position Latch"]
pub mod reg_qposilat;
#[doc = "REG_QPOSSLAT (rw) register accessor: Strobe Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposslat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposslat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposslat`]
module"]
#[doc(alias = "REG_QPOSSLAT")]
pub type RegQposslat = crate::Reg<reg_qposslat::RegQposslatSpec>;
#[doc = "Strobe Position Latch"]
pub mod reg_qposslat;
#[doc = "REG_QPOSLAT (rw) register accessor: Position Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposlat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposlat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposlat`]
module"]
#[doc(alias = "REG_QPOSLAT")]
pub type RegQposlat = crate::Reg<reg_qposlat::RegQposlatSpec>;
#[doc = "Position Latch"]
pub mod reg_qposlat;
#[doc = "REG_QUTMR (rw) register accessor: QEP Unit Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qutmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qutmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qutmr`]
module"]
#[doc(alias = "REG_QUTMR")]
pub type RegQutmr = crate::Reg<reg_qutmr::RegQutmrSpec>;
#[doc = "QEP Unit Timer"]
pub mod reg_qutmr;
#[doc = "REG_QUPRD (rw) register accessor: QEP Unit Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_quprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_quprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_quprd`]
module"]
#[doc(alias = "REG_QUPRD")]
pub type RegQuprd = crate::Reg<reg_quprd::RegQuprdSpec>;
#[doc = "QEP Unit Period"]
pub mod reg_quprd;
#[doc = "REG_QWDTMR (rw) register accessor: QEP Watchdog Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qwdtmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qwdtmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qwdtmr`]
module"]
#[doc(alias = "REG_QWDTMR")]
pub type RegQwdtmr = crate::Reg<reg_qwdtmr::RegQwdtmrSpec>;
#[doc = "QEP Watchdog Timer"]
pub mod reg_qwdtmr;
#[doc = "REG_QWDPRD (rw) register accessor: QEP Watchdog Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qwdprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qwdprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qwdprd`]
module"]
#[doc(alias = "REG_QWDPRD")]
pub type RegQwdprd = crate::Reg<reg_qwdprd::RegQwdprdSpec>;
#[doc = "QEP Watchdog Period"]
pub mod reg_qwdprd;
#[doc = "REG_QDECCTL_TYPE2 (rw) register accessor: Quadrature Decoder Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qdecctl_type2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qdecctl_type2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qdecctl_type2`]
module"]
#[doc(alias = "REG_QDECCTL_TYPE2")]
pub type RegQdecctlType2 = crate::Reg<reg_qdecctl_type2::RegQdecctlType2Spec>;
#[doc = "Quadrature Decoder Control"]
pub mod reg_qdecctl_type2;
#[doc = "REG_QEPCTL (rw) register accessor: QEP Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qepctl`]
module"]
#[doc(alias = "REG_QEPCTL")]
pub type RegQepctl = crate::Reg<reg_qepctl::RegQepctlSpec>;
#[doc = "QEP Control"]
pub mod reg_qepctl;
#[doc = "REG_QCAPCTL (rw) register accessor: Qaudrature Capture Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcapctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcapctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qcapctl`]
module"]
#[doc(alias = "REG_QCAPCTL")]
pub type RegQcapctl = crate::Reg<reg_qcapctl::RegQcapctlSpec>;
#[doc = "Qaudrature Capture Control"]
pub mod reg_qcapctl;
#[doc = "REG_QPOSCTL (rw) register accessor: Position Compare Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qposctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qposctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qposctl`]
module"]
#[doc(alias = "REG_QPOSCTL")]
pub type RegQposctl = crate::Reg<reg_qposctl::RegQposctlSpec>;
#[doc = "Position Compare Control"]
pub mod reg_qposctl;
#[doc = "REG_QEINT_TYPE1 (rw) register accessor: QEP Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qeint_type1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qeint_type1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qeint_type1`]
module"]
#[doc(alias = "REG_QEINT_TYPE1")]
pub type RegQeintType1 = crate::Reg<reg_qeint_type1::RegQeintType1Spec>;
#[doc = "QEP Interrupt Control"]
pub mod reg_qeint_type1;
#[doc = "REG_QFLG_TYPE1 (rw) register accessor: QEP Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qflg_type1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qflg_type1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qflg_type1`]
module"]
#[doc(alias = "REG_QFLG_TYPE1")]
pub type RegQflgType1 = crate::Reg<reg_qflg_type1::RegQflgType1Spec>;
#[doc = "QEP Interrupt Flag"]
pub mod reg_qflg_type1;
#[doc = "REG_QCLR_TYPE1 (rw) register accessor: QEP Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qclr_type1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qclr_type1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qclr_type1`]
module"]
#[doc(alias = "REG_QCLR_TYPE1")]
pub type RegQclrType1 = crate::Reg<reg_qclr_type1::RegQclrType1Spec>;
#[doc = "QEP Interrupt Clear"]
pub mod reg_qclr_type1;
#[doc = "REG_QFRC_TYPE1 (rw) register accessor: QEP Interrupt Force\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qfrc_type1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qfrc_type1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qfrc_type1`]
module"]
#[doc(alias = "REG_QFRC_TYPE1")]
pub type RegQfrcType1 = crate::Reg<reg_qfrc_type1::RegQfrcType1Spec>;
#[doc = "QEP Interrupt Force"]
pub mod reg_qfrc_type1;
#[doc = "REG_QEPSTS_TYPE1 (rw) register accessor: QEP Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepsts_type1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepsts_type1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qepsts_type1`]
module"]
#[doc(alias = "REG_QEPSTS_TYPE1")]
pub type RegQepstsType1 = crate::Reg<reg_qepsts_type1::RegQepstsType1Spec>;
#[doc = "QEP Status"]
pub mod reg_qepsts_type1;
#[doc = "REG_QCTMR (rw) register accessor: QEP Capture Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qctmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qctmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qctmr`]
module"]
#[doc(alias = "REG_QCTMR")]
pub type RegQctmr = crate::Reg<reg_qctmr::RegQctmrSpec>;
#[doc = "QEP Capture Timer"]
pub mod reg_qctmr;
#[doc = "REG_QCPRD (rw) register accessor: QEP Capture Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qcprd`]
module"]
#[doc(alias = "REG_QCPRD")]
pub type RegQcprd = crate::Reg<reg_qcprd::RegQcprdSpec>;
#[doc = "QEP Capture Period"]
pub mod reg_qcprd;
#[doc = "REG_QCTMRLAT (rw) register accessor: QEP Capture Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qctmrlat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qctmrlat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qctmrlat`]
module"]
#[doc(alias = "REG_QCTMRLAT")]
pub type RegQctmrlat = crate::Reg<reg_qctmrlat::RegQctmrlatSpec>;
#[doc = "QEP Capture Latch"]
pub mod reg_qctmrlat;
#[doc = "REG_QCPRDLAT (rw) register accessor: QEP Capture Period Latch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qcprdlat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qcprdlat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qcprdlat`]
module"]
#[doc(alias = "REG_QCPRDLAT")]
pub type RegQcprdlat = crate::Reg<reg_qcprdlat::RegQcprdlatSpec>;
#[doc = "QEP Capture Period Latch"]
pub mod reg_qcprdlat;
#[doc = "REG_Reserved_1 (rw) register accessor: REG_Reserved_1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_reserved_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_reserved_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_reserved_1`]
module"]
#[doc(alias = "REG_Reserved_1")]
pub type RegReserved1 = crate::Reg<reg_reserved_1::RegReserved1Spec>;
#[doc = "REG_Reserved_1"]
pub mod reg_reserved_1;
#[doc = "REG_REV_TYPE2 (rw) register accessor: QEP Revision Number\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_rev_type2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_rev_type2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_rev_type2`]
module"]
#[doc(alias = "REG_REV_TYPE2")]
pub type RegRevType2 = crate::Reg<reg_rev_type2::RegRevType2Spec>;
#[doc = "QEP Revision Number"]
pub mod reg_rev_type2;
#[doc = "REG_QEPSTROBESEL (rw) register accessor: QEP Strobe select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepstrobesel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepstrobesel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qepstrobesel`]
module"]
#[doc(alias = "REG_QEPSTROBESEL")]
pub type RegQepstrobesel = crate::Reg<reg_qepstrobesel::RegQepstrobeselSpec>;
#[doc = "QEP Strobe select register"]
pub mod reg_qepstrobesel;
#[doc = "REG_QMACTRL (rw) register accessor: QMA Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qmactrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qmactrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qmactrl`]
module"]
#[doc(alias = "REG_QMACTRL")]
pub type RegQmactrl = crate::Reg<reg_qmactrl::RegQmactrlSpec>;
#[doc = "QMA Control register"]
pub mod reg_qmactrl;
#[doc = "REG_QEPSRCSEL (rw) register accessor: QEP Source Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_qepsrcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_qepsrcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_qepsrcsel`]
module"]
#[doc(alias = "REG_QEPSRCSEL")]
pub type RegQepsrcsel = crate::Reg<reg_qepsrcsel::RegQepsrcselSpec>;
#[doc = "QEP Source Select Register"]
pub mod reg_qepsrcsel;
#[doc = "REG_Reserved_2 (rw) register accessor: REG_Reserved_2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_reserved_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_reserved_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg_reserved_2`]
module"]
#[doc(alias = "REG_Reserved_2")]
pub type RegReserved2 = crate::Reg<reg_reserved_2::RegReserved2Spec>;
#[doc = "REG_Reserved_2"]
pub mod reg_reserved_2;
