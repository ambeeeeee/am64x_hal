#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1396` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1396` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec>;
#[doc = "Field `PHY_PAD_ERR_DRIVE2` reader - 27:0\\]
Controls drive settings for error pads."]
pub type PhyPadErrDrive2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_ERR_DRIVE2` writer - 27:0\\]
Controls drive settings for error pads."]
pub type PhyPadErrDrive2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Controls drive settings for error pads."]
    #[inline(always)]
    pub fn phy_pad_err_drive2(&self) -> PhyPadErrDrive2R {
        PhyPadErrDrive2R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Controls drive settings for error pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_err_drive2(
        &mut self,
    ) -> PhyPadErrDrive2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec> {
        PhyPadErrDrive2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1396\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1396::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1396::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1396::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1396::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1396 to value 0x1677_6960"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1396Spec {
    const RESET_VALUE: u32 = 0x1677_6960;
}
