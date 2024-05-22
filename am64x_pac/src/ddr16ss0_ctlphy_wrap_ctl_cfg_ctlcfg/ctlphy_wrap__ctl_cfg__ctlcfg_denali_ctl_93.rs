#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_93` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_93` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec>;
#[doc = "Field `TCKEHCS_F2` reader - 4:0\\]
DRAM TCKEHCS value in cycles. FC=2"]
pub type TckehcsF2R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F2` writer - 4:0\\]
DRAM TCKEHCS value in cycles. FC=2"]
pub type TckehcsF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMRWCKEL_F2` reader - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=2"]
pub type TmrwckelF2R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F2` writer - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=2"]
pub type TmrwckelF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TZQCKE_F2` reader - 19:16\\]
DRAM TZQCKE value in cycles. FC=2"]
pub type TzqckeF2R = crate::FieldReader;
#[doc = "Field `TZQCKE_F2` writer - 19:16\\]
DRAM TZQCKE value in cycles. FC=2"]
pub type TzqckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCSCKE_F0` reader - 28:24\\]
DRAM TCSCKE value in cycles. FC=0"]
pub type TcsckeF0R = crate::FieldReader;
#[doc = "Field `TCSCKE_F0` writer - 28:24\\]
DRAM TCSCKE value in cycles. FC=0"]
pub type TcsckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckehcs_f2(&self) -> TckehcsF2R {
        TckehcsF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=2"]
    #[inline(always)]
    pub fn tmrwckel_f2(&self) -> TmrwckelF2R {
        TmrwckelF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=2"]
    #[inline(always)]
    pub fn tzqcke_f2(&self) -> TzqckeF2R {
        TzqckeF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCSCKE value in cycles. FC=0"]
    #[inline(always)]
    pub fn tcscke_f0(&self) -> TcsckeF0R {
        TcsckeF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCS value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f2(&mut self) -> TckehcsF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec> {
        TckehcsF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TMRWCKEL value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f2(&mut self) -> TmrwckelF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec> {
        TmrwckelF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
DRAM TZQCKE value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f2(&mut self) -> TzqckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec> {
        TzqckeF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCSCKE value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f0(&mut self) -> TcsckeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec> {
        TcsckeF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_93\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_93::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_93::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_93::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_93::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_93 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl93Spec {
    const RESET_VALUE: u32 = 0;
}
