#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_325` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_325` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec>;
#[doc = "Field `PI_MR4_DATA_F0_1` reader - 16:0\\]
Data to program into memory mode register 4 for chip select 1 for frequency set 0."]
pub type PiMr4DataF0_1R = crate::FieldReader<u32>;
#[doc = "Field `PI_MR4_DATA_F0_1` writer - 16:0\\]
Data to program into memory mode register 4 for chip select 1 for frequency set 0."]
pub type PiMr4DataF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 4 for chip select 1 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr4_data_f0_1(&self) -> PiMr4DataF0_1R {
        PiMr4DataF0_1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 4 for chip select 1 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr4_data_f0_1(&mut self) -> PiMr4DataF0_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec> {
        PiMr4DataF0_1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_325\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_325::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_325::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_325::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_325::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_325 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi325Spec {
    const RESET_VALUE: u32 = 0;
}
