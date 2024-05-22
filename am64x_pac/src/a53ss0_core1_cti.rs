#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    apbaddr_cti_cpu1_cticontrol: ApbaddrCtiCpu1Cticontrol,
    _reserved1: [u8; 0x0c],
    apbaddr_cti_cpu1_ctiintack: ApbaddrCtiCpu1Ctiintack,
    apbaddr_cti_cpu1_ctiappset: ApbaddrCtiCpu1Ctiappset,
    apbaddr_cti_cpu1_ctiappclear: ApbaddrCtiCpu1Ctiappclear,
    apbaddr_cti_cpu1_ctiapppulse: ApbaddrCtiCpu1Ctiapppulse,
    apbaddr_cti_cpu1_ctiinen0: ApbaddrCtiCpu1Ctiinen0,
    apbaddr_cti_cpu1_ctiinen1: ApbaddrCtiCpu1Ctiinen1,
    apbaddr_cti_cpu1_ctiinen2: ApbaddrCtiCpu1Ctiinen2,
    apbaddr_cti_cpu1_ctiinen3: ApbaddrCtiCpu1Ctiinen3,
    apbaddr_cti_cpu1_ctiinen4: ApbaddrCtiCpu1Ctiinen4,
    apbaddr_cti_cpu1_ctiinen5: ApbaddrCtiCpu1Ctiinen5,
    apbaddr_cti_cpu1_ctiinen6: ApbaddrCtiCpu1Ctiinen6,
    apbaddr_cti_cpu1_ctiinen7: ApbaddrCtiCpu1Ctiinen7,
    _reserved13: [u8; 0x60],
    apbaddr_cti_cpu1_ctiouten0: ApbaddrCtiCpu1Ctiouten0,
    apbaddr_cti_cpu1_ctiouten1: ApbaddrCtiCpu1Ctiouten1,
    apbaddr_cti_cpu1_ctiouten2: ApbaddrCtiCpu1Ctiouten2,
    apbaddr_cti_cpu1_ctiouten3: ApbaddrCtiCpu1Ctiouten3,
    apbaddr_cti_cpu1_ctiouten4: ApbaddrCtiCpu1Ctiouten4,
    apbaddr_cti_cpu1_ctiouten5: ApbaddrCtiCpu1Ctiouten5,
    apbaddr_cti_cpu1_ctiouten6: ApbaddrCtiCpu1Ctiouten6,
    apbaddr_cti_cpu1_ctiouten7: ApbaddrCtiCpu1Ctiouten7,
    _reserved21: [u8; 0x70],
    apbaddr_cti_cpu1_ctitriginstatus: ApbaddrCtiCpu1Ctitriginstatus,
    apbaddr_cti_cpu1_ctitrigoutstatus: ApbaddrCtiCpu1Ctitrigoutstatus,
    apbaddr_cti_cpu1_ctichinstatus: ApbaddrCtiCpu1Ctichinstatus,
    apbaddr_cti_cpu1_ctichoutstatus: ApbaddrCtiCpu1Ctichoutstatus,
    apbaddr_cti_cpu1_ctigate: ApbaddrCtiCpu1Ctigate,
    apbaddr_cti_cpu1_asicctl: ApbaddrCtiCpu1Asicctl,
    _reserved27: [u8; 0x0db8],
    apbaddr_cti_cpu1_ctiitctrl: ApbaddrCtiCpu1Ctiitctrl,
    _reserved28: [u8; 0x9c],
    apbaddr_cti_cpu1_cticlaimset: ApbaddrCtiCpu1Cticlaimset,
    apbaddr_cti_cpu1_cticlaimclr: ApbaddrCtiCpu1Cticlaimclr,
    apbaddr_cti_cpu1_ctidevaff0: ApbaddrCtiCpu1Ctidevaff0,
    apbaddr_cti_cpu1_ctidevaff1: ApbaddrCtiCpu1Ctidevaff1,
    apbaddr_cti_cpu1_ctilar: ApbaddrCtiCpu1Ctilar,
    apbaddr_cti_cpu1_ctilsr: ApbaddrCtiCpu1Ctilsr,
    apbaddr_cti_cpu1_ctiauthstatus: ApbaddrCtiCpu1Ctiauthstatus,
    apbaddr_cti_cpu1_ctidevarch: ApbaddrCtiCpu1Ctidevarch,
    apbaddr_cti_cpu1_ctidevid2: ApbaddrCtiCpu1Ctidevid2,
    apbaddr_cti_cpu1_ctidevid1: ApbaddrCtiCpu1Ctidevid1,
    apbaddr_cti_cpu1_ctidevid: ApbaddrCtiCpu1Ctidevid,
    apbaddr_cti_cpu1_ctidevtype: ApbaddrCtiCpu1Ctidevtype,
    apbaddr_cti_cpu1_ctipidr4: ApbaddrCtiCpu1Ctipidr4,
    apbaddr_cti_cpu1_ctipidr5: ApbaddrCtiCpu1Ctipidr5,
    apbaddr_cti_cpu1_ctipidr6: ApbaddrCtiCpu1Ctipidr6,
    apbaddr_cti_cpu1_ctipidr7: ApbaddrCtiCpu1Ctipidr7,
    apbaddr_cti_cpu1_ctipidr0: ApbaddrCtiCpu1Ctipidr0,
    apbaddr_cti_cpu1_ctipidr1: ApbaddrCtiCpu1Ctipidr1,
    apbaddr_cti_cpu1_ctipidr2: ApbaddrCtiCpu1Ctipidr2,
    apbaddr_cti_cpu1_ctipidr3: ApbaddrCtiCpu1Ctipidr3,
    apbaddr_cti_cpu1_cticidr0: ApbaddrCtiCpu1Cticidr0,
    apbaddr_cti_cpu1_cticidr1: ApbaddrCtiCpu1Cticidr1,
    apbaddr_cti_cpu1_cticidr2: ApbaddrCtiCpu1Cticidr2,
    apbaddr_cti_cpu1_cticidr3: ApbaddrCtiCpu1Cticidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - CTI Control Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticontrol(&self) -> &ApbaddrCtiCpu1Cticontrol {
        &self.apbaddr_cti_cpu1_cticontrol
    }
    #[doc = "0x10 - CTI Output Trigger Acknowledge Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiintack(&self) -> &ApbaddrCtiCpu1Ctiintack {
        &self.apbaddr_cti_cpu1_ctiintack
    }
    #[doc = "0x14 - CTI Application Trigger Set Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiappset(&self) -> &ApbaddrCtiCpu1Ctiappset {
        &self.apbaddr_cti_cpu1_ctiappset
    }
    #[doc = "0x18 - CTI Application Trigger Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiappclear(&self) -> &ApbaddrCtiCpu1Ctiappclear {
        &self.apbaddr_cti_cpu1_ctiappclear
    }
    #[doc = "0x1c - CTI Application Pulse Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiapppulse(&self) -> &ApbaddrCtiCpu1Ctiapppulse {
        &self.apbaddr_cti_cpu1_ctiapppulse
    }
    #[doc = "0x20 - CTI Input Trigger to Output Channel Enable Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen0(&self) -> &ApbaddrCtiCpu1Ctiinen0 {
        &self.apbaddr_cti_cpu1_ctiinen0
    }
    #[doc = "0x24 - CTI Input Trigger to Output Channel Enable Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen1(&self) -> &ApbaddrCtiCpu1Ctiinen1 {
        &self.apbaddr_cti_cpu1_ctiinen1
    }
    #[doc = "0x28 - CTI Input Trigger to Output Channel Enable Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen2(&self) -> &ApbaddrCtiCpu1Ctiinen2 {
        &self.apbaddr_cti_cpu1_ctiinen2
    }
    #[doc = "0x2c - CTI Input Trigger to Output Channel Enable Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen3(&self) -> &ApbaddrCtiCpu1Ctiinen3 {
        &self.apbaddr_cti_cpu1_ctiinen3
    }
    #[doc = "0x30 - CTI Input Trigger to Output Channel Enable Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen4(&self) -> &ApbaddrCtiCpu1Ctiinen4 {
        &self.apbaddr_cti_cpu1_ctiinen4
    }
    #[doc = "0x34 - CTI Input Trigger to Output Channel Enable Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen5(&self) -> &ApbaddrCtiCpu1Ctiinen5 {
        &self.apbaddr_cti_cpu1_ctiinen5
    }
    #[doc = "0x38 - CTI Input Trigger to Output Channel Enable Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen6(&self) -> &ApbaddrCtiCpu1Ctiinen6 {
        &self.apbaddr_cti_cpu1_ctiinen6
    }
    #[doc = "0x3c - CTI Input Trigger to Output Channel Enable Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiinen7(&self) -> &ApbaddrCtiCpu1Ctiinen7 {
        &self.apbaddr_cti_cpu1_ctiinen7
    }
    #[doc = "0xa0 - CTI Input Channel to Output Trigger Enable Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten0(&self) -> &ApbaddrCtiCpu1Ctiouten0 {
        &self.apbaddr_cti_cpu1_ctiouten0
    }
    #[doc = "0xa4 - CTI Input Channel to Output Trigger Enable Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten1(&self) -> &ApbaddrCtiCpu1Ctiouten1 {
        &self.apbaddr_cti_cpu1_ctiouten1
    }
    #[doc = "0xa8 - CTI Input Channel to Output Trigger Enable Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten2(&self) -> &ApbaddrCtiCpu1Ctiouten2 {
        &self.apbaddr_cti_cpu1_ctiouten2
    }
    #[doc = "0xac - CTI Input Channel to Output Trigger Enable Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten3(&self) -> &ApbaddrCtiCpu1Ctiouten3 {
        &self.apbaddr_cti_cpu1_ctiouten3
    }
    #[doc = "0xb0 - CTI Input Channel to Output Trigger Enable Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten4(&self) -> &ApbaddrCtiCpu1Ctiouten4 {
        &self.apbaddr_cti_cpu1_ctiouten4
    }
    #[doc = "0xb4 - CTI Input Channel to Output Trigger Enable Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten5(&self) -> &ApbaddrCtiCpu1Ctiouten5 {
        &self.apbaddr_cti_cpu1_ctiouten5
    }
    #[doc = "0xb8 - CTI Input Channel to Output Trigger Enable Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten6(&self) -> &ApbaddrCtiCpu1Ctiouten6 {
        &self.apbaddr_cti_cpu1_ctiouten6
    }
    #[doc = "0xbc - CTI Input Channel to Output Trigger Enable Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiouten7(&self) -> &ApbaddrCtiCpu1Ctiouten7 {
        &self.apbaddr_cti_cpu1_ctiouten7
    }
    #[doc = "0x130 - CTI Trigger In Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctitriginstatus(&self) -> &ApbaddrCtiCpu1Ctitriginstatus {
        &self.apbaddr_cti_cpu1_ctitriginstatus
    }
    #[doc = "0x134 - CTI Trigger Out Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctitrigoutstatus(&self) -> &ApbaddrCtiCpu1Ctitrigoutstatus {
        &self.apbaddr_cti_cpu1_ctitrigoutstatus
    }
    #[doc = "0x138 - CTI Channel In Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctichinstatus(&self) -> &ApbaddrCtiCpu1Ctichinstatus {
        &self.apbaddr_cti_cpu1_ctichinstatus
    }
    #[doc = "0x13c - CTI Channel Out Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctichoutstatus(&self) -> &ApbaddrCtiCpu1Ctichoutstatus {
        &self.apbaddr_cti_cpu1_ctichoutstatus
    }
    #[doc = "0x140 - CTI Channel Gate Enable Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctigate(&self) -> &ApbaddrCtiCpu1Ctigate {
        &self.apbaddr_cti_cpu1_ctigate
    }
    #[doc = "0x144 - CTI External Multiplexor Control register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_asicctl(&self) -> &ApbaddrCtiCpu1Asicctl {
        &self.apbaddr_cti_cpu1_asicctl
    }
    #[doc = "0xf00 - CTI Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiitctrl(&self) -> &ApbaddrCtiCpu1Ctiitctrl {
        &self.apbaddr_cti_cpu1_ctiitctrl
    }
    #[doc = "0xfa0 - CTI Claim Set"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticlaimset(&self) -> &ApbaddrCtiCpu1Cticlaimset {
        &self.apbaddr_cti_cpu1_cticlaimset
    }
    #[doc = "0xfa4 - CTI Claim Clear"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticlaimclr(&self) -> &ApbaddrCtiCpu1Cticlaimclr {
        &self.apbaddr_cti_cpu1_cticlaimclr
    }
    #[doc = "0xfa8 - CTI Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevaff0(&self) -> &ApbaddrCtiCpu1Ctidevaff0 {
        &self.apbaddr_cti_cpu1_ctidevaff0
    }
    #[doc = "0xfac - CTI Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevaff1(&self) -> &ApbaddrCtiCpu1Ctidevaff1 {
        &self.apbaddr_cti_cpu1_ctidevaff1
    }
    #[doc = "0xfb0 - CTI Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctilar(&self) -> &ApbaddrCtiCpu1Ctilar {
        &self.apbaddr_cti_cpu1_ctilar
    }
    #[doc = "0xfb4 - CTI Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctilsr(&self) -> &ApbaddrCtiCpu1Ctilsr {
        &self.apbaddr_cti_cpu1_ctilsr
    }
    #[doc = "0xfb8 - CTI Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctiauthstatus(&self) -> &ApbaddrCtiCpu1Ctiauthstatus {
        &self.apbaddr_cti_cpu1_ctiauthstatus
    }
    #[doc = "0xfbc - CTI Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevarch(&self) -> &ApbaddrCtiCpu1Ctidevarch {
        &self.apbaddr_cti_cpu1_ctidevarch
    }
    #[doc = "0xfc0 - CTI Device ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevid2(&self) -> &ApbaddrCtiCpu1Ctidevid2 {
        &self.apbaddr_cti_cpu1_ctidevid2
    }
    #[doc = "0xfc4 - CTI Device ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevid1(&self) -> &ApbaddrCtiCpu1Ctidevid1 {
        &self.apbaddr_cti_cpu1_ctidevid1
    }
    #[doc = "0xfc8 - CTI Device ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevid(&self) -> &ApbaddrCtiCpu1Ctidevid {
        &self.apbaddr_cti_cpu1_ctidevid
    }
    #[doc = "0xfcc - CTI Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctidevtype(&self) -> &ApbaddrCtiCpu1Ctidevtype {
        &self.apbaddr_cti_cpu1_ctidevtype
    }
    #[doc = "0xfd0 - CTI Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr4(&self) -> &ApbaddrCtiCpu1Ctipidr4 {
        &self.apbaddr_cti_cpu1_ctipidr4
    }
    #[doc = "0xfd4 - CTI Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr5(&self) -> &ApbaddrCtiCpu1Ctipidr5 {
        &self.apbaddr_cti_cpu1_ctipidr5
    }
    #[doc = "0xfd8 - CTI Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr6(&self) -> &ApbaddrCtiCpu1Ctipidr6 {
        &self.apbaddr_cti_cpu1_ctipidr6
    }
    #[doc = "0xfdc - CTI Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr7(&self) -> &ApbaddrCtiCpu1Ctipidr7 {
        &self.apbaddr_cti_cpu1_ctipidr7
    }
    #[doc = "0xfe0 - CTI Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr0(&self) -> &ApbaddrCtiCpu1Ctipidr0 {
        &self.apbaddr_cti_cpu1_ctipidr0
    }
    #[doc = "0xfe4 - CTI Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr1(&self) -> &ApbaddrCtiCpu1Ctipidr1 {
        &self.apbaddr_cti_cpu1_ctipidr1
    }
    #[doc = "0xfe8 - CTI Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr2(&self) -> &ApbaddrCtiCpu1Ctipidr2 {
        &self.apbaddr_cti_cpu1_ctipidr2
    }
    #[doc = "0xfec - CTI Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_ctipidr3(&self) -> &ApbaddrCtiCpu1Ctipidr3 {
        &self.apbaddr_cti_cpu1_ctipidr3
    }
    #[doc = "0xff0 - CTI Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticidr0(&self) -> &ApbaddrCtiCpu1Cticidr0 {
        &self.apbaddr_cti_cpu1_cticidr0
    }
    #[doc = "0xff4 - CTI Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticidr1(&self) -> &ApbaddrCtiCpu1Cticidr1 {
        &self.apbaddr_cti_cpu1_cticidr1
    }
    #[doc = "0xff8 - CTI Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticidr2(&self) -> &ApbaddrCtiCpu1Cticidr2 {
        &self.apbaddr_cti_cpu1_cticidr2
    }
    #[doc = "0xffc - CTI Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu1_cticidr3(&self) -> &ApbaddrCtiCpu1Cticidr3 {
        &self.apbaddr_cti_cpu1_cticidr3
    }
}
#[doc = "APBADDR_CTI_CPU1_CTICONTROL (rw) register accessor: CTI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticontrol`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICONTROL")]
pub type ApbaddrCtiCpu1Cticontrol =
    crate::Reg<apbaddr_cti_cpu1_cticontrol::ApbaddrCtiCpu1CticontrolSpec>;
#[doc = "CTI Control Register"]
pub mod apbaddr_cti_cpu1_cticontrol;
#[doc = "APBADDR_CTI_CPU1_CTIINTACK (rw) register accessor: CTI Output Trigger Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiintack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiintack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiintack`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINTACK")]
pub type ApbaddrCtiCpu1Ctiintack =
    crate::Reg<apbaddr_cti_cpu1_ctiintack::ApbaddrCtiCpu1CtiintackSpec>;
#[doc = "CTI Output Trigger Acknowledge Register"]
pub mod apbaddr_cti_cpu1_ctiintack;
#[doc = "APBADDR_CTI_CPU1_CTIAPPSET (rw) register accessor: CTI Application Trigger Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiappset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiappset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiappset`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIAPPSET")]
pub type ApbaddrCtiCpu1Ctiappset =
    crate::Reg<apbaddr_cti_cpu1_ctiappset::ApbaddrCtiCpu1CtiappsetSpec>;
#[doc = "CTI Application Trigger Set Register"]
pub mod apbaddr_cti_cpu1_ctiappset;
#[doc = "APBADDR_CTI_CPU1_CTIAPPCLEAR (rw) register accessor: CTI Application Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiappclear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiappclear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiappclear`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIAPPCLEAR")]
pub type ApbaddrCtiCpu1Ctiappclear =
    crate::Reg<apbaddr_cti_cpu1_ctiappclear::ApbaddrCtiCpu1CtiappclearSpec>;
#[doc = "CTI Application Trigger Clear Register"]
pub mod apbaddr_cti_cpu1_ctiappclear;
#[doc = "APBADDR_CTI_CPU1_CTIAPPPULSE (rw) register accessor: CTI Application Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiapppulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiapppulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiapppulse`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIAPPPULSE")]
pub type ApbaddrCtiCpu1Ctiapppulse =
    crate::Reg<apbaddr_cti_cpu1_ctiapppulse::ApbaddrCtiCpu1CtiapppulseSpec>;
#[doc = "CTI Application Pulse Register"]
pub mod apbaddr_cti_cpu1_ctiapppulse;
#[doc = "APBADDR_CTI_CPU1_CTIINEN0 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN0")]
pub type ApbaddrCtiCpu1Ctiinen0 = crate::Reg<apbaddr_cti_cpu1_ctiinen0::ApbaddrCtiCpu1Ctiinen0Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 0"]
pub mod apbaddr_cti_cpu1_ctiinen0;
#[doc = "APBADDR_CTI_CPU1_CTIINEN1 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN1")]
pub type ApbaddrCtiCpu1Ctiinen1 = crate::Reg<apbaddr_cti_cpu1_ctiinen1::ApbaddrCtiCpu1Ctiinen1Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 1"]
pub mod apbaddr_cti_cpu1_ctiinen1;
#[doc = "APBADDR_CTI_CPU1_CTIINEN2 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN2")]
pub type ApbaddrCtiCpu1Ctiinen2 = crate::Reg<apbaddr_cti_cpu1_ctiinen2::ApbaddrCtiCpu1Ctiinen2Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 2"]
pub mod apbaddr_cti_cpu1_ctiinen2;
#[doc = "APBADDR_CTI_CPU1_CTIINEN3 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN3")]
pub type ApbaddrCtiCpu1Ctiinen3 = crate::Reg<apbaddr_cti_cpu1_ctiinen3::ApbaddrCtiCpu1Ctiinen3Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 3"]
pub mod apbaddr_cti_cpu1_ctiinen3;
#[doc = "APBADDR_CTI_CPU1_CTIINEN4 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN4")]
pub type ApbaddrCtiCpu1Ctiinen4 = crate::Reg<apbaddr_cti_cpu1_ctiinen4::ApbaddrCtiCpu1Ctiinen4Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 4"]
pub mod apbaddr_cti_cpu1_ctiinen4;
#[doc = "APBADDR_CTI_CPU1_CTIINEN5 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN5")]
pub type ApbaddrCtiCpu1Ctiinen5 = crate::Reg<apbaddr_cti_cpu1_ctiinen5::ApbaddrCtiCpu1Ctiinen5Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 5"]
pub mod apbaddr_cti_cpu1_ctiinen5;
#[doc = "APBADDR_CTI_CPU1_CTIINEN6 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN6")]
pub type ApbaddrCtiCpu1Ctiinen6 = crate::Reg<apbaddr_cti_cpu1_ctiinen6::ApbaddrCtiCpu1Ctiinen6Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 6"]
pub mod apbaddr_cti_cpu1_ctiinen6;
#[doc = "APBADDR_CTI_CPU1_CTIINEN7 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiinen7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiinen7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiinen7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIINEN7")]
pub type ApbaddrCtiCpu1Ctiinen7 = crate::Reg<apbaddr_cti_cpu1_ctiinen7::ApbaddrCtiCpu1Ctiinen7Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 7"]
pub mod apbaddr_cti_cpu1_ctiinen7;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN0 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN0")]
pub type ApbaddrCtiCpu1Ctiouten0 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten0::ApbaddrCtiCpu1Ctiouten0Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 0"]
pub mod apbaddr_cti_cpu1_ctiouten0;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN1 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN1")]
pub type ApbaddrCtiCpu1Ctiouten1 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten1::ApbaddrCtiCpu1Ctiouten1Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 1"]
pub mod apbaddr_cti_cpu1_ctiouten1;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN2 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN2")]
pub type ApbaddrCtiCpu1Ctiouten2 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten2::ApbaddrCtiCpu1Ctiouten2Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 2"]
pub mod apbaddr_cti_cpu1_ctiouten2;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN3 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN3")]
pub type ApbaddrCtiCpu1Ctiouten3 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten3::ApbaddrCtiCpu1Ctiouten3Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 3"]
pub mod apbaddr_cti_cpu1_ctiouten3;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN4 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN4")]
pub type ApbaddrCtiCpu1Ctiouten4 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten4::ApbaddrCtiCpu1Ctiouten4Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 4"]
pub mod apbaddr_cti_cpu1_ctiouten4;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN5 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN5")]
pub type ApbaddrCtiCpu1Ctiouten5 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten5::ApbaddrCtiCpu1Ctiouten5Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 5"]
pub mod apbaddr_cti_cpu1_ctiouten5;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN6 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN6")]
pub type ApbaddrCtiCpu1Ctiouten6 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten6::ApbaddrCtiCpu1Ctiouten6Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 6"]
pub mod apbaddr_cti_cpu1_ctiouten6;
#[doc = "APBADDR_CTI_CPU1_CTIOUTEN7 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiouten7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiouten7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiouten7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIOUTEN7")]
pub type ApbaddrCtiCpu1Ctiouten7 =
    crate::Reg<apbaddr_cti_cpu1_ctiouten7::ApbaddrCtiCpu1Ctiouten7Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 7"]
pub mod apbaddr_cti_cpu1_ctiouten7;
#[doc = "APBADDR_CTI_CPU1_CTITRIGINSTATUS (rw) register accessor: CTI Trigger In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctitriginstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctitriginstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctitriginstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTITRIGINSTATUS")]
pub type ApbaddrCtiCpu1Ctitriginstatus =
    crate::Reg<apbaddr_cti_cpu1_ctitriginstatus::ApbaddrCtiCpu1CtitriginstatusSpec>;
#[doc = "CTI Trigger In Status Register"]
pub mod apbaddr_cti_cpu1_ctitriginstatus;
#[doc = "APBADDR_CTI_CPU1_CTITRIGOUTSTATUS (rw) register accessor: CTI Trigger Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctitrigoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctitrigoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctitrigoutstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTITRIGOUTSTATUS")]
pub type ApbaddrCtiCpu1Ctitrigoutstatus =
    crate::Reg<apbaddr_cti_cpu1_ctitrigoutstatus::ApbaddrCtiCpu1CtitrigoutstatusSpec>;
#[doc = "CTI Trigger Out Status Register"]
pub mod apbaddr_cti_cpu1_ctitrigoutstatus;
#[doc = "APBADDR_CTI_CPU1_CTICHINSTATUS (rw) register accessor: CTI Channel In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctichinstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctichinstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctichinstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICHINSTATUS")]
pub type ApbaddrCtiCpu1Ctichinstatus =
    crate::Reg<apbaddr_cti_cpu1_ctichinstatus::ApbaddrCtiCpu1CtichinstatusSpec>;
#[doc = "CTI Channel In Status Register"]
pub mod apbaddr_cti_cpu1_ctichinstatus;
#[doc = "APBADDR_CTI_CPU1_CTICHOUTSTATUS (rw) register accessor: CTI Channel Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctichoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctichoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctichoutstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICHOUTSTATUS")]
pub type ApbaddrCtiCpu1Ctichoutstatus =
    crate::Reg<apbaddr_cti_cpu1_ctichoutstatus::ApbaddrCtiCpu1CtichoutstatusSpec>;
#[doc = "CTI Channel Out Status Register"]
pub mod apbaddr_cti_cpu1_ctichoutstatus;
#[doc = "APBADDR_CTI_CPU1_CTIGATE (rw) register accessor: CTI Channel Gate Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctigate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctigate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctigate`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIGATE")]
pub type ApbaddrCtiCpu1Ctigate = crate::Reg<apbaddr_cti_cpu1_ctigate::ApbaddrCtiCpu1CtigateSpec>;
#[doc = "CTI Channel Gate Enable Register"]
pub mod apbaddr_cti_cpu1_ctigate;
#[doc = "APBADDR_CTI_CPU1_ASICCTL (rw) register accessor: CTI External Multiplexor Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_asicctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_asicctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_asicctl`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_ASICCTL")]
pub type ApbaddrCtiCpu1Asicctl = crate::Reg<apbaddr_cti_cpu1_asicctl::ApbaddrCtiCpu1AsicctlSpec>;
#[doc = "CTI External Multiplexor Control register"]
pub mod apbaddr_cti_cpu1_asicctl;
#[doc = "APBADDR_CTI_CPU1_CTIITCTRL (rw) register accessor: CTI Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiitctrl`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIITCTRL")]
pub type ApbaddrCtiCpu1Ctiitctrl =
    crate::Reg<apbaddr_cti_cpu1_ctiitctrl::ApbaddrCtiCpu1CtiitctrlSpec>;
#[doc = "CTI Integration mode Control Register"]
pub mod apbaddr_cti_cpu1_ctiitctrl;
#[doc = "APBADDR_CTI_CPU1_CTICLAIMSET (rw) register accessor: CTI Claim Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticlaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticlaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticlaimset`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICLAIMSET")]
pub type ApbaddrCtiCpu1Cticlaimset =
    crate::Reg<apbaddr_cti_cpu1_cticlaimset::ApbaddrCtiCpu1CticlaimsetSpec>;
#[doc = "CTI Claim Set"]
pub mod apbaddr_cti_cpu1_cticlaimset;
#[doc = "APBADDR_CTI_CPU1_CTICLAIMCLR (rw) register accessor: CTI Claim Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticlaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticlaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticlaimclr`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICLAIMCLR")]
pub type ApbaddrCtiCpu1Cticlaimclr =
    crate::Reg<apbaddr_cti_cpu1_cticlaimclr::ApbaddrCtiCpu1CticlaimclrSpec>;
#[doc = "CTI Claim Clear"]
pub mod apbaddr_cti_cpu1_cticlaimclr;
#[doc = "APBADDR_CTI_CPU1_CTIDEVAFF0 (rw) register accessor: CTI Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevaff0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVAFF0")]
pub type ApbaddrCtiCpu1Ctidevaff0 =
    crate::Reg<apbaddr_cti_cpu1_ctidevaff0::ApbaddrCtiCpu1Ctidevaff0Spec>;
#[doc = "CTI Device Affinity Register 0"]
pub mod apbaddr_cti_cpu1_ctidevaff0;
#[doc = "APBADDR_CTI_CPU1_CTIDEVAFF1 (rw) register accessor: CTI Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevaff1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVAFF1")]
pub type ApbaddrCtiCpu1Ctidevaff1 =
    crate::Reg<apbaddr_cti_cpu1_ctidevaff1::ApbaddrCtiCpu1Ctidevaff1Spec>;
#[doc = "CTI Device Affinity Register 1"]
pub mod apbaddr_cti_cpu1_ctidevaff1;
#[doc = "APBADDR_CTI_CPU1_CTILAR (rw) register accessor: CTI Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctilar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctilar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctilar`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTILAR")]
pub type ApbaddrCtiCpu1Ctilar = crate::Reg<apbaddr_cti_cpu1_ctilar::ApbaddrCtiCpu1CtilarSpec>;
#[doc = "CTI Lock Access Register"]
pub mod apbaddr_cti_cpu1_ctilar;
#[doc = "APBADDR_CTI_CPU1_CTILSR (rw) register accessor: CTI Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctilsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctilsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctilsr`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTILSR")]
pub type ApbaddrCtiCpu1Ctilsr = crate::Reg<apbaddr_cti_cpu1_ctilsr::ApbaddrCtiCpu1CtilsrSpec>;
#[doc = "CTI Lock Status Register"]
pub mod apbaddr_cti_cpu1_ctilsr;
#[doc = "APBADDR_CTI_CPU1_CTIAUTHSTATUS (rw) register accessor: CTI Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctiauthstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIAUTHSTATUS")]
pub type ApbaddrCtiCpu1Ctiauthstatus =
    crate::Reg<apbaddr_cti_cpu1_ctiauthstatus::ApbaddrCtiCpu1CtiauthstatusSpec>;
#[doc = "CTI Authentication Status Register"]
pub mod apbaddr_cti_cpu1_ctiauthstatus;
#[doc = "APBADDR_CTI_CPU1_CTIDEVARCH (rw) register accessor: CTI Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevarch`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVARCH")]
pub type ApbaddrCtiCpu1Ctidevarch =
    crate::Reg<apbaddr_cti_cpu1_ctidevarch::ApbaddrCtiCpu1CtidevarchSpec>;
#[doc = "CTI Device Architecture Register"]
pub mod apbaddr_cti_cpu1_ctidevarch;
#[doc = "APBADDR_CTI_CPU1_CTIDEVID2 (rw) register accessor: CTI Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevid2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVID2")]
pub type ApbaddrCtiCpu1Ctidevid2 =
    crate::Reg<apbaddr_cti_cpu1_ctidevid2::ApbaddrCtiCpu1Ctidevid2Spec>;
#[doc = "CTI Device ID Register 2"]
pub mod apbaddr_cti_cpu1_ctidevid2;
#[doc = "APBADDR_CTI_CPU1_CTIDEVID1 (rw) register accessor: CTI Device ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevid1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVID1")]
pub type ApbaddrCtiCpu1Ctidevid1 =
    crate::Reg<apbaddr_cti_cpu1_ctidevid1::ApbaddrCtiCpu1Ctidevid1Spec>;
#[doc = "CTI Device ID Register 1"]
pub mod apbaddr_cti_cpu1_ctidevid1;
#[doc = "APBADDR_CTI_CPU1_CTIDEVID (rw) register accessor: CTI Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevid`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVID")]
pub type ApbaddrCtiCpu1Ctidevid = crate::Reg<apbaddr_cti_cpu1_ctidevid::ApbaddrCtiCpu1CtidevidSpec>;
#[doc = "CTI Device ID Register 0"]
pub mod apbaddr_cti_cpu1_ctidevid;
#[doc = "APBADDR_CTI_CPU1_CTIDEVTYPE (rw) register accessor: CTI Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctidevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctidevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctidevtype`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIDEVTYPE")]
pub type ApbaddrCtiCpu1Ctidevtype =
    crate::Reg<apbaddr_cti_cpu1_ctidevtype::ApbaddrCtiCpu1CtidevtypeSpec>;
#[doc = "CTI Device Type Register"]
pub mod apbaddr_cti_cpu1_ctidevtype;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR4 (rw) register accessor: CTI Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR4")]
pub type ApbaddrCtiCpu1Ctipidr4 = crate::Reg<apbaddr_cti_cpu1_ctipidr4::ApbaddrCtiCpu1Ctipidr4Spec>;
#[doc = "CTI Peripheral Identification Register 4"]
pub mod apbaddr_cti_cpu1_ctipidr4;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR5 (rw) register accessor: CTI Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR5")]
pub type ApbaddrCtiCpu1Ctipidr5 = crate::Reg<apbaddr_cti_cpu1_ctipidr5::ApbaddrCtiCpu1Ctipidr5Spec>;
#[doc = "CTI Peripheral Identification Register 5"]
pub mod apbaddr_cti_cpu1_ctipidr5;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR6 (rw) register accessor: CTI Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR6")]
pub type ApbaddrCtiCpu1Ctipidr6 = crate::Reg<apbaddr_cti_cpu1_ctipidr6::ApbaddrCtiCpu1Ctipidr6Spec>;
#[doc = "CTI Peripheral Identification Register 6"]
pub mod apbaddr_cti_cpu1_ctipidr6;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR7 (rw) register accessor: CTI Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR7")]
pub type ApbaddrCtiCpu1Ctipidr7 = crate::Reg<apbaddr_cti_cpu1_ctipidr7::ApbaddrCtiCpu1Ctipidr7Spec>;
#[doc = "CTI Peripheral Identification Register 7"]
pub mod apbaddr_cti_cpu1_ctipidr7;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR0 (rw) register accessor: CTI Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR0")]
pub type ApbaddrCtiCpu1Ctipidr0 = crate::Reg<apbaddr_cti_cpu1_ctipidr0::ApbaddrCtiCpu1Ctipidr0Spec>;
#[doc = "CTI Peripheral Identification Register 0"]
pub mod apbaddr_cti_cpu1_ctipidr0;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR1 (rw) register accessor: CTI Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR1")]
pub type ApbaddrCtiCpu1Ctipidr1 = crate::Reg<apbaddr_cti_cpu1_ctipidr1::ApbaddrCtiCpu1Ctipidr1Spec>;
#[doc = "CTI Peripheral Identification Register 1"]
pub mod apbaddr_cti_cpu1_ctipidr1;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR2 (rw) register accessor: CTI Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR2")]
pub type ApbaddrCtiCpu1Ctipidr2 = crate::Reg<apbaddr_cti_cpu1_ctipidr2::ApbaddrCtiCpu1Ctipidr2Spec>;
#[doc = "CTI Peripheral Identification Register 2"]
pub mod apbaddr_cti_cpu1_ctipidr2;
#[doc = "APBADDR_CTI_CPU1_CTIPIDR3 (rw) register accessor: CTI Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctipidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctipidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_ctipidr3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTIPIDR3")]
pub type ApbaddrCtiCpu1Ctipidr3 = crate::Reg<apbaddr_cti_cpu1_ctipidr3::ApbaddrCtiCpu1Ctipidr3Spec>;
#[doc = "CTI Peripheral Identification Register 3"]
pub mod apbaddr_cti_cpu1_ctipidr3;
#[doc = "APBADDR_CTI_CPU1_CTICIDR0 (rw) register accessor: CTI Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticidr0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICIDR0")]
pub type ApbaddrCtiCpu1Cticidr0 = crate::Reg<apbaddr_cti_cpu1_cticidr0::ApbaddrCtiCpu1Cticidr0Spec>;
#[doc = "CTI Component Identification Register 0"]
pub mod apbaddr_cti_cpu1_cticidr0;
#[doc = "APBADDR_CTI_CPU1_CTICIDR1 (rw) register accessor: CTI Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticidr1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICIDR1")]
pub type ApbaddrCtiCpu1Cticidr1 = crate::Reg<apbaddr_cti_cpu1_cticidr1::ApbaddrCtiCpu1Cticidr1Spec>;
#[doc = "CTI Component Identification Register 1"]
pub mod apbaddr_cti_cpu1_cticidr1;
#[doc = "APBADDR_CTI_CPU1_CTICIDR2 (rw) register accessor: CTI Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticidr2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICIDR2")]
pub type ApbaddrCtiCpu1Cticidr2 = crate::Reg<apbaddr_cti_cpu1_cticidr2::ApbaddrCtiCpu1Cticidr2Spec>;
#[doc = "CTI Component Identification Register 2"]
pub mod apbaddr_cti_cpu1_cticidr2;
#[doc = "APBADDR_CTI_CPU1_CTICIDR3 (rw) register accessor: CTI Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_cticidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_cticidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu1_cticidr3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU1_CTICIDR3")]
pub type ApbaddrCtiCpu1Cticidr3 = crate::Reg<apbaddr_cti_cpu1_cticidr3::ApbaddrCtiCpu1Cticidr3Spec>;
#[doc = "CTI Component Identification Register 3"]
pub mod apbaddr_cti_cpu1_cticidr3;
