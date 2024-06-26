#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1283` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1283` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec>;
#[doc = "Field `PHY_SW_GRP1_SHIFT_1` reader - 4:0\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp1Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP1_SHIFT_1` writer - 4:0\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp1Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP2_SHIFT_1` reader - 12:8\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp2Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP2_SHIFT_1` writer - 12:8\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp2Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP3_SHIFT_1` reader - 20:16\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp3Shift1R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP3_SHIFT_1` writer - 20:16\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp3Shift1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_SW_GRP0_SHIFT_2` reader - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift2R = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP0_SHIFT_2` writer - 28:24\\]
Address slice slave delay setting for address slice 4."]
pub type PhySwGrp0Shift2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp1_shift_1(&self) -> PhySwGrp1Shift1R {
        PhySwGrp1Shift1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp2_shift_1(&self) -> PhySwGrp2Shift1R {
        PhySwGrp2Shift1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp3_shift_1(&self) -> PhySwGrp3Shift1R {
        PhySwGrp3Shift1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    pub fn phy_sw_grp0_shift_2(&self) -> PhySwGrp0Shift2R {
        PhySwGrp0Shift2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp1_shift_1(
        &mut self,
    ) -> PhySwGrp1Shift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec> {
        PhySwGrp1Shift1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp2_shift_1(
        &mut self,
    ) -> PhySwGrp2Shift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec> {
        PhySwGrp2Shift1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp3_shift_1(
        &mut self,
    ) -> PhySwGrp3Shift1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec> {
        PhySwGrp3Shift1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Address slice slave delay setting for address slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp0_shift_2(
        &mut self,
    ) -> PhySwGrp0Shift2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec> {
        PhySwGrp0Shift2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1283\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1283::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1283::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1283::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1283::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1283 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1283Spec {
    const RESET_VALUE: u32 = 0;
}
