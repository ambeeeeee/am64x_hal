#[doc = "Register `CFG0_MAIN_CLKGATE_CTRL0_PROXY` reader"]
pub type R = crate::R<Cfg0MainClkgateCtrl0ProxySpec>;
#[doc = "Register `CFG0_MAIN_CLKGATE_CTRL0_PROXY` writer"]
pub type W = crate::W<Cfg0MainClkgateCtrl0ProxySpec>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_CBA_NOGATE_PROXY` reader - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraCbaNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_CBA_NOGATE_PROXY` writer - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraCbaNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_ECC_AGG_NOGATE_PROXY` reader - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraEccAggNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_INFRA_ECC_AGG_NOGATE_PROXY` writer - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
pub type MainClkgateCtrl0MainInfraEccAggNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_NOGATE_PROXY` reader - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_NOGATE_PROXY` writer - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_FW_CBA_NOGATE_PROXY` reader - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainFwCbaNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_FW_CBA_NOGATE_PROXY` writer - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
pub type MainClkgateCtrl0MainFwCbaNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_ECC_AGG_NOGATE_PROXY` reader - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaEccAggNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_CBA_ECC_AGG_NOGATE_PROXY` writer - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
pub type MainClkgateCtrl0MainCbaEccAggNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_NOGATE_PROXY` reader - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_NOGATE_PROXY` writer - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_ACP_NOGATE_PROXY` reader - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0AcpNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_ACP_NOGATE_PROXY` writer - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0AcpNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_CFG_NOGATE_PROXY` reader - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0CfgNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_CFG_NOGATE_PROXY` writer - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0CfgNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_DBG_NOGATE_PROXY` reader - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0DbgNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_A53_0_DBG_NOGATE_PROXY` writer - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
pub type MainClkgateCtrl0MainA53_0DbgNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_GIC500_NOGATE_PROXY` reader - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
pub type MainClkgateCtrl0MainGic500NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_GIC500_NOGATE_PROXY` writer - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
pub type MainClkgateCtrl0MainGic500NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSS_NOGATE_PROXY` reader - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmssNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSS_NOGATE_PROXY` writer - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmssNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA0_NOGATE_PROXY` reader - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma0NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA0_NOGATE_PROXY` writer - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma0NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA1_NOGATE_PROXY` reader - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma1NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_PDMA1_NOGATE_PROXY` writer - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainPdma1NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG0_NOGATE_PROXY` reader - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg0NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG0_NOGATE_PROXY` writer - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg0NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG1_NOGATE_PROXY` reader - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg1NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_ICSSG1_NOGATE_PROXY` writer - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
pub type MainClkgateCtrl0MainIcssg1NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_TIMERMGR_NOGATE_PROXY` reader - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainTimermgrNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_TIMERMGR_NOGATE_PROXY` writer - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainTimermgrNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS0_NOGATE_PROXY` reader - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss0NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS0_NOGATE_PROXY` writer - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss0NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS1_NOGATE_PROXY` reader - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss1NogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_R5FSS1_NOGATE_PROXY` writer - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
pub type MainClkgateCtrl0MainR5fss1NogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DBG_CBA_NOGATE_PROXY` reader - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
pub type MainClkgateCtrl0MainDbgCbaNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DBG_CBA_NOGATE_PROXY` writer - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
pub type MainClkgateCtrl0MainDbgCbaNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSC_NOGATE_PROXY` reader - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmscNogateProxyR = crate::BitReader;
#[doc = "Field `MAIN_CLKGATE_CTRL0_MAIN_DMSC_NOGATE_PROXY` writer - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
pub type MainClkgateCtrl0MainDmscNogateProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_infra_cba_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainInfraCbaNogateProxyR {
        MainClkgateCtrl0MainInfraCbaNogateProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_infra_ecc_agg_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainInfraEccAggNogateProxyR {
        MainClkgateCtrl0MainInfraEccAggNogateProxyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_cba_nogate_proxy(&self) -> MainClkgateCtrl0MainCbaNogateProxyR {
        MainClkgateCtrl0MainCbaNogateProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_fw_cba_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainFwCbaNogateProxyR {
        MainClkgateCtrl0MainFwCbaNogateProxyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_cba_ecc_agg_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainCbaEccAggNogateProxyR {
        MainClkgateCtrl0MainCbaEccAggNogateProxyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainA53_0NogateProxyR {
        MainClkgateCtrl0MainA53_0NogateProxyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_acp_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainA53_0AcpNogateProxyR {
        MainClkgateCtrl0MainA53_0AcpNogateProxyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_cfg_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainA53_0CfgNogateProxyR {
        MainClkgateCtrl0MainA53_0CfgNogateProxyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_a53_0_dbg_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainA53_0DbgNogateProxyR {
        MainClkgateCtrl0MainA53_0DbgNogateProxyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_gic500_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainGic500NogateProxyR {
        MainClkgateCtrl0MainGic500NogateProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dmss_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainDmssNogateProxyR {
        MainClkgateCtrl0MainDmssNogateProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_pdma0_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainPdma0NogateProxyR {
        MainClkgateCtrl0MainPdma0NogateProxyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_pdma1_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainPdma1NogateProxyR {
        MainClkgateCtrl0MainPdma1NogateProxyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_icssg0_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainIcssg0NogateProxyR {
        MainClkgateCtrl0MainIcssg0NogateProxyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_icssg1_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainIcssg1NogateProxyR {
        MainClkgateCtrl0MainIcssg1NogateProxyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_timermgr_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainTimermgrNogateProxyR {
        MainClkgateCtrl0MainTimermgrNogateProxyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_r5fss0_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainR5fss0NogateProxyR {
        MainClkgateCtrl0MainR5fss0NogateProxyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_r5fss1_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainR5fss1NogateProxyR {
        MainClkgateCtrl0MainR5fss1NogateProxyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dbg_cba_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainDbgCbaNogateProxyR {
        MainClkgateCtrl0MainDbgCbaNogateProxyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    pub fn main_clkgate_ctrl0_main_dmsc_nogate_proxy(
        &self,
    ) -> MainClkgateCtrl0MainDmscNogateProxyR {
        MainClkgateCtrl0MainDmscNogateProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MAIN domain Infrastructure bus (main_infra_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_infra_cba_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainInfraCbaNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainInfraCbaNogateProxyW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MAIN domain Infrastructure ECC aggragator (main_infra_ecc_aggr) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_infra_ecc_agg_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainInfraEccAggNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainInfraEccAggNogateProxyW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
MAIN domain data bus (main_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_cba_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainCbaNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainCbaNogateProxyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
MAIN domain datal bus (main_fw_cbass) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_fw_cba_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainFwCbaNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainFwCbaNogateProxyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
MAIN domain data bus ECC aggragator (main_cba_ecc_aggr_main_0) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_cba_ecc_agg_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainCbaEccAggNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainCbaEccAggNogateProxyW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
MAIN A53SS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainA53_0NogateProxyW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
MAIN A53SS0 ACP clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_acp_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0AcpNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainA53_0AcpNogateProxyW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
MAIN A53SS0 Configuration Port clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_cfg_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0CfgNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainA53_0CfgNogateProxyW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
MAIN A53SS0 Debug Port clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_a53_0_dbg_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainA53_0DbgNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainA53_0DbgNogateProxyW::new(self, 10)
    }
    #[doc = "Bit 15 - 15:15\\]
MAIN A53SS0 (gic500_1_2) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_gic500_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainGic500NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainGic500NogateProxyW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
MAIN domain DMSS (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dmss_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainDmssNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainDmssNogateProxyW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
MAIN domain PDMA0 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_pdma0_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainPdma0NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainPdma0NogateProxyW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
MAIN domain PDMA1 (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_pdma1_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainPdma1NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainPdma1NogateProxyW::new(self, 18)
    }
    #[doc = "Bit 20 - 20:20\\]
MAIN domain ICSSG0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_icssg0_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainIcssg0NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainIcssg0NogateProxyW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
MAIN domain ICSSG1 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_icssg1_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainIcssg1NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainIcssg1NogateProxyW::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
MAIN domain TIMERMGR (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_timermgr_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainTimermgrNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainTimermgrNogateProxyW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
MAIN domain R5FSS0 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_r5fss0_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainR5fss0NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainR5fss0NogateProxyW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
MAIN domain R5FSS1 clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_r5fss1_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainR5fss1NogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainR5fss1NogateProxyW::new(self, 26)
    }
    #[doc = "Bit 28 - 28:28\\]
MAIN domain Debug bus clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dbg_cba_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainDbgCbaNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainDbgCbaNogateProxyW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
MAIN domain DMSC (pwr_dis_nogate) clock gate deactivate."]
    #[inline(always)]
    #[must_use]
    pub fn main_clkgate_ctrl0_main_dmsc_nogate_proxy(
        &mut self,
    ) -> MainClkgateCtrl0MainDmscNogateProxyW<Cfg0MainClkgateCtrl0ProxySpec> {
        MainClkgateCtrl0MainDmscNogateProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MAIN_CLKGATE_CTRL0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_main_clkgate_ctrl0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_main_clkgate_ctrl0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0MainClkgateCtrl0ProxySpec;
impl crate::RegisterSpec for Cfg0MainClkgateCtrl0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_main_clkgate_ctrl0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0MainClkgateCtrl0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_main_clkgate_ctrl0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0MainClkgateCtrl0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MAIN_CLKGATE_CTRL0_PROXY to value 0"]
impl crate::Resettable for Cfg0MainClkgateCtrl0ProxySpec {
    const RESET_VALUE: u32 = 0;
}
