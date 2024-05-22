#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_48` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_48` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec>;
#[doc = "Field `PI_RDLVL_RESP_MASK` reader - 1:0\\]
Mask for the dfi_rdlvl_resp signal during data eye training."]
pub type PiRdlvlRespMaskR = crate::FieldReader;
#[doc = "Field `PI_RDLVL_RESP_MASK` writer - 1:0\\]
Mask for the dfi_rdlvl_resp signal during data eye training."]
pub type PiRdlvlRespMaskW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_TDFI_RDLVL_EN` reader - 15:8\\]
Defines the DFI tRDLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR. Set to 1 means the minium value\\[1 cycle\\], set to 0 means the maxium value"]
pub type PiTdfiRdlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_RDLVL_EN` writer - 15:8\\]
Defines the DFI tRDLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR. Set to 1 means the minium value\\[1 cycle\\], set to 0 means the maxium value"]
pub type PiTdfiRdlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    pub fn pi_rdlvl_resp_mask(&self) -> PiRdlvlRespMaskR {
        PiRdlvlRespMaskR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tRDLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR. Set to 1 means the minium value\\[1 cycle\\], set to 0 means the maxium value"]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_en(&self) -> PiTdfiRdlvlEnR {
        PiTdfiRdlvlEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Mask for the dfi_rdlvl_resp signal during data eye training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_resp_mask(
        &mut self,
    ) -> PiRdlvlRespMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec> {
        PiRdlvlRespMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tRDLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion to the first read or MRR. Set to 1 means the minium value\\[1 cycle\\], set to 0 means the maxium value"]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_en(&mut self) -> PiTdfiRdlvlEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec> {
        PiTdfiRdlvlEnW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_48::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_48::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_48 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi48Spec {
    const RESET_VALUE: u32 = 0;
}
