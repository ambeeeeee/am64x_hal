#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_4` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_4` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec>;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_0` reader - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq4Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ4_SHIFT_0` writer - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq4Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_0` reader - 13:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq5Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ5_SHIFT_0` writer - 13:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq5Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_0` reader - 21:16\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq6Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ6_SHIFT_0` writer - 21:16\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq6Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_0` reader - 29:24\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq7Shift0R = crate::FieldReader;
#[doc = "Field `PHY_SW_WRDQ7_SHIFT_0` writer - 29:24\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
pub type PhySwWrdq7Shift0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq4_shift_0(&self) -> PhySwWrdq4Shift0R {
        PhySwWrdq4Shift0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq5_shift_0(&self) -> PhySwWrdq5Shift0R {
        PhySwWrdq5Shift0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq6_shift_0(&self) -> PhySwWrdq6Shift0R {
        PhySwWrdq6Shift0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    pub fn phy_sw_wrdq7_shift_0(&self) -> PhySwWrdq7Shift0R {
        PhySwWrdq7Shift0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ4 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq4_shift_0(
        &mut self,
    ) -> PhySwWrdq4Shift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec> {
        PhySwWrdq4Shift0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ5 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq5_shift_0(
        &mut self,
    ) -> PhySwWrdq5Shift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec> {
        PhySwWrdq5Shift0W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ6 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq6_shift_0(
        &mut self,
    ) -> PhySwWrdq6Shift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec> {
        PhySwWrdq6Shift0W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Manual override of automatic half_cycle_shift/cycle_shift for write DQ7 for slice 0. Bit \\[0\\]
enables override of half_cycle_shift. Bit \\[1\\]
is the half_cycle_shift value. Bit \\[2\\]
enables override of cycle shift. Bits \\[4:3\\]
are the cycle_shift value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_wrdq7_shift_0(
        &mut self,
    ) -> PhySwWrdq7Shift0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec> {
        PhySwWrdq7Shift0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_4::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_4::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_4 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy4Spec {
    const RESET_VALUE: u32 = 0;
}
