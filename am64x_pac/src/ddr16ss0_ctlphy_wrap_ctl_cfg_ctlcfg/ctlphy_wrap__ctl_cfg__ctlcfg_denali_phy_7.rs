#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_7` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_7` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_OE_DLY_0` reader - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 0."]
pub type PhyLp4BootRddataEnOeDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_OE_DLY_0` writer - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 0."]
pub type PhyLp4BootRddataEnOeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_CTRL_LPBK_EN_0` reader - 9:8\\]
Loopback control en for slice 0."]
pub type PhyCtrlLpbkEn0R = crate::FieldReader;
#[doc = "Field `PHY_CTRL_LPBK_EN_0` writer - 9:8\\]
Loopback control en for slice 0."]
pub type PhyCtrlLpbkEn0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LPBK_CONTROL_0` reader - 24:16\\]
Loopback control bits for slice 0."]
pub type PhyLpbkControl0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LPBK_CONTROL_0` writer - 24:16\\]
Loopback control bits for slice 0."]
pub type PhyLpbkControl0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_oe_dly_0(&self) -> PhyLp4BootRddataEnOeDly0R {
        PhyLp4BootRddataEnOeDly0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Loopback control en for slice 0."]
    #[inline(always)]
    pub fn phy_ctrl_lpbk_en_0(&self) -> PhyCtrlLpbkEn0R {
        PhyCtrlLpbkEn0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Loopback control bits for slice 0."]
    #[inline(always)]
    pub fn phy_lpbk_control_0(&self) -> PhyLpbkControl0R {
        PhyLpbkControl0R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_oe_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnOeDly0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec> {
        PhyLp4BootRddataEnOeDly0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Loopback control en for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ctrl_lpbk_en_0(
        &mut self,
    ) -> PhyCtrlLpbkEn0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec> {
        PhyCtrlLpbkEn0W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Loopback control bits for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_0(
        &mut self,
    ) -> PhyLpbkControl0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec> {
        PhyLpbkControl0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_7::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_7::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_7 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy7Spec {
    const RESET_VALUE: u32 = 0;
}
