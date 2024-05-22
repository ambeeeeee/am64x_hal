#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_5` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_5` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec>;
#[doc = "Field `PHY_SW_WRDM_SHIFT_0` reader - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdmShift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDM_SHIFT_0` writer - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdmShift0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_0` reader - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
pub type PhySwWrdqsShift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_0` writer - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
pub type PhySwWrdqsShift0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_0` reader - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyLp4BootRddataEnIeDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_0` writer - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyLp4BootRddataEnIeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_0` reader - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyLp4BootRddataEnDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_0` writer - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyLp4BootRddataEnDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdm_shift_0(&self) -> PhySwWrdmShift0R {
        PhySwWrdmShift0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdqs_shift_0(&self) -> PhySwWrdqsShift0R {
        PhySwWrdqsShift0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_0(&self) -> PhyLp4BootRddataEnIeDly0R {
        PhyLp4BootRddataEnIeDly0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_0(&self) -> PhyLp4BootRddataEnDly0R {
        PhyLp4BootRddataEnDly0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdm_shift_0(
        &mut self,
    ) -> PhySwWrdmShift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec> {
        PhySwWrdmShift0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdqs_shift_0(
        &mut self,
    ) -> PhySwWrdqsShift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec> {
        PhySwWrdqsShift0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec> {
        PhyLp4BootRddataEnIeDly0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec> {
        PhyLp4BootRddataEnDly0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_5::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_5::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_5 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy5Spec {
    const RESET_VALUE: u32 = 0;
}
