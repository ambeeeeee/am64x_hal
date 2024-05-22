#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l` reader"]
pub type R = crate::R<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec>;
#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l` writer"]
pub type W = crate::W<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec>;
#[doc = "Field `END_ADDRESS_LSB` reader - 11:0\\]
End address bits 11 to 0 are forced to 1s as address must be 4KB aligned minus 1."]
pub type EndAddressLsbR = crate::FieldReader<u16>;
#[doc = "Field `END_ADDRESS_LSB` writer - 11:0\\]
End address bits 11 to 0 are forced to 1s as address must be 4KB aligned minus 1."]
pub type EndAddressLsbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `END_ADDRESS_L` reader - 31:12\\]
End address bits 31 to 12 to include in the match. Lowest 12 bits are forced to 1s as address must be 4KB aligned."]
pub type EndAddressLR = crate::FieldReader<u32>;
#[doc = "Field `END_ADDRESS_L` writer - 31:12\\]
End address bits 31 to 12 to include in the match. Lowest 12 bits are forced to 1s as address must be 4KB aligned."]
pub type EndAddressLW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
End address bits 11 to 0 are forced to 1s as address must be 4KB aligned minus 1."]
    #[inline(always)]
    pub fn end_address_lsb(&self) -> EndAddressLsbR {
        EndAddressLsbR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
End address bits 31 to 12 to include in the match. Lowest 12 bits are forced to 1s as address must be 4KB aligned."]
    #[inline(always)]
    pub fn end_address_l(&self) -> EndAddressLR {
        EndAddressLR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
End address bits 11 to 0 are forced to 1s as address must be 4KB aligned minus 1."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_lsb(
        &mut self,
    ) -> EndAddressLsbW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec> {
        EndAddressLsbW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
End address bits 31 to 12 to include in the match. Lowest 12 bits are forced to 1s as address must be 4KB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn end_address_l(
        &mut self,
    ) -> EndAddressLW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec> {
        EndAddressLW::new(self, 12)
    }
}
#[doc = "The FW Region 13 End Address Low Register defines the end address bits 31 to 0 to include for the slave br_scrm_64b_clk2_to_scrp_misc_clk2_l0 region 13 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec;
impl crate::RegisterSpec for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l::R`](R) reader structure"]
impl crate::Readable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l::W`](W) writer structure"]
impl crate::Writable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_13_end_address_l to value 0x4095"]
impl crate::Resettable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion13EndAddressLSpec {
    const RESET_VALUE: u32 = 0x4095;
}
