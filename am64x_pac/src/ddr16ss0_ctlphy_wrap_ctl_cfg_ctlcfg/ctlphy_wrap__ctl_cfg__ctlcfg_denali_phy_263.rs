#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_263` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_263` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_OE_DLY_1` reader - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 1."]
pub type PhyLp4BootRddataEnOeDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_OE_DLY_1` writer - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 1."]
pub type PhyLp4BootRddataEnOeDly1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PHY_CTRL_LPBK_EN_1` reader - 9:8\\]
Loopback control en for slice 1."]
pub type PhyCtrlLpbkEn1R = crate::FieldReader;
#[doc = "Field `PHY_CTRL_LPBK_EN_1` writer - 9:8\\]
Loopback control en for slice 1."]
pub type PhyCtrlLpbkEn1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LPBK_CONTROL_1` reader - 24:16\\]
Loopback control bits for slice 1."]
pub type PhyLpbkControl1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_LPBK_CONTROL_1` writer - 24:16\\]
Loopback control bits for slice 1."]
pub type PhyLpbkControl1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_oe_dly_1(&self) -> PhyLp4BootRddataEnOeDly1R {
        PhyLp4BootRddataEnOeDly1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Loopback control en for slice 1."]
    #[inline(always)]
    pub fn phy_ctrl_lpbk_en_1(&self) -> PhyCtrlLpbkEn1R {
        PhyCtrlLpbkEn1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Loopback control bits for slice 1."]
    #[inline(always)]
    pub fn phy_lpbk_control_1(&self) -> PhyLpbkControl1R {
        PhyLpbkControl1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for extended OE generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_oe_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnOeDly1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec> {
        PhyLp4BootRddataEnOeDly1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Loopback control en for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ctrl_lpbk_en_1(
        &mut self,
    ) -> PhyCtrlLpbkEn1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec> {
        PhyCtrlLpbkEn1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Loopback control bits for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_1(
        &mut self,
    ) -> PhyLpbkControl1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec> {
        PhyLpbkControl1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_263\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_263::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_263::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_263::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_263::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_263 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy263Spec {
    const RESET_VALUE: u32 = 0;
}
