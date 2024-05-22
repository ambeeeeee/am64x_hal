#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1392` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1392` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec>;
#[doc = "Field `PHY_PAD_ADDR_DRIVE2` reader - 27:0\\]
Controls drive settings for the address/control pads."]
pub type PhyPadAddrDrive2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_PAD_ADDR_DRIVE2` writer - 27:0\\]
Controls drive settings for the address/control pads."]
pub type PhyPadAddrDrive2W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Controls drive settings for the address/control pads."]
    #[inline(always)]
    pub fn phy_pad_addr_drive2(&self) -> PhyPadAddrDrive2R {
        PhyPadAddrDrive2R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Controls drive settings for the address/control pads."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_addr_drive2(
        &mut self,
    ) -> PhyPadAddrDrive2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec> {
        PhyPadAddrDrive2W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1392\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1392::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1392::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1392::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1392::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1392 to value 0x1677_6960"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1392Spec {
    const RESET_VALUE: u32 = 0x1677_6960;
}
