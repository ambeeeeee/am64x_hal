#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_ctrl_reg` reader"]
pub type R = crate::R<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_ctrl_reg` writer"]
pub type W = crate::W<
    Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
>;
#[doc = "Field `START_FLD` reader - 0:0\\]
Start Indirect Write: Writing a 1 to this bit will trigger an indirect write operation. The assumption is that the indirect start address and the indirect number of bytes register is setup before triggering the indirect write operation."]
pub type StartFldR = crate::BitReader;
#[doc = "Field `START_FLD` writer - 0:0\\]
Start Indirect Write: Writing a 1 to this bit will trigger an indirect write operation. The assumption is that the indirect start address and the indirect number of bytes register is setup before triggering the indirect write operation."]
pub type StartFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL_FLD` reader - 1:1\\]
Cancel Indirect Write: Writing a 1 to this bit will cancel all ongoing indirect write operations."]
pub type CancelFldR = crate::BitReader;
#[doc = "Field `CANCEL_FLD` writer - 1:1\\]
Cancel Indirect Write: Writing a 1 to this bit will cancel all ongoing indirect write operations."]
pub type CancelFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_STATUS_FLD` reader - 2:2\\]
Indirect Write Status: Indirect write operation in progress \\[status\\]"]
pub type WrStatusFldR = crate::BitReader;
#[doc = "Field `WR_STATUS_FLD` writer - 2:2\\]
Indirect Write Status: Indirect write operation in progress \\[status\\]"]
pub type WrStatusFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_QUEUED_FLD` reader - 4:4\\]
Two indirect write operations have been queued"]
pub type WrQueuedFldR = crate::BitReader;
#[doc = "Field `WR_QUEUED_FLD` writer - 4:4\\]
Two indirect write operations have been queued"]
pub type WrQueuedFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IND_OPS_DONE_STATUS_FLD` reader - 5:5\\]
Indirect Completion Status: This field is set to 1 when an indirect operation has completed. Write a 1 to this field to clear it."]
pub type IndOpsDoneStatusFldR = crate::BitReader;
#[doc = "Field `IND_OPS_DONE_STATUS_FLD` writer - 5:5\\]
Indirect Completion Status: This field is set to 1 when an indirect operation has completed. Write a 1 to this field to clear it."]
pub type IndOpsDoneStatusFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_IND_OPS_DONE_FLD` reader - 7:6\\]
This field contains the number of indirect operations which have been completed. This is used in conjunction with the indirect completion status field \\[bit 5\\]. It is incremented by hardware when an indirect operation has completed. Write a 1 to bit 5 of this register to decrement it."]
pub type NumIndOpsDoneFldR = crate::FieldReader;
#[doc = "Field `NUM_IND_OPS_DONE_FLD` writer - 7:6\\]
This field contains the number of indirect operations which have been completed. This is used in conjunction with the indirect completion status field \\[bit 5\\]. It is incremented by hardware when an indirect operation has completed. Write a 1 to bit 5 of this register to decrement it."]
pub type NumIndOpsDoneFldW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start Indirect Write: Writing a 1 to this bit will trigger an indirect write operation. The assumption is that the indirect start address and the indirect number of bytes register is setup before triggering the indirect write operation."]
    #[inline(always)]
    pub fn start_fld(&self) -> StartFldR {
        StartFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Cancel Indirect Write: Writing a 1 to this bit will cancel all ongoing indirect write operations."]
    #[inline(always)]
    pub fn cancel_fld(&self) -> CancelFldR {
        CancelFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Write Status: Indirect write operation in progress \\[status\\]"]
    #[inline(always)]
    pub fn wr_status_fld(&self) -> WrStatusFldR {
        WrStatusFldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Two indirect write operations have been queued"]
    #[inline(always)]
    pub fn wr_queued_fld(&self) -> WrQueuedFldR {
        WrQueuedFldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indirect Completion Status: This field is set to 1 when an indirect operation has completed. Write a 1 to this field to clear it."]
    #[inline(always)]
    pub fn ind_ops_done_status_fld(&self) -> IndOpsDoneStatusFldR {
        IndOpsDoneStatusFldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
This field contains the number of indirect operations which have been completed. This is used in conjunction with the indirect completion status field \\[bit 5\\]. It is incremented by hardware when an indirect operation has completed. Write a 1 to bit 5 of this register to decrement it."]
    #[inline(always)]
    pub fn num_ind_ops_done_fld(&self) -> NumIndOpsDoneFldR {
        NumIndOpsDoneFldR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start Indirect Write: Writing a 1 to this bit will trigger an indirect write operation. The assumption is that the indirect start address and the indirect number of bytes register is setup before triggering the indirect write operation."]
    #[inline(always)]
    #[must_use]
    pub fn start_fld(
        &mut self,
    ) -> StartFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        StartFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Cancel Indirect Write: Writing a 1 to this bit will cancel all ongoing indirect write operations."]
    #[inline(always)]
    #[must_use]
    pub fn cancel_fld(
        &mut self,
    ) -> CancelFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        CancelFldW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indirect Write Status: Indirect write operation in progress \\[status\\]"]
    #[inline(always)]
    #[must_use]
    pub fn wr_status_fld(
        &mut self,
    ) -> WrStatusFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        WrStatusFldW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Two indirect write operations have been queued"]
    #[inline(always)]
    #[must_use]
    pub fn wr_queued_fld(
        &mut self,
    ) -> WrQueuedFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        WrQueuedFldW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indirect Completion Status: This field is set to 1 when an indirect operation has completed. Write a 1 to this field to clear it."]
    #[inline(always)]
    #[must_use]
    pub fn ind_ops_done_status_fld(
        &mut self,
    ) -> IndOpsDoneStatusFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        IndOpsDoneStatusFldW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
This field contains the number of indirect operations which have been completed. This is used in conjunction with the indirect completion status field \\[bit 5\\]. It is incremented by hardware when an indirect operation has completed. Write a 1 to bit 5 of this register to decrement it."]
    #[inline(always)]
    #[must_use]
    pub fn num_ind_ops_done_fld(
        &mut self,
    ) -> NumIndOpsDoneFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec,
    > {
        NumIndOpsDoneFldW::new(self, 6)
    }
}
#[doc = "Indirect Write Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::R`](R) reader structure"]
impl crate::Readable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec
{
}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_indirect_write_xfer_ctrl_reg::W`](W) writer structure"]
impl crate::Writable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_indirect_write_xfer_ctrl_reg to value 0"]
impl crate::Resettable
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsIndirectWriteXferCtrlRegSpec
{
    const RESET_VALUE: u32 = 0;
}
