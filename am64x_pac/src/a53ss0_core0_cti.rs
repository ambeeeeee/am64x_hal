#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    apbaddr_cti_cpu0_cticontrol: ApbaddrCtiCpu0Cticontrol,
    _reserved1: [u8; 0x0c],
    apbaddr_cti_cpu0_ctiintack: ApbaddrCtiCpu0Ctiintack,
    apbaddr_cti_cpu0_ctiappset: ApbaddrCtiCpu0Ctiappset,
    apbaddr_cti_cpu0_ctiappclear: ApbaddrCtiCpu0Ctiappclear,
    apbaddr_cti_cpu0_ctiapppulse: ApbaddrCtiCpu0Ctiapppulse,
    apbaddr_cti_cpu0_ctiinen0: ApbaddrCtiCpu0Ctiinen0,
    apbaddr_cti_cpu0_ctiinen1: ApbaddrCtiCpu0Ctiinen1,
    apbaddr_cti_cpu0_ctiinen2: ApbaddrCtiCpu0Ctiinen2,
    apbaddr_cti_cpu0_ctiinen3: ApbaddrCtiCpu0Ctiinen3,
    apbaddr_cti_cpu0_ctiinen4: ApbaddrCtiCpu0Ctiinen4,
    apbaddr_cti_cpu0_ctiinen5: ApbaddrCtiCpu0Ctiinen5,
    apbaddr_cti_cpu0_ctiinen6: ApbaddrCtiCpu0Ctiinen6,
    apbaddr_cti_cpu0_ctiinen7: ApbaddrCtiCpu0Ctiinen7,
    _reserved13: [u8; 0x60],
    apbaddr_cti_cpu0_ctiouten0: ApbaddrCtiCpu0Ctiouten0,
    apbaddr_cti_cpu0_ctiouten1: ApbaddrCtiCpu0Ctiouten1,
    apbaddr_cti_cpu0_ctiouten2: ApbaddrCtiCpu0Ctiouten2,
    apbaddr_cti_cpu0_ctiouten3: ApbaddrCtiCpu0Ctiouten3,
    apbaddr_cti_cpu0_ctiouten4: ApbaddrCtiCpu0Ctiouten4,
    apbaddr_cti_cpu0_ctiouten5: ApbaddrCtiCpu0Ctiouten5,
    apbaddr_cti_cpu0_ctiouten6: ApbaddrCtiCpu0Ctiouten6,
    apbaddr_cti_cpu0_ctiouten7: ApbaddrCtiCpu0Ctiouten7,
    _reserved21: [u8; 0x70],
    apbaddr_cti_cpu0_ctitriginstatus: ApbaddrCtiCpu0Ctitriginstatus,
    apbaddr_cti_cpu0_ctitrigoutstatus: ApbaddrCtiCpu0Ctitrigoutstatus,
    apbaddr_cti_cpu0_ctichinstatus: ApbaddrCtiCpu0Ctichinstatus,
    apbaddr_cti_cpu0_ctichoutstatus: ApbaddrCtiCpu0Ctichoutstatus,
    apbaddr_cti_cpu0_ctigate: ApbaddrCtiCpu0Ctigate,
    apbaddr_cti_cpu0_asicctl: ApbaddrCtiCpu0Asicctl,
    _reserved27: [u8; 0x0db8],
    apbaddr_cti_cpu0_ctiitctrl: ApbaddrCtiCpu0Ctiitctrl,
    _reserved28: [u8; 0x9c],
    apbaddr_cti_cpu0_cticlaimset: ApbaddrCtiCpu0Cticlaimset,
    apbaddr_cti_cpu0_cticlaimclr: ApbaddrCtiCpu0Cticlaimclr,
    apbaddr_cti_cpu0_ctidevaff0: ApbaddrCtiCpu0Ctidevaff0,
    apbaddr_cti_cpu0_ctidevaff1: ApbaddrCtiCpu0Ctidevaff1,
    apbaddr_cti_cpu0_ctilar: ApbaddrCtiCpu0Ctilar,
    apbaddr_cti_cpu0_ctilsr: ApbaddrCtiCpu0Ctilsr,
    apbaddr_cti_cpu0_ctiauthstatus: ApbaddrCtiCpu0Ctiauthstatus,
    apbaddr_cti_cpu0_ctidevarch: ApbaddrCtiCpu0Ctidevarch,
    apbaddr_cti_cpu0_ctidevid2: ApbaddrCtiCpu0Ctidevid2,
    apbaddr_cti_cpu0_ctidevid1: ApbaddrCtiCpu0Ctidevid1,
    apbaddr_cti_cpu0_ctidevid: ApbaddrCtiCpu0Ctidevid,
    apbaddr_cti_cpu0_ctidevtype: ApbaddrCtiCpu0Ctidevtype,
    apbaddr_cti_cpu0_ctipidr4: ApbaddrCtiCpu0Ctipidr4,
    apbaddr_cti_cpu0_ctipidr5: ApbaddrCtiCpu0Ctipidr5,
    apbaddr_cti_cpu0_ctipidr6: ApbaddrCtiCpu0Ctipidr6,
    apbaddr_cti_cpu0_ctipidr7: ApbaddrCtiCpu0Ctipidr7,
    apbaddr_cti_cpu0_ctipidr0: ApbaddrCtiCpu0Ctipidr0,
    apbaddr_cti_cpu0_ctipidr1: ApbaddrCtiCpu0Ctipidr1,
    apbaddr_cti_cpu0_ctipidr2: ApbaddrCtiCpu0Ctipidr2,
    apbaddr_cti_cpu0_ctipidr3: ApbaddrCtiCpu0Ctipidr3,
    apbaddr_cti_cpu0_cticidr0: ApbaddrCtiCpu0Cticidr0,
    apbaddr_cti_cpu0_cticidr1: ApbaddrCtiCpu0Cticidr1,
    apbaddr_cti_cpu0_cticidr2: ApbaddrCtiCpu0Cticidr2,
    apbaddr_cti_cpu0_cticidr3: ApbaddrCtiCpu0Cticidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - CTI Control Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticontrol(&self) -> &ApbaddrCtiCpu0Cticontrol {
        &self.apbaddr_cti_cpu0_cticontrol
    }
    #[doc = "0x10 - CTI Output Trigger Acknowledge Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiintack(&self) -> &ApbaddrCtiCpu0Ctiintack {
        &self.apbaddr_cti_cpu0_ctiintack
    }
    #[doc = "0x14 - CTI Application Trigger Set Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiappset(&self) -> &ApbaddrCtiCpu0Ctiappset {
        &self.apbaddr_cti_cpu0_ctiappset
    }
    #[doc = "0x18 - CTI Application Trigger Clear Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiappclear(&self) -> &ApbaddrCtiCpu0Ctiappclear {
        &self.apbaddr_cti_cpu0_ctiappclear
    }
    #[doc = "0x1c - CTI Application Pulse Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiapppulse(&self) -> &ApbaddrCtiCpu0Ctiapppulse {
        &self.apbaddr_cti_cpu0_ctiapppulse
    }
    #[doc = "0x20 - CTI Input Trigger to Output Channel Enable Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen0(&self) -> &ApbaddrCtiCpu0Ctiinen0 {
        &self.apbaddr_cti_cpu0_ctiinen0
    }
    #[doc = "0x24 - CTI Input Trigger to Output Channel Enable Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen1(&self) -> &ApbaddrCtiCpu0Ctiinen1 {
        &self.apbaddr_cti_cpu0_ctiinen1
    }
    #[doc = "0x28 - CTI Input Trigger to Output Channel Enable Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen2(&self) -> &ApbaddrCtiCpu0Ctiinen2 {
        &self.apbaddr_cti_cpu0_ctiinen2
    }
    #[doc = "0x2c - CTI Input Trigger to Output Channel Enable Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen3(&self) -> &ApbaddrCtiCpu0Ctiinen3 {
        &self.apbaddr_cti_cpu0_ctiinen3
    }
    #[doc = "0x30 - CTI Input Trigger to Output Channel Enable Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen4(&self) -> &ApbaddrCtiCpu0Ctiinen4 {
        &self.apbaddr_cti_cpu0_ctiinen4
    }
    #[doc = "0x34 - CTI Input Trigger to Output Channel Enable Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen5(&self) -> &ApbaddrCtiCpu0Ctiinen5 {
        &self.apbaddr_cti_cpu0_ctiinen5
    }
    #[doc = "0x38 - CTI Input Trigger to Output Channel Enable Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen6(&self) -> &ApbaddrCtiCpu0Ctiinen6 {
        &self.apbaddr_cti_cpu0_ctiinen6
    }
    #[doc = "0x3c - CTI Input Trigger to Output Channel Enable Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiinen7(&self) -> &ApbaddrCtiCpu0Ctiinen7 {
        &self.apbaddr_cti_cpu0_ctiinen7
    }
    #[doc = "0xa0 - CTI Input Channel to Output Trigger Enable Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten0(&self) -> &ApbaddrCtiCpu0Ctiouten0 {
        &self.apbaddr_cti_cpu0_ctiouten0
    }
    #[doc = "0xa4 - CTI Input Channel to Output Trigger Enable Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten1(&self) -> &ApbaddrCtiCpu0Ctiouten1 {
        &self.apbaddr_cti_cpu0_ctiouten1
    }
    #[doc = "0xa8 - CTI Input Channel to Output Trigger Enable Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten2(&self) -> &ApbaddrCtiCpu0Ctiouten2 {
        &self.apbaddr_cti_cpu0_ctiouten2
    }
    #[doc = "0xac - CTI Input Channel to Output Trigger Enable Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten3(&self) -> &ApbaddrCtiCpu0Ctiouten3 {
        &self.apbaddr_cti_cpu0_ctiouten3
    }
    #[doc = "0xb0 - CTI Input Channel to Output Trigger Enable Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten4(&self) -> &ApbaddrCtiCpu0Ctiouten4 {
        &self.apbaddr_cti_cpu0_ctiouten4
    }
    #[doc = "0xb4 - CTI Input Channel to Output Trigger Enable Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten5(&self) -> &ApbaddrCtiCpu0Ctiouten5 {
        &self.apbaddr_cti_cpu0_ctiouten5
    }
    #[doc = "0xb8 - CTI Input Channel to Output Trigger Enable Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten6(&self) -> &ApbaddrCtiCpu0Ctiouten6 {
        &self.apbaddr_cti_cpu0_ctiouten6
    }
    #[doc = "0xbc - CTI Input Channel to Output Trigger Enable Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiouten7(&self) -> &ApbaddrCtiCpu0Ctiouten7 {
        &self.apbaddr_cti_cpu0_ctiouten7
    }
    #[doc = "0x130 - CTI Trigger In Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctitriginstatus(&self) -> &ApbaddrCtiCpu0Ctitriginstatus {
        &self.apbaddr_cti_cpu0_ctitriginstatus
    }
    #[doc = "0x134 - CTI Trigger Out Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctitrigoutstatus(&self) -> &ApbaddrCtiCpu0Ctitrigoutstatus {
        &self.apbaddr_cti_cpu0_ctitrigoutstatus
    }
    #[doc = "0x138 - CTI Channel In Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctichinstatus(&self) -> &ApbaddrCtiCpu0Ctichinstatus {
        &self.apbaddr_cti_cpu0_ctichinstatus
    }
    #[doc = "0x13c - CTI Channel Out Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctichoutstatus(&self) -> &ApbaddrCtiCpu0Ctichoutstatus {
        &self.apbaddr_cti_cpu0_ctichoutstatus
    }
    #[doc = "0x140 - CTI Channel Gate Enable Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctigate(&self) -> &ApbaddrCtiCpu0Ctigate {
        &self.apbaddr_cti_cpu0_ctigate
    }
    #[doc = "0x144 - CTI External Multiplexor Control register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_asicctl(&self) -> &ApbaddrCtiCpu0Asicctl {
        &self.apbaddr_cti_cpu0_asicctl
    }
    #[doc = "0xf00 - CTI Integration mode Control Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiitctrl(&self) -> &ApbaddrCtiCpu0Ctiitctrl {
        &self.apbaddr_cti_cpu0_ctiitctrl
    }
    #[doc = "0xfa0 - CTI Claim Set"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticlaimset(&self) -> &ApbaddrCtiCpu0Cticlaimset {
        &self.apbaddr_cti_cpu0_cticlaimset
    }
    #[doc = "0xfa4 - CTI Claim Clear"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticlaimclr(&self) -> &ApbaddrCtiCpu0Cticlaimclr {
        &self.apbaddr_cti_cpu0_cticlaimclr
    }
    #[doc = "0xfa8 - CTI Device Affinity Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevaff0(&self) -> &ApbaddrCtiCpu0Ctidevaff0 {
        &self.apbaddr_cti_cpu0_ctidevaff0
    }
    #[doc = "0xfac - CTI Device Affinity Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevaff1(&self) -> &ApbaddrCtiCpu0Ctidevaff1 {
        &self.apbaddr_cti_cpu0_ctidevaff1
    }
    #[doc = "0xfb0 - CTI Lock Access Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctilar(&self) -> &ApbaddrCtiCpu0Ctilar {
        &self.apbaddr_cti_cpu0_ctilar
    }
    #[doc = "0xfb4 - CTI Lock Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctilsr(&self) -> &ApbaddrCtiCpu0Ctilsr {
        &self.apbaddr_cti_cpu0_ctilsr
    }
    #[doc = "0xfb8 - CTI Authentication Status Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctiauthstatus(&self) -> &ApbaddrCtiCpu0Ctiauthstatus {
        &self.apbaddr_cti_cpu0_ctiauthstatus
    }
    #[doc = "0xfbc - CTI Device Architecture Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevarch(&self) -> &ApbaddrCtiCpu0Ctidevarch {
        &self.apbaddr_cti_cpu0_ctidevarch
    }
    #[doc = "0xfc0 - CTI Device ID Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevid2(&self) -> &ApbaddrCtiCpu0Ctidevid2 {
        &self.apbaddr_cti_cpu0_ctidevid2
    }
    #[doc = "0xfc4 - CTI Device ID Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevid1(&self) -> &ApbaddrCtiCpu0Ctidevid1 {
        &self.apbaddr_cti_cpu0_ctidevid1
    }
    #[doc = "0xfc8 - CTI Device ID Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevid(&self) -> &ApbaddrCtiCpu0Ctidevid {
        &self.apbaddr_cti_cpu0_ctidevid
    }
    #[doc = "0xfcc - CTI Device Type Register"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctidevtype(&self) -> &ApbaddrCtiCpu0Ctidevtype {
        &self.apbaddr_cti_cpu0_ctidevtype
    }
    #[doc = "0xfd0 - CTI Peripheral Identification Register 4"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr4(&self) -> &ApbaddrCtiCpu0Ctipidr4 {
        &self.apbaddr_cti_cpu0_ctipidr4
    }
    #[doc = "0xfd4 - CTI Peripheral Identification Register 5"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr5(&self) -> &ApbaddrCtiCpu0Ctipidr5 {
        &self.apbaddr_cti_cpu0_ctipidr5
    }
    #[doc = "0xfd8 - CTI Peripheral Identification Register 6"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr6(&self) -> &ApbaddrCtiCpu0Ctipidr6 {
        &self.apbaddr_cti_cpu0_ctipidr6
    }
    #[doc = "0xfdc - CTI Peripheral Identification Register 7"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr7(&self) -> &ApbaddrCtiCpu0Ctipidr7 {
        &self.apbaddr_cti_cpu0_ctipidr7
    }
    #[doc = "0xfe0 - CTI Peripheral Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr0(&self) -> &ApbaddrCtiCpu0Ctipidr0 {
        &self.apbaddr_cti_cpu0_ctipidr0
    }
    #[doc = "0xfe4 - CTI Peripheral Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr1(&self) -> &ApbaddrCtiCpu0Ctipidr1 {
        &self.apbaddr_cti_cpu0_ctipidr1
    }
    #[doc = "0xfe8 - CTI Peripheral Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr2(&self) -> &ApbaddrCtiCpu0Ctipidr2 {
        &self.apbaddr_cti_cpu0_ctipidr2
    }
    #[doc = "0xfec - CTI Peripheral Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_ctipidr3(&self) -> &ApbaddrCtiCpu0Ctipidr3 {
        &self.apbaddr_cti_cpu0_ctipidr3
    }
    #[doc = "0xff0 - CTI Component Identification Register 0"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticidr0(&self) -> &ApbaddrCtiCpu0Cticidr0 {
        &self.apbaddr_cti_cpu0_cticidr0
    }
    #[doc = "0xff4 - CTI Component Identification Register 1"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticidr1(&self) -> &ApbaddrCtiCpu0Cticidr1 {
        &self.apbaddr_cti_cpu0_cticidr1
    }
    #[doc = "0xff8 - CTI Component Identification Register 2"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticidr2(&self) -> &ApbaddrCtiCpu0Cticidr2 {
        &self.apbaddr_cti_cpu0_cticidr2
    }
    #[doc = "0xffc - CTI Component Identification Register 3"]
    #[inline(always)]
    pub const fn apbaddr_cti_cpu0_cticidr3(&self) -> &ApbaddrCtiCpu0Cticidr3 {
        &self.apbaddr_cti_cpu0_cticidr3
    }
}
#[doc = "APBADDR_CTI_CPU0_CTICONTROL (rw) register accessor: CTI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticontrol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticontrol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticontrol`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICONTROL")]
pub type ApbaddrCtiCpu0Cticontrol =
    crate::Reg<apbaddr_cti_cpu0_cticontrol::ApbaddrCtiCpu0CticontrolSpec>;
#[doc = "CTI Control Register"]
pub mod apbaddr_cti_cpu0_cticontrol;
#[doc = "APBADDR_CTI_CPU0_CTIINTACK (rw) register accessor: CTI Output Trigger Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiintack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiintack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiintack`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINTACK")]
pub type ApbaddrCtiCpu0Ctiintack =
    crate::Reg<apbaddr_cti_cpu0_ctiintack::ApbaddrCtiCpu0CtiintackSpec>;
#[doc = "CTI Output Trigger Acknowledge Register"]
pub mod apbaddr_cti_cpu0_ctiintack;
#[doc = "APBADDR_CTI_CPU0_CTIAPPSET (rw) register accessor: CTI Application Trigger Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiappset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiappset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiappset`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIAPPSET")]
pub type ApbaddrCtiCpu0Ctiappset =
    crate::Reg<apbaddr_cti_cpu0_ctiappset::ApbaddrCtiCpu0CtiappsetSpec>;
#[doc = "CTI Application Trigger Set Register"]
pub mod apbaddr_cti_cpu0_ctiappset;
#[doc = "APBADDR_CTI_CPU0_CTIAPPCLEAR (rw) register accessor: CTI Application Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiappclear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiappclear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiappclear`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIAPPCLEAR")]
pub type ApbaddrCtiCpu0Ctiappclear =
    crate::Reg<apbaddr_cti_cpu0_ctiappclear::ApbaddrCtiCpu0CtiappclearSpec>;
#[doc = "CTI Application Trigger Clear Register"]
pub mod apbaddr_cti_cpu0_ctiappclear;
#[doc = "APBADDR_CTI_CPU0_CTIAPPPULSE (rw) register accessor: CTI Application Pulse Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiapppulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiapppulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiapppulse`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIAPPPULSE")]
pub type ApbaddrCtiCpu0Ctiapppulse =
    crate::Reg<apbaddr_cti_cpu0_ctiapppulse::ApbaddrCtiCpu0CtiapppulseSpec>;
#[doc = "CTI Application Pulse Register"]
pub mod apbaddr_cti_cpu0_ctiapppulse;
#[doc = "APBADDR_CTI_CPU0_CTIINEN0 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN0")]
pub type ApbaddrCtiCpu0Ctiinen0 = crate::Reg<apbaddr_cti_cpu0_ctiinen0::ApbaddrCtiCpu0Ctiinen0Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 0"]
pub mod apbaddr_cti_cpu0_ctiinen0;
#[doc = "APBADDR_CTI_CPU0_CTIINEN1 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN1")]
pub type ApbaddrCtiCpu0Ctiinen1 = crate::Reg<apbaddr_cti_cpu0_ctiinen1::ApbaddrCtiCpu0Ctiinen1Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 1"]
pub mod apbaddr_cti_cpu0_ctiinen1;
#[doc = "APBADDR_CTI_CPU0_CTIINEN2 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN2")]
pub type ApbaddrCtiCpu0Ctiinen2 = crate::Reg<apbaddr_cti_cpu0_ctiinen2::ApbaddrCtiCpu0Ctiinen2Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 2"]
pub mod apbaddr_cti_cpu0_ctiinen2;
#[doc = "APBADDR_CTI_CPU0_CTIINEN3 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN3")]
pub type ApbaddrCtiCpu0Ctiinen3 = crate::Reg<apbaddr_cti_cpu0_ctiinen3::ApbaddrCtiCpu0Ctiinen3Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 3"]
pub mod apbaddr_cti_cpu0_ctiinen3;
#[doc = "APBADDR_CTI_CPU0_CTIINEN4 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN4")]
pub type ApbaddrCtiCpu0Ctiinen4 = crate::Reg<apbaddr_cti_cpu0_ctiinen4::ApbaddrCtiCpu0Ctiinen4Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 4"]
pub mod apbaddr_cti_cpu0_ctiinen4;
#[doc = "APBADDR_CTI_CPU0_CTIINEN5 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN5")]
pub type ApbaddrCtiCpu0Ctiinen5 = crate::Reg<apbaddr_cti_cpu0_ctiinen5::ApbaddrCtiCpu0Ctiinen5Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 5"]
pub mod apbaddr_cti_cpu0_ctiinen5;
#[doc = "APBADDR_CTI_CPU0_CTIINEN6 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN6")]
pub type ApbaddrCtiCpu0Ctiinen6 = crate::Reg<apbaddr_cti_cpu0_ctiinen6::ApbaddrCtiCpu0Ctiinen6Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 6"]
pub mod apbaddr_cti_cpu0_ctiinen6;
#[doc = "APBADDR_CTI_CPU0_CTIINEN7 (rw) register accessor: CTI Input Trigger to Output Channel Enable Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiinen7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiinen7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiinen7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIINEN7")]
pub type ApbaddrCtiCpu0Ctiinen7 = crate::Reg<apbaddr_cti_cpu0_ctiinen7::ApbaddrCtiCpu0Ctiinen7Spec>;
#[doc = "CTI Input Trigger to Output Channel Enable Register 7"]
pub mod apbaddr_cti_cpu0_ctiinen7;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN0 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN0")]
pub type ApbaddrCtiCpu0Ctiouten0 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten0::ApbaddrCtiCpu0Ctiouten0Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 0"]
pub mod apbaddr_cti_cpu0_ctiouten0;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN1 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN1")]
pub type ApbaddrCtiCpu0Ctiouten1 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten1::ApbaddrCtiCpu0Ctiouten1Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 1"]
pub mod apbaddr_cti_cpu0_ctiouten1;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN2 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN2")]
pub type ApbaddrCtiCpu0Ctiouten2 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten2::ApbaddrCtiCpu0Ctiouten2Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 2"]
pub mod apbaddr_cti_cpu0_ctiouten2;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN3 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN3")]
pub type ApbaddrCtiCpu0Ctiouten3 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten3::ApbaddrCtiCpu0Ctiouten3Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 3"]
pub mod apbaddr_cti_cpu0_ctiouten3;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN4 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN4")]
pub type ApbaddrCtiCpu0Ctiouten4 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten4::ApbaddrCtiCpu0Ctiouten4Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 4"]
pub mod apbaddr_cti_cpu0_ctiouten4;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN5 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN5")]
pub type ApbaddrCtiCpu0Ctiouten5 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten5::ApbaddrCtiCpu0Ctiouten5Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 5"]
pub mod apbaddr_cti_cpu0_ctiouten5;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN6 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN6")]
pub type ApbaddrCtiCpu0Ctiouten6 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten6::ApbaddrCtiCpu0Ctiouten6Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 6"]
pub mod apbaddr_cti_cpu0_ctiouten6;
#[doc = "APBADDR_CTI_CPU0_CTIOUTEN7 (rw) register accessor: CTI Input Channel to Output Trigger Enable Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiouten7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiouten7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiouten7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIOUTEN7")]
pub type ApbaddrCtiCpu0Ctiouten7 =
    crate::Reg<apbaddr_cti_cpu0_ctiouten7::ApbaddrCtiCpu0Ctiouten7Spec>;
#[doc = "CTI Input Channel to Output Trigger Enable Register 7"]
pub mod apbaddr_cti_cpu0_ctiouten7;
#[doc = "APBADDR_CTI_CPU0_CTITRIGINSTATUS (rw) register accessor: CTI Trigger In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctitriginstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctitriginstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctitriginstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTITRIGINSTATUS")]
pub type ApbaddrCtiCpu0Ctitriginstatus =
    crate::Reg<apbaddr_cti_cpu0_ctitriginstatus::ApbaddrCtiCpu0CtitriginstatusSpec>;
#[doc = "CTI Trigger In Status Register"]
pub mod apbaddr_cti_cpu0_ctitriginstatus;
#[doc = "APBADDR_CTI_CPU0_CTITRIGOUTSTATUS (rw) register accessor: CTI Trigger Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctitrigoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctitrigoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctitrigoutstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTITRIGOUTSTATUS")]
pub type ApbaddrCtiCpu0Ctitrigoutstatus =
    crate::Reg<apbaddr_cti_cpu0_ctitrigoutstatus::ApbaddrCtiCpu0CtitrigoutstatusSpec>;
#[doc = "CTI Trigger Out Status Register"]
pub mod apbaddr_cti_cpu0_ctitrigoutstatus;
#[doc = "APBADDR_CTI_CPU0_CTICHINSTATUS (rw) register accessor: CTI Channel In Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctichinstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctichinstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctichinstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICHINSTATUS")]
pub type ApbaddrCtiCpu0Ctichinstatus =
    crate::Reg<apbaddr_cti_cpu0_ctichinstatus::ApbaddrCtiCpu0CtichinstatusSpec>;
#[doc = "CTI Channel In Status Register"]
pub mod apbaddr_cti_cpu0_ctichinstatus;
#[doc = "APBADDR_CTI_CPU0_CTICHOUTSTATUS (rw) register accessor: CTI Channel Out Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctichoutstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctichoutstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctichoutstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICHOUTSTATUS")]
pub type ApbaddrCtiCpu0Ctichoutstatus =
    crate::Reg<apbaddr_cti_cpu0_ctichoutstatus::ApbaddrCtiCpu0CtichoutstatusSpec>;
#[doc = "CTI Channel Out Status Register"]
pub mod apbaddr_cti_cpu0_ctichoutstatus;
#[doc = "APBADDR_CTI_CPU0_CTIGATE (rw) register accessor: CTI Channel Gate Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctigate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctigate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctigate`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIGATE")]
pub type ApbaddrCtiCpu0Ctigate = crate::Reg<apbaddr_cti_cpu0_ctigate::ApbaddrCtiCpu0CtigateSpec>;
#[doc = "CTI Channel Gate Enable Register"]
pub mod apbaddr_cti_cpu0_ctigate;
#[doc = "APBADDR_CTI_CPU0_ASICCTL (rw) register accessor: CTI External Multiplexor Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_asicctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_asicctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_asicctl`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_ASICCTL")]
pub type ApbaddrCtiCpu0Asicctl = crate::Reg<apbaddr_cti_cpu0_asicctl::ApbaddrCtiCpu0AsicctlSpec>;
#[doc = "CTI External Multiplexor Control register"]
pub mod apbaddr_cti_cpu0_asicctl;
#[doc = "APBADDR_CTI_CPU0_CTIITCTRL (rw) register accessor: CTI Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiitctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiitctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiitctrl`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIITCTRL")]
pub type ApbaddrCtiCpu0Ctiitctrl =
    crate::Reg<apbaddr_cti_cpu0_ctiitctrl::ApbaddrCtiCpu0CtiitctrlSpec>;
#[doc = "CTI Integration mode Control Register"]
pub mod apbaddr_cti_cpu0_ctiitctrl;
#[doc = "APBADDR_CTI_CPU0_CTICLAIMSET (rw) register accessor: CTI Claim Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticlaimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticlaimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticlaimset`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICLAIMSET")]
pub type ApbaddrCtiCpu0Cticlaimset =
    crate::Reg<apbaddr_cti_cpu0_cticlaimset::ApbaddrCtiCpu0CticlaimsetSpec>;
#[doc = "CTI Claim Set"]
pub mod apbaddr_cti_cpu0_cticlaimset;
#[doc = "APBADDR_CTI_CPU0_CTICLAIMCLR (rw) register accessor: CTI Claim Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticlaimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticlaimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticlaimclr`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICLAIMCLR")]
pub type ApbaddrCtiCpu0Cticlaimclr =
    crate::Reg<apbaddr_cti_cpu0_cticlaimclr::ApbaddrCtiCpu0CticlaimclrSpec>;
#[doc = "CTI Claim Clear"]
pub mod apbaddr_cti_cpu0_cticlaimclr;
#[doc = "APBADDR_CTI_CPU0_CTIDEVAFF0 (rw) register accessor: CTI Device Affinity Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevaff0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevaff0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevaff0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVAFF0")]
pub type ApbaddrCtiCpu0Ctidevaff0 =
    crate::Reg<apbaddr_cti_cpu0_ctidevaff0::ApbaddrCtiCpu0Ctidevaff0Spec>;
#[doc = "CTI Device Affinity Register 0"]
pub mod apbaddr_cti_cpu0_ctidevaff0;
#[doc = "APBADDR_CTI_CPU0_CTIDEVAFF1 (rw) register accessor: CTI Device Affinity Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevaff1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevaff1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevaff1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVAFF1")]
pub type ApbaddrCtiCpu0Ctidevaff1 =
    crate::Reg<apbaddr_cti_cpu0_ctidevaff1::ApbaddrCtiCpu0Ctidevaff1Spec>;
#[doc = "CTI Device Affinity Register 1"]
pub mod apbaddr_cti_cpu0_ctidevaff1;
#[doc = "APBADDR_CTI_CPU0_CTILAR (rw) register accessor: CTI Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctilar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctilar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctilar`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTILAR")]
pub type ApbaddrCtiCpu0Ctilar = crate::Reg<apbaddr_cti_cpu0_ctilar::ApbaddrCtiCpu0CtilarSpec>;
#[doc = "CTI Lock Access Register"]
pub mod apbaddr_cti_cpu0_ctilar;
#[doc = "APBADDR_CTI_CPU0_CTILSR (rw) register accessor: CTI Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctilsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctilsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctilsr`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTILSR")]
pub type ApbaddrCtiCpu0Ctilsr = crate::Reg<apbaddr_cti_cpu0_ctilsr::ApbaddrCtiCpu0CtilsrSpec>;
#[doc = "CTI Lock Status Register"]
pub mod apbaddr_cti_cpu0_ctilsr;
#[doc = "APBADDR_CTI_CPU0_CTIAUTHSTATUS (rw) register accessor: CTI Authentication Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiauthstatus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiauthstatus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctiauthstatus`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIAUTHSTATUS")]
pub type ApbaddrCtiCpu0Ctiauthstatus =
    crate::Reg<apbaddr_cti_cpu0_ctiauthstatus::ApbaddrCtiCpu0CtiauthstatusSpec>;
#[doc = "CTI Authentication Status Register"]
pub mod apbaddr_cti_cpu0_ctiauthstatus;
#[doc = "APBADDR_CTI_CPU0_CTIDEVARCH (rw) register accessor: CTI Device Architecture Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevarch`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVARCH")]
pub type ApbaddrCtiCpu0Ctidevarch =
    crate::Reg<apbaddr_cti_cpu0_ctidevarch::ApbaddrCtiCpu0CtidevarchSpec>;
#[doc = "CTI Device Architecture Register"]
pub mod apbaddr_cti_cpu0_ctidevarch;
#[doc = "APBADDR_CTI_CPU0_CTIDEVID2 (rw) register accessor: CTI Device ID Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevid2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevid2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevid2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVID2")]
pub type ApbaddrCtiCpu0Ctidevid2 =
    crate::Reg<apbaddr_cti_cpu0_ctidevid2::ApbaddrCtiCpu0Ctidevid2Spec>;
#[doc = "CTI Device ID Register 2"]
pub mod apbaddr_cti_cpu0_ctidevid2;
#[doc = "APBADDR_CTI_CPU0_CTIDEVID1 (rw) register accessor: CTI Device ID Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevid1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevid1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevid1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVID1")]
pub type ApbaddrCtiCpu0Ctidevid1 =
    crate::Reg<apbaddr_cti_cpu0_ctidevid1::ApbaddrCtiCpu0Ctidevid1Spec>;
#[doc = "CTI Device ID Register 1"]
pub mod apbaddr_cti_cpu0_ctidevid1;
#[doc = "APBADDR_CTI_CPU0_CTIDEVID (rw) register accessor: CTI Device ID Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevid`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVID")]
pub type ApbaddrCtiCpu0Ctidevid = crate::Reg<apbaddr_cti_cpu0_ctidevid::ApbaddrCtiCpu0CtidevidSpec>;
#[doc = "CTI Device ID Register 0"]
pub mod apbaddr_cti_cpu0_ctidevid;
#[doc = "APBADDR_CTI_CPU0_CTIDEVTYPE (rw) register accessor: CTI Device Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctidevtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctidevtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctidevtype`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIDEVTYPE")]
pub type ApbaddrCtiCpu0Ctidevtype =
    crate::Reg<apbaddr_cti_cpu0_ctidevtype::ApbaddrCtiCpu0CtidevtypeSpec>;
#[doc = "CTI Device Type Register"]
pub mod apbaddr_cti_cpu0_ctidevtype;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR4 (rw) register accessor: CTI Peripheral Identification Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr4`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR4")]
pub type ApbaddrCtiCpu0Ctipidr4 = crate::Reg<apbaddr_cti_cpu0_ctipidr4::ApbaddrCtiCpu0Ctipidr4Spec>;
#[doc = "CTI Peripheral Identification Register 4"]
pub mod apbaddr_cti_cpu0_ctipidr4;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR5 (rw) register accessor: CTI Peripheral Identification Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr5`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR5")]
pub type ApbaddrCtiCpu0Ctipidr5 = crate::Reg<apbaddr_cti_cpu0_ctipidr5::ApbaddrCtiCpu0Ctipidr5Spec>;
#[doc = "CTI Peripheral Identification Register 5"]
pub mod apbaddr_cti_cpu0_ctipidr5;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR6 (rw) register accessor: CTI Peripheral Identification Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr6`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR6")]
pub type ApbaddrCtiCpu0Ctipidr6 = crate::Reg<apbaddr_cti_cpu0_ctipidr6::ApbaddrCtiCpu0Ctipidr6Spec>;
#[doc = "CTI Peripheral Identification Register 6"]
pub mod apbaddr_cti_cpu0_ctipidr6;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR7 (rw) register accessor: CTI Peripheral Identification Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr7`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR7")]
pub type ApbaddrCtiCpu0Ctipidr7 = crate::Reg<apbaddr_cti_cpu0_ctipidr7::ApbaddrCtiCpu0Ctipidr7Spec>;
#[doc = "CTI Peripheral Identification Register 7"]
pub mod apbaddr_cti_cpu0_ctipidr7;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR0 (rw) register accessor: CTI Peripheral Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR0")]
pub type ApbaddrCtiCpu0Ctipidr0 = crate::Reg<apbaddr_cti_cpu0_ctipidr0::ApbaddrCtiCpu0Ctipidr0Spec>;
#[doc = "CTI Peripheral Identification Register 0"]
pub mod apbaddr_cti_cpu0_ctipidr0;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR1 (rw) register accessor: CTI Peripheral Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR1")]
pub type ApbaddrCtiCpu0Ctipidr1 = crate::Reg<apbaddr_cti_cpu0_ctipidr1::ApbaddrCtiCpu0Ctipidr1Spec>;
#[doc = "CTI Peripheral Identification Register 1"]
pub mod apbaddr_cti_cpu0_ctipidr1;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR2 (rw) register accessor: CTI Peripheral Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR2")]
pub type ApbaddrCtiCpu0Ctipidr2 = crate::Reg<apbaddr_cti_cpu0_ctipidr2::ApbaddrCtiCpu0Ctipidr2Spec>;
#[doc = "CTI Peripheral Identification Register 2"]
pub mod apbaddr_cti_cpu0_ctipidr2;
#[doc = "APBADDR_CTI_CPU0_CTIPIDR3 (rw) register accessor: CTI Peripheral Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctipidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctipidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_ctipidr3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTIPIDR3")]
pub type ApbaddrCtiCpu0Ctipidr3 = crate::Reg<apbaddr_cti_cpu0_ctipidr3::ApbaddrCtiCpu0Ctipidr3Spec>;
#[doc = "CTI Peripheral Identification Register 3"]
pub mod apbaddr_cti_cpu0_ctipidr3;
#[doc = "APBADDR_CTI_CPU0_CTICIDR0 (rw) register accessor: CTI Component Identification Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticidr0`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICIDR0")]
pub type ApbaddrCtiCpu0Cticidr0 = crate::Reg<apbaddr_cti_cpu0_cticidr0::ApbaddrCtiCpu0Cticidr0Spec>;
#[doc = "CTI Component Identification Register 0"]
pub mod apbaddr_cti_cpu0_cticidr0;
#[doc = "APBADDR_CTI_CPU0_CTICIDR1 (rw) register accessor: CTI Component Identification Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticidr1`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICIDR1")]
pub type ApbaddrCtiCpu0Cticidr1 = crate::Reg<apbaddr_cti_cpu0_cticidr1::ApbaddrCtiCpu0Cticidr1Spec>;
#[doc = "CTI Component Identification Register 1"]
pub mod apbaddr_cti_cpu0_cticidr1;
#[doc = "APBADDR_CTI_CPU0_CTICIDR2 (rw) register accessor: CTI Component Identification Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticidr2`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICIDR2")]
pub type ApbaddrCtiCpu0Cticidr2 = crate::Reg<apbaddr_cti_cpu0_cticidr2::ApbaddrCtiCpu0Cticidr2Spec>;
#[doc = "CTI Component Identification Register 2"]
pub mod apbaddr_cti_cpu0_cticidr2;
#[doc = "APBADDR_CTI_CPU0_CTICIDR3 (rw) register accessor: CTI Component Identification Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbaddr_cti_cpu0_cticidr3`]
module"]
#[doc(alias = "APBADDR_CTI_CPU0_CTICIDR3")]
pub type ApbaddrCtiCpu0Cticidr3 = crate::Reg<apbaddr_cti_cpu0_cticidr3::ApbaddrCtiCpu0Cticidr3Spec>;
#[doc = "CTI Component Identification Register 3"]
pub mod apbaddr_cti_cpu0_cticidr3;
