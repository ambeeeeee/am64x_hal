#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1401` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1401` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec>;
#[doc = "Field `PHY_PAD_CS_DRIVE` reader - 29:0\\]
Controls drive settings for cs pads."]
pub type PhyPadCsDriveR = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_CS_DRIVE` writer - 29:0\\]
Controls drive settings for cs pads."]
pub type PhyPadCsDriveW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - 29:0\\]
Controls drive settings for cs pads."]
    #[inline(always)]
    pub fn phy_pad_cs_drive(&self) -> PhyPadCsDriveR {
        PhyPadCsDriveR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - 29:0\\]
Controls drive settings for cs pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_cs_drive(
        &mut self,
    ) -> PhyPadCsDriveW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec> {
        PhyPadCsDriveW::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1401\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1401::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1401::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1401::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1401::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1401 to value 0x0255"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1401Spec {
    const RESET_VALUE: u32 = 0x0255;
}
