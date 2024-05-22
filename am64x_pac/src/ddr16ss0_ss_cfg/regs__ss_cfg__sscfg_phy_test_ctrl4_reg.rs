#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec>;
#[doc = "Field `JTAG_DATAOUT_RESET_N_OE` reader - 0:0\\]
Controls jtag_dataout_reset_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RESET_N_OE` writer - 0:0\\]
Controls jtag_dataout_reset_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutResetNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CKE_OE` reader - 2:1\\]
Controls jtag_dataout_cke_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CKE_OE` writer - 2:1\\]
Controls jtag_dataout_cke_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCkeOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0_OE` reader - 3:3\\]
Controls jtag_dataout_mem_clk_0_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0OeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_MEM_CLK_0_OE` writer - 3:3\\]
Controls jtag_dataout_mem_clk_0_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutMemClk0OeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CS_N_OE` reader - 5:4\\]
Controls jtag_dataout_cs_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_CS_N_OE` writer - 5:4\\]
Controls jtag_dataout_cs_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCsNOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ACT_N_OE` reader - 6:6\\]
Controls jtag_dataout_act_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ACT_N_OE` writer - 6:6\\]
Controls jtag_dataout_act_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutActNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_RAS_N_OE` reader - 7:7\\]
Controls jtag_dataout_ras_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_RAS_N_OE` writer - 7:7\\]
Controls jtag_dataout_ras_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutRasNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_CAS_N_OE` reader - 8:8\\]
Controls jtag_dataout_cas_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_CAS_N_OE` writer - 8:8\\]
Controls jtag_dataout_cas_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutCasNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_WE_N_OE` reader - 9:9\\]
Controls jtag_dataout_we_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_WE_N_OE` writer - 9:9\\]
Controls jtag_dataout_we_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutWeNOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_BG_OE` reader - 11:10\\]
Controls jtag_dataout_bg_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BG_OE` writer - 11:10\\]
Controls jtag_dataout_bg_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBgOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_BANK_OE` reader - 13:12\\]
Controls jtag_dataout_bank_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_BANK_OE` writer - 13:12\\]
Controls jtag_dataout_bank_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutBankOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS_OE` reader - 27:14\\]
Controls jtag_dataout_address_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressOeR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_ADDRESS_OE` writer - 27:14\\]
Controls jtag_dataout_address_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAddressOeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `JTAG_DATAOUT_ODT_OE` reader - 29:28\\]
Controls jtag_dataout_odt_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_ODT_OE` writer - 29:28\\]
Controls jtag_dataout_odt_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutOdtOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN_OE` reader - 30:30\\]
Controls jtag_dataout_parity_in_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PARITY_IN_OE` writer - 30:30\\]
Controls jtag_dataout_parity_in_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutParityInOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_ERROR_N_OE` reader - 31:31\\]
Controls jtag_dataout_error_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNOeR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ERROR_N_OE` writer - 31:31\\]
Controls jtag_dataout_error_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutErrorNOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_reset_n_oe(&self) -> JtagDataoutResetNOeR {
        JtagDataoutResetNOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cke_oe(&self) -> JtagDataoutCkeOeR {
        JtagDataoutCkeOeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_mem_clk_0_oe(&self) -> JtagDataoutMemClk0OeR {
        JtagDataoutMemClk0OeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cs_n_oe(&self) -> JtagDataoutCsNOeR {
        JtagDataoutCsNOeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_act_n_oe(&self) -> JtagDataoutActNOeR {
        JtagDataoutActNOeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_ras_n_oe(&self) -> JtagDataoutRasNOeR {
        JtagDataoutRasNOeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_cas_n_oe(&self) -> JtagDataoutCasNOeR {
        JtagDataoutCasNOeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_we_n_oe(&self) -> JtagDataoutWeNOeR {
        JtagDataoutWeNOeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bg_oe(&self) -> JtagDataoutBgOeR {
        JtagDataoutBgOeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_bank_oe(&self) -> JtagDataoutBankOeR {
        JtagDataoutBankOeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_address_oe(&self) -> JtagDataoutAddressOeR {
        JtagDataoutAddressOeR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_odt_oe(&self) -> JtagDataoutOdtOeR {
        JtagDataoutOdtOeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_parity_in_oe(&self) -> JtagDataoutParityInOeR {
        JtagDataoutParityInOeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_error_n_oe(&self) -> JtagDataoutErrorNOeR {
        JtagDataoutErrorNOeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls jtag_dataout_reset_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_reset_n_oe(
        &mut self,
    ) -> JtagDataoutResetNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutResetNOeW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Controls jtag_dataout_cke_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cke_oe(
        &mut self,
    ) -> JtagDataoutCkeOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutCkeOeW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Controls jtag_dataout_mem_clk_0_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_mem_clk_0_oe(
        &mut self,
    ) -> JtagDataoutMemClk0OeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutMemClk0OeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Controls jtag_dataout_cs_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cs_n_oe(
        &mut self,
    ) -> JtagDataoutCsNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutCsNOeW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Controls jtag_dataout_act_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_act_n_oe(
        &mut self,
    ) -> JtagDataoutActNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutActNOeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls jtag_dataout_ras_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_ras_n_oe(
        &mut self,
    ) -> JtagDataoutRasNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutRasNOeW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Controls jtag_dataout_cas_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_cas_n_oe(
        &mut self,
    ) -> JtagDataoutCasNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutCasNOeW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Controls jtag_dataout_we_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_we_n_oe(
        &mut self,
    ) -> JtagDataoutWeNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutWeNOeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Controls jtag_dataout_bg_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bg_oe(&mut self) -> JtagDataoutBgOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutBgOeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Controls jtag_dataout_bank_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_bank_oe(
        &mut self,
    ) -> JtagDataoutBankOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutBankOeW::new(self, 12)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Controls jtag_dataout_address_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_address_oe(
        &mut self,
    ) -> JtagDataoutAddressOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutAddressOeW::new(self, 14)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Controls jtag_dataout_odt_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_odt_oe(
        &mut self,
    ) -> JtagDataoutOdtOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutOdtOeW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_parity_in_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_parity_in_oe(
        &mut self,
    ) -> JtagDataoutParityInOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutParityInOeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_error_n_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_error_n_oe(
        &mut self,
    ) -> JtagDataoutErrorNOeW<Regs_SsCfg_SscfgPhyTestCtrl4RegSpec> {
        JtagDataoutErrorNOeW::new(self, 31)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl4RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl4RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl4RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl4_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl4RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL4_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl4RegSpec {
    const RESET_VALUE: u32 = 0;
}
