#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_configuration_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_configuration_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec>;
#[doc = "Field `PHY_CONFIG_RX_DLL_DELAY_FLD` reader - 6:0\\]
RX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and rx_dll_clk."]
pub type PhyConfigRxDllDelayFldR = crate::FieldReader;
#[doc = "Field `PHY_CONFIG_RX_DLL_DELAY_FLD` writer - 6:0\\]
RX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and rx_dll_clk."]
pub type PhyConfigRxDllDelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_CONFIG_TX_DLL_DELAY_FLD` reader - 22:16\\]
TX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and spi_clk."]
pub type PhyConfigTxDllDelayFldR = crate::FieldReader;
#[doc = "Field `PHY_CONFIG_TX_DLL_DELAY_FLD` writer - 22:16\\]
TX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and spi_clk."]
pub type PhyConfigTxDllDelayFldW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_CONFIG_RX_DLL_BYPASS_FLD` reader - 29:29\\]
RX DLL Bypass: This field determines id RX DLL is bypassed."]
pub type PhyConfigRxDllBypassFldR = crate::BitReader;
#[doc = "Field `PHY_CONFIG_RX_DLL_BYPASS_FLD` writer - 29:29\\]
RX DLL Bypass: This field determines id RX DLL is bypassed."]
pub type PhyConfigRxDllBypassFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CONFIG_RESET_FLD` reader - 30:30\\]
DLL Reset bit: This bit is used for reset of Delay Lines by software."]
pub type PhyConfigResetFldR = crate::BitReader;
#[doc = "Field `PHY_CONFIG_RESET_FLD` writer - 30:30\\]
DLL Reset bit: This bit is used for reset of Delay Lines by software."]
pub type PhyConfigResetFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_CONFIG_RESYNC_FLD` reader - 31:31\\]
This bit is used for re-synchronisation delay lines to update them with values from TX DLL Delay and RX DLL Delay fields."]
pub type PhyConfigResyncFldR = crate::BitReader;
#[doc = "Field `PHY_CONFIG_RESYNC_FLD` writer - 31:31\\]
This bit is used for re-synchronisation delay lines to update them with values from TX DLL Delay and RX DLL Delay fields."]
pub type PhyConfigResyncFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
RX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and rx_dll_clk."]
    #[inline(always)]
    pub fn phy_config_rx_dll_delay_fld(&self) -> PhyConfigRxDllDelayFldR {
        PhyConfigRxDllDelayFldR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 22:16\\]
TX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and spi_clk."]
    #[inline(always)]
    pub fn phy_config_tx_dll_delay_fld(&self) -> PhyConfigTxDllDelayFldR {
        PhyConfigTxDllDelayFldR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
RX DLL Bypass: This field determines id RX DLL is bypassed."]
    #[inline(always)]
    pub fn phy_config_rx_dll_bypass_fld(&self) -> PhyConfigRxDllBypassFldR {
        PhyConfigRxDllBypassFldR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
DLL Reset bit: This bit is used for reset of Delay Lines by software."]
    #[inline(always)]
    pub fn phy_config_reset_fld(&self) -> PhyConfigResetFldR {
        PhyConfigResetFldR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is used for re-synchronisation delay lines to update them with values from TX DLL Delay and RX DLL Delay fields."]
    #[inline(always)]
    pub fn phy_config_resync_fld(&self) -> PhyConfigResyncFldR {
        PhyConfigResyncFldR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
RX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and rx_dll_clk."]
    #[inline(always)]
    #[must_use]
    pub fn phy_config_rx_dll_delay_fld(
        &mut self,
    ) -> PhyConfigRxDllDelayFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec,
    > {
        PhyConfigRxDllDelayFldW::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
TX DLL Delay: This field determines the number of delay elements to insert on data path between ref_clk and spi_clk."]
    #[inline(always)]
    #[must_use]
    pub fn phy_config_tx_dll_delay_fld(
        &mut self,
    ) -> PhyConfigTxDllDelayFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec,
    > {
        PhyConfigTxDllDelayFldW::new(self, 16)
    }
    #[doc = "Bit 29 - 29:29\\]
RX DLL Bypass: This field determines id RX DLL is bypassed."]
    #[inline(always)]
    #[must_use]
    pub fn phy_config_rx_dll_bypass_fld(
        &mut self,
    ) -> PhyConfigRxDllBypassFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec,
    > {
        PhyConfigRxDllBypassFldW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
DLL Reset bit: This bit is used for reset of Delay Lines by software."]
    #[inline(always)]
    #[must_use]
    pub fn phy_config_reset_fld(
        &mut self,
    ) -> PhyConfigResetFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec,
    > {
        PhyConfigResetFldW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is used for re-synchronisation delay lines to update them with values from TX DLL Delay and RX DLL Delay fields."]
    #[inline(always)]
    #[must_use]
    pub fn phy_config_resync_fld(
        &mut self,
    ) -> PhyConfigResyncFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec,
    > {
        PhyConfigResyncFldW::new(self, 31)
    }
}
#[doc = "PHY Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_phy_configuration_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_phy_configuration_reg to value 0x4000_0000"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsPhyConfigurationRegSpec
{
    const RESET_VALUE: u32 = 0x4000_0000;
}
