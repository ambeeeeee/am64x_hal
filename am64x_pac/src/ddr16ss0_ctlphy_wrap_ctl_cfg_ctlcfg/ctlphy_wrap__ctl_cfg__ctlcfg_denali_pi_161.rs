#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_161` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_161` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec>;
#[doc = "Field `PI_FREQ_CHANGE_REG_COPY` reader - 4:0\\]
In non-DFI 4.0 mode, contains the frequency copy value."]
pub type PiFreqChangeRegCopyR = crate::FieldReader;
#[doc = "Field `PI_FREQ_CHANGE_REG_COPY` writer - 4:0\\]
In non-DFI 4.0 mode, contains the frequency copy value."]
pub type PiFreqChangeRegCopyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
In non-DFI 4.0 mode, contains the frequency copy value."]
    #[inline(always)]
    pub fn pi_freq_change_reg_copy(&self) -> PiFreqChangeRegCopyR {
        PiFreqChangeRegCopyR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
In non-DFI 4.0 mode, contains the frequency copy value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq_change_reg_copy(
        &mut self,
    ) -> PiFreqChangeRegCopyW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec> {
        PiFreqChangeRegCopyW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_161\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_161::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_161::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_161::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_161::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_161 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi161Spec {
    const RESET_VALUE: u32 = 0;
}
