#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_120` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_120` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec>;
#[doc = "Field `PI_BIST_ADDR_MASK_9_0` reader - 31:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask9_0R = crate::FieldReader<u32>;
#[doc = "Field `PI_BIST_ADDR_MASK_9_0` writer - 31:0\\]
Defines an address to be masked during the BIST operation.."]
pub type PiBistAddrMask9_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    pub fn pi_bist_addr_mask_9_0(&self) -> PiBistAddrMask9_0R {
        PiBistAddrMask9_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Defines an address to be masked during the BIST operation.."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bist_addr_mask_9_0(
        &mut self,
    ) -> PiBistAddrMask9_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec> {
        PiBistAddrMask9_0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_120\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_120::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_120::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_120::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_120::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_120 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi120Spec {
    const RESET_VALUE: u32 = 0;
}
