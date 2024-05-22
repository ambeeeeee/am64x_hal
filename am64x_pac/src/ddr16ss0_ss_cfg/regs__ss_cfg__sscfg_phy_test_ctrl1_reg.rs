#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec>;
#[doc = "Field `HVM_TEST_EN` reader - 0:0\\]
1=Enable HVM test functionality. 0=Disable HVM test functionality. Setting a 1 will enable control of PHY ports using PHY Test Control registers, and will enable 50 MHz clock to the jtag_dataout_pad_dslice_io_cfg\\[1\\], jtag_dataout_pad_acs_io_cfg\\[1\\], and jtag_dataout_pad_adr_io_cfg\\[1\\]
ports on the PHY."]
pub type HvmTestEnR = crate::BitReader;
#[doc = "Field `HVM_TEST_EN` writer - 0:0\\]
1=Enable HVM test functionality. 0=Disable HVM test functionality. Setting a 1 will enable control of PHY ports using PHY Test Control registers, and will enable 50 MHz clock to the jtag_dataout_pad_dslice_io_cfg\\[1\\], jtag_dataout_pad_acs_io_cfg\\[1\\], and jtag_dataout_pad_adr_io_cfg\\[1\\]
ports on the PHY."]
pub type HvmTestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_ENABLE` reader - 1:1\\]
Controls jtag_enable port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableR = crate::BitReader;
#[doc = "Field `JTAG_ENABLE` writer - 1:1\\]
Controls jtag_enable port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_ENABLE_DRIVE` reader - 2:2\\]
Controls jtag_enable_drive port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableDriveR = crate::BitReader;
#[doc = "Field `JTAG_ENABLE_DRIVE` writer - 2:2\\]
Controls jtag_enable_drive port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableDriveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_ENABLE_IE` reader - 3:3\\]
Controls jtag_enable_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableIeR = crate::BitReader;
#[doc = "Field `JTAG_ENABLE_IE` writer - 3:3\\]
Controls jtag_enable_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_ENABLE_OE` reader - 4:4\\]
Controls jtag_enable_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableOeR = crate::BitReader;
#[doc = "Field `JTAG_ENABLE_OE` writer - 4:4\\]
Controls jtag_enable_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_ENABLE_TERM` reader - 5:5\\]
Controls jtag_enable_term port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableTermR = crate::BitReader;
#[doc = "Field `JTAG_ENABLE_TERM` writer - 5:5\\]
Controls jtag_enable_term port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagEnableTermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_TSEL_EN` reader - 6:6\\]
Controls jtag_dataout_tsel_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselEnR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_TSEL_EN` writer - 6:6\\]
Controls jtag_dataout_tsel_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_TSEL_ADDR_EN` reader - 7:7\\]
Controls jtag_dataout_tsel_addr_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselAddrEnR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_TSEL_ADDR_EN` writer - 7:7\\]
Controls jtag_dataout_tsel_addr_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselAddrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_TSEL_ADDR_SEL` reader - 15:8\\]
Controls jtag_dataout_tsel_addr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselAddrSelR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_TSEL_ADDR_SEL` writer - 15:8\\]
Controls jtag_dataout_tsel_addr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselAddrSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `JTAG_DATAOUT_TSEL_WR_SEL` reader - 23:16\\]
Controls jtag_dataout_tsel_wr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselWrSelR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_TSEL_WR_SEL` writer - 23:16\\]
Controls jtag_dataout_tsel_wr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselWrSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `JTAG_DATAOUT_TSEL_RD_SEL` reader - 31:24\\]
Controls jtag_dataout_tsel_rd_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselRdSelR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_TSEL_RD_SEL` writer - 31:24\\]
Controls jtag_dataout_tsel_rd_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutTselRdSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1=Enable HVM test functionality. 0=Disable HVM test functionality. Setting a 1 will enable control of PHY ports using PHY Test Control registers, and will enable 50 MHz clock to the jtag_dataout_pad_dslice_io_cfg\\[1\\], jtag_dataout_pad_acs_io_cfg\\[1\\], and jtag_dataout_pad_adr_io_cfg\\[1\\]
ports on the PHY."]
    #[inline(always)]
    pub fn hvm_test_en(&self) -> HvmTestEnR {
        HvmTestEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls jtag_enable port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_enable(&self) -> JtagEnableR {
        JtagEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls jtag_enable_drive port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_enable_drive(&self) -> JtagEnableDriveR {
        JtagEnableDriveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_enable_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_enable_ie(&self) -> JtagEnableIeR {
        JtagEnableIeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls jtag_enable_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_enable_oe(&self) -> JtagEnableOeR {
        JtagEnableOeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls jtag_enable_term port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_enable_term(&self) -> JtagEnableTermR {
        JtagEnableTermR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_tsel_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_tsel_en(&self) -> JtagDataoutTselEnR {
        JtagDataoutTselEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_tsel_addr_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_tsel_addr_en(&self) -> JtagDataoutTselAddrEnR {
        JtagDataoutTselAddrEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Controls jtag_dataout_tsel_addr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_tsel_addr_sel(&self) -> JtagDataoutTselAddrSelR {
        JtagDataoutTselAddrSelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Controls jtag_dataout_tsel_wr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_tsel_wr_sel(&self) -> JtagDataoutTselWrSelR {
        JtagDataoutTselWrSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Controls jtag_dataout_tsel_rd_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_tsel_rd_sel(&self) -> JtagDataoutTselRdSelR {
        JtagDataoutTselRdSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1=Enable HVM test functionality. 0=Disable HVM test functionality. Setting a 1 will enable control of PHY ports using PHY Test Control registers, and will enable 50 MHz clock to the jtag_dataout_pad_dslice_io_cfg\\[1\\], jtag_dataout_pad_acs_io_cfg\\[1\\], and jtag_dataout_pad_adr_io_cfg\\[1\\]
ports on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn hvm_test_en(&mut self) -> HvmTestEnW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        HvmTestEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls jtag_enable port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_enable(&mut self) -> JtagEnableW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagEnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Controls jtag_enable_drive port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_enable_drive(&mut self) -> JtagEnableDriveW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagEnableDriveW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_enable_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_enable_ie(&mut self) -> JtagEnableIeW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagEnableIeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Controls jtag_enable_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_enable_oe(&mut self) -> JtagEnableOeW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagEnableOeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Controls jtag_enable_term port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_enable_term(&mut self) -> JtagEnableTermW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagEnableTermW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_tsel_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_tsel_en(
        &mut self,
    ) -> JtagDataoutTselEnW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagDataoutTselEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_tsel_addr_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_tsel_addr_en(
        &mut self,
    ) -> JtagDataoutTselAddrEnW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagDataoutTselAddrEnW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Controls jtag_dataout_tsel_addr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_tsel_addr_sel(
        &mut self,
    ) -> JtagDataoutTselAddrSelW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagDataoutTselAddrSelW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Controls jtag_dataout_tsel_wr_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_tsel_wr_sel(
        &mut self,
    ) -> JtagDataoutTselWrSelW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagDataoutTselWrSelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Controls jtag_dataout_tsel_rd_sel port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_tsel_rd_sel(
        &mut self,
    ) -> JtagDataoutTselRdSelW<Regs_SsCfg_SscfgPhyTestCtrl1RegSpec> {
        JtagDataoutTselRdSelW::new(self, 24)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL1_REG to value 0x0257_55c0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl1RegSpec {
    const RESET_VALUE: u32 = 0x0257_55c0;
}
