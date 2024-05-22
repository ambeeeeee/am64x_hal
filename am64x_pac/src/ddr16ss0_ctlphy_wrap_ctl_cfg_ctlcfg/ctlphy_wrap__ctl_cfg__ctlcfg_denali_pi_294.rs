#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_294` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_294` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec>;
#[doc = "Field `PI_MRSINGLE_DATA_1` reader - 16:0\\]
Data to program into memory mode register single write to chip select 1."]
pub type PiMrsingleData1R = crate::FieldReader<u32>;
#[doc = "Field `PI_MRSINGLE_DATA_1` writer - 16:0\\]
Data to program into memory mode register single write to chip select 1."]
pub type PiMrsingleData1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PI_ZQ_CAL_START_MAP_0` reader - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type PiZqCalStartMap0R = crate::FieldReader;
#[doc = "Field `PI_ZQ_CAL_START_MAP_0` writer - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
pub type PiZqCalStartMap0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register single write to chip select 1."]
    #[inline(always)]
    pub fn pi_mrsingle_data_1(&self) -> PiMrsingleData1R {
        PiMrsingleData1R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    pub fn pi_zq_cal_start_map_0(&self) -> PiZqCalStartMap0R {
        PiZqCalStartMap0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Data to program into memory mode register single write to chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_mrsingle_data_1(
        &mut self,
    ) -> PiMrsingleData1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec> {
        PiMrsingleData1W::new(self, 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip select\\[s\\]
will receive ZQ calibration start commands simultaneously on iteration 0 of the ZQ START initialization and periodic command sequences. Clear to all zeros for no ZQ START commands."]
    #[inline(always)]
    #[must_use]
    pub fn pi_zq_cal_start_map_0(
        &mut self,
    ) -> PiZqCalStartMap0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec> {
        PiZqCalStartMap0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_294\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_294::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_294::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_294::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_294::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_294 to value 0x0100_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi294Spec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
