#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_91` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_91` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec>;
#[doc = "Field `TCKEHCS_F0` reader - 4:0\\]
DRAM TCKEHCS value in cycles. FC=0"]
pub type TckehcsF0R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F0` writer - 4:0\\]
DRAM TCKEHCS value in cycles. FC=0"]
pub type TckehcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMRWCKEL_F0` reader - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=0"]
pub type TmrwckelF0R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F0` writer - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=0"]
pub type TmrwckelF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TZQCKE_F0` reader - 19:16\\]
DRAM TZQCKE value in cycles. FC=0"]
pub type TzqckeF0R = crate::FieldReader;
#[doc = "Field `TZQCKE_F0` writer - 19:16\\]
DRAM TZQCKE value in cycles. FC=0"]
pub type TzqckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCS_F1` reader - 28:24\\]
DRAM TCKELCS value in cycles. FC=1"]
pub type TckelcsF1R = crate::FieldReader;
#[doc = "Field `TCKELCS_F1` writer - 28:24\\]
DRAM TCKELCS value in cycles. FC=1"]
pub type TckelcsF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckehcs_f0(&self) -> TckehcsF0R {
        TckehcsF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=0"]
    #[inline(always)]
    pub fn tmrwckel_f0(&self) -> TmrwckelF0R {
        TmrwckelF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=0"]
    #[inline(always)]
    pub fn tzqcke_f0(&self) -> TzqckeF0R {
        TzqckeF0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckelcs_f1(&self) -> TckelcsF1R {
        TckelcsF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f0(&mut self) -> TckehcsF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec> {
        TckehcsF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f0(&mut self) -> TmrwckelF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec> {
        TmrwckelF0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f0(&mut self) -> TzqckeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec> {
        TzqckeF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCS value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f1(&mut self) -> TckelcsF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec> {
        TckelcsF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_91\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_91::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_91::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_91::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_91::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_91 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl91Spec {
    const RESET_VALUE: u32 = 0;
}
