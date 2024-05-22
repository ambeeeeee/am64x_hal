#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_258` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_258` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec>;
#[doc = "Field `PI_TCKSRX_F2` reader - 7:0\\]
DRAM tCKSRX value."]
pub type PiTcksrxF2R = crate::FieldReader;
#[doc = "Field `PI_TCKSRX_F2` writer - 7:0\\]
DRAM tCKSRX value."]
pub type PiTcksrxF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCKSRE_F2` reader - 15:8\\]
DRAM tCKSRE value."]
pub type PiTcksreF2R = crate::FieldReader;
#[doc = "Field `PI_TCKSRE_F2` writer - 15:8\\]
DRAM tCKSRE value."]
pub type PiTcksreF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKSRX value."]
    #[inline(always)]
    pub fn pi_tcksrx_f2(&self) -> PiTcksrxF2R {
        PiTcksrxF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tCKSRE value."]
    #[inline(always)]
    pub fn pi_tcksre_f2(&self) -> PiTcksreF2R {
        PiTcksreF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKSRX value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksrx_f2(&mut self) -> PiTcksrxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec> {
        PiTcksrxF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tCKSRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksre_f2(&mut self) -> PiTcksreF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec> {
        PiTcksreF2W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_258\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_258::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_258::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_258::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_258::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_258 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi258Spec {
    const RESET_VALUE: u32 = 0;
}
