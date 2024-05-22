#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_199` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_199` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec>;
#[doc = "Field `PI_CALVL_EN_F0` reader - 1:0\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 0. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF0R = crate::FieldReader;
#[doc = "Field `PI_CALVL_EN_F0` writer - 1:0\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 0. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_CALVL_EN_F1` reader - 9:8\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 1. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF1R = crate::FieldReader;
#[doc = "Field `PI_CALVL_EN_F1` writer - 9:8\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 1. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_CALVL_EN_F2` reader - 17:16\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 2. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF2R = crate::FieldReader;
#[doc = "Field `PI_CALVL_EN_F2` writer - 17:16\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 2. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiCalvlEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TMRZ_F0` reader - 28:24\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 0."]
pub type PiTmrzF0R = crate::FieldReader;
#[doc = "Field `PI_TMRZ_F0` writer - 28:24\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 0."]
pub type PiTmrzF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 0. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_en_f0(&self) -> PiCalvlEnF0R {
        PiCalvlEnF0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 1. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_en_f1(&self) -> PiCalvlEnF1R {
        PiCalvlEnF1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 2. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_calvl_en_f2(&self) -> PiCalvlEnF2R {
        PiCalvlEnF2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 0."]
    #[inline(always)]
    pub fn pi_tmrz_f0(&self) -> PiTmrzF0R {
        PiTmrzF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 0. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_en_f0(&mut self) -> PiCalvlEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec> {
        PiCalvlEnF0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 1. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_en_f1(&mut self) -> PiCalvlEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec> {
        PiCalvlEnF1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable the PI CA training module. Bit\\[1\\]
represents the support when non-initialization for frequency set 2. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_en_f2(&mut self) -> PiCalvlEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec> {
        PiCalvlEnF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Defines the delay between a MRW CA exit command and the DQ tristate in memory clocks for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tmrz_f0(&mut self) -> PiTmrzF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec> {
        PiTmrzF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_199\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_199::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_199::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_199::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_199::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_199 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi199Spec {
    const RESET_VALUE: u32 = 0;
}
