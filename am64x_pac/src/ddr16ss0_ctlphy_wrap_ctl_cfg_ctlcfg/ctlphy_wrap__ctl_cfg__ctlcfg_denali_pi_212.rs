#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_212` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_212` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec>;
#[doc = "Field `PI_TCKELCK_F2` reader - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 2."]
pub type PiTckelckF2R = crate::FieldReader;
#[doc = "Field `PI_TCKELCK_F2` writer - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 2."]
pub type PiTckelckF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TDFI_INIT_START_F0` reader - 31:8\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type PiTdfiInitStartF0R = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_INIT_START_F0` writer - 31:8\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
pub type PiTdfiInitStartF0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 2."]
    #[inline(always)]
    pub fn pi_tckelck_f2(&self) -> PiTckelckF2R {
        PiTckelckF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    pub fn pi_tdfi_init_start_f0(&self) -> PiTdfiInitStartF0R {
        PiTdfiInitStartF0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Valid Clock Requirement after CKE deassert for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tckelck_f2(&mut self) -> PiTckelckF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec> {
        PiTckelckF2W::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Defines the DFI tINIT_START timing parameter \\[in DFI clocks\\]
for frequency set 0, the maximum number of cycles between a dfi_init_start assertion and a dfi_init_complete de-assertion from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_init_start_f0(
        &mut self,
    ) -> PiTdfiInitStartF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec> {
        PiTdfiInitStartF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_212\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_212::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_212::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_212::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_212::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_212 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi212Spec {
    const RESET_VALUE: u32 = 0;
}
