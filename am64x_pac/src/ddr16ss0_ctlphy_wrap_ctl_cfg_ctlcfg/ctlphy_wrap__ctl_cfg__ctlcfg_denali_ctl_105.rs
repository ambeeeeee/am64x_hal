#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_105` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_105` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec>;
#[doc = "Field `TCKELPD_F2` reader - 4:0\\]
DRAM TCKELPD value in cycles. FC=2"]
pub type TckelpdF2R = crate::FieldReader;
#[doc = "Field `TCKELPD_F2` writer - 4:0\\]
DRAM TCKELPD value in cycles. FC=2"]
pub type TckelpdF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCMDCKE_F0` reader - 12:8\\]
DRAM TCMDCKE value in cycles. FC=0"]
pub type TcmdckeF0R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F0` writer - 12:8\\]
DRAM TCMDCKE value in cycles. FC=0"]
pub type TcmdckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCMDCKE_F1` reader - 20:16\\]
DRAM TCMDCKE value in cycles. FC=1"]
pub type TcmdckeF1R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F1` writer - 20:16\\]
DRAM TCMDCKE value in cycles. FC=1"]
pub type TcmdckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCMDCKE_F2` reader - 28:24\\]
DRAM TCMDCKE value in cycles. FC=2"]
pub type TcmdckeF2R = crate::FieldReader;
#[doc = "Field `TCMDCKE_F2` writer - 28:24\\]
DRAM TCMDCKE value in cycles. FC=2"]
pub type TcmdckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKELPD value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckelpd_f2(&self) -> TckelpdF2R {
        TckelpdF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCMDCKE value in cycles. FC=0"]
    #[inline(always)]
    pub fn tcmdcke_f0(&self) -> TcmdckeF0R {
        TcmdckeF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCMDCKE value in cycles. FC=1"]
    #[inline(always)]
    pub fn tcmdcke_f1(&self) -> TcmdckeF1R {
        TcmdckeF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCMDCKE value in cycles. FC=2"]
    #[inline(always)]
    pub fn tcmdcke_f2(&self) -> TcmdckeF2R {
        TcmdckeF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKELPD value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f2(&mut self) -> TckelpdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec> {
        TckelpdF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCMDCKE value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f0(&mut self) -> TcmdckeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec> {
        TcmdckeF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCMDCKE value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f1(&mut self) -> TcmdckeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec> {
        TcmdckeF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCMDCKE value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tcmdcke_f2(&mut self) -> TcmdckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec> {
        TcmdckeF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_105\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_105::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_105::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_105::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_105::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_105 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl105Spec {
    const RESET_VALUE: u32 = 0;
}
