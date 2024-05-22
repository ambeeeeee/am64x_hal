#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_vendor_specfic_ptr` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_vendor_specfic_ptr` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec>;
#[doc = "Field `VENDOR_SPECFIC_PTR` reader - 15:0\\]
Pointer for Vendor Specific Area"]
pub type VendorSpecficPtrR = crate::FieldReader<u16>;
#[doc = "Field `VENDOR_SPECFIC_PTR` writer - 15:0\\]
Pointer for Vendor Specific Area"]
pub type VendorSpecficPtrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for Vendor Specific Area"]
    #[inline(always)]
    pub fn vendor_specfic_ptr(&self) -> VendorSpecficPtrR {
        VendorSpecficPtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Pointer for Vendor Specific Area"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_specfic_ptr(
        &mut self,
    ) -> VendorSpecficPtrW<SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec> {
        VendorSpecficPtrW::new(self, 0)
    }
}
#[doc = "This register is pointer for UHS-II Vendor Specific Pointer Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_vendor_specfic_ptr::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_vendor_specfic_ptr to value 0x0320"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgVendorSpecficPtrSpec {
    const RESET_VALUE: u16 = 0x0320;
}
