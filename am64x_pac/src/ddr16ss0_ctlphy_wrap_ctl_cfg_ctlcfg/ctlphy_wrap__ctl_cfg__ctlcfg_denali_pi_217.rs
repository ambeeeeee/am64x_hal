#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_217` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_217` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F2` reader - 23:0\\]
Defines the DFI tINIT_COMPLETE timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY."]
pub type PiTdfiInitCompleteF2R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_INIT_COMPLETE_F2` writer - 23:0\\]
Defines the DFI tINIT_COMPLETE timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY."]
pub type PiTdfiInitCompleteF2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PI_TCKEHDQS_F0` reader - 29:24\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 0."]
pub type PiTckehdqsF0R = crate::FieldReader;
#[doc = "Field `PI_TCKEHDQS_F0` writer - 29:24\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 0."]
pub type PiTckehdqsF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_COMPLETE timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    pub fn pi_tdfi_init_complete_f2(&self) -> PiTdfiInitCompleteF2R {
        PiTdfiInitCompleteF2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:29 - 29:24\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 0."]
    #[inline(always)]
    pub fn pi_tckehdqs_f0(&self) -> PiTckehdqsF0R {
        PiTckehdqsF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Defines the DFI tINIT_COMPLETE timing parameter \\[in DFI clocks\\]
for frequency set 2, the maximum cycles between a dfi_init_start de-assertion and a dfi_init_complete assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_complete_f2(
        &mut self,
    ) -> PiTdfiInitCompleteF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec> {
        PiTdfiInitCompleteF2W::new(self, 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
The DRAM timing tCKEHDQS, minimum delay from CKE high to strobe high impedance for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckehdqs_f0(&mut self) -> PiTckehdqsF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec> {
        PiTckehdqsF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_217\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_217::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_217::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_217::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_217::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_217 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi217Spec {
    const RESET_VALUE: u32 = 0;
}
