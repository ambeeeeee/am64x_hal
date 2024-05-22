#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    epwm_regs_tbctl: EpwmRegsTbctl,
    epwm_regs_tbsts: EpwmRegsTbsts,
    epwm_regs_tbphshr: EpwmRegsTbphshr,
    epwm_regs_tbphs: EpwmRegsTbphs,
    epwm_regs_tbcnt: EpwmRegsTbcnt,
    epwm_regs_tbprd: EpwmRegsTbprd,
    _reserved6: [u8; 0x02],
    epwm_regs_cmpctl: EpwmRegsCmpctl,
    epwm_regs_cmpahr: EpwmRegsCmpahr,
    epwm_regs_cmpa: EpwmRegsCmpa,
    epwm_regs_cmpb: EpwmRegsCmpb,
    epwm_regs_aqctla: EpwmRegsAqctla,
    epwm_regs_aqctlb: EpwmRegsAqctlb,
    epwm_regs_aqsfrc: EpwmRegsAqsfrc,
    epwm_regs_aqcsfrc: EpwmRegsAqcsfrc,
    epwm_regs_dbctl: EpwmRegsDbctl,
    epwm_regs_dbred: EpwmRegsDbred,
    epwm_regs_dbfed: EpwmRegsDbfed,
    epwm_regs_tzsel: EpwmRegsTzsel,
    _reserved18: [u8; 0x02],
    epwm_regs_tzctl: EpwmRegsTzctl,
    epwm_regs_tzeint: EpwmRegsTzeint,
    epwm_regs_tzflg: EpwmRegsTzflg,
    epwm_regs_tzclr: EpwmRegsTzclr,
    epwm_regs_tzfrc: EpwmRegsTzfrc,
    epwm_regs_etsel: EpwmRegsEtsel,
    epwm_regs_etps: EpwmRegsEtps,
    epwm_regs_etflg: EpwmRegsEtflg,
    epwm_regs_etclr: EpwmRegsEtclr,
    epwm_regs_etfrc: EpwmRegsEtfrc,
    epwm_regs_pcctl: EpwmRegsPcctl,
    _reserved29: [u8; 0x1e],
    epwm_regs_pid: EpwmRegsPid,
}
impl RegisterBlock {
    #[doc = "0x00 - Time-Base Control Register"]
    #[inline(always)]
    pub const fn epwm_regs_tbctl(&self) -> &EpwmRegsTbctl {
        &self.epwm_regs_tbctl
    }
    #[doc = "0x02 - Time-Base Status Register"]
    #[inline(always)]
    pub const fn epwm_regs_tbsts(&self) -> &EpwmRegsTbsts {
        &self.epwm_regs_tbsts
    }
    #[doc = "0x04 - Time Base Phase High Resolution Register"]
    #[inline(always)]
    pub const fn epwm_regs_tbphshr(&self) -> &EpwmRegsTbphshr {
        &self.epwm_regs_tbphshr
    }
    #[doc = "0x06 - Time Base Phase Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension, otherwise, this location is reserved."]
    #[inline(always)]
    pub const fn epwm_regs_tbphs(&self) -> &EpwmRegsTbphs {
        &self.epwm_regs_tbphs
    }
    #[doc = "0x08 - Time Base Counter Register"]
    #[inline(always)]
    pub const fn epwm_regs_tbcnt(&self) -> &EpwmRegsTbcnt {
        &self.epwm_regs_tbcnt
    }
    #[doc = "0x0a - Time Base Period Register"]
    #[inline(always)]
    pub const fn epwm_regs_tbprd(&self) -> &EpwmRegsTbprd {
        &self.epwm_regs_tbprd
    }
    #[doc = "0x0e - Counter Compare Control Register"]
    #[inline(always)]
    pub const fn epwm_regs_cmpctl(&self) -> &EpwmRegsCmpctl {
        &self.epwm_regs_cmpctl
    }
    #[doc = "0x10 - Counter Compare A High Resolution Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension; otherwise, this location is reserved."]
    #[inline(always)]
    pub const fn epwm_regs_cmpahr(&self) -> &EpwmRegsCmpahr {
        &self.epwm_regs_cmpahr
    }
    #[doc = "0x12 - Counter Compare A Register"]
    #[inline(always)]
    pub const fn epwm_regs_cmpa(&self) -> &EpwmRegsCmpa {
        &self.epwm_regs_cmpa
    }
    #[doc = "0x14 - Counter Compare B Register"]
    #[inline(always)]
    pub const fn epwm_regs_cmpb(&self) -> &EpwmRegsCmpb {
        &self.epwm_regs_cmpb
    }
    #[doc = "0x16 - Action Qualifier Control Register For Output A"]
    #[inline(always)]
    pub const fn epwm_regs_aqctla(&self) -> &EpwmRegsAqctla {
        &self.epwm_regs_aqctla
    }
    #[doc = "0x18 - Action Qualifier Control Register For Output B"]
    #[inline(always)]
    pub const fn epwm_regs_aqctlb(&self) -> &EpwmRegsAqctlb {
        &self.epwm_regs_aqctlb
    }
    #[doc = "0x1a - Action Qualifier Software Force Register"]
    #[inline(always)]
    pub const fn epwm_regs_aqsfrc(&self) -> &EpwmRegsAqsfrc {
        &self.epwm_regs_aqsfrc
    }
    #[doc = "0x1c - Action Qualifier Continuous S/W Force Register"]
    #[inline(always)]
    pub const fn epwm_regs_aqcsfrc(&self) -> &EpwmRegsAqcsfrc {
        &self.epwm_regs_aqcsfrc
    }
    #[doc = "0x1e - Dead-Band Generator Control Register"]
    #[inline(always)]
    pub const fn epwm_regs_dbctl(&self) -> &EpwmRegsDbctl {
        &self.epwm_regs_dbctl
    }
    #[doc = "0x20 - Dead-Band Generator Rising Edge Delay Count Register"]
    #[inline(always)]
    pub const fn epwm_regs_dbred(&self) -> &EpwmRegsDbred {
        &self.epwm_regs_dbred
    }
    #[doc = "0x22 - Dead-Band Generator Falling Edge Delay Count Register"]
    #[inline(always)]
    pub const fn epwm_regs_dbfed(&self) -> &EpwmRegsDbfed {
        &self.epwm_regs_dbfed
    }
    #[doc = "0x24 - Trip Zone Select Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzsel(&self) -> &EpwmRegsTzsel {
        &self.epwm_regs_tzsel
    }
    #[doc = "0x28 - Trip Zone Control Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzctl(&self) -> &EpwmRegsTzctl {
        &self.epwm_regs_tzctl
    }
    #[doc = "0x2a - Trip Zone Enable Interrupt Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzeint(&self) -> &EpwmRegsTzeint {
        &self.epwm_regs_tzeint
    }
    #[doc = "0x2c - Trip Zone Flag Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzflg(&self) -> &EpwmRegsTzflg {
        &self.epwm_regs_tzflg
    }
    #[doc = "0x2e - Trip Zone Clear Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzclr(&self) -> &EpwmRegsTzclr {
        &self.epwm_regs_tzclr
    }
    #[doc = "0x30 - Trip Zone Force Register"]
    #[inline(always)]
    pub const fn epwm_regs_tzfrc(&self) -> &EpwmRegsTzfrc {
        &self.epwm_regs_tzfrc
    }
    #[doc = "0x32 - Event Trigger Selection Register"]
    #[inline(always)]
    pub const fn epwm_regs_etsel(&self) -> &EpwmRegsEtsel {
        &self.epwm_regs_etsel
    }
    #[doc = "0x34 - Event Trigger Pre-Scale Register"]
    #[inline(always)]
    pub const fn epwm_regs_etps(&self) -> &EpwmRegsEtps {
        &self.epwm_regs_etps
    }
    #[doc = "0x36 - Event Trigger Flag Register"]
    #[inline(always)]
    pub const fn epwm_regs_etflg(&self) -> &EpwmRegsEtflg {
        &self.epwm_regs_etflg
    }
    #[doc = "0x38 - Event Trigger Clear Register"]
    #[inline(always)]
    pub const fn epwm_regs_etclr(&self) -> &EpwmRegsEtclr {
        &self.epwm_regs_etclr
    }
    #[doc = "0x3a - Event Trigger Force Register"]
    #[inline(always)]
    pub const fn epwm_regs_etfrc(&self) -> &EpwmRegsEtfrc {
        &self.epwm_regs_etfrc
    }
    #[doc = "0x3c - PWM Chopper Control Register"]
    #[inline(always)]
    pub const fn epwm_regs_pcctl(&self) -> &EpwmRegsPcctl {
        &self.epwm_regs_pcctl
    }
    #[doc = "0x5c - EHRPWM Peripheral ID Register. The IP revision register is used by software to track features, bugs, and compatibility."]
    #[inline(always)]
    pub const fn epwm_regs_pid(&self) -> &EpwmRegsPid {
        &self.epwm_regs_pid
    }
}
#[doc = "EPWM_REGS_TBCTL (rw) register accessor: Time-Base Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbctl`]
module"]
#[doc(alias = "EPWM_REGS_TBCTL")]
pub type EpwmRegsTbctl = crate::Reg<epwm_regs_tbctl::EpwmRegsTbctlSpec>;
#[doc = "Time-Base Control Register"]
pub mod epwm_regs_tbctl;
#[doc = "EPWM_REGS_TBSTS (rw) register accessor: Time-Base Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbsts`]
module"]
#[doc(alias = "EPWM_REGS_TBSTS")]
pub type EpwmRegsTbsts = crate::Reg<epwm_regs_tbsts::EpwmRegsTbstsSpec>;
#[doc = "Time-Base Status Register"]
pub mod epwm_regs_tbsts;
#[doc = "EPWM_REGS_TBPHSHR (rw) register accessor: Time Base Phase High Resolution Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbphshr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbphshr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbphshr`]
module"]
#[doc(alias = "EPWM_REGS_TBPHSHR")]
pub type EpwmRegsTbphshr = crate::Reg<epwm_regs_tbphshr::EpwmRegsTbphshrSpec>;
#[doc = "Time Base Phase High Resolution Register"]
pub mod epwm_regs_tbphshr;
#[doc = "EPWM_REGS_TBPHS (rw) register accessor: Time Base Phase Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension, otherwise, this location is reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbphs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbphs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbphs`]
module"]
#[doc(alias = "EPWM_REGS_TBPHS")]
pub type EpwmRegsTbphs = crate::Reg<epwm_regs_tbphs::EpwmRegsTbphsSpec>;
#[doc = "Time Base Phase Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension, otherwise, this location is reserved."]
pub mod epwm_regs_tbphs;
#[doc = "EPWM_REGS_TBCNT (rw) register accessor: Time Base Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbcnt`]
module"]
#[doc(alias = "EPWM_REGS_TBCNT")]
pub type EpwmRegsTbcnt = crate::Reg<epwm_regs_tbcnt::EpwmRegsTbcntSpec>;
#[doc = "Time Base Counter Register"]
pub mod epwm_regs_tbcnt;
#[doc = "EPWM_REGS_TBPRD (rw) register accessor: Time Base Period Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tbprd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tbprd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tbprd`]
module"]
#[doc(alias = "EPWM_REGS_TBPRD")]
pub type EpwmRegsTbprd = crate::Reg<epwm_regs_tbprd::EpwmRegsTbprdSpec>;
#[doc = "Time Base Period Register"]
pub mod epwm_regs_tbprd;
#[doc = "EPWM_REGS_CMPCTL (rw) register accessor: Counter Compare Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_cmpctl`]
module"]
#[doc(alias = "EPWM_REGS_CMPCTL")]
pub type EpwmRegsCmpctl = crate::Reg<epwm_regs_cmpctl::EpwmRegsCmpctlSpec>;
#[doc = "Counter Compare Control Register"]
pub mod epwm_regs_cmpctl;
#[doc = "EPWM_REGS_CMPAHR (rw) register accessor: Counter Compare A High Resolution Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension; otherwise, this location is reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpahr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpahr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_cmpahr`]
module"]
#[doc(alias = "EPWM_REGS_CMPAHR")]
pub type EpwmRegsCmpahr = crate::Reg<epwm_regs_cmpahr::EpwmRegsCmpahrSpec>;
#[doc = "Counter Compare A High Resolution Register. This register is only available on ePWM instances that include the high-resolution PWM (HRPWM) extension; otherwise, this location is reserved."]
pub mod epwm_regs_cmpahr;
#[doc = "EPWM_REGS_CMPA (rw) register accessor: Counter Compare A Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_cmpa`]
module"]
#[doc(alias = "EPWM_REGS_CMPA")]
pub type EpwmRegsCmpa = crate::Reg<epwm_regs_cmpa::EpwmRegsCmpaSpec>;
#[doc = "Counter Compare A Register"]
pub mod epwm_regs_cmpa;
#[doc = "EPWM_REGS_CMPB (rw) register accessor: Counter Compare B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_cmpb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_cmpb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_cmpb`]
module"]
#[doc(alias = "EPWM_REGS_CMPB")]
pub type EpwmRegsCmpb = crate::Reg<epwm_regs_cmpb::EpwmRegsCmpbSpec>;
#[doc = "Counter Compare B Register"]
pub mod epwm_regs_cmpb;
#[doc = "EPWM_REGS_AQCTLA (rw) register accessor: Action Qualifier Control Register For Output A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqctla::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqctla::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_aqctla`]
module"]
#[doc(alias = "EPWM_REGS_AQCTLA")]
pub type EpwmRegsAqctla = crate::Reg<epwm_regs_aqctla::EpwmRegsAqctlaSpec>;
#[doc = "Action Qualifier Control Register For Output A"]
pub mod epwm_regs_aqctla;
#[doc = "EPWM_REGS_AQCTLB (rw) register accessor: Action Qualifier Control Register For Output B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqctlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqctlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_aqctlb`]
module"]
#[doc(alias = "EPWM_REGS_AQCTLB")]
pub type EpwmRegsAqctlb = crate::Reg<epwm_regs_aqctlb::EpwmRegsAqctlbSpec>;
#[doc = "Action Qualifier Control Register For Output B"]
pub mod epwm_regs_aqctlb;
#[doc = "EPWM_REGS_AQSFRC (rw) register accessor: Action Qualifier Software Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqsfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqsfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_aqsfrc`]
module"]
#[doc(alias = "EPWM_REGS_AQSFRC")]
pub type EpwmRegsAqsfrc = crate::Reg<epwm_regs_aqsfrc::EpwmRegsAqsfrcSpec>;
#[doc = "Action Qualifier Software Force Register"]
pub mod epwm_regs_aqsfrc;
#[doc = "EPWM_REGS_AQCSFRC (rw) register accessor: Action Qualifier Continuous S/W Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_aqcsfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_aqcsfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_aqcsfrc`]
module"]
#[doc(alias = "EPWM_REGS_AQCSFRC")]
pub type EpwmRegsAqcsfrc = crate::Reg<epwm_regs_aqcsfrc::EpwmRegsAqcsfrcSpec>;
#[doc = "Action Qualifier Continuous S/W Force Register"]
pub mod epwm_regs_aqcsfrc;
#[doc = "EPWM_REGS_DBCTL (rw) register accessor: Dead-Band Generator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_dbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_dbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_dbctl`]
module"]
#[doc(alias = "EPWM_REGS_DBCTL")]
pub type EpwmRegsDbctl = crate::Reg<epwm_regs_dbctl::EpwmRegsDbctlSpec>;
#[doc = "Dead-Band Generator Control Register"]
pub mod epwm_regs_dbctl;
#[doc = "EPWM_REGS_DBRED (rw) register accessor: Dead-Band Generator Rising Edge Delay Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_dbred::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_dbred::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_dbred`]
module"]
#[doc(alias = "EPWM_REGS_DBRED")]
pub type EpwmRegsDbred = crate::Reg<epwm_regs_dbred::EpwmRegsDbredSpec>;
#[doc = "Dead-Band Generator Rising Edge Delay Count Register"]
pub mod epwm_regs_dbred;
#[doc = "EPWM_REGS_DBFED (rw) register accessor: Dead-Band Generator Falling Edge Delay Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_dbfed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_dbfed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_dbfed`]
module"]
#[doc(alias = "EPWM_REGS_DBFED")]
pub type EpwmRegsDbfed = crate::Reg<epwm_regs_dbfed::EpwmRegsDbfedSpec>;
#[doc = "Dead-Band Generator Falling Edge Delay Count Register"]
pub mod epwm_regs_dbfed;
#[doc = "EPWM_REGS_TZSEL (rw) register accessor: Trip Zone Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzsel`]
module"]
#[doc(alias = "EPWM_REGS_TZSEL")]
pub type EpwmRegsTzsel = crate::Reg<epwm_regs_tzsel::EpwmRegsTzselSpec>;
#[doc = "Trip Zone Select Register"]
pub mod epwm_regs_tzsel;
#[doc = "EPWM_REGS_TZCTL (rw) register accessor: Trip Zone Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzctl`]
module"]
#[doc(alias = "EPWM_REGS_TZCTL")]
pub type EpwmRegsTzctl = crate::Reg<epwm_regs_tzctl::EpwmRegsTzctlSpec>;
#[doc = "Trip Zone Control Register"]
pub mod epwm_regs_tzctl;
#[doc = "EPWM_REGS_TZEINT (rw) register accessor: Trip Zone Enable Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzeint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzeint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzeint`]
module"]
#[doc(alias = "EPWM_REGS_TZEINT")]
pub type EpwmRegsTzeint = crate::Reg<epwm_regs_tzeint::EpwmRegsTzeintSpec>;
#[doc = "Trip Zone Enable Interrupt Register"]
pub mod epwm_regs_tzeint;
#[doc = "EPWM_REGS_TZFLG (rw) register accessor: Trip Zone Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzflg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzflg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzflg`]
module"]
#[doc(alias = "EPWM_REGS_TZFLG")]
pub type EpwmRegsTzflg = crate::Reg<epwm_regs_tzflg::EpwmRegsTzflgSpec>;
#[doc = "Trip Zone Flag Register"]
pub mod epwm_regs_tzflg;
#[doc = "EPWM_REGS_TZCLR (rw) register accessor: Trip Zone Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzclr`]
module"]
#[doc(alias = "EPWM_REGS_TZCLR")]
pub type EpwmRegsTzclr = crate::Reg<epwm_regs_tzclr::EpwmRegsTzclrSpec>;
#[doc = "Trip Zone Clear Register"]
pub mod epwm_regs_tzclr;
#[doc = "EPWM_REGS_TZFRC (rw) register accessor: Trip Zone Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_tzfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_tzfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_tzfrc`]
module"]
#[doc(alias = "EPWM_REGS_TZFRC")]
pub type EpwmRegsTzfrc = crate::Reg<epwm_regs_tzfrc::EpwmRegsTzfrcSpec>;
#[doc = "Trip Zone Force Register"]
pub mod epwm_regs_tzfrc;
#[doc = "EPWM_REGS_ETSEL (rw) register accessor: Event Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_etsel`]
module"]
#[doc(alias = "EPWM_REGS_ETSEL")]
pub type EpwmRegsEtsel = crate::Reg<epwm_regs_etsel::EpwmRegsEtselSpec>;
#[doc = "Event Trigger Selection Register"]
pub mod epwm_regs_etsel;
#[doc = "EPWM_REGS_ETPS (rw) register accessor: Event Trigger Pre-Scale Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_etps`]
module"]
#[doc(alias = "EPWM_REGS_ETPS")]
pub type EpwmRegsEtps = crate::Reg<epwm_regs_etps::EpwmRegsEtpsSpec>;
#[doc = "Event Trigger Pre-Scale Register"]
pub mod epwm_regs_etps;
#[doc = "EPWM_REGS_ETFLG (rw) register accessor: Event Trigger Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etflg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etflg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_etflg`]
module"]
#[doc(alias = "EPWM_REGS_ETFLG")]
pub type EpwmRegsEtflg = crate::Reg<epwm_regs_etflg::EpwmRegsEtflgSpec>;
#[doc = "Event Trigger Flag Register"]
pub mod epwm_regs_etflg;
#[doc = "EPWM_REGS_ETCLR (rw) register accessor: Event Trigger Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_etclr`]
module"]
#[doc(alias = "EPWM_REGS_ETCLR")]
pub type EpwmRegsEtclr = crate::Reg<epwm_regs_etclr::EpwmRegsEtclrSpec>;
#[doc = "Event Trigger Clear Register"]
pub mod epwm_regs_etclr;
#[doc = "EPWM_REGS_ETFRC (rw) register accessor: Event Trigger Force Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_etfrc`]
module"]
#[doc(alias = "EPWM_REGS_ETFRC")]
pub type EpwmRegsEtfrc = crate::Reg<epwm_regs_etfrc::EpwmRegsEtfrcSpec>;
#[doc = "Event Trigger Force Register"]
pub mod epwm_regs_etfrc;
#[doc = "EPWM_REGS_PCCTL (rw) register accessor: PWM Chopper Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_pcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_pcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_pcctl`]
module"]
#[doc(alias = "EPWM_REGS_PCCTL")]
pub type EpwmRegsPcctl = crate::Reg<epwm_regs_pcctl::EpwmRegsPcctlSpec>;
#[doc = "PWM Chopper Control Register"]
pub mod epwm_regs_pcctl;
#[doc = "EPWM_REGS_PID (rw) register accessor: EHRPWM Peripheral ID Register. The IP revision register is used by software to track features, bugs, and compatibility.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_pid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_pid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epwm_regs_pid`]
module"]
#[doc(alias = "EPWM_REGS_PID")]
pub type EpwmRegsPid = crate::Reg<epwm_regs_pid::EpwmRegsPidSpec>;
#[doc = "EHRPWM Peripheral ID Register. The IP revision register is used by software to track features, bugs, and compatibility."]
pub mod epwm_regs_pid;
