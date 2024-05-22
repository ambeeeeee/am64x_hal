#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestStat1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestStat1RegSpec>;
#[doc = "Field `JTAG_DATAIN_RESET_N` reader - 0:0\\]
Displays value of jtag_datain_reset_n port on the PHY."]
pub type JtagDatainResetNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_RESET_N` writer - 0:0\\]
Displays value of jtag_datain_reset_n port on the PHY."]
pub type JtagDatainResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_CKE` reader - 2:1\\]
Displays value of jtag_datain_cke port on the PHY."]
pub type JtagDatainCkeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_CKE` writer - 2:1\\]
Displays value of jtag_datain_cke port on the PHY."]
pub type JtagDatainCkeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_MEM_CLK_0` reader - 3:3\\]
Displays value of jtag_datain_mem_clk_0 port on the PHY."]
pub type JtagDatainMemClk0R = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_MEM_CLK_0` writer - 3:3\\]
Displays value of jtag_datain_mem_clk_0 port on the PHY."]
pub type JtagDatainMemClk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_CS_N` reader - 5:4\\]
Displays value of jtag_datain_cs_n port on the PHY."]
pub type JtagDatainCsNR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_CS_N` writer - 5:4\\]
Displays value of jtag_datain_cs_n port on the PHY."]
pub type JtagDatainCsNW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_ACT_N` reader - 6:6\\]
Displays value of jtag_datain_act_n port on the PHY."]
pub type JtagDatainActNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_ACT_N` writer - 6:6\\]
Displays value of jtag_datain_act_n port on the PHY."]
pub type JtagDatainActNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_RAS_N` reader - 7:7\\]
Displays value of jtag_datain_ras_n port on the PHY."]
pub type JtagDatainRasNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_RAS_N` writer - 7:7\\]
Displays value of jtag_datain_ras_n port on the PHY."]
pub type JtagDatainRasNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_CAS_N` reader - 8:8\\]
Displays value of jtag_datain_cas_n port on the PHY."]
pub type JtagDatainCasNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_CAS_N` writer - 8:8\\]
Displays value of jtag_datain_cas_n port on the PHY."]
pub type JtagDatainCasNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_WE_N` reader - 9:9\\]
Displays value of jtag_datain_we_n port on the PHY."]
pub type JtagDatainWeNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_WE_N` writer - 9:9\\]
Displays value of jtag_datain_we_n port on the PHY."]
pub type JtagDatainWeNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_BG` reader - 11:10\\]
Displays value of jtag_datain_bg port on the PHY."]
pub type JtagDatainBgR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_BG` writer - 11:10\\]
Displays value of jtag_datain_bg port on the PHY."]
pub type JtagDatainBgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_BANK` reader - 13:12\\]
Displays value of jtag_datain_bank port on the PHY."]
pub type JtagDatainBankR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_BANK` writer - 13:12\\]
Displays value of jtag_datain_bank port on the PHY."]
pub type JtagDatainBankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_ADDRESS` reader - 27:14\\]
Displays value of jtag_datain_address port on the PHY."]
pub type JtagDatainAddressR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAIN_ADDRESS` writer - 27:14\\]
Displays value of jtag_datain_address port on the PHY."]
pub type JtagDatainAddressW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `JTAG_DATAIN_ODT` reader - 29:28\\]
Displays value of jtag_datain_odt port on the PHY."]
pub type JtagDatainOdtR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_ODT` writer - 29:28\\]
Displays value of jtag_datain_odt port on the PHY."]
pub type JtagDatainOdtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_PARITY_IN` reader - 30:30\\]
Displays value of jtag_datain_parity_in port on the PHY."]
pub type JtagDatainParityInR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_PARITY_IN` writer - 30:30\\]
Displays value of jtag_datain_parity_in port on the PHY."]
pub type JtagDatainParityInW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAIN_ERROR_N` reader - 31:31\\]
Displays value of jtag_datain_error_n port on the PHY."]
pub type JtagDatainErrorNR = crate::BitReader;
#[doc = "Field `JTAG_DATAIN_ERROR_N` writer - 31:31\\]
Displays value of jtag_datain_error_n port on the PHY."]
pub type JtagDatainErrorNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Displays value of jtag_datain_reset_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_reset_n(&self) -> JtagDatainResetNR {
        JtagDatainResetNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Displays value of jtag_datain_cke port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_cke(&self) -> JtagDatainCkeR {
        JtagDatainCkeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Displays value of jtag_datain_mem_clk_0 port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_mem_clk_0(&self) -> JtagDatainMemClk0R {
        JtagDatainMemClk0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Displays value of jtag_datain_cs_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_cs_n(&self) -> JtagDatainCsNR {
        JtagDatainCsNR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Displays value of jtag_datain_act_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_act_n(&self) -> JtagDatainActNR {
        JtagDatainActNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Displays value of jtag_datain_ras_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_ras_n(&self) -> JtagDatainRasNR {
        JtagDatainRasNR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Displays value of jtag_datain_cas_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_cas_n(&self) -> JtagDatainCasNR {
        JtagDatainCasNR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Displays value of jtag_datain_we_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_we_n(&self) -> JtagDatainWeNR {
        JtagDatainWeNR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Displays value of jtag_datain_bg port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_bg(&self) -> JtagDatainBgR {
        JtagDatainBgR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Displays value of jtag_datain_bank port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_bank(&self) -> JtagDatainBankR {
        JtagDatainBankR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Displays value of jtag_datain_address port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_address(&self) -> JtagDatainAddressR {
        JtagDatainAddressR::new(((self.bits >> 14) & 0x3fff) as u16)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Displays value of jtag_datain_odt port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_odt(&self) -> JtagDatainOdtR {
        JtagDatainOdtR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Displays value of jtag_datain_parity_in port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_parity_in(&self) -> JtagDatainParityInR {
        JtagDatainParityInR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Displays value of jtag_datain_error_n port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_error_n(&self) -> JtagDatainErrorNR {
        JtagDatainErrorNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Displays value of jtag_datain_reset_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_reset_n(
        &mut self,
    ) -> JtagDatainResetNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainResetNW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Displays value of jtag_datain_cke port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_cke(&mut self) -> JtagDatainCkeW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainCkeW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
Displays value of jtag_datain_mem_clk_0 port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_mem_clk_0(
        &mut self,
    ) -> JtagDatainMemClk0W<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainMemClk0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Displays value of jtag_datain_cs_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_cs_n(&mut self) -> JtagDatainCsNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainCsNW::new(self, 4)
    }
    #[doc = "Bit 6 - 6:6\\]
Displays value of jtag_datain_act_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_act_n(&mut self) -> JtagDatainActNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainActNW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Displays value of jtag_datain_ras_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_ras_n(&mut self) -> JtagDatainRasNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainRasNW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Displays value of jtag_datain_cas_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_cas_n(&mut self) -> JtagDatainCasNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainCasNW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Displays value of jtag_datain_we_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_we_n(&mut self) -> JtagDatainWeNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainWeNW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Displays value of jtag_datain_bg port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_bg(&mut self) -> JtagDatainBgW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainBgW::new(self, 10)
    }
    #[doc = "Bits 12:13 - 13:12\\]
Displays value of jtag_datain_bank port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_bank(&mut self) -> JtagDatainBankW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainBankW::new(self, 12)
    }
    #[doc = "Bits 14:27 - 27:14\\]
Displays value of jtag_datain_address port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_address(
        &mut self,
    ) -> JtagDatainAddressW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainAddressW::new(self, 14)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Displays value of jtag_datain_odt port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_odt(&mut self) -> JtagDatainOdtW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainOdtW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Displays value of jtag_datain_parity_in port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_parity_in(
        &mut self,
    ) -> JtagDatainParityInW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainParityInW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Displays value of jtag_datain_error_n port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_error_n(
        &mut self,
    ) -> JtagDatainErrorNW<Regs_SsCfg_SscfgPhyTestStat1RegSpec> {
        JtagDatainErrorNW::new(self, 31)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_stat1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_stat1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestStat1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestStat1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_stat1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestStat1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_stat1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestStat1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_STAT1_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestStat1RegSpec {
    const RESET_VALUE: u32 = 0;
}
