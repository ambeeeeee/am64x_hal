#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1393` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1393` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec>;
#[doc = "Field `PHY_PAD_CLK_DRIVE` reader - 31:0\\]
Controls drive settings for clock pads."]
pub type PhyPadClkDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CLK_DRIVE` writer - 31:0\\]
Controls drive settings for clock pads."]
pub type PhyPadClkDriveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Controls drive settings for clock pads."]
    #[inline(always)]
    pub fn phy_pad_clk_drive(&self) -> PhyPadClkDriveR {
        PhyPadClkDriveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Controls drive settings for clock pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_clk_drive(
        &mut self,
    ) -> PhyPadClkDriveW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec> {
        PhyPadClkDriveW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1393\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1393::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1393::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1393::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1393::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1393 to value 0x0255"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1393Spec {
    const RESET_VALUE: u32 = 0x0255;
}
