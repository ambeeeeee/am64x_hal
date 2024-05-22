#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma_sys_address` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma_sys_address` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec>;
#[doc = "Field `ADMA_ADDR` reader - 63:0\\]
The 32-bit addressing Host Driver uses lower 32-bit of this register \\[upper 32-bit should be set to 0\\]
and shall program Descriptor Table on 32-bit boundary andset 32-bit boundary address to this register. DMA2/3 ignores lower 2-bit of this register and assumes it to be 00b. DMA in 64-bit addressing. The 64-bit addressing Host Driver uses all bits of this register and shall program Descriptor Table on 64-bit boundary and set 64-bit boundary address to this register. DMA2/3 ignores lower 3-bit of this register andassumes it to be 000b. SDMA If Host Version 4.00 Enable is set to 1, SDMA use this register to indicate System Address of data location instead of using SDMA System Address register \\[Offset 003-000h\\]. SDMA can be used in 32-bit and 64-bit addressing in Version 4.00. ADMA2 This register holds byte address of executing command of the Descriptor table. At the start of ADMA2, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. ADMA3 This register is set by ADMA3. Host Driver is not necessary to set this register. The ADMA3 increments address of this register,which points to next line, when every time fetching a Descriptor line. When Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. Register Value - 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
pub type AdmaAddrR = crate::FieldReader<u64>;
#[doc = "Field `ADMA_ADDR` writer - 63:0\\]
The 32-bit addressing Host Driver uses lower 32-bit of this register \\[upper 32-bit should be set to 0\\]
and shall program Descriptor Table on 32-bit boundary andset 32-bit boundary address to this register. DMA2/3 ignores lower 2-bit of this register and assumes it to be 00b. DMA in 64-bit addressing. The 64-bit addressing Host Driver uses all bits of this register and shall program Descriptor Table on 64-bit boundary and set 64-bit boundary address to this register. DMA2/3 ignores lower 3-bit of this register andassumes it to be 000b. SDMA If Host Version 4.00 Enable is set to 1, SDMA use this register to indicate System Address of data location instead of using SDMA System Address register \\[Offset 003-000h\\]. SDMA can be used in 32-bit and 64-bit addressing in Version 4.00. ADMA2 This register holds byte address of executing command of the Descriptor table. At the start of ADMA2, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. ADMA3 This register is set by ADMA3. Host Driver is not necessary to set this register. The ADMA3 increments address of this register,which points to next line, when every time fetching a Descriptor line. When Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. Register Value - 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
pub type AdmaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - 63:0\\]
The 32-bit addressing Host Driver uses lower 32-bit of this register \\[upper 32-bit should be set to 0\\]
and shall program Descriptor Table on 32-bit boundary andset 32-bit boundary address to this register. DMA2/3 ignores lower 2-bit of this register and assumes it to be 00b. DMA in 64-bit addressing. The 64-bit addressing Host Driver uses all bits of this register and shall program Descriptor Table on 64-bit boundary and set 64-bit boundary address to this register. DMA2/3 ignores lower 3-bit of this register andassumes it to be 000b. SDMA If Host Version 4.00 Enable is set to 1, SDMA use this register to indicate System Address of data location instead of using SDMA System Address register \\[Offset 003-000h\\]. SDMA can be used in 32-bit and 64-bit addressing in Version 4.00. ADMA2 This register holds byte address of executing command of the Descriptor table. At the start of ADMA2, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. ADMA3 This register is set by ADMA3. Host Driver is not necessary to set this register. The ADMA3 increments address of this register,which points to next line, when every time fetching a Descriptor line. When Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. Register Value - 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
    #[inline(always)]
    pub fn adma_addr(&self) -> AdmaAddrR {
        AdmaAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - 63:0\\]
The 32-bit addressing Host Driver uses lower 32-bit of this register \\[upper 32-bit should be set to 0\\]
and shall program Descriptor Table on 32-bit boundary andset 32-bit boundary address to this register. DMA2/3 ignores lower 2-bit of this register and assumes it to be 00b. DMA in 64-bit addressing. The 64-bit addressing Host Driver uses all bits of this register and shall program Descriptor Table on 64-bit boundary and set 64-bit boundary address to this register. DMA2/3 ignores lower 3-bit of this register andassumes it to be 000b. SDMA If Host Version 4.00 Enable is set to 1, SDMA use this register to indicate System Address of data location instead of using SDMA System Address register \\[Offset 003-000h\\]. SDMA can be used in 32-bit and 64-bit addressing in Version 4.00. ADMA2 This register holds byte address of executing command of the Descriptor table. At the start of ADMA2, the Host Driver shall set start address of the Descriptor table. The ADMA increments this register address, which points to next line, when every fetching a Descriptor line. When the ADMA Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. ADMA3 This register is set by ADMA3. Host Driver is not necessary to set this register. The ADMA3 increments address of this register,which points to next line, when every time fetching a Descriptor line. When Error Interrupt is generated, this register shall hold the Descriptor address depending on the ADMA state. Register Value - 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
    #[inline(always)]
    #[must_use]
    pub fn adma_addr(&mut self) -> AdmaAddrW<SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec> {
        AdmaAddrW::new(self, 0)
    }
}
#[doc = "This register contains the physical address used for ADMA data transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_adma_sys_address::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_adma_sys_address to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgAdmaSysAddressSpec {
    const RESET_VALUE: u64 = 0;
}
