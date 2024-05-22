#[doc = "Register `ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l` reader"]
pub type R = crate::R<IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec>;
#[doc = "Register `ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l` writer"]
pub type W = crate::W<IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec>;
#[doc = "Field `END_ADDRESS_LSB` reader - 11:0\\]
End address bits 11 to 0 are forced to Fs as address must be 4KB aligned."]
pub type EndAddressLsbR = crate::FieldReader<u16>;
#[doc = "Field `END_ADDRESS_LSB` writer - 11:0\\]
End address bits 11 to 0 are forced to Fs as address must be 4KB aligned."]
pub type EndAddressLsbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `END_ADDRESS_L` reader - 31:12\\]
End address bits 31 to 12 to include in the match."]
pub type EndAddressLR = crate::FieldReader<u32>;
#[doc = "Field `END_ADDRESS_L` writer - 31:12\\]
End address bits 31 to 12 to include in the match."]
pub type EndAddressLW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
End address bits 11 to 0 are forced to Fs as address must be 4KB aligned."]
    #[inline(always)]
    pub fn end_address_lsb(&self) -> EndAddressLsbR {
        EndAddressLsbR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
End address bits 31 to 12 to include in the match."]
    #[inline(always)]
    pub fn end_address_l(&self) -> EndAddressLR {
        EndAddressLR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
End address bits 11 to 0 are forced to Fs as address must be 4KB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_lsb(
        &mut self,
    ) -> EndAddressLsbW<IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec> {
        EndAddressLsbW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
End address bits 31 to 12 to include in the match."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_l(
        &mut self,
    ) -> EndAddressLW<IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec> {
        EndAddressLW::new(self, 12)
    }
}
#[doc = "The ISC Region 3 End Address Low Register defines the end included address bits 31 to 0 for the master Iusb3p0ss64_16ffc_main_0.mstw0 region 3 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec;
impl crate::RegisterSpec for IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::R`](R) reader structure"]
impl crate::Readable for IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec {}
#[doc = "`write(|w| ..)` method takes [`isc_regs_iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l::W`](W) writer structure"]
impl crate::Writable for IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Iusb3p0ss64_16ffc_main_0_mstw0_isc_region_3_end_address_l to value 0x4095"]
impl crate::Resettable for IscRegsIusb3p0ss64_16ffcMain0Mstw0IscRegion3EndAddressLSpec {
    const RESET_VALUE: u32 = 0x4095;
}
