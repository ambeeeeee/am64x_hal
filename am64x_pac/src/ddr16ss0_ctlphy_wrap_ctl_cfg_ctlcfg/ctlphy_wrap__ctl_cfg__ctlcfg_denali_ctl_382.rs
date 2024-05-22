#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_382` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_382` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec>;
#[doc = "Field `TDQSCK_MIN_F0` reader - 2:0\\]
Additional delay needed for tDQSCK. FC=0"]
pub type TdqsckMinF0R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F0` writer - 2:0\\]
Additional delay needed for tDQSCK. FC=0"]
pub type TdqsckMinF0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDQSCK_MAX_F1` reader - 11:8\\]
Additional delay needed for tDQSCK. FC=1"]
pub type TdqsckMaxF1R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F1` writer - 11:8\\]
Additional delay needed for tDQSCK. FC=1"]
pub type TdqsckMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDQSCK_MIN_F1` reader - 18:16\\]
Additional delay needed for tDQSCK. FC=1"]
pub type TdqsckMinF1R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F1` writer - 18:16\\]
Additional delay needed for tDQSCK. FC=1"]
pub type TdqsckMinF1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDQSCK_MAX_F2` reader - 27:24\\]
Additional delay needed for tDQSCK. FC=2"]
pub type TdqsckMaxF2R = crate::FieldReader;
#[doc = "Field `TDQSCK_MAX_F2` writer - 27:24\\]
Additional delay needed for tDQSCK. FC=2"]
pub type TdqsckMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Additional delay needed for tDQSCK. FC=0"]
    #[inline(always)]
    pub fn tdqsck_min_f0(&self) -> TdqsckMinF0R {
        TdqsckMinF0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Additional delay needed for tDQSCK. FC=1"]
    #[inline(always)]
    pub fn tdqsck_max_f1(&self) -> TdqsckMaxF1R {
        TdqsckMaxF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Additional delay needed for tDQSCK. FC=1"]
    #[inline(always)]
    pub fn tdqsck_min_f1(&self) -> TdqsckMinF1R {
        TdqsckMinF1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Additional delay needed for tDQSCK. FC=2"]
    #[inline(always)]
    pub fn tdqsck_max_f2(&self) -> TdqsckMaxF2R {
        TdqsckMaxF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Additional delay needed for tDQSCK. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f0(&mut self) -> TdqsckMinF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec> {
        TdqsckMinF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Additional delay needed for tDQSCK. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f1(&mut self) -> TdqsckMaxF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec> {
        TdqsckMaxF1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Additional delay needed for tDQSCK. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f1(&mut self) -> TdqsckMinF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec> {
        TdqsckMinF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Additional delay needed for tDQSCK. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_max_f2(&mut self) -> TdqsckMaxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec> {
        TdqsckMaxF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_382\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_382::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_382::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_382::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_382::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_382 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl382Spec {
    const RESET_VALUE: u32 = 0;
}
