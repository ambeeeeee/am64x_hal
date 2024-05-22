#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_103` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_103` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec>;
#[doc = "Field `TCKCKEL_F1` reader - 4:0\\]
DRAM TCKCKEL value in cycles. FC=1"]
pub type TckckelF1R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F1` writer - 4:0\\]
DRAM TCKCKEL value in cycles. FC=1"]
pub type TckckelF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKELPD_F1` reader - 12:8\\]
DRAM TCKELPD value in cycles. FC=1"]
pub type TckelpdF1R = crate::FieldReader;
#[doc = "Field `TCKELPD_F1` writer - 12:8\\]
DRAM TCKELPD value in cycles. FC=1"]
pub type TckelpdF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSR_F2` reader - 23:16\\]
DRAM TSR value in cycles. FC=2"]
pub type TsrF2R = crate::FieldReader;
#[doc = "Field `TSR_F2` writer - 23:16\\]
DRAM TSR value in cycles. FC=2"]
pub type TsrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESCKE_F2` reader - 26:24\\]
DRAM TESCKE value in cycles. FC=2"]
pub type TesckeF2R = crate::FieldReader;
#[doc = "Field `TESCKE_F2` writer - 26:24\\]
DRAM TESCKE value in cycles. FC=2"]
pub type TesckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKCKEL value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckckel_f1(&self) -> TckckelF1R {
        TckckelF1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKELPD value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckelpd_f1(&self) -> TckelpdF1R {
        TckelpdF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TSR value in cycles. FC=2"]
    #[inline(always)]
    pub fn tsr_f2(&self) -> TsrF2R {
        TsrF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
DRAM TESCKE value in cycles. FC=2"]
    #[inline(always)]
    pub fn tescke_f2(&self) -> TesckeF2R {
        TesckeF2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKCKEL value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f1(&mut self) -> TckckelF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec> {
        TckckelF1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKELPD value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f1(&mut self) -> TckelpdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec> {
        TckelpdF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TSR value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f2(&mut self) -> TsrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec> {
        TsrF2W::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
DRAM TESCKE value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f2(&mut self) -> TesckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec> {
        TesckeF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_103\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_103::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_103::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_103::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_103::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_103 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl103Spec {
    const RESET_VALUE: u32 = 0;
}
