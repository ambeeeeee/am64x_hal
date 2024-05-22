#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_54` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_54` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec>;
#[doc = "Field `PI_CALVL_CS` reader - 0:0\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type PiCalvlCsR = crate::BitReader;
#[doc = "Field `PI_CALVL_CS` writer - 0:0\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
pub type PiCalvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_SEQ_EN` reader - 25:24\\]
Specifies which CA training patterns will be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1 and 2, or set to 3 for all patterns."]
pub type PiCalvlSeqEnR = crate::FieldReader;
#[doc = "Field `PI_CALVL_SEQ_EN` writer - 25:24\\]
Specifies which CA training patterns will be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1 and 2, or set to 3 for all patterns."]
pub type PiCalvlSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    pub fn pi_calvl_cs(&self) -> PiCalvlCsR {
        PiCalvlCsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies which CA training patterns will be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1 and 2, or set to 3 for all patterns."]
    #[inline(always)]
    pub fn pi_calvl_seq_en(&self) -> PiCalvlSeqEnR {
        PiCalvlSeqEnR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Specifies the target chip select for the CA training operation initiated through the CALVL_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_cs(&mut self) -> PiCalvlCsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec> {
        PiCalvlCsW::new(self, 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Specifies which CA training patterns will be used. Set to 0 for pattern 0 only, set to 1 for patterns 0 and 1, set to 2 for patterns 0, 1 and 2, or set to 3 for all patterns."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_seq_en(&mut self) -> PiCalvlSeqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec> {
        PiCalvlSeqEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_54::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_54::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_54 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi54Spec {
    const RESET_VALUE: u32 = 0;
}
