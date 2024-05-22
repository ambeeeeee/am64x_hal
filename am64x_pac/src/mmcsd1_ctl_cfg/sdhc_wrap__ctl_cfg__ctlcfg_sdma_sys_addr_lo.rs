#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_lo` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_lo` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec>;
#[doc = "Field `SDMA_ADDRESS` reader - 15:0\\]
When Host Version 4 Enable is set to 0 in the Host Control 2 register,DMA uses this register as system address in only 32-bit addressing mode. Auto CMD23 cannot be used with SDMA. When Host Version 4 Enable is set to 1, SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of using this register to sup- port both 32-bit and 64-bit addressing. This register is re-assigned to 32-bit Block Count and then SDMA may use Auto CMD23.\\[1\\]
SDMA System Address \\[Host Version 4 Enable = 0\\]
This register contains the system memory address for a SDMA transfer in 32-bit addressing mode. When the Host Controller stops a SDMA transfer, this register shall point to the system address of the next contiguous data position. It can be accessed only if no transaction is executing \\[i.e., after a transaction has stopped\\]. Reading this register during SDMA transfers may return an invalid value. The Host Driver shall initialize this register before starting a SDMA transaction. After SDMA has stopped, the next system address of the next contiguous data posi- tion can be read from this register. The SDMA transfer waits at the every boundary specified by the SDMA Buffer Boundary in the Block Size register. The Host Controller generates DMA Interrupt to request the Host Driver to update this register. The Host Driver sets the next system address of the next data position to this register. When the most upper byte of this register \\[003h\\]
is written, the Host Controller restarts the SDMA transfer. When restarting SDMA by setting Continue Request in the Block Gap Control register, the Host Controller shall start at the next contiguous address stored here in the SDMA System Address register. ADMA does not use this register. \\[2\\]
32-bit Block Count \\[Host Version 4 Enable = 1\\]
Host Controller Version 4.10 re-defines this register as 32-bit Block Count \\[Refer to Section 1.15 for more details about block count extension\\]. In version 4.00, this register may be used as 32-bit block count only for Auto CMD23 to set the argument of the CMD23 while executing Auto CMD23. The Host Controller would decrement the block count of this register every block transfer and data transfer stops when the count reaches zero. This register should be accessed only when no transaction is executing. Reading this register during data transfers may return invalid value."]
pub type SdmaAddressR = crate::FieldReader<u16>;
#[doc = "Field `SDMA_ADDRESS` writer - 15:0\\]
When Host Version 4 Enable is set to 0 in the Host Control 2 register,DMA uses this register as system address in only 32-bit addressing mode. Auto CMD23 cannot be used with SDMA. When Host Version 4 Enable is set to 1, SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of using this register to sup- port both 32-bit and 64-bit addressing. This register is re-assigned to 32-bit Block Count and then SDMA may use Auto CMD23.\\[1\\]
SDMA System Address \\[Host Version 4 Enable = 0\\]
This register contains the system memory address for a SDMA transfer in 32-bit addressing mode. When the Host Controller stops a SDMA transfer, this register shall point to the system address of the next contiguous data position. It can be accessed only if no transaction is executing \\[i.e., after a transaction has stopped\\]. Reading this register during SDMA transfers may return an invalid value. The Host Driver shall initialize this register before starting a SDMA transaction. After SDMA has stopped, the next system address of the next contiguous data posi- tion can be read from this register. The SDMA transfer waits at the every boundary specified by the SDMA Buffer Boundary in the Block Size register. The Host Controller generates DMA Interrupt to request the Host Driver to update this register. The Host Driver sets the next system address of the next data position to this register. When the most upper byte of this register \\[003h\\]
is written, the Host Controller restarts the SDMA transfer. When restarting SDMA by setting Continue Request in the Block Gap Control register, the Host Controller shall start at the next contiguous address stored here in the SDMA System Address register. ADMA does not use this register. \\[2\\]
32-bit Block Count \\[Host Version 4 Enable = 1\\]
Host Controller Version 4.10 re-defines this register as 32-bit Block Count \\[Refer to Section 1.15 for more details about block count extension\\]. In version 4.00, this register may be used as 32-bit block count only for Auto CMD23 to set the argument of the CMD23 while executing Auto CMD23. The Host Controller would decrement the block count of this register every block transfer and data transfer stops when the count reaches zero. This register should be accessed only when no transaction is executing. Reading this register during data transfers may return invalid value."]
pub type SdmaAddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
When Host Version 4 Enable is set to 0 in the Host Control 2 register,DMA uses this register as system address in only 32-bit addressing mode. Auto CMD23 cannot be used with SDMA. When Host Version 4 Enable is set to 1, SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of using this register to sup- port both 32-bit and 64-bit addressing. This register is re-assigned to 32-bit Block Count and then SDMA may use Auto CMD23.\\[1\\]
SDMA System Address \\[Host Version 4 Enable = 0\\]
This register contains the system memory address for a SDMA transfer in 32-bit addressing mode. When the Host Controller stops a SDMA transfer, this register shall point to the system address of the next contiguous data position. It can be accessed only if no transaction is executing \\[i.e., after a transaction has stopped\\]. Reading this register during SDMA transfers may return an invalid value. The Host Driver shall initialize this register before starting a SDMA transaction. After SDMA has stopped, the next system address of the next contiguous data posi- tion can be read from this register. The SDMA transfer waits at the every boundary specified by the SDMA Buffer Boundary in the Block Size register. The Host Controller generates DMA Interrupt to request the Host Driver to update this register. The Host Driver sets the next system address of the next data position to this register. When the most upper byte of this register \\[003h\\]
is written, the Host Controller restarts the SDMA transfer. When restarting SDMA by setting Continue Request in the Block Gap Control register, the Host Controller shall start at the next contiguous address stored here in the SDMA System Address register. ADMA does not use this register. \\[2\\]
32-bit Block Count \\[Host Version 4 Enable = 1\\]
Host Controller Version 4.10 re-defines this register as 32-bit Block Count \\[Refer to Section 1.15 for more details about block count extension\\]. In version 4.00, this register may be used as 32-bit block count only for Auto CMD23 to set the argument of the CMD23 while executing Auto CMD23. The Host Controller would decrement the block count of this register every block transfer and data transfer stops when the count reaches zero. This register should be accessed only when no transaction is executing. Reading this register during data transfers may return invalid value."]
    #[inline(always)]
    pub fn sdma_address(&self) -> SdmaAddressR {
        SdmaAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
When Host Version 4 Enable is set to 0 in the Host Control 2 register,DMA uses this register as system address in only 32-bit addressing mode. Auto CMD23 cannot be used with SDMA. When Host Version 4 Enable is set to 1, SDMA uses ADMA System Address register \\[05Fh-058h\\]
instead of using this register to sup- port both 32-bit and 64-bit addressing. This register is re-assigned to 32-bit Block Count and then SDMA may use Auto CMD23.\\[1\\]
SDMA System Address \\[Host Version 4 Enable = 0\\]
This register contains the system memory address for a SDMA transfer in 32-bit addressing mode. When the Host Controller stops a SDMA transfer, this register shall point to the system address of the next contiguous data position. It can be accessed only if no transaction is executing \\[i.e., after a transaction has stopped\\]. Reading this register during SDMA transfers may return an invalid value. The Host Driver shall initialize this register before starting a SDMA transaction. After SDMA has stopped, the next system address of the next contiguous data posi- tion can be read from this register. The SDMA transfer waits at the every boundary specified by the SDMA Buffer Boundary in the Block Size register. The Host Controller generates DMA Interrupt to request the Host Driver to update this register. The Host Driver sets the next system address of the next data position to this register. When the most upper byte of this register \\[003h\\]
is written, the Host Controller restarts the SDMA transfer. When restarting SDMA by setting Continue Request in the Block Gap Control register, the Host Controller shall start at the next contiguous address stored here in the SDMA System Address register. ADMA does not use this register. \\[2\\]
32-bit Block Count \\[Host Version 4 Enable = 1\\]
Host Controller Version 4.10 re-defines this register as 32-bit Block Count \\[Refer to Section 1.15 for more details about block count extension\\]. In version 4.00, this register may be used as 32-bit block count only for Auto CMD23 to set the argument of the CMD23 while executing Auto CMD23. The Host Controller would decrement the block count of this register every block transfer and data transfer stops when the count reaches zero. This register should be accessed only when no transaction is executing. Reading this register during data transfers may return invalid value."]
    #[inline(always)]
    #[must_use]
    pub fn sdma_address(&mut self) -> SdmaAddressW<SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec> {
        SdmaAddressW::new(self, 0)
    }
}
#[doc = "This register contains the Lower 16-bit of physical system memory address used for DMA transfers or the second argument for the Auto CMD23 in Host version 3.0 and as 32-bit Block Count in Version 4.10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_sdma_sys_addr_lo::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_sdma_sys_addr_lo to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgSdmaSysAddrLoSpec {
    const RESET_VALUE: u16 = 0;
}
