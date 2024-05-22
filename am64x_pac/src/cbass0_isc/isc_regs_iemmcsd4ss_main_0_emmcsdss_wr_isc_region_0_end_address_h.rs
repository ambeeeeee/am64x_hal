#[doc = "Register `ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h` reader"]
pub type R = crate::R<IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec>;
#[doc = "Register `ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h` writer"]
pub type W = crate::W<IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec>;
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
    ) -> EndAddressHW<IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec> {
        EndAddressHW::new(self, 0)
    }
}
#[doc = "The ISC Region 0 End Address High Register defines the end address bits 47 to 32 for the master Iemmcsd4ss_main_0.emmcsdss_wr region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec;
impl crate::RegisterSpec for IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::R`](R) reader structure"]
impl crate::Readable for IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec {}
#[doc = "`write(|w| ..)` method takes [`isc_regs_iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h::W`](W) writer structure"]
impl crate::Writable for IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Iemmcsd4ss_main_0_emmcsdss_wr_isc_region_0_end_address_h to value 0"]
impl crate::Resettable for IscRegsIemmcsd4ssMain0EmmcsdssWrIscRegion0EndAddressHSpec {
    const RESET_VALUE: u32 = 0;
}