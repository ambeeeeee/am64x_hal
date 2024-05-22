#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec>;
#[doc = "Field `JTAG_DATAOUT_RESET_N_IE` reader - 0:0\\]
Controls jtag_dataout_reset_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RESET_N_IE` writer - 0:0\\]
Controls jtag_dataout_reset_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CKE_IE` reader - 2:1\\]
Controls jtag_dataout_cke_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CKE_IE` writer - 2:1\\]
Controls jtag_dataout_cke_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0_IE` reader - 3:3\\]
Controls jtag_dataout_mem_clk_0_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0IeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0_IE` writer - 3:3\\]
Controls jtag_dataout_mem_clk_0_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CS_N_IE` reader - 5:4\\]
Controls jtag_dataout_cs_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CS_N_IE` writer - 5:4\\]
Controls jtag_dataout_cs_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ACT_N_IE` reader - 6:6\\]
Controls jtag_dataout_act_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ACT_N_IE` writer - 6:6\\]
Controls jtag_dataout_act_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_RAS_N_IE` reader - 7:7\\]
Controls jtag_dataout_ras_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RAS_N_IE` writer - 7:7\\]
Controls jtag_dataout_ras_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CAS_N_IE` reader - 8:8\\]
Controls jtag_dataout_cas_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_CAS_N_IE` writer - 8:8\\]
Controls jtag_dataout_cas_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_WE_N_IE` reader - 9:9\\]
Controls jtag_dataout_we_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_WE_N_IE` writer - 9:9\\]
Controls jtag_dataout_we_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_BG_IE` reader - 11:10\\]
Controls jtag_dataout_bg_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BG_IE` writer - 11:10\\]
Controls jtag_dataout_bg_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_BANK_IE` reader - 13:12\\]
Controls jtag_dataout_bank_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BANK_IE` writer - 13:12\\]
Controls jtag_dataout_bank_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS_IE` reader - 27:14\\]
Controls jtag_dataout_address_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressIeR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS_IE` writer - 27:14\\]
Controls jtag_dataout_address_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressIeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `JTAG_DATAOUT_ODT_IE` reader - 29:28\\]
Controls jtag_dataout_odt_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_ODT_IE` writer - 29:28\\]
Controls jtag_dataout_odt_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN_IE` reader - 30:30\\]
Controls jtag_dataout_parity_in_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN_IE` writer - 30:30\\]
Controls jtag_dataout_parity_in_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_ERROR_N_IE` reader - 31:31\\]
Controls jtag_dataout_error_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNIeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ERROR_N_IE` writer - 31:31\\]
Controls jtag_dataout_error_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_reset_n_ie(&self) -> JtagDataoutResetNIeR {
        JtagDataoutResetNIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cke_ie(&self) -> JtagDataoutCkeIeR {
        JtagDataoutCkeIeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_mem_clk_0_ie(&self) -> JtagDataoutMemClk0IeR {
        JtagDataoutMemClk0IeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cs_n_ie(&self) -> JtagDataoutCsNIeR {
        JtagDataoutCsNIeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_act_n_ie(&self) -> JtagDataoutActNIeR {
        JtagDataoutActNIeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_ras_n_ie(&self) -> JtagDataoutRasNIeR {
        JtagDataoutRasNIeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cas_n_ie(&self) -> JtagDataoutCasNIeR {
        JtagDataoutCasNIeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_we_n_ie(&self) -> JtagDataoutWeNIeR {
        JtagDataoutWeNIeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bg_ie(&self) -> JtagDataoutBgIeR {
        JtagDataoutBgIeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bank_ie(&self) -> JtagDataoutBankIeR {
        JtagDataoutBankIeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_address_ie(&self) -> JtagDataoutAddressIeR {
        JtagDataoutAddressIeR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_odt_ie(&self) -> JtagDataoutOdtIeR {
        JtagDataoutOdtIeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_parity_in_ie(&self) -> JtagDataoutParityInIeR {
        JtagDataoutParityInIeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_error_n_ie(&self) -> JtagDataoutErrorNIeR {
        JtagDataoutErrorNIeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_reset_n_ie(
        &mut self,
    ) -> JtagDataoutResetNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutResetNIeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cke_ie(
        &mut self,
    ) -> JtagDataoutCkeIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutCkeIeW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_mem_clk_0_ie(
        &mut self,
    ) -> JtagDataoutMemClk0IeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutMemClk0IeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cs_n_ie(
        &mut self,
    ) -> JtagDataoutCsNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutCsNIeW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_act_n_ie(
        &mut self,
    ) -> JtagDataoutActNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutActNIeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_ras_n_ie(
        &mut self,
    ) -> JtagDataoutRasNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutRasNIeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cas_n_ie(
        &mut self,
    ) -> JtagDataoutCasNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutCasNIeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_we_n_ie(
        &mut self,
    ) -> JtagDataoutWeNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutWeNIeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bg_ie(&mut self) -> JtagDataoutBgIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutBgIeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bank_ie(
        &mut self,
    ) -> JtagDataoutBankIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutBankIeW::new(self, 12)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_address_ie(
        &mut self,
    ) -> JtagDataoutAddressIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutAddressIeW::new(self, 14)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_odt_ie(
        &mut self,
    ) -> JtagDataoutOdtIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutOdtIeW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_parity_in_ie(
        &mut self,
    ) -> JtagDataoutParityInIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutParityInIeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_error_n_ie(
        &mut self,
    ) -> JtagDataoutErrorNIeW<Regs_SsCfg_SscfgPhyTestCtrl8RegSpec> {
        JtagDataoutErrorNIeW::new(self, 31)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl8RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl8RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl8RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl8_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl8RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL8_REG to value 0xf8e0_ffff"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl8RegSpec {
    const RESET_VALUE: u32 = 0xf8e0_ffff;
}
