#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_50` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_50` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec>;
#[doc = "Field `PI_RDLVL_ERROR_STATUS` reader - 0:0\\]
Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved. READ-ONLY"]
pub type PiRdlvlErrorStatusR = crate::BitReader;
#[doc = "Field `PI_RDLVL_ERROR_STATUS` writer - 0:0\\]
Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved. READ-ONLY"]
pub type PiRdlvlErrorStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_RDLVL_INTERVAL` reader - 23:8\\]
Number of long count sequences counted between automatic data eye training commands."]
pub type PiRdlvlIntervalR = crate::FieldReader<u16>;
#[doc = "Field `PI_RDLVL_INTERVAL` writer - 23:8\\]
Number of long count sequences counted between automatic data eye training commands."]
pub type PiRdlvlIntervalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved. READ-ONLY"]
    #[inline(always)]
    pub fn pi_rdlvl_error_status(&self) -> PiRdlvlErrorStatusR {
        PiRdlvlErrorStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Number of long count sequences counted between automatic data eye training commands."]
    #[inline(always)]
    pub fn pi_rdlvl_interval(&self) -> PiRdlvlIntervalR {
        PiRdlvlIntervalR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Holds the error associated with the data eye training error or gate training error interrupt. Uppermost bit set indicates a TDFI_RDLVL_RESP parameter violation. Next uppermost bit set indicates a TDFI_RDLVL_MAX parameter violation. Lower bits are reserved. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_error_status(
        &mut self,
    ) -> PiRdlvlErrorStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec> {
        PiRdlvlErrorStatusW::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Number of long count sequences counted between automatic data eye training commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_interval(
        &mut self,
    ) -> PiRdlvlIntervalW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec> {
        PiRdlvlIntervalW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_50\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_50::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_50::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_50::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_50::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_50 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi50Spec {
    const RESET_VALUE: u32 = 0;
}
