#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_115` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_115` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec>;
#[doc = "Field `PI_BIST_ADDR_MASK_6_1` reader - 1:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask6_1R = crate::FieldReader;
#[doc = "Field `PI_BIST_ADDR_MASK_6_1` writer - 1:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask6_1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    pub fn pi_bist_addr_mask_6_1(&self) -> PiBistAddrMask6_1R {
        PiBistAddrMask6_1R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_addr_mask_6_1(
        &mut self,
    ) -> PiBistAddrMask6_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec> {
        PiBistAddrMask6_1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_115\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_115::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_115::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_115::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_115::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_115 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi115Spec {
    const RESET_VALUE: u32 = 0;
}
