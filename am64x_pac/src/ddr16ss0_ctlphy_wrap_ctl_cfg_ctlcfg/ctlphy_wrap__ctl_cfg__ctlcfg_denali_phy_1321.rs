#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1321` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1321` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec>;
#[doc = "Field `PHY_DS_EXIT_CTRL` reader - 16:0\\]
Controls to reduce the deep sleep exit latency, when bit 16 is 1 deep sleep exit ack won't wait master delay line lock."]
pub type PhyDsExitCtrlR = crate::FieldReader<u32>;
#[doc = "Field `PHY_DS_EXIT_CTRL` writer - 16:0\\]
Controls to reduce the deep sleep exit latency, when bit 16 is 1 deep sleep exit ack won't wait master delay line lock."]
pub type PhyDsExitCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `PHY_TDFI_PHY_WRDELAY` reader - 24:24\\]
DFI timing parameter TDFI_PHY_WRDELAY."]
pub type PhyTdfiPhyWrdelayR = crate::BitReader;
#[doc = "Field `PHY_TDFI_PHY_WRDELAY` writer - 24:24\\]
DFI timing parameter TDFI_PHY_WRDELAY."]
pub type PhyTdfiPhyWrdelayW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:16 - 16:0\\]
Controls to reduce the deep sleep exit latency, when bit 16 is 1 deep sleep exit ack won't wait master delay line lock."]
    #[inline(always)]
    pub fn phy_ds_exit_ctrl(&self) -> PhyDsExitCtrlR {
        PhyDsExitCtrlR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
DFI timing parameter TDFI_PHY_WRDELAY."]
    #[inline(always)]
    pub fn phy_tdfi_phy_wrdelay(&self) -> PhyTdfiPhyWrdelayR {
        PhyTdfiPhyWrdelayR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - 16:0\\]
Controls to reduce the deep sleep exit latency, when bit 16 is 1 deep sleep exit ack won't wait master delay line lock."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ds_exit_ctrl(
        &mut self,
    ) -> PhyDsExitCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec> {
        PhyDsExitCtrlW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
DFI timing parameter TDFI_PHY_WRDELAY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_tdfi_phy_wrdelay(
        &mut self,
    ) -> PhyTdfiPhyWrdelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec> {
        PhyTdfiPhyWrdelayW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1321\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1321::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1321::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1321::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1321::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1321 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1321Spec {
    const RESET_VALUE: u32 = 0;
}
