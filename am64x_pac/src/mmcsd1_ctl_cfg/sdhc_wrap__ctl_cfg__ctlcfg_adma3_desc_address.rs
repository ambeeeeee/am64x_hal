#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma3_desc_address` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_adma3_desc_address` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec>;
#[doc = "Field `INTG_DESC_ADDR` reader - 63:0\\]
The start address of Integrated DMA Descriptor is set to this register. Writing to a specific address starts ADMA3 depends on 32-bit/64-bit address-ing. The ADMA3 fetches one Descriptor Address and increments this field to indicate the next Descriptor address. The 32-bit addressing Host Driver uses lower 32- bit of this register and shall program Descriptor Table on 32-bit boundary.ADMA3 ignores lower 2-bit of this register and assumes it to be 00b. Writing to 07Bh starts ADMA3 data transfer. The 64-bit addressing Host Driver uses all 64-bit of this register and shall program Descriptor Table on 64-bit boundary. ADMA3 ignores lower 3-bit of this register and assumes it to be 000b. Writing to 07Fh starts ADMA3 data transfer. Register Value- 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
pub type IntgDescAddrR = crate::FieldReader<u64>;
#[doc = "Field `INTG_DESC_ADDR` writer - 63:0\\]
The start address of Integrated DMA Descriptor is set to this register. Writing to a specific address starts ADMA3 depends on 32-bit/64-bit address-ing. The ADMA3 fetches one Descriptor Address and increments this field to indicate the next Descriptor address. The 32-bit addressing Host Driver uses lower 32- bit of this register and shall program Descriptor Table on 32-bit boundary.ADMA3 ignores lower 2-bit of this register and assumes it to be 00b. Writing to 07Bh starts ADMA3 data transfer. The 64-bit addressing Host Driver uses all 64-bit of this register and shall program Descriptor Table on 64-bit boundary. ADMA3 ignores lower 3-bit of this register and assumes it to be 000b. Writing to 07Fh starts ADMA3 data transfer. Register Value- 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
pub type IntgDescAddrW<'a, REG> = crate::FieldWriter<'a, REG, 64, u64>;
impl R {
    #[doc = "Bits 0:63 - 63:0\\]
The start address of Integrated DMA Descriptor is set to this register. Writing to a specific address starts ADMA3 depends on 32-bit/64-bit address-ing. The ADMA3 fetches one Descriptor Address and increments this field to indicate the next Descriptor address. The 32-bit addressing Host Driver uses lower 32- bit of this register and shall program Descriptor Table on 32-bit boundary.ADMA3 ignores lower 2-bit of this register and assumes it to be 00b. Writing to 07Bh starts ADMA3 data transfer. The 64-bit addressing Host Driver uses all 64-bit of this register and shall program Descriptor Table on 64-bit boundary. ADMA3 ignores lower 3-bit of this register and assumes it to be 000b. Writing to 07Fh starts ADMA3 data transfer. Register Value- 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
    #[inline(always)]
    pub fn intg_desc_addr(&self) -> IntgDescAddrR {
        IntgDescAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:63 - 63:0\\]
The start address of Integrated DMA Descriptor is set to this register. Writing to a specific address starts ADMA3 depends on 32-bit/64-bit address-ing. The ADMA3 fetches one Descriptor Address and increments this field to indicate the next Descriptor address. The 32-bit addressing Host Driver uses lower 32- bit of this register and shall program Descriptor Table on 32-bit boundary.ADMA3 ignores lower 2-bit of this register and assumes it to be 00b. Writing to 07Bh starts ADMA3 data transfer. The 64-bit addressing Host Driver uses all 64-bit of this register and shall program Descriptor Table on 64-bit boundary. ADMA3 ignores lower 3-bit of this register and assumes it to be 000b. Writing to 07Fh starts ADMA3 data transfer. Register Value- 00000000_xxxxxxxxh Addressing Mode - 32-bit System Address Register Value - xxxxxxxx_xxxxxxxxh Addressing Mode - 64-bit System Address"]
    #[inline(always)]
    #[must_use]
    pub fn intg_desc_addr(&mut self) -> IntgDescAddrW<SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec> {
        IntgDescAddrW::new(self, 0)
    }
}
#[doc = "The start address of Integrated DMA Descriptor is set to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_adma3_desc_address::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_adma3_desc_address to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgAdma3DescAddressSpec {
    const RESET_VALUE: u64 = 0;
}
