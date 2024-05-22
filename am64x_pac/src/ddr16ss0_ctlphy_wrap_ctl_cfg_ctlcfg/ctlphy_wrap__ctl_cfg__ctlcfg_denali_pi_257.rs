#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_257` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_257` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec>;
#[doc = "Field `PI_TCKSRX_F0` reader - 7:0\\]
DRAM tCKSRX value."]
pub type PiTcksrxF0R = crate::FieldReader;
#[doc = "Field `PI_TCKSRX_F0` writer - 7:0\\]
DRAM tCKSRX value."]
pub type PiTcksrxF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCKSRE_F0` reader - 15:8\\]
DRAM tCKSRE value."]
pub type PiTcksreF0R = crate::FieldReader;
#[doc = "Field `PI_TCKSRE_F0` writer - 15:8\\]
DRAM tCKSRE value."]
pub type PiTcksreF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCKSRX_F1` reader - 23:16\\]
DRAM tCKSRX value."]
pub type PiTcksrxF1R = crate::FieldReader;
#[doc = "Field `PI_TCKSRX_F1` writer - 23:16\\]
DRAM tCKSRX value."]
pub type PiTcksrxF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TCKSRE_F1` reader - 31:24\\]
DRAM tCKSRE value."]
pub type PiTcksreF1R = crate::FieldReader;
#[doc = "Field `PI_TCKSRE_F1` writer - 31:24\\]
DRAM tCKSRE value."]
pub type PiTcksreF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKSRX value."]
    #[inline(always)]
    pub fn pi_tcksrx_f0(&self) -> PiTcksrxF0R {
        PiTcksrxF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tCKSRE value."]
    #[inline(always)]
    pub fn pi_tcksre_f0(&self) -> PiTcksreF0R {
        PiTcksreF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tCKSRX value."]
    #[inline(always)]
    pub fn pi_tcksrx_f1(&self) -> PiTcksrxF1R {
        PiTcksrxF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tCKSRE value."]
    #[inline(always)]
    pub fn pi_tcksre_f1(&self) -> PiTcksreF1R {
        PiTcksreF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM tCKSRX value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksrx_f0(&mut self) -> PiTcksrxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec> {
        PiTcksrxF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM tCKSRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksre_f0(&mut self) -> PiTcksreF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec> {
        PiTcksreF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM tCKSRX value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksrx_f1(&mut self) -> PiTcksrxF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec> {
        PiTcksrxF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM tCKSRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tcksre_f1(&mut self) -> PiTcksreF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec> {
        PiTcksreF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_257\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_257::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_257::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_257::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_257::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_257 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi257Spec {
    const RESET_VALUE: u32 = 0;
}
