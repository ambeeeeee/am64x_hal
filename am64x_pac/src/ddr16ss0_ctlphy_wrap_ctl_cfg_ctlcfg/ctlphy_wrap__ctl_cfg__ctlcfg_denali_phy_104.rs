#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_104` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_104` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec>;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_0` reader - 3:0\\]
DQ/DM bit swizzling 1 for slice 0. Bits \\[3:0\\]
inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_0R = crate::FieldReader;
#[doc = "Field `PHY_DQ_DM_SWIZZLE1_0` writer - 3:0\\]
DQ/DM bit swizzling 1 for slice 0. Bits \\[3:0\\]
inform the PHY which bit in {DM,DQ\\]} map to DM."]
pub type PhyDqDmSwizzle1_0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DQ/DM bit swizzling 1 for slice 0. Bits \\[3:0\\]
inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    pub fn phy_dq_dm_swizzle1_0(&self) -> PhyDqDmSwizzle1_0R {
        PhyDqDmSwizzle1_0R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DQ/DM bit swizzling 1 for slice 0. Bits \\[3:0\\]
inform the PHY which bit in {DM,DQ\\]} map to DM."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dq_dm_swizzle1_0(
        &mut self,
    ) -> PhyDqDmSwizzle1_0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec> {
        PhyDqDmSwizzle1_0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_104::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_104::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_104 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy104Spec {
    const RESET_VALUE: u32 = 0;
}
