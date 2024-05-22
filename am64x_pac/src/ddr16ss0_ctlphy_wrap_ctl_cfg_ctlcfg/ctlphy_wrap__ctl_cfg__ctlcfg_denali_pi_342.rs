#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_342` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_342` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec>;
#[doc = "Field `PI_MR5_DATA_F2_1` reader - 16:0\\]
Data to program into memory mode register 5 for chip select 1 for frequency set 2."]
pub type PiMr5DataF2_1R = crate::FieldReader<u32>;
#[doc = "Field `PI_MR5_DATA_F2_1` writer - 16:0\\]
Data to program into memory mode register 5 for chip select 1 for frequency set 2."]
pub type PiMr5DataF2_1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 5 for chip select 1 for frequency set 2."]
    #[inline(always)]
    pub fn pi_mr5_data_f2_1(&self) -> PiMr5DataF2_1R {
        PiMr5DataF2_1R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register 5 for chip select 1 for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mr5_data_f2_1(&mut self) -> PiMr5DataF2_1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec> {
        PiMr5DataF2_1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_342\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_342::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_342::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_342::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_342::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_342 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi342Spec {
    const RESET_VALUE: u32 = 0;
}
