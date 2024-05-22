#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_59` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_59` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec>;
#[doc = "Field `PI_CALVL_RESP_MASK` reader - 0:0\\]
Mask for the dfi_calvl_resp signal during CA training."]
pub type PiCalvlRespMaskR = crate::BitReader;
#[doc = "Field `PI_CALVL_RESP_MASK` writer - 0:0\\]
Mask for the dfi_calvl_resp signal during CA training."]
pub type PiCalvlRespMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CALVL_ERROR_STATUS` reader - 9:8\\]
Holds the error associated with the CA training error interrupt. Bit \\[0\\]
set indicates a TDFI_CALVL_RESP parameter violation and bit \\[1\\]
set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
pub type PiCalvlErrorStatusR = crate::FieldReader;
#[doc = "Field `PI_CALVL_ERROR_STATUS` writer - 9:8\\]
Holds the error associated with the CA training error interrupt. Bit \\[0\\]
set indicates a TDFI_CALVL_RESP parameter violation and bit \\[1\\]
set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
pub type PiCalvlErrorStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_CALVL_INTERVAL` reader - 31:16\\]
Number of long count sequences counted between automatic CA training commands."]
pub type PiCalvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_CALVL_INTERVAL` writer - 31:16\\]
Number of long count sequences counted between automatic CA training commands."]
pub type PiCalvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    pub fn pi_calvl_resp_mask(&self) -> PiCalvlRespMaskR {
        PiCalvlRespMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Holds the error associated with the CA training error interrupt. Bit \\[0\\]
set indicates a TDFI_CALVL_RESP parameter violation and bit \\[1\\]
set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
    #[inline(always)]
    pub fn pi_calvl_error_status(&self) -> PiCalvlErrorStatusR {
        PiCalvlErrorStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Number of long count sequences counted between automatic CA training commands."]
    #[inline(always)]
    pub fn pi_calvl_interval(&self) -> PiCalvlIntervalR {
        PiCalvlIntervalR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Mask for the dfi_calvl_resp signal during CA training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_resp_mask(
        &mut self,
    ) -> PiCalvlRespMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec> {
        PiCalvlRespMaskW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Holds the error associated with the CA training error interrupt. Bit \\[0\\]
set indicates a TDFI_CALVL_RESP parameter violation and bit \\[1\\]
set indicates a TDFI_CALVL_MAX parameter violation. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_error_status(
        &mut self,
    ) -> PiCalvlErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec> {
        PiCalvlErrorStatusW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Number of long count sequences counted between automatic CA training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_calvl_interval(
        &mut self,
    ) -> PiCalvlIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec> {
        PiCalvlIntervalW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_59::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_59::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_59 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi59Spec {
    const RESET_VALUE: u32 = 0;
}
