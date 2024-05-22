#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_220` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_220` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec>;
#[doc = "Field `PI_TFC_F2` reader - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 2."]
pub type PiTfcF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TFC_F2` writer - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 2."]
pub type PiTfcF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_VREF_EN_F0` reader - 17:16\\]
Enable VREF training during power-up initialization for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF0R = crate::FieldReader;
#[doc = "Field `PI_VREF_EN_F0` writer - 17:16\\]
Enable VREF training during power-up initialization for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_VREF_EN_F1` reader - 25:24\\]
Enable VREF training during power-up initialization for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF1R = crate::FieldReader;
#[doc = "Field `PI_VREF_EN_F1` writer - 25:24\\]
Enable VREF training during power-up initialization for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
pub type PiVrefEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 2."]
    #[inline(always)]
    pub fn pi_tfc_f2(&self) -> PiTfcF2R {
        PiTfcF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable VREF training during power-up initialization for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_vref_en_f0(&self) -> PiVrefEnF0R {
        PiVrefEnF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable VREF training during power-up initialization for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_vref_en_f1(&self) -> PiVrefEnF1R {
        PiVrefEnF1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
The delay in PHY clock cycles from setting MR13.OP7 to any valid command for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tfc_f2(&mut self) -> PiTfcF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec> {
        PiTfcF2W::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Enable VREF training during power-up initialization for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_en_f0(&mut self) -> PiVrefEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec> {
        PiVrefEnF0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Enable VREF training during power-up initialization for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_vref_en_f1(&mut self) -> PiVrefEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec> {
        PiVrefEnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_220\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_220::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_220::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_220::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_220::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_220 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi220Spec {
    const RESET_VALUE: u32 = 0;
}
