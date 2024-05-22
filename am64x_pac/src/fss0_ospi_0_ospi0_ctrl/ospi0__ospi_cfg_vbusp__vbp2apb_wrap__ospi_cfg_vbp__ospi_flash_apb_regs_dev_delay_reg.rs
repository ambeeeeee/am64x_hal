#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_delay_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_delay_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec>;
#[doc = "Field `D_INIT_FLD` reader - 7:0\\]
Clock Delay with n_ss_out: Delay in master reference clocks between setting n_ss_out low and first bit transfer."]
pub type DInitFldR = crate::FieldReader;
#[doc = "Field `D_INIT_FLD` writer - 7:0\\]
Clock Delay with n_ss_out: Delay in master reference clocks between setting n_ss_out low and first bit transfer."]
pub type DInitFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `D_AFTER_FLD` reader - 15:8\\]
Clock Delay for Last Transaction Bit: Delay in master reference clocks between last bit of current transaction and deasserting the device chip select \\[n_ss_out\\]. By default, the chip select will be deasserted on the cycle following the completion of the current transaction."]
pub type DAfterFldR = crate::FieldReader;
#[doc = "Field `D_AFTER_FLD` writer - 15:8\\]
Clock Delay for Last Transaction Bit: Delay in master reference clocks between last bit of current transaction and deasserting the device chip select \\[n_ss_out\\]. By default, the chip select will be deasserted on the cycle following the completion of the current transaction."]
pub type DAfterFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `D_BTWN_FLD` reader - 23:16\\]
Clock Delay for Chip Select Deactivation: Delay in master reference clocks between one chip select being de-activated and the activation of another. This is used to ensure a quiet period between the selection of two different slaves and requires the transmit FIFO to be empty."]
pub type DBtwnFldR = crate::FieldReader;
#[doc = "Field `D_BTWN_FLD` writer - 23:16\\]
Clock Delay for Chip Select Deactivation: Delay in master reference clocks between one chip select being de-activated and the activation of another. This is used to ensure a quiet period between the selection of two different slaves and requires the transmit FIFO to be empty."]
pub type DBtwnFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `D_NSS_FLD` reader - 31:24\\]
Clock Delay for Chip Select Deassert: Delay in master reference clocks for the length that the master mode chip select outputs are de-asserted between transactions. The minimum delay is always SCLK period to ensure the chip select is never re-asserted within an SCLK period."]
pub type DNssFldR = crate::FieldReader;
#[doc = "Field `D_NSS_FLD` writer - 31:24\\]
Clock Delay for Chip Select Deassert: Delay in master reference clocks for the length that the master mode chip select outputs are de-asserted between transactions. The minimum delay is always SCLK period to ensure the chip select is never re-asserted within an SCLK period."]
pub type DNssFldW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock Delay with n_ss_out: Delay in master reference clocks between setting n_ss_out low and first bit transfer."]
    #[inline(always)]
    pub fn d_init_fld(&self) -> DInitFldR {
        DInitFldR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock Delay for Last Transaction Bit: Delay in master reference clocks between last bit of current transaction and deasserting the device chip select \\[n_ss_out\\]. By default, the chip select will be deasserted on the cycle following the completion of the current transaction."]
    #[inline(always)]
    pub fn d_after_fld(&self) -> DAfterFldR {
        DAfterFldR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock Delay for Chip Select Deactivation: Delay in master reference clocks between one chip select being de-activated and the activation of another. This is used to ensure a quiet period between the selection of two different slaves and requires the transmit FIFO to be empty."]
    #[inline(always)]
    pub fn d_btwn_fld(&self) -> DBtwnFldR {
        DBtwnFldR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock Delay for Chip Select Deassert: Delay in master reference clocks for the length that the master mode chip select outputs are de-asserted between transactions. The minimum delay is always SCLK period to ensure the chip select is never re-asserted within an SCLK period."]
    #[inline(always)]
    pub fn d_nss_fld(&self) -> DNssFldR {
        DNssFldR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock Delay with n_ss_out: Delay in master reference clocks between setting n_ss_out low and first bit transfer."]
    #[inline(always)]
    #[must_use]
    pub fn d_init_fld(
        &mut self,
    ) -> DInitFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec> {
        DInitFldW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Clock Delay for Last Transaction Bit: Delay in master reference clocks between last bit of current transaction and deasserting the device chip select \\[n_ss_out\\]. By default, the chip select will be deasserted on the cycle following the completion of the current transaction."]
    #[inline(always)]
    #[must_use]
    pub fn d_after_fld(
        &mut self,
    ) -> DAfterFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec> {
        DAfterFldW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock Delay for Chip Select Deactivation: Delay in master reference clocks between one chip select being de-activated and the activation of another. This is used to ensure a quiet period between the selection of two different slaves and requires the transmit FIFO to be empty."]
    #[inline(always)]
    #[must_use]
    pub fn d_btwn_fld(
        &mut self,
    ) -> DBtwnFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec> {
        DBtwnFldW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Clock Delay for Chip Select Deassert: Delay in master reference clocks for the length that the master mode chip select outputs are de-asserted between transactions. The minimum delay is always SCLK period to ensure the chip select is never re-asserted within an SCLK period."]
    #[inline(always)]
    #[must_use]
    pub fn d_nss_fld(
        &mut self,
    ) -> DNssFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec> {
        DNssFldW::new(self, 24)
    }
}
#[doc = "Octal-SPI Device Delay Register: This register is used to introduce relative delays into the generation of the master output signals. All timings are defined in cycles of the SPI REFERENCE CLOCK/ext_clk, defined in this table as SPI master ref clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_dev_delay_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_dev_delay_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsDevDelayRegSpec
{
    const RESET_VALUE: u32 = 0;
}
