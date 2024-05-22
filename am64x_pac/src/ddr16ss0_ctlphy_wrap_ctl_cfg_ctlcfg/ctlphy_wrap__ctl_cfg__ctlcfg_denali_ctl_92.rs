#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_92` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_92` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec>;
#[doc = "Field `TCKEHCS_F1` reader - 4:0\\]
DRAM TCKEHCS value in cycles. FC=1"]
pub type TckehcsF1R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F1` writer - 4:0\\]
DRAM TCKEHCS value in cycles. FC=1"]
pub type TckehcsF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMRWCKEL_F1` reader - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=1"]
pub type TmrwckelF1R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F1` writer - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=1"]
pub type TmrwckelF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TZQCKE_F1` reader - 19:16\\]
DRAM TZQCKE value in cycles. FC=1"]
pub type TzqckeF1R = crate::FieldReader;
#[doc = "Field `TZQCKE_F1` writer - 19:16\\]
DRAM TZQCKE value in cycles. FC=1"]
pub type TzqckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCS_F2` reader - 28:24\\]
DRAM TCKELCS value in cycles. FC=2"]
pub type TckelcsF2R = crate::FieldReader;
#[doc = "Field `TCKELCS_F2` writer - 28:24\\]
DRAM TCKELCS value in cycles. FC=2"]
pub type TckelcsF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckehcs_f1(&self) -> TckehcsF1R {
        TckehcsF1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=1"]
    #[inline(always)]
    pub fn tmrwckel_f1(&self) -> TmrwckelF1R {
        TmrwckelF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=1"]
    #[inline(always)]
    pub fn tzqcke_f1(&self) -> TzqckeF1R {
        TzqckeF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckelcs_f2(&self) -> TckelcsF2R {
        TckelcsF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f1(&mut self) -> TckehcsF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec> {
        TckehcsF1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f1(&mut self) -> TmrwckelF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec> {
        TmrwckelF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f1(&mut self) -> TzqckeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec> {
        TzqckeF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f2(&mut self) -> TckelcsF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec> {
        TckelcsF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_92\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_92::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_92::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_92::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_92::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_92 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl92Spec {
    const RESET_VALUE: u32 = 0;
}
