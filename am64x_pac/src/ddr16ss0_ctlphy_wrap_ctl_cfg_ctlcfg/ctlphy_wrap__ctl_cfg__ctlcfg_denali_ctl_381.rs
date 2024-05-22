#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_381` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_381` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec>;
#[doc = "Field `R2R_SAMECS_DLY` reader - 4:0\\]
Additional delay to insert between two reads to the same chip select. Any value including 0 supported."]
pub type R2rSamecsDlyR = crate::FieldReader;
#[doc = "Field `R2R_SAMECS_DLY` writer - 4:0\\]
Additional delay to insert between two reads to the same chip select. Any value including 0 supported."]
pub type R2rSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2R_SAMECS_DLY` reader - 12:8\\]
Additional delay to insert between writes and reads to the same chip select."]
pub type W2rSamecsDlyR = crate::FieldReader;
#[doc = "Field `W2R_SAMECS_DLY` writer - 12:8\\]
Additional delay to insert between writes and reads to the same chip select."]
pub type W2rSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W2W_SAMECS_DLY` reader - 20:16\\]
Additional delay to insert between two writes to the same chip select. Any value including 0 supported."]
pub type W2wSamecsDlyR = crate::FieldReader;
#[doc = "Field `W2W_SAMECS_DLY` writer - 20:16\\]
Additional delay to insert between two writes to the same chip select. Any value including 0 supported."]
pub type W2wSamecsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TDQSCK_MAX_F0` reader - 27:24\\]
Additional delay needed for tDQSCK. FC=0"]
pub type TdqsckMaxF0R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F0` writer - 27:24\\]
Additional delay needed for tDQSCK. FC=0"]
pub type TdqsckMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Additional delay to insert between two reads to the same chip select. Any value including 0 supported."]
    #[inline(always)]
    pub fn r2r_samecs_dly(&self) -> R2rSamecsDlyR {
        R2rSamecsDlyR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Additional delay to insert between writes and reads to the same chip select."]
    #[inline(always)]
    pub fn w2r_samecs_dly(&self) -> W2rSamecsDlyR {
        W2rSamecsDlyR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between two writes to the same chip select. Any value including 0 supported."]
    #[inline(always)]
    pub fn w2w_samecs_dly(&self) -> W2wSamecsDlyR {
        W2wSamecsDlyR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Additional delay needed for tDQSCK. FC=0"]
    #[inline(always)]
    pub fn tdqsck_max_f0(&self) -> TdqsckMaxF0R {
        TdqsckMaxF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Additional delay to insert between two reads to the same chip select. Any value including 0 supported."]
    #[inline(always)]
    #[must_use]
    pub fn r2r_samecs_dly(&mut self) -> R2rSamecsDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec> {
        R2rSamecsDlyW::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Additional delay to insert between writes and reads to the same chip select."]
    #[inline(always)]
    #[must_use]
    pub fn w2r_samecs_dly(&mut self) -> W2rSamecsDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec> {
        W2rSamecsDlyW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Additional delay to insert between two writes to the same chip select. Any value including 0 supported."]
    #[inline(always)]
    #[must_use]
    pub fn w2w_samecs_dly(&mut self) -> W2wSamecsDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec> {
        W2wSamecsDlyW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Additional delay needed for tDQSCK. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f0(&mut self) -> TdqsckMaxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec> {
        TdqsckMaxF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_381\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_381::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_381::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_381::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_381::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_381 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl381Spec {
    const RESET_VALUE: u32 = 0;
}
