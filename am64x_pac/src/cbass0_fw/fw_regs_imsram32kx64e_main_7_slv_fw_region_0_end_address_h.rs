#[doc = "Register `FW_REGS_Imsram32kx64e_main_7_slv_fw_region_0_end_address_h` reader"]
pub type R = crate::R<FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec>;
#[doc = "Register `FW_REGS_Imsram32kx64e_main_7_slv_fw_region_0_end_address_h` writer"]
pub type W = crate::W<FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec>;
#[doc = "Field `END_ADDRESS_H` reader - 15:0\\]
End address bits 47 to 32 to include in the match."]
pub type EndAddressHR = crate::FieldReader<u16>;
#[doc = "Field `END_ADDRESS_H` writer - 15:0\\]
End address bits 47 to 32 to include in the match."]
pub type EndAddressHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
End address bits 47 to 32 to include in the match."]
    #[inline(always)]
    pub fn end_address_h(&self) -> EndAddressHR {
        EndAddressHR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
End address bits 47 to 32 to include in the match."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_h(
        &mut self,
    ) -> EndAddressHW<FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec> {
        EndAddressHW::new(self, 0)
    }
}
#[doc = "The FW Region 0 End Address High Register defines the end address bits 47 to 32 to include for the slave Imsram32kx64e_main_7.slv region 0 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_imsram32kx64e_main_7_slv_fw_region_0_end_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_imsram32kx64e_main_7_slv_fw_region_0_end_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec;
impl crate::RegisterSpec for FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_imsram32kx64e_main_7_slv_fw_region_0_end_address_h::R`](R) reader structure"]
impl crate::Readable for FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_imsram32kx64e_main_7_slv_fw_region_0_end_address_h::W`](W) writer structure"]
impl crate::Writable for FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_Imsram32kx64e_main_7_slv_fw_region_0_end_address_h to value 0"]
impl crate::Resettable for FwRegsImsram32kx64eMain7SlvFwRegion0EndAddressHSpec {
    const RESET_VALUE: u32 = 0;
}
