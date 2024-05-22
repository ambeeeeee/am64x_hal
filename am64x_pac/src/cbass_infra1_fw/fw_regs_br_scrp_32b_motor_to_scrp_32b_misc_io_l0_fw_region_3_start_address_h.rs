#[doc = "Register `FW_REGS_br_SCRP_32b_motor_to_SCRP_32b_miscIO_l0_fw_region_3_start_address_h` reader"]
pub type R = crate::R<FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec>;
#[doc = "Register `FW_REGS_br_SCRP_32b_motor_to_SCRP_32b_miscIO_l0_fw_region_3_start_address_h` writer"]
pub type W = crate::W<FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec>;
#[doc = "Field `START_ADDRESS_H` reader - 15:0\\]
Start address bits 47 to 32."]
pub type StartAddressHR = crate::FieldReader<u16>;
#[doc = "Field `START_ADDRESS_H` writer - 15:0\\]
Start address bits 47 to 32."]
pub type StartAddressHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Start address bits 47 to 32."]
    #[inline(always)]
    pub fn start_address_h(&self) -> StartAddressHR {
        StartAddressHR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Start address bits 47 to 32."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_h(
        &mut self,
    ) -> StartAddressHW<FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec> {
        StartAddressHW::new(self, 0)
    }
}
#[doc = "The FW Region 3 Start Address High Register defines the start address bits 47 to 32 for the slave br_SCRP_32b_motor_to_SCRP_32b_miscIO_l0 region 3 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_br_scrp_32b_motor_to_scrp_32b_misc_io_l0_fw_region_3_start_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_br_scrp_32b_motor_to_scrp_32b_misc_io_l0_fw_region_3_start_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec;
impl crate::RegisterSpec for FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_br_scrp_32b_motor_to_scrp_32b_misc_io_l0_fw_region_3_start_address_h::R`](R) reader structure"]
impl crate::Readable for FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_br_scrp_32b_motor_to_scrp_32b_misc_io_l0_fw_region_3_start_address_h::W`](W) writer structure"]
impl crate::Writable for FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_br_SCRP_32b_motor_to_SCRP_32b_miscIO_l0_fw_region_3_start_address_h to value 0"]
impl crate::Resettable for FwRegsBrScrp32bMotorToScrp32bMiscIoL0FwRegion3StartAddressHSpec {
    const RESET_VALUE: u32 = 0;
}
