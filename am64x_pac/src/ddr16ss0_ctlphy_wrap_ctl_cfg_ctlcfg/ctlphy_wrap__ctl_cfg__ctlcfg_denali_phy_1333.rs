#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1333` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1333` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec>;
#[doc = "Field `PHY_CAL_MODE_0` reader - 12:0\\]
Pad calibration mode bits for block 0. Bit \\[0\\]
disables pad calibration upon initialization. Bit \\[1\\]
enables automatic interval based calibration. Bits \\[3:2\\]
set the base interval for the interval counter. Bits \\[7:4\\]
are direct connections to pad control signals."]
pub type PhyCalMode0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_CAL_MODE_0` writer - 12:0\\]
Pad calibration mode bits for block 0. Bit \\[0\\]
disables pad calibration upon initialization. Bit \\[1\\]
enables automatic interval based calibration. Bits \\[3:2\\]
set the base interval for the interval counter. Bits \\[7:4\\]
are direct connections to pad control signals."]
pub type PhyCalMode0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `PHY_CAL_CLEAR_0` reader - 16:16\\]
Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalClear0R = crate::BitReader;
#[doc = "Field `PHY_CAL_CLEAR_0` writer - 16:16\\]
Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalClear0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CAL_START_0` reader - 24:24\\]
Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalStart0R = crate::BitReader;
#[doc = "Field `PHY_CAL_START_0` writer - 24:24\\]
Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
pub type PhyCalStart0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Pad calibration mode bits for block 0. Bit \\[0\\]
disables pad calibration upon initialization. Bit \\[1\\]
enables automatic interval based calibration. Bits \\[3:2\\]
set the base interval for the interval counter. Bits \\[7:4\\]
are direct connections to pad control signals."]
    #[inline(always)]
    pub fn phy_cal_mode_0(&self) -> PhyCalMode0R {
        PhyCalMode0R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn phy_cal_clear_0(&self) -> PhyCalClear0R {
        PhyCalClear0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn phy_cal_start_0(&self) -> PhyCalStart0R {
        PhyCalStart0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Pad calibration mode bits for block 0. Bit \\[0\\]
disables pad calibration upon initialization. Bit \\[1\\]
enables automatic interval based calibration. Bits \\[3:2\\]
set the base interval for the interval counter. Bits \\[7:4\\]
are direct connections to pad control signals."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_mode_0(&mut self) -> PhyCalMode0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec> {
        PhyCalMode0W::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear the pad calibration state machine and results for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_clear_0(&mut self) -> PhyCalClear0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec> {
        PhyCalClear0W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Manual start for the pad calibration state machine for block 0. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_cal_start_0(&mut self) -> PhyCalStart0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec> {
        PhyCalStart0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1333\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1333::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1333::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1333::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1333::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1333 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1333Spec {
    const RESET_VALUE: u32 = 0;
}
