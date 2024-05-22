#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_46` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_46` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec>;
#[doc = "Field `PI_TDFI_RDLVL_RR` reader - 9:0\\]
Defines the DFI tRDLVL_RR timing parameter \\[in DFI clocks\\], the minimum cycles between read commands."]
pub type PiTdfiRdlvlRrR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_RDLVL_RR` writer - 9:0\\]
Defines the DFI tRDLVL_RR timing parameter \\[in DFI clocks\\], the minimum cycles between read commands."]
pub type PiTdfiRdlvlRrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tRDLVL_RR timing parameter \\[in DFI clocks\\], the minimum cycles between read commands."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_rr(&self) -> PiTdfiRdlvlRrR {
        PiTdfiRdlvlRrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Defines the DFI tRDLVL_RR timing parameter \\[in DFI clocks\\], the minimum cycles between read commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_rr(&mut self) -> PiTdfiRdlvlRrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec> {
        PiTdfiRdlvlRrW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_46\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_46::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_46::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_46::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_46::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_46 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi46Spec {
    const RESET_VALUE: u32 = 0;
}