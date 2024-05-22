#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_56` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_56` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec>;
#[doc = "Field `PI_CALVL_CS_MAP` reader - 1:0\\]
Defines the chip select map for CA training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
pub type PiCalvlCsMapR = crate::FieldReader;
#[doc = "Field `PI_CALVL_CS_MAP` writer - 1:0\\]
Defines the chip select map for CA training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
pub type PiCalvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_CALVL_EN` reader - 15:8\\]
Defines the DFI tCALVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
pub type PiTdfiCalvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_CALVL_EN` writer - 15:8\\]
Defines the DFI tCALVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
pub type PiTdfiCalvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines the chip select map for CA training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
    #[inline(always)]
    pub fn pi_calvl_cs_map(&self) -> PiCalvlCsMapR {
        PiCalvlCsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tCALVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_en(&self) -> PiTdfiCalvlEnR {
        PiTdfiCalvlEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines the chip select map for CA training operations. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable chip for CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_cs_map(&mut self) -> PiCalvlCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec> {
        PiCalvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tCALVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles between a dfi_calvl_en assertion and a dfi_cke de-assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_en(&mut self) -> PiTdfiCalvlEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec> {
        PiTdfiCalvlEnW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_56::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_56::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_56 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi56Spec {
    const RESET_VALUE: u32 = 0;
}
