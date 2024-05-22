#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_189` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_189` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec>;
#[doc = "Field `PI_RDLVL_EN_F1` reader - 1:0\\]
Enable the PI data eye training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_EN_F1` writer - 1:0\\]
Enable the PI data eye training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_GATE_EN_F1` reader - 9:8\\]
Enable the PI gate training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnF1R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_EN_F1` writer - 9:8\\]
Enable the PI gate training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_EN_F2` reader - 17:16\\]
Enable the PI data eye training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_EN_F2` writer - 17:16\\]
Enable the PI data eye training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_GATE_EN_F2` reader - 25:24\\]
Enable the PI gate training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_EN_F2` writer - 25:24\\]
Enable the PI gate training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiRdlvlGateEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI data eye training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_en_f1(&self) -> PiRdlvlEnF1R {
        PiRdlvlEnF1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable the PI gate training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_en_f1(&self) -> PiRdlvlGateEnF1R {
        PiRdlvlGateEnF1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI data eye training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_en_f2(&self) -> PiRdlvlEnF2R {
        PiRdlvlEnF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable the PI gate training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_en_f2(&self) -> PiRdlvlGateEnF2R {
        PiRdlvlGateEnF2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI data eye training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_en_f1(&mut self) -> PiRdlvlEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec> {
        PiRdlvlEnF1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable the PI gate training module for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_en_f1(
        &mut self,
    ) -> PiRdlvlGateEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec> {
        PiRdlvlGateEnF1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI data eye training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_en_f2(&mut self) -> PiRdlvlEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec> {
        PiRdlvlEnF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable the PI gate training module for frequency set 2. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_en_f2(
        &mut self,
    ) -> PiRdlvlGateEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec> {
        PiRdlvlGateEnF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_189\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_189::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_189::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_189::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_189::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_189 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi189Spec {
    const RESET_VALUE: u32 = 0;
}
