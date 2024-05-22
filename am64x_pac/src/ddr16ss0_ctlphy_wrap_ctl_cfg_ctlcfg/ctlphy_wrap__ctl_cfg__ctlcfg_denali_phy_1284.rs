#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1284` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1284` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec>;
#[doc = "Field `PHY_SW_GRP1_SHIFT_2` reader - 4:0\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp1Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP1_SHIFT_2` writer - 4:0\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp1Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP2_SHIFT_2` reader - 12:8\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp2Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP2_SHIFT_2` writer - 12:8\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp2Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP3_SHIFT_2` reader - 20:16\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp3Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP3_SHIFT_2` writer - 20:16\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp3Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP0_SHIFT_3` reader - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift3R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP0_SHIFT_3` writer - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp1_shift_2(&self) -> PhySwGrp1Shift2R {
        PhySwGrp1Shift2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp2_shift_2(&self) -> PhySwGrp2Shift2R {
        PhySwGrp2Shift2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp3_shift_2(&self) -> PhySwGrp3Shift2R {
        PhySwGrp3Shift2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp0_shift_3(&self) -> PhySwGrp0Shift3R {
        PhySwGrp0Shift3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp1_shift_2(
        &mut self,
    ) -> PhySwGrp1Shift2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec> {
        PhySwGrp1Shift2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp2_shift_2(
        &mut self,
    ) -> PhySwGrp2Shift2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec> {
        PhySwGrp2Shift2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp3_shift_2(
        &mut self,
    ) -> PhySwGrp3Shift2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec> {
        PhySwGrp3Shift2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp0_shift_3(
        &mut self,
    ) -> PhySwGrp0Shift3W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec> {
        PhySwGrp0Shift3W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1284\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1284::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1284::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1284::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1284::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1284 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1284Spec {
    const RESET_VALUE: u32 = 0;
}
