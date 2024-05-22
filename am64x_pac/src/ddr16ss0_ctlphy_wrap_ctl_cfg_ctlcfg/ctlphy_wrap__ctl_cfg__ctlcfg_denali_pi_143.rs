#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_143` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_143` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec>;
#[doc = "Field `PI_PERIPHERAL_MRR_DATA_0` reader - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter, Bits \\[15:0\\]
define MRR data, \\[23:16\\]
define the chip select. READ-ONLY"]
pub type PiPeripheralMrrData0R = crate::FieldReader<u32>;
#[doc = "Field `PI_PERIPHERAL_MRR_DATA_0` writer - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter, Bits \\[15:0\\]
define MRR data, \\[23:16\\]
define the chip select. READ-ONLY"]
pub type PiPeripheralMrrData0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PI_NO_ZQ_INIT` reader - 24:24\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
pub type PiNoZqInitR = crate::BitReader;
#[doc = "Field `PI_NO_ZQ_INIT` writer - 24:24\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
pub type PiNoZqInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter, Bits \\[15:0\\]
define MRR data, \\[23:16\\]
define the chip select. READ-ONLY"]
    #[inline(always)]
    pub fn pi_peripheral_mrr_data_0(&self) -> PiPeripheralMrrData0R {
        PiPeripheralMrrData0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn pi_no_zq_init(&self) -> PiNoZqInitR {
        PiNoZqInitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Data and chip returned from memory mode register read requested by the READ_MODEREG parameter, Bits \\[15:0\\]
define MRR data, \\[23:16\\]
define the chip select. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_peripheral_mrr_data_0(
        &mut self,
    ) -> PiPeripheralMrrData0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec> {
        PiPeripheralMrrData0W::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable ZQ operations during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_zq_init(&mut self) -> PiNoZqInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec> {
        PiNoZqInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_143\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_143::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_143::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_143::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_143::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_143 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi143Spec {
    const RESET_VALUE: u32 = 0;
}
