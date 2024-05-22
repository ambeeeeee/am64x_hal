#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_273` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_273` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec>;
#[doc = "Field `PI_TXSNR_F2` reader - 15:0\\]
DRAM tXSNR value for frequency set 2 in cycles."]
pub type PiTxsnrF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TXSNR_F2` writer - 15:0\\]
DRAM tXSNR value for frequency set 2 in cycles."]
pub type PiTxsnrF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM tXSNR value for frequency set 2 in cycles."]
    #[inline(always)]
    pub fn pi_txsnr_f2(&self) -> PiTxsnrF2R {
        PiTxsnrF2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM tXSNR value for frequency set 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn pi_txsnr_f2(&mut self) -> PiTxsnrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec> {
        PiTxsnrF2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_273\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_273::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_273::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_273::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_273::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_273 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi273Spec {
    const RESET_VALUE: u32 = 0;
}
