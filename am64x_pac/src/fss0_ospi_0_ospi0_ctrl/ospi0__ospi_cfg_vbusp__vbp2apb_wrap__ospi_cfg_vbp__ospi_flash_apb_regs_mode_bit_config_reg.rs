#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_mode_bit_config_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_mode_bit_config_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec>;
#[doc = "Field `MODE_FLD` reader - 7:0\\]
These are the 8 mode bits that are sent to the device following the address bytes if mode bit transmission has been enabled."]
pub type ModeFldR = crate::FieldReader;
#[doc = "Field `MODE_FLD` writer - 7:0\\]
These are the 8 mode bits that are sent to the device following the address bytes if mode bit transmission has been enabled."]
pub type ModeFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHUNK_SIZE_FLD` reader - 10:8\\]
It defines size of chunk after which CRC data is expected to show up on the SPI interface for write and read data transfers."]
pub type ChunkSizeFldR = crate::FieldReader;
#[doc = "Field `CHUNK_SIZE_FLD` writer - 10:8\\]
It defines size of chunk after which CRC data is expected to show up on the SPI interface for write and read data transfers."]
pub type ChunkSizeFldW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRC_OUT_ENABLE_FLD` reader - 15:15\\]
CRC# output enable bit When enabled, the controller expects the Flash Device to toggle CRC data on both SPI clock edges in CRC->CRC# sequence and calculates CRC compliance accordingly."]
pub type CrcOutEnableFldR = crate::BitReader;
#[doc = "Field `CRC_OUT_ENABLE_FLD` writer - 15:15\\]
CRC# output enable bit When enabled, the controller expects the Flash Device to toggle CRC data on both SPI clock edges in CRC->CRC# sequence and calculates CRC compliance accordingly."]
pub type CrcOutEnableFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CRC_DATA_UP_FLD` reader - 23:16\\]
RX CRC data \\[upper\\]
The second CRC byte returned after RX data chunk."]
pub type RxCrcDataUpFldR = crate::FieldReader;
#[doc = "Field `RX_CRC_DATA_UP_FLD` writer - 23:16\\]
RX CRC data \\[upper\\]
The second CRC byte returned after RX data chunk."]
pub type RxCrcDataUpFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RX_CRC_DATA_LOW_FLD` reader - 31:24\\]
RX CRC data \\[lower\\]
The first CRC byte returned after RX data chunk."]
pub type RxCrcDataLowFldR = crate::FieldReader;
#[doc = "Field `RX_CRC_DATA_LOW_FLD` writer - 31:24\\]
RX CRC data \\[lower\\]
The first CRC byte returned after RX data chunk."]
pub type RxCrcDataLowFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These are the 8 mode bits that are sent to the device following the address bytes if mode bit transmission has been enabled."]
    #[inline(always)]
    pub fn mode_fld(&self) -> ModeFldR {
        ModeFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
It defines size of chunk after which CRC data is expected to show up on the SPI interface for write and read data transfers."]
    #[inline(always)]
    pub fn chunk_size_fld(&self) -> ChunkSizeFldR {
        ChunkSizeFldR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
CRC# output enable bit When enabled, the controller expects the Flash Device to toggle CRC data on both SPI clock edges in CRC->CRC# sequence and calculates CRC compliance accordingly."]
    #[inline(always)]
    pub fn crc_out_enable_fld(&self) -> CrcOutEnableFldR {
        CrcOutEnableFldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RX CRC data \\[upper\\]
The second CRC byte returned after RX data chunk."]
    #[inline(always)]
    pub fn rx_crc_data_up_fld(&self) -> RxCrcDataUpFldR {
        RxCrcDataUpFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RX CRC data \\[lower\\]
The first CRC byte returned after RX data chunk."]
    #[inline(always)]
    pub fn rx_crc_data_low_fld(&self) -> RxCrcDataLowFldR {
        RxCrcDataLowFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These are the 8 mode bits that are sent to the device following the address bytes if mode bit transmission has been enabled."]
    #[inline(always)]
    #[must_use]
    pub fn mode_fld(
        &mut self,
    ) -> ModeFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec>
    {
        ModeFldW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
It defines size of chunk after which CRC data is expected to show up on the SPI interface for write and read data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn chunk_size_fld(
        &mut self,
    ) -> ChunkSizeFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec>
    {
        ChunkSizeFldW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
CRC# output enable bit When enabled, the controller expects the Flash Device to toggle CRC data on both SPI clock edges in CRC->CRC# sequence and calculates CRC compliance accordingly."]
    #[inline(always)]
    #[must_use]
    pub fn crc_out_enable_fld(
        &mut self,
    ) -> CrcOutEnableFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec,
    > {
        CrcOutEnableFldW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
RX CRC data \\[upper\\]
The second CRC byte returned after RX data chunk."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_up_fld(
        &mut self,
    ) -> RxCrcDataUpFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec,
    > {
        RxCrcDataUpFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RX CRC data \\[lower\\]
The first CRC byte returned after RX data chunk."]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_low_fld(
        &mut self,
    ) -> RxCrcDataLowFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec,
    > {
        RxCrcDataLowFldW::new(self, 24)
    }
}
#[doc = "Mode Bit Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_mode_bit_config_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_mode_bit_config_reg to value 0x0200"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsModeBitConfigRegSpec
{
    const RESET_VALUE: u32 = 0x0200;
}
