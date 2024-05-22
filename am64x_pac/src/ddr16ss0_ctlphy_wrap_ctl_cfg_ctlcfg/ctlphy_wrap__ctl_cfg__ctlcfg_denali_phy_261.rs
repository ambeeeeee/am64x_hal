#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_261` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_261` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec>;
#[doc = "Field `PHY_SW_WRDM_SHIFT_1` reader - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdmShift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDM_SHIFT_1` writer - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdmShift1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_1` reader - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
pub type PhySwWrdqsShift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQS_SHIFT_1` writer - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
pub type PhySwWrdqsShift1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_1` reader - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyLp4BootRddataEnIeDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_1` writer - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyLp4BootRddataEnIeDly1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_1` reader - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyLp4BootRddataEnDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_1` writer - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyLp4BootRddataEnDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdm_shift_1(&self) -> PhySwWrdmShift1R {
        PhySwWrdmShift1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdqs_shift_1(&self) -> PhySwWrdqsShift1R {
        PhySwWrdqsShift1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_1(&self) -> PhyLp4BootRddataEnIeDly1R {
        PhyLp4BootRddataEnIeDly1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_1(&self) -> PhyLp4BootRddataEnDly1R {
        PhyLp4BootRddataEnDly1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DM for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdm_shift_1(
        &mut self,
    ) -> PhySwWrdmShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec> {
        PhySwWrdmShift1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQS for slice 1. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bit \\[3\\]
is the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdqs_shift_1(
        &mut self,
    ) -> PhySwWrdqsShift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec> {
        PhySwWrdqsShift1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec> {
        PhyLp4BootRddataEnIeDly1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec> {
        PhyLp4BootRddataEnDly1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_261\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_261::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_261::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_261::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_261::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_261 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy261Spec {
    const RESET_VALUE: u32 = 0;
}
