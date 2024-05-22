#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_331` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_331` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec>;
#[doc = "Field `PREAMBLE_SUPPORT_F0` reader - 1:0\\]
Selection the preamble for read and write burst transfers. FC=0"]
pub type PreambleSupportF0R = crate::FieldReader;
#[doc = "Field `PREAMBLE_SUPPORT_F0` writer - 1:0\\]
Selection the preamble for read and write burst transfers. FC=0"]
pub type PreambleSupportF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREAMBLE_SUPPORT_F1` reader - 9:8\\]
Selection the preamble for read and write burst transfers. FC=1"]
pub type PreambleSupportF1R = crate::FieldReader;
#[doc = "Field `PREAMBLE_SUPPORT_F1` writer - 9:8\\]
Selection the preamble for read and write burst transfers. FC=1"]
pub type PreambleSupportF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREAMBLE_SUPPORT_F2` reader - 17:16\\]
Selection the preamble for read and write burst transfers. FC=2"]
pub type PreambleSupportF2R = crate::FieldReader;
#[doc = "Field `PREAMBLE_SUPPORT_F2` writer - 17:16\\]
Selection the preamble for read and write burst transfers. FC=2"]
pub type PreambleSupportF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_PREAMBLE_TRAINING_EN` reader - 24:24\\]
Enable read preamble training during gate training. Set to 1 to enable."]
pub type RdPreambleTrainingEnR = crate::BitReader;
#[doc = "Field `RD_PREAMBLE_TRAINING_EN` writer - 24:24\\]
Enable read preamble training during gate training. Set to 1 to enable."]
pub type RdPreambleTrainingEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selection the preamble for read and write burst transfers. FC=0"]
    #[inline(always)]
    pub fn preamble_support_f0(&self) -> PreambleSupportF0R {
        PreambleSupportF0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selection the preamble for read and write burst transfers. FC=1"]
    #[inline(always)]
    pub fn preamble_support_f1(&self) -> PreambleSupportF1R {
        PreambleSupportF1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selection the preamble for read and write burst transfers. FC=2"]
    #[inline(always)]
    pub fn preamble_support_f2(&self) -> PreambleSupportF2R {
        PreambleSupportF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rd_preamble_training_en(&self) -> RdPreambleTrainingEnR {
        RdPreambleTrainingEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selection the preamble for read and write burst transfers. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_support_f0(
        &mut self,
    ) -> PreambleSupportF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec> {
        PreambleSupportF0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selection the preamble for read and write burst transfers. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_support_f1(
        &mut self,
    ) -> PreambleSupportF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec> {
        PreambleSupportF1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Selection the preamble for read and write burst transfers. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn preamble_support_f2(
        &mut self,
    ) -> PreambleSupportF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec> {
        PreambleSupportF2W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_preamble_training_en(
        &mut self,
    ) -> RdPreambleTrainingEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec> {
        RdPreambleTrainingEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_331\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_331::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_331::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_331::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_331::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_331 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl331Spec {
    const RESET_VALUE: u32 = 0;
}
