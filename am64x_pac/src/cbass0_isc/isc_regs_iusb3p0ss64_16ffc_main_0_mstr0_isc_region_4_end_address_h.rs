#[doc = "Register `ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h` reader"]
pub type R = crate::R<IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec>;
#[doc = "Register `ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h` writer"]
pub type W = crate::W<IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec>;
#[doc = "Field `END_ADDRESS_H` reader - 15:0\\]
End address bits 47 to 32."]
pub type EndAddressHR = crate::FieldReader<u16>;
#[doc = "Field `END_ADDRESS_H` writer - 15:0\\]
End address bits 47 to 32."]
pub type EndAddressHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
End address bits 47 to 32."]
    #[inline(always)]
    pub fn end_address_h(&self) -> EndAddressHR {
        EndAddressHR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
End address bits 47 to 32."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_h(
        &mut self,
    ) -> EndAddressHW<IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec> {
        EndAddressHW::new(self, 0)
    }
}
#[doc = "The ISC Region 4 End Address High Register defines the end address bits 47 to 32 for the master Iusb3p0ss64_16ffc_main_0.mstr0 region 4 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec;
impl crate::RegisterSpec for IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::R`](R) reader structure"]
impl crate::Readable for IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec {}
#[doc = "`write(|w| ..)` method takes [`isc_regs_iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h::W`](W) writer structure"]
impl crate::Writable for IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstr0_isc_region_4_end_address_h to value 0"]
impl crate::Resettable for IscRegsIusb3p0ss64_16ffcMain0Mstr0IscRegion4EndAddressHSpec {
    const RESET_VALUE: u32 = 0;
}
