#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_90` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_90` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec>;
#[doc = "Field `TMRRI_F0` reader - 7:0\\]
DRAM TMRRI value in cycles. FC=0"]
pub type TmrriF0R = crate::FieldReader;
#[doc = "Field `TMRRI_F0` writer - 7:0\\]
DRAM TMRRI value in cycles. FC=0"]
pub type TmrriF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRRI_F1` reader - 15:8\\]
DRAM TMRRI value in cycles. FC=1"]
pub type TmrriF1R = crate::FieldReader;
#[doc = "Field `TMRRI_F1` writer - 15:8\\]
DRAM TMRRI value in cycles. FC=1"]
pub type TmrriF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRRI_F2` reader - 23:16\\]
DRAM TMRRI value in cycles. FC=2"]
pub type TmrriF2R = crate::FieldReader;
#[doc = "Field `TMRRI_F2` writer - 23:16\\]
DRAM TMRRI value in cycles. FC=2"]
pub type TmrriF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCKELCS_F0` reader - 28:24\\]
DRAM TCKELCS value in cycles. FC=0"]
pub type TckelcsF0R = crate::FieldReader;
#[doc = "Field `TCKELCS_F0` writer - 28:24\\]
DRAM TCKELCS value in cycles. FC=0"]
pub type TckelcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRRI value in cycles. FC=0"]
    #[inline(always)]
    pub fn tmrri_f0(&self) -> TmrriF0R {
        TmrriF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRRI value in cycles. FC=1"]
    #[inline(always)]
    pub fn tmrri_f1(&self) -> TmrriF1R {
        TmrriF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMRRI value in cycles. FC=2"]
    #[inline(always)]
    pub fn tmrri_f2(&self) -> TmrriF2R {
        TmrriF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckelcs_f0(&self) -> TckelcsF0R {
        TckelcsF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TMRRI value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f0(&mut self) -> TmrriF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec> {
        TmrriF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRRI value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f1(&mut self) -> TmrriF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec> {
        TmrriF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TMRRI value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f2(&mut self) -> TmrriF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec> {
        TmrriF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f0(&mut self) -> TckelcsF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec> {
        TckelcsF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_90::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_90::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_90::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_90::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_90 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl90Spec {
    const RESET_VALUE: u32 = 0;
}
