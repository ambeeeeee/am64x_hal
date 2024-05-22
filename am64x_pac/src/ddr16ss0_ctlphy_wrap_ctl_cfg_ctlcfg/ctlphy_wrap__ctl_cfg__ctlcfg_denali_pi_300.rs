#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_300` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_300` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec>;
#[doc = "Field `PI_MR3_DATA_F0_0` reader - 16:0\\]
Data to program into memory mode register 3 for chip select 0 for frequency set 0."]
pub type PiMr3DataF0_0R = crate::FieldReader<u32>;
#[doc = "Field `PI_MR3_DATA_F0_0` writer - 16:0\\]
Data to program into memory mode register 3 for chip select 0 for frequency set 0."]
pub type PiMr3DataF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 3 for chip select 0 for frequency set 0."]
    #[inline(always)]
    pub fn pi_mr3_data_f0_0(&self) -> PiMr3DataF0_0R {
        PiMr3DataF0_0R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 3 for chip select 0 for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr3_data_f0_0(&mut self) -> PiMr3DataF0_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec> {
        PiMr3DataF0_0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_300\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_300::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_300::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_300::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_300::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_300 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi300Spec {
    const RESET_VALUE: u32 = 0;
}
