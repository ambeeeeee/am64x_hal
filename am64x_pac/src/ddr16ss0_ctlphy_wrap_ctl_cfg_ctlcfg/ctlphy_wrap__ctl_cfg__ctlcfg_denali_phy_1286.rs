#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1286` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1286` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec>;
#[doc = "Field `PHY_GRP_BYPASS_SLAVE_DELAY` reader - 10:0\\]
Address/control group slice bypass mode slave delay setting."]
pub type PhyGrpBypassSlaveDelayR = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP_BYPASS_SLAVE_DELAY` writer - 10:0\\]
Address/control group slice bypass mode slave delay setting."]
pub type PhyGrpBypassSlaveDelayW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_SW_GRP_BYPASS_SHIFT` reader - 20:16\\]
Address/control group slice bypass mode shift settings."]
pub type PhySwGrpBypassShiftR = crate::FieldReader;
#[doc = "Field `PHY_SW_GRP_BYPASS_SHIFT` writer - 20:16\\]
Address/control group slice bypass mode shift settings."]
pub type PhySwGrpBypassShiftW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_GRP_BYPASS_OVERRIDE` reader - 24:24\\]
Address/control group slice bypass mode override setting."]
pub type PhyGrpBypassOverrideR = crate::BitReader;
#[doc = "Field `PHY_GRP_BYPASS_OVERRIDE` writer - 24:24\\]
Address/control group slice bypass mode override setting."]
pub type PhyGrpBypassOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address/control group slice bypass mode slave delay setting."]
    #[inline(always)]
    pub fn phy_grp_bypass_slave_delay(&self) -> PhyGrpBypassSlaveDelayR {
        PhyGrpBypassSlaveDelayR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address/control group slice bypass mode shift settings."]
    #[inline(always)]
    pub fn phy_sw_grp_bypass_shift(&self) -> PhySwGrpBypassShiftR {
        PhySwGrpBypassShiftR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Address/control group slice bypass mode override setting."]
    #[inline(always)]
    pub fn phy_grp_bypass_override(&self) -> PhyGrpBypassOverrideR {
        PhyGrpBypassOverrideR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Address/control group slice bypass mode slave delay setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_bypass_slave_delay(
        &mut self,
    ) -> PhyGrpBypassSlaveDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec> {
        PhyGrpBypassSlaveDelayW::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Address/control group slice bypass mode shift settings."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_grp_bypass_shift(
        &mut self,
    ) -> PhySwGrpBypassShiftW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec> {
        PhySwGrpBypassShiftW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Address/control group slice bypass mode override setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp_bypass_override(
        &mut self,
    ) -> PhyGrpBypassOverrideW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec> {
        PhyGrpBypassOverrideW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1286\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1286::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1286::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1286::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1286::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1286 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1286Spec {
    const RESET_VALUE: u32 = 0;
}
