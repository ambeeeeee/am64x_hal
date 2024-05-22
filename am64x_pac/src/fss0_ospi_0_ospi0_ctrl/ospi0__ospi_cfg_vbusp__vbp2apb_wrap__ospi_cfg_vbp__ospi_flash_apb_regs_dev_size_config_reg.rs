#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_size_config_reg` reader"]
pub type R =
    crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_size_config_reg` writer"]
pub type W =
    crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec>;
#[doc = "Field `NUM_ADDR_BYTES_FLD` reader - 3:0\\]
Number of address bytes. A value of 0 indicates 1 byte."]
pub type NumAddrBytesFldR = crate::FieldReader;
#[doc = "Field `NUM_ADDR_BYTES_FLD` writer - 3:0\\]
Number of address bytes. A value of 0 indicates 1 byte."]
pub type NumAddrBytesFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BYTES_PER_DEVICE_PAGE_FLD` reader - 15:4\\]
Number of bytes per device page. This is required by the controller for performing FLASH writes up to and across page boundaries."]
pub type BytesPerDevicePageFldR = crate::FieldReader<u16>;
#[doc = "Field `BYTES_PER_DEVICE_PAGE_FLD` writer - 15:4\\]
Number of bytes per device page. This is required by the controller for performing FLASH writes up to and across page boundaries."]
pub type BytesPerDevicePageFldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BYTES_PER_SUBSECTOR_FLD` reader - 20:16\\]
Number of bytes per Block. This is required by the controller for performing the write protection logic. The number of bytes per block must be a power of 2 number."]
pub type BytesPerSubsectorFldR = crate::FieldReader;
#[doc = "Field `BYTES_PER_SUBSECTOR_FLD` writer - 20:16\\]
Number of bytes per Block. This is required by the controller for performing the write protection logic. The number of bytes per block must be a power of 2 number."]
pub type BytesPerSubsectorFldW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MEM_SIZE_ON_CS0_FLD` reader - 22:21\\]
Size of Flash Device connected to CS\\[0\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs0FldR = crate::FieldReader;
#[doc = "Field `MEM_SIZE_ON_CS0_FLD` writer - 22:21\\]
Size of Flash Device connected to CS\\[0\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs0FldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_SIZE_ON_CS1_FLD` reader - 24:23\\]
Size of Flash Device connected to CS\\[1\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs1FldR = crate::FieldReader;
#[doc = "Field `MEM_SIZE_ON_CS1_FLD` writer - 24:23\\]
Size of Flash Device connected to CS\\[1\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs1FldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_SIZE_ON_CS2_FLD` reader - 26:25\\]
Size of Flash Device connected to CS\\[2\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs2FldR = crate::FieldReader;
#[doc = "Field `MEM_SIZE_ON_CS2_FLD` writer - 26:25\\]
Size of Flash Device connected to CS\\[2\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs2FldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_SIZE_ON_CS3_FLD` reader - 28:27\\]
Size of Flash Device connected to CS\\[3\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs3FldR = crate::FieldReader;
#[doc = "Field `MEM_SIZE_ON_CS3_FLD` writer - 28:27\\]
Size of Flash Device connected to CS\\[3\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
pub type MemSizeOnCs3FldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of address bytes. A value of 0 indicates 1 byte."]
    #[inline(always)]
    pub fn num_addr_bytes_fld(&self) -> NumAddrBytesFldR {
        NumAddrBytesFldR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of bytes per device page. This is required by the controller for performing FLASH writes up to and across page boundaries."]
    #[inline(always)]
    pub fn bytes_per_device_page_fld(&self) -> BytesPerDevicePageFldR {
        BytesPerDevicePageFldR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of bytes per Block. This is required by the controller for performing the write protection logic. The number of bytes per block must be a power of 2 number."]
    #[inline(always)]
    pub fn bytes_per_subsector_fld(&self) -> BytesPerSubsectorFldR {
        BytesPerSubsectorFldR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Size of Flash Device connected to CS\\[0\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    pub fn mem_size_on_cs0_fld(&self) -> MemSizeOnCs0FldR {
        MemSizeOnCs0FldR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Size of Flash Device connected to CS\\[1\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    pub fn mem_size_on_cs1_fld(&self) -> MemSizeOnCs1FldR {
        MemSizeOnCs1FldR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Size of Flash Device connected to CS\\[2\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    pub fn mem_size_on_cs2_fld(&self) -> MemSizeOnCs2FldR {
        MemSizeOnCs2FldR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Size of Flash Device connected to CS\\[3\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    pub fn mem_size_on_cs3_fld(&self) -> MemSizeOnCs3FldR {
        MemSizeOnCs3FldR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of address bytes. A value of 0 indicates 1 byte."]
    #[inline(always)]
    #[must_use]
    pub fn num_addr_bytes_fld(
        &mut self,
    ) -> NumAddrBytesFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        NumAddrBytesFldW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of bytes per device page. This is required by the controller for performing FLASH writes up to and across page boundaries."]
    #[inline(always)]
    #[must_use]
    pub fn bytes_per_device_page_fld(
        &mut self,
    ) -> BytesPerDevicePageFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        BytesPerDevicePageFldW::new(self, 4)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Number of bytes per Block. This is required by the controller for performing the write protection logic. The number of bytes per block must be a power of 2 number."]
    #[inline(always)]
    #[must_use]
    pub fn bytes_per_subsector_fld(
        &mut self,
    ) -> BytesPerSubsectorFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        BytesPerSubsectorFldW::new(self, 16)
    }
    #[doc = "Bits 21:22 - 22:21\\]
Size of Flash Device connected to CS\\[0\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size_on_cs0_fld(
        &mut self,
    ) -> MemSizeOnCs0FldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        MemSizeOnCs0FldW::new(self, 21)
    }
    #[doc = "Bits 23:24 - 24:23\\]
Size of Flash Device connected to CS\\[1\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size_on_cs1_fld(
        &mut self,
    ) -> MemSizeOnCs1FldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        MemSizeOnCs1FldW::new(self, 23)
    }
    #[doc = "Bits 25:26 - 26:25\\]
Size of Flash Device connected to CS\\[2\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size_on_cs2_fld(
        &mut self,
    ) -> MemSizeOnCs2FldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        MemSizeOnCs2FldW::new(self, 25)
    }
    #[doc = "Bits 27:28 - 28:27\\]
Size of Flash Device connected to CS\\[3\\]
pin: Value=00 : size of 512Mb. Value=01 : size of 1Gb. Value=10 : size of 2Gb. Value=11 : size of 4Gb."]
    #[inline(always)]
    #[must_use]
    pub fn mem_size_on_cs3_fld(
        &mut self,
    ) -> MemSizeOnCs3FldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec,
    > {
        MemSizeOnCs3FldW::new(self, 27)
    }
}
#[doc = "Device Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_size_config_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_size_config_reg to value 0x0016_2562"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevSizeConfigRegSpec
{
    const RESET_VALUE: u32 = 0x0016_2562;
}
