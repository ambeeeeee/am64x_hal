#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_51` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_51` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec>;
#[doc = "Field `PI_RDLVL_GATE_INTERVAL` reader - 15:0\\]
Number of long count sequences counted between automatic gate training commands."]
pub type PiRdlvlGateIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_RDLVL_GATE_INTERVAL` writer - 15:0\\]
Number of long count sequences counted between automatic gate training commands."]
pub type PiRdlvlGateIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PI_RDLVL_PATTERN_START` reader - 19:16\\]
Defines the start pattern in read leveling."]
pub type PiRdlvlPatternStartR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PATTERN_START` writer - 19:16\\]
Defines the start pattern in read leveling."]
pub type PiRdlvlPatternStartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_RDLVL_PATTERN_NUM` reader - 27:24\\]
Defines the number of pattern supported in read leveling."]
pub type PiRdlvlPatternNumR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_PATTERN_NUM` writer - 27:24\\]
Defines the number of pattern supported in read leveling."]
pub type PiRdlvlPatternNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Number of long count sequences counted between automatic gate training commands."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_interval(&self) -> PiRdlvlGateIntervalR {
        PiRdlvlGateIntervalR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the start pattern in read leveling."]
    #[inline(always)]
    pub fn pi_rdlvl_pattern_start(&self) -> PiRdlvlPatternStartR {
        PiRdlvlPatternStartR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the number of pattern supported in read leveling."]
    #[inline(always)]
    pub fn pi_rdlvl_pattern_num(&self) -> PiRdlvlPatternNumR {
        PiRdlvlPatternNumR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Number of long count sequences counted between automatic gate training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_interval(
        &mut self,
    ) -> PiRdlvlGateIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec> {
        PiRdlvlGateIntervalW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the start pattern in read leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pattern_start(
        &mut self,
    ) -> PiRdlvlPatternStartW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec> {
        PiRdlvlPatternStartW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Defines the number of pattern supported in read leveling."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_pattern_num(
        &mut self,
    ) -> PiRdlvlPatternNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec> {
        PiRdlvlPatternNumW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_51\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_51::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_51::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_51::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_51::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_51 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi51Spec {
    const RESET_VALUE: u32 = 0;
}
