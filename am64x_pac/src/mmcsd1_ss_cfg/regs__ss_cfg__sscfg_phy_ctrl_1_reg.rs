#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl1RegSpec>;
#[doc = "Field `IOMUX_ENABLE` reader - 31:31\\]
IO mux enable. Set 1 for GPIO. Set 0 for eMMC/SD"]
pub type IomuxEnableR = crate::BitReader;
#[doc = "Field `IOMUX_ENABLE` writer - 31:31\\]
IO mux enable. Set 1 for GPIO. Set 0 for eMMC/SD"]
pub type IomuxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - 31:31\\]
IO mux enable. Set 1 for GPIO. Set 0 for eMMC/SD"]
    #[inline(always)]
    pub fn iomux_enable(&self) -> IomuxEnableR {
        IomuxEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
IO mux enable. Set 1 for GPIO. Set 0 for eMMC/SD"]
    #[inline(always)]
    #[must_use]
    pub fn iomux_enable(&mut self) -> IomuxEnableW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        IomuxEnableW::new(self, 31)
    }
}
#[doc = "The PHY Control 1 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG to value 0x8000_0000"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
