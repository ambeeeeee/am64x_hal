#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1288` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1288` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec>;
#[doc = "Field `PHY_CSLVL_COARSE_DLY` reader - 10:0\\]
Defines the CS training DDL coarse cycle delay value."]
pub type PhyCslvlCoarseDlyR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_COARSE_DLY` writer - 10:0\\]
Defines the CS training DDL coarse cycle delay value."]
pub type PhyCslvlCoarseDlyW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_CSLVL_DEBUG_MODE` reader - 16:16\\]
Enables CS training debug mode. Set to 1 to enable."]
pub type PhyCslvlDebugModeR = crate::BitReader;
#[doc = "Field `PHY_CSLVL_DEBUG_MODE` writer - 16:16\\]
Enables CS training debug mode. Set to 1 to enable."]
pub type PhyCslvlDebugModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC_PHY_CSLVL_DEBUG_CONT` reader - 24:24\\]
Allows the CS training state machine to advance \\[when in debug mode\\]. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyCslvlDebugContR = crate::BitReader;
#[doc = "Field `SC_PHY_CSLVL_DEBUG_CONT` writer - 24:24\\]
Allows the CS training state machine to advance \\[when in debug mode\\]. Set to 1 to trigger. WRITE-ONLY"]
pub type ScPhyCslvlDebugContW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the CS training DDL coarse cycle delay value."]
    #[inline(always)]
    pub fn phy_cslvl_coarse_dly(&self) -> PhyCslvlCoarseDlyR {
        PhyCslvlCoarseDlyR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables CS training debug mode. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_cslvl_debug_mode(&self) -> PhyCslvlDebugModeR {
        PhyCslvlDebugModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Allows the CS training state machine to advance \\[when in debug mode\\]. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn sc_phy_cslvl_debug_cont(&self) -> ScPhyCslvlDebugContR {
        ScPhyCslvlDebugContR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Defines the CS training DDL coarse cycle delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_coarse_dly(
        &mut self,
    ) -> PhyCslvlCoarseDlyW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec> {
        PhyCslvlCoarseDlyW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables CS training debug mode. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_debug_mode(
        &mut self,
    ) -> PhyCslvlDebugModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec> {
        PhyCslvlDebugModeW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Allows the CS training state machine to advance \\[when in debug mode\\]. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn sc_phy_cslvl_debug_cont(
        &mut self,
    ) -> ScPhyCslvlDebugContW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec> {
        ScPhyCslvlDebugContW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1288\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1288::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1288::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1288::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1288::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1288 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1288Spec {
    const RESET_VALUE: u32 = 0;
}
