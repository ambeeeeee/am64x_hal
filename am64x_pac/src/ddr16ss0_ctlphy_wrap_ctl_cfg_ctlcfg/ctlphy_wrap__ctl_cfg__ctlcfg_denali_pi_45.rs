#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_45` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_45` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec>;
#[doc = "Field `PI_RDLVL_GATE_ROTATE` reader - 0:0\\]
Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
pub type PiRdlvlGateRotateR = crate::BitReader;
#[doc = "Field `PI_RDLVL_GATE_ROTATE` writer - 0:0\\]
Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
pub type PiRdlvlGateRotateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_CS_MAP` reader - 9:8\\]
Defines the chip select map for data eye training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
pub type PiRdlvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_CS_MAP` writer - 9:8\\]
Defines the chip select map for data eye training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
pub type PiRdlvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RDLVL_GATE_CS_MAP` reader - 17:16\\]
Defines the chip select map for gate training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
pub type PiRdlvlGateCsMapR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_GATE_CS_MAP` writer - 17:16\\]
Defines the chip select map for gate training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
pub type PiRdlvlGateCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_rotate(&self) -> PiRdlvlGateRotateR {
        PiRdlvlGateRotateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the chip select map for data eye training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_cs_map(&self) -> PiRdlvlCsMapR {
        PiRdlvlCsMapR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the chip select map for gate training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
    #[inline(always)]
    pub fn pi_rdlvl_gate_cs_map(&self) -> PiRdlvlGateCsMapR {
        PiRdlvlGateCsMapR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables rotational CS for interval gate training. Set to 1 for rotating CS."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_rotate(
        &mut self,
    ) -> PiRdlvlGateRotateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec> {
        PiRdlvlGateRotateW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Defines the chip select map for data eye training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_cs_map(&mut self) -> PiRdlvlCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec> {
        PiRdlvlCsMapW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Defines the chip select map for gate training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for gate training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_gate_cs_map(
        &mut self,
    ) -> PiRdlvlGateCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec> {
        PiRdlvlGateCsMapW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_45\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_45::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_45::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_45::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_45::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_45 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi45Spec {
    const RESET_VALUE: u32 = 0;
}
