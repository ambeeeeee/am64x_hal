#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h` reader"]
pub type R = crate::R<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec>;
#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h` writer"]
pub type W = crate::W<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec>;
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
    ) -> EndAddressHW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec> {
        EndAddressHW::new(self, 0)
    }
}
#[doc = "The FW Region 12 End Address High Register defines the end address bits 47 to 32 to include for the slave br_scrm_64b_clk2_to_scrp_misc_clk2_l0 region 12 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec;
impl crate::RegisterSpec for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h::R`](R) reader structure"]
impl crate::Readable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h::W`](W) writer structure"]
impl crate::Writable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_12_end_address_h to value 0"]
impl crate::Resettable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion12EndAddressHSpec {
    const RESET_VALUE: u32 = 0;
}
