#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_6_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl6RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_6_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl6RegSpec>;
#[doc = "Field `TESTCTRL` reader - 7:0\\]
PHY test control: 8'b00010000 => Test EMMC IOs sink impedance, 8'b00010001 => Test EMMC IOs source impedance, 8'b00100000 => Test RX clock phases on data lines, 8'b00110000 => Test TX clock phases on data lines, 8'b01000000 => Test STRB clock phases on data lines."]
pub type TestctrlR = crate::FieldReader;
#[doc = "Field `TESTCTRL` writer - 7:0\\]
PHY test control: 8'b00010000 => Test EMMC IOs sink impedance, 8'b00010001 => Test EMMC IOs source impedance, 8'b00100000 => Test RX clock phases on data lines, 8'b00110000 => Test TX clock phases on data lines, 8'b01000000 => Test STRB clock phases on data lines."]
pub type TestctrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BISTMODE` reader - 27:24\\]
Internal BIST mode Select. Select the embedded BIST mode of operation."]
pub type BistmodeR = crate::FieldReader;
#[doc = "Field `BISTMODE` writer - 27:24\\]
Internal BIST mode Select. Select the embedded BIST mode of operation."]
pub type BistmodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BISTSTART` reader - 30:30\\]
Internal BIST start. Starts the embedded BIST operation."]
pub type BiststartR = crate::BitReader;
#[doc = "Field `BISTSTART` writer - 30:30\\]
Internal BIST start. Starts the embedded BIST operation."]
pub type BiststartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTENABLE` reader - 31:31\\]
Internal BIST operation enable. Enables the embedded BIST."]
pub type BistenableR = crate::BitReader;
#[doc = "Field `BISTENABLE` writer - 31:31\\]
Internal BIST operation enable. Enables the embedded BIST."]
pub type BistenableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
PHY test control: 8'b00010000 => Test EMMC IOs sink impedance, 8'b00010001 => Test EMMC IOs source impedance, 8'b00100000 => Test RX clock phases on data lines, 8'b00110000 => Test TX clock phases on data lines, 8'b01000000 => Test STRB clock phases on data lines."]
    #[inline(always)]
    pub fn testctrl(&self) -> TestctrlR {
        TestctrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal BIST mode Select. Select the embedded BIST mode of operation."]
    #[inline(always)]
    pub fn bistmode(&self) -> BistmodeR {
        BistmodeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal BIST start. Starts the embedded BIST operation."]
    #[inline(always)]
    pub fn biststart(&self) -> BiststartR {
        BiststartR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal BIST operation enable. Enables the embedded BIST."]
    #[inline(always)]
    pub fn bistenable(&self) -> BistenableR {
        BistenableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
PHY test control: 8'b00010000 => Test EMMC IOs sink impedance, 8'b00010001 => Test EMMC IOs source impedance, 8'b00100000 => Test RX clock phases on data lines, 8'b00110000 => Test TX clock phases on data lines, 8'b01000000 => Test STRB clock phases on data lines."]
    #[inline(always)]
    #[must_use]
    pub fn testctrl(&mut self) -> TestctrlW<Regs_SsCfg_SscfgPhyCtrl6RegSpec> {
        TestctrlW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal BIST mode Select. Select the embedded BIST mode of operation."]
    #[inline(always)]
    #[must_use]
    pub fn bistmode(&mut self) -> BistmodeW<Regs_SsCfg_SscfgPhyCtrl6RegSpec> {
        BistmodeW::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal BIST start. Starts the embedded BIST operation."]
    #[inline(always)]
    #[must_use]
    pub fn biststart(&mut self) -> BiststartW<Regs_SsCfg_SscfgPhyCtrl6RegSpec> {
        BiststartW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal BIST operation enable. Enables the embedded BIST."]
    #[inline(always)]
    #[must_use]
    pub fn bistenable(&mut self) -> BistenableW<Regs_SsCfg_SscfgPhyCtrl6RegSpec> {
        BistenableW::new(self, 31)
    }
}
#[doc = "The PHY Control 6 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl6RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl6RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl6RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_6_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl6RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_6_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl6RegSpec {
    const RESET_VALUE: u32 = 0;
}
