#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_153` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_153` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec>;
#[doc = "Field `PI_DLL_LOCK` reader - 0:0\\]
Monitor dfi_init_complete from PHY. READ-ONLY."]
pub type PiDllLockR = crate::BitReader;
#[doc = "Field `PI_DLL_LOCK` writer - 0:0\\]
Monitor dfi_init_complete from PHY. READ-ONLY."]
pub type PiDllLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_FREQ_NUMBER_STATUS` reader - 12:8\\]
Monitor active freq number in PI. READ-ONLY."]
pub type PiFreqNumberStatusR = crate::FieldReader;
#[doc = "Field `PI_FREQ_NUMBER_STATUS` writer - 12:8\\]
Monitor active freq number in PI. READ-ONLY."]
pub type PiFreqNumberStatusW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_FREQ_RETENTION_NUM` reader - 20:16\\]
Monitor active freq number in PI for data_retention"]
pub type PiFreqRetentionNumR = crate::FieldReader;
#[doc = "Field `PI_FREQ_RETENTION_NUM` writer - 20:16\\]
Monitor active freq number in PI for data_retention"]
pub type PiFreqRetentionNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Monitor dfi_init_complete from PHY. READ-ONLY."]
    #[inline(always)]
    pub fn pi_dll_lock(&self) -> PiDllLockR {
        PiDllLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Monitor active freq number in PI. READ-ONLY."]
    #[inline(always)]
    pub fn pi_freq_number_status(&self) -> PiFreqNumberStatusR {
        PiFreqNumberStatusR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Monitor active freq number in PI for data_retention"]
    #[inline(always)]
    pub fn pi_freq_retention_num(&self) -> PiFreqRetentionNumR {
        PiFreqRetentionNumR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Monitor dfi_init_complete from PHY. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_dll_lock(&mut self) -> PiDllLockW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec> {
        PiDllLockW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Monitor active freq number in PI. READ-ONLY."]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq_number_status(
        &mut self,
    ) -> PiFreqNumberStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec> {
        PiFreqNumberStatusW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Monitor active freq number in PI for data_retention"]
    #[inline(always)]
    #[must_use]
    pub fn pi_freq_retention_num(
        &mut self,
    ) -> PiFreqRetentionNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec> {
        PiFreqRetentionNumW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_153\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_153::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_153::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_153::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_153::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_153 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi153Spec {
    const RESET_VALUE: u32 = 0;
}
