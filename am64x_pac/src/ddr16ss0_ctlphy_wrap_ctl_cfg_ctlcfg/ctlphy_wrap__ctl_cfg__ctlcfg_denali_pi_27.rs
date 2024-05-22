#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_27` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_27` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec>;
#[doc = "Field `PI_WRLVL_ERROR_STATUS` reader - 0:0\\]
Holds the error associated with the write level error interrupt. Bit \\[0\\]
set indicates a TDFI_WRLVL_MAX parameter violation and bit \\[1\\]
set indicates a TDFI_WRLVL_RESP parameter violation. READ-ONLY"]
pub type PiWrlvlErrorStatusR = crate::BitReader;
#[doc = "Field `PI_WRLVL_ERROR_STATUS` writer - 0:0\\]
Holds the error associated with the write level error interrupt. Bit \\[0\\]
set indicates a TDFI_WRLVL_MAX parameter violation and bit \\[1\\]
set indicates a TDFI_WRLVL_RESP parameter violation. READ-ONLY"]
pub type PiWrlvlErrorStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_TDFI_WRLVL_EN` reader - 15:8\\]
Defines the DFI tWRLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
pub type PiTdfiWrlvlEnR = crate::FieldReader;
#[doc = "Field `PI_TDFI_WRLVL_EN` writer - 15:8\\]
Defines the DFI tWRLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
pub type PiTdfiWrlvlEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Holds the error associated with the write level error interrupt. Bit \\[0\\]
set indicates a TDFI_WRLVL_MAX parameter violation and bit \\[1\\]
set indicates a TDFI_WRLVL_RESP parameter violation. READ-ONLY"]
    #[inline(always)]
    pub fn pi_wrlvl_error_status(&self) -> PiWrlvlErrorStatusR {
        PiWrlvlErrorStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tWRLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_en(&self) -> PiTdfiWrlvlEnR {
        PiTdfiWrlvlEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Holds the error associated with the write level error interrupt. Bit \\[0\\]
set indicates a TDFI_WRLVL_MAX parameter violation and bit \\[1\\]
set indicates a TDFI_WRLVL_RESP parameter violation. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_error_status(
        &mut self,
    ) -> PiWrlvlErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec> {
        PiWrlvlErrorStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the DFI tWRLVL_EN timing parameter \\[in DFI clocks\\], the minimum cycles from a dfi_wrlvl_en assertion to the first dfi_wrlvl_strobe assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_en(&mut self) -> PiTdfiWrlvlEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec> {
        PiTdfiWrlvlEnW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_27::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_27::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_27 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi27Spec {
    const RESET_VALUE: u32 = 0;
}
