#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec>;
#[doc = "Field `JTAG_DATAOUT_RESET_N` reader - 0:0\\]
Controls jtag_dataout_reset_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RESET_N` writer - 0:0\\]
Controls jtag_dataout_reset_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CKE` reader - 2:1\\]
Controls jtag_dataout_cke port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CKE` writer - 2:1\\]
Controls jtag_dataout_cke port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0` reader - 3:3\\]
Controls jtag_dataout_mem_clk_0 port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0R = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0` writer - 3:3\\]
Controls jtag_dataout_mem_clk_0 port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CS_N` reader - 5:4\\]
Controls jtag_dataout_cs_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CS_N` writer - 5:4\\]
Controls jtag_dataout_cs_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ACT_N` reader - 6:6\\]
Controls jtag_dataout_act_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ACT_N` writer - 6:6\\]
Controls jtag_dataout_act_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_RAS_N` reader - 7:7\\]
Controls jtag_dataout_ras_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RAS_N` writer - 7:7\\]
Controls jtag_dataout_ras_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CAS_N` reader - 8:8\\]
Controls jtag_dataout_cas_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_CAS_N` writer - 8:8\\]
Controls jtag_dataout_cas_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_WE_N` reader - 9:9\\]
Controls jtag_dataout_we_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_WE_N` writer - 9:9\\]
Controls jtag_dataout_we_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_BG` reader - 11:10\\]
Controls jtag_dataout_bg port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BG` writer - 11:10\\]
Controls jtag_dataout_bg port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_BANK` reader - 13:12\\]
Controls jtag_dataout_bank port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BANK` writer - 13:12\\]
Controls jtag_dataout_bank port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS` reader - 27:14\\]
Controls jtag_dataout_address port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS` writer - 27:14\\]
Controls jtag_dataout_address port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `JTAG_DATAOUT_ODT` reader - 29:28\\]
Controls jtag_dataout_odt port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_ODT` writer - 29:28\\]
Controls jtag_dataout_odt port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN` reader - 30:30\\]
Controls jtag_dataout_parity_in port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN` writer - 30:30\\]
Controls jtag_dataout_parity_in port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_ERROR_N` reader - 31:31\\]
Controls jtag_dataout_error_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ERROR_N` writer - 31:31\\]
Controls jtag_dataout_error_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_reset_n(&self) -> JtagDataoutResetNR {
        JtagDataoutResetNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cke(&self) -> JtagDataoutCkeR {
        JtagDataoutCkeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0 port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_mem_clk_0(&self) -> JtagDataoutMemClk0R {
        JtagDataoutMemClk0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cs_n(&self) -> JtagDataoutCsNR {
        JtagDataoutCsNR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_act_n(&self) -> JtagDataoutActNR {
        JtagDataoutActNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_ras_n(&self) -> JtagDataoutRasNR {
        JtagDataoutRasNR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cas_n(&self) -> JtagDataoutCasNR {
        JtagDataoutCasNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_we_n(&self) -> JtagDataoutWeNR {
        JtagDataoutWeNR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bg(&self) -> JtagDataoutBgR {
        JtagDataoutBgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bank(&self) -> JtagDataoutBankR {
        JtagDataoutBankR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_address(&self) -> JtagDataoutAddressR {
        JtagDataoutAddressR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_odt(&self) -> JtagDataoutOdtR {
        JtagDataoutOdtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_parity_in(&self) -> JtagDataoutParityInR {
        JtagDataoutParityInR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_error_n(&self) -> JtagDataoutErrorNR {
        JtagDataoutErrorNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_reset_n(
        &mut self,
    ) -> JtagDataoutResetNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutResetNW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cke(&mut self) -> JtagDataoutCkeW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutCkeW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0 port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_mem_clk_0(
        &mut self,
    ) -> JtagDataoutMemClk0W<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutMemClk0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cs_n(&mut self) -> JtagDataoutCsNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutCsNW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_act_n(&mut self) -> JtagDataoutActNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutActNW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_ras_n(&mut self) -> JtagDataoutRasNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutRasNW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cas_n(&mut self) -> JtagDataoutCasNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutCasNW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_we_n(&mut self) -> JtagDataoutWeNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutWeNW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bg(&mut self) -> JtagDataoutBgW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutBgW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bank(&mut self) -> JtagDataoutBankW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutBankW::new(self, 12)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_address(
        &mut self,
    ) -> JtagDataoutAddressW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutAddressW::new(self, 14)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_odt(&mut self) -> JtagDataoutOdtW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutOdtW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_parity_in(
        &mut self,
    ) -> JtagDataoutParityInW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutParityInW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_error_n(
        &mut self,
    ) -> JtagDataoutErrorNW<Regs_SsCfg_SscfgPhyTestCtrl6RegSpec> {
        JtagDataoutErrorNW::new(self, 31)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl6RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl6RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl6RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl6_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl6RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL6_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl6RegSpec {
    const RESET_VALUE: u32 = 0;
}
