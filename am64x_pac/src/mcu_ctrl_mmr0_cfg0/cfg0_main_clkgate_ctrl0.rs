#[doc = "Register `CFG0_MAIN_CLKGATE_CTRL0` reader"]
pub type R = crate::R<Cfg0MainClkgateCtrl0Spec>;
#[doc = "Register `CFG0_MAIN_CLKGATE_CTRL0` writer"]
pub type W = crate::W<Cfg0MainClkgateCtrl0Spec>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_CBA_NOGATE` reader - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraCbaNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_CBA_NOGATE` writer - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraCbaNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_ECC_AGG_NOGATE` reader - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraEccAggNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_ECC_AGG_NOGATE` writer - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraEccAggNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_NOGATE` reader - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_NOGATE` writer - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_FW_CBA_NOGATE` reader - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainFwCbaNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_FW_CBA_NOGATE` writer - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainFwCbaNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_ECC_AGG_NOGATE` reader - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaEccAggNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_ECC_AGG_NOGATE` writer - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaEccAggNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_NOGATE` reader - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_NOGATE` writer - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_ACP_NOGATE` reader - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0AcpNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_ACP_NOGATE` writer - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0AcpNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_CFG_NOGATE` reader - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0CfgNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_CFG_NOGATE` writer - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0CfgNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_DBG_NOGATE` reader - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0DbgNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_DBG_NOGATE` writer - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0DbgNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_GIC500_NOGATE` reader - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
pub type MainClkgateCtrl0MainGic500NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_GIC500_NOGATE` writer - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
pub type MainClkgateCtrl0MainGic500NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSS_NOGATE` reader - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmssNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSS_NOGATE` writer - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmssNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA0_NOGATE` reader - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma0NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA0_NOGATE` writer - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma0NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA1_NOGATE` reader - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma1NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA1_NOGATE` writer - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma1NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG0_NOGATE` reader - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg0NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG0_NOGATE` writer - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg0NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG1_NOGATE` reader - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg1NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG1_NOGATE` writer - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg1NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_TIMERMGR_NOGATE` reader - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainTimermgrNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_TIMERMGR_NOGATE` writer - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainTimermgrNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS0_NOGATE` reader - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss0NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS0_NOGATE` writer - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss0NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS1_NOGATE` reader - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss1NogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS1_NOGATE` writer - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss1NogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DBG_CBA_NOGATE` reader - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
pub type MainClkgateCtrl0MainDbgCbaNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DBG_CBA_NOGATE` writer - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
pub type MainClkgateCtrl0MainDbgCbaNogateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSC_NOGATE` reader - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmscNogateR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSC_NOGATE` writer - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmscNogateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_infra_cba_nogate(&self) -> MainClkgateCtrl0MainInfraCbaNogateR {
        MainClkgateCtrl0MainInfraCbaNogateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_infra_ecc_agg_nogate(
        &self,
    ) -> MainClkgateCtrl0MainInfraEccAggNogateR {
        MainClkgateCtrl0MainInfraEccAggNogateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_cba_nogate(&self) -> MainClkgateCtrl0MainCbaNogateR {
        MainClkgateCtrl0MainCbaNogateR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_fw_cba_nogate(&self) -> MainClkgateCtrl0MainFwCbaNogateR {
        MainClkgateCtrl0MainFwCbaNogateR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_cba_ecc_agg_nogate(
        &self,
    ) -> MainClkgateCtrl0MainCbaEccAggNogateR {
        MainClkgateCtrl0MainCbaEccAggNogateR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_nogate(&self) -> MainClkgateCtrl0MainA53_0NogateR {
        MainClkgateCtrl0MainA53_0NogateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_acp_nogate(&self) -> MainClkgateCtrl0MainA53_0AcpNogateR {
        MainClkgateCtrl0MainA53_0AcpNogateR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_cfg_nogate(&self) -> MainClkgateCtrl0MainA53_0CfgNogateR {
        MainClkgateCtrl0MainA53_0CfgNogateR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_dbg_nogate(&self) -> MainClkgateCtrl0MainA53_0DbgNogateR {
        MainClkgateCtrl0MainA53_0DbgNogateR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_gic500_nogate(&self) -> MainClkgateCtrl0MainGic500NogateR {
        MainClkgateCtrl0MainGic500NogateR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dmss_nogate(&self) -> MainClkgateCtrl0MainDmssNogateR {
        MainClkgateCtrl0MainDmssNogateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_pdma0_nogate(&self) -> MainClkgateCtrl0MainPdma0NogateR {
        MainClkgateCtrl0MainPdma0NogateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_pdma1_nogate(&self) -> MainClkgateCtrl0MainPdma1NogateR {
        MainClkgateCtrl0MainPdma1NogateR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_icssg0_nogate(&self) -> MainClkgateCtrl0MainIcssg0NogateR {
        MainClkgateCtrl0MainIcssg0NogateR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_icssg1_nogate(&self) -> MainClkgateCtrl0MainIcssg1NogateR {
        MainClkgateCtrl0MainIcssg1NogateR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_timermgr_nogate(&self) -> MainClkgateCtrl0MainTimermgrNogateR {
        MainClkgateCtrl0MainTimermgrNogateR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_r5fss0_nogate(&self) -> MainClkgateCtrl0MainR5fss0NogateR {
        MainClkgateCtrl0MainR5fss0NogateR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_r5fss1_nogate(&self) -> MainClkgateCtrl0MainR5fss1NogateR {
        MainClkgateCtrl0MainR5fss1NogateR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dbg_cba_nogate(&self) -> MainClkgateCtrl0MainDbgCbaNogateR {
        MainClkgateCtrl0MainDbgCbaNogateR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dmsc_nogate(&self) -> MainClkgateCtrl0MainDmscNogateR {
        MainClkgateCtrl0MainDmscNogateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_infra_cba_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainInfraCbaNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainInfraCbaNogateW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_infra_ecc_agg_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainInfraEccAggNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainInfraEccAggNogateW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_cba_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainCbaNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainCbaNogateW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_fw_cba_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainFwCbaNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainFwCbaNogateW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_cba_ecc_agg_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainCbaEccAggNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainCbaEccAggNogateW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainA53_0NogateW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_acp_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0AcpNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainA53_0AcpNogateW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_cfg_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0CfgNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainA53_0CfgNogateW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_dbg_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0DbgNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainA53_0DbgNogateW::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_gic500_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainGic500NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainGic500NogateW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dmss_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainDmssNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainDmssNogateW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_pdma0_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainPdma0NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainPdma0NogateW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_pdma1_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainPdma1NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainPdma1NogateW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_icssg0_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainIcssg0NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainIcssg0NogateW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_icssg1_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainIcssg1NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainIcssg1NogateW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_timermgr_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainTimermgrNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainTimermgrNogateW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_r5fss0_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainR5fss0NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainR5fss0NogateW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_r5fss1_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainR5fss1NogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainR5fss1NogateW::new(self, 26)
    }
    #[doc = "Bit 28 - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dbg_cba_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainDbgCbaNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainDbgCbaNogateW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dmsc_nogate(
        &mut self,
    ) -> MainClkgateCtrl0MainDmscNogateW<Cfg0MainClkgateCtrl0Spec> {
        MainClkgateCtrl0MainDmscNogateW::new(self, 31)
    }
}
#[doc = "CFG0_MAIN_CLKGATE_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_clkgate_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_clkgate_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainClkgateCtrl0Spec;
impl crate::RegisterSpec for Cfg0MainClkgateCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_clkgate_ctrl0::R`](R) reader structure"]
impl crate::Readable for Cfg0MainClkgateCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_clkgate_ctrl0::W`](W) writer structure"]
impl crate::Writable for Cfg0MainClkgateCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_CLKGATE_CTRL0 to value 0"]
impl crate::Resettable for Cfg0MainClkgateCtrl0Spec {
    const RESET_VALUE: u32 = 0;
}
