#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_34` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_34` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec>;
#[doc = "Field `PI_RDLVL_CS_SW` reader - 1:0\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type PiRdlvlCsSwR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_CS_SW` writer - 1:0\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type PiRdlvlCsSwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_CS` reader - 8:8\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type PiRdlvlCsR = crate::BitReader;
#[doc = "Field `PI_RDLVL_CS` writer - 8:8\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
pub type PiRdlvlCsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    pub fn pi_rdlvl_cs_sw(&self) -> PiRdlvlCsSwR {
        PiRdlvlCsSwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    pub fn pi_rdlvl_cs(&self) -> PiRdlvlCsR {
        PiRdlvlCsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_cs_sw(&mut self) -> PiRdlvlCsSwW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec> {
        PiRdlvlCsSwW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Specifies the target chip select for the data eye training operation initiated through the RDLVL_REQ parameter or the gate training operation initiated through the RDLVL_GATE_REQ parameter."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_cs(&mut self) -> PiRdlvlCsW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec> {
        PiRdlvlCsW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_34::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_34::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_34::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_34::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_34 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi34Spec {
    const RESET_VALUE: u32 = 0;
}
