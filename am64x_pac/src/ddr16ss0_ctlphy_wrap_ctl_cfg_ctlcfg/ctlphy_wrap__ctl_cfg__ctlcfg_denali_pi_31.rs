#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_31` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_31` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec>;
#[doc = "Field `PI_ADDRESS_MIRRORING` reader - 1:0\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
pub type PiAddressMirroringR = crate::FieldReader;
#[doc = "Field `PI_ADDRESS_MIRRORING` writer - 1:0\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
pub type PiAddressMirroringW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn pi_address_mirroring(&self) -> PiAddressMirroringR {
        PiAddressMirroringR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates which chip selects support address mirroring. Bit \\[0\\]
controls cs0, bit \\[1\\]
controls cs1, etc. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_address_mirroring(
        &mut self,
    ) -> PiAddressMirroringW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec> {
        PiAddressMirroringW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_31::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_31::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_31 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi31Spec {
    const RESET_VALUE: u32 = 0;
}
