#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l` reader"]
pub type R = crate::R<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec>;
#[doc = "Register `FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l` writer"]
pub type W = crate::W<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec>;
#[doc = "Field `START_ADDRESS_LSB` reader - 11:0\\]
Start address bits 11 to 0 are forced to 0 as address must be 4KB aligned."]
pub type StartAddressLsbR = crate::FieldReader<u16>;
#[doc = "Field `START_ADDRESS_LSB` writer - 11:0\\]
Start address bits 11 to 0 are forced to 0 as address must be 4KB aligned."]
pub type StartAddressLsbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `START_ADDRESS_L` reader - 31:12\\]
Start address bits 31 to 12. Lowest 12 bits are forced to 0 as address must be 4KB aligned."]
pub type StartAddressLR = crate::FieldReader<u32>;
#[doc = "Field `START_ADDRESS_L` writer - 31:12\\]
Start address bits 31 to 12. Lowest 12 bits are forced to 0 as address must be 4KB aligned."]
pub type StartAddressLW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Start address bits 11 to 0 are forced to 0 as address must be 4KB aligned."]
    #[inline(always)]
    pub fn start_address_lsb(&self) -> StartAddressLsbR {
        StartAddressLsbR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Start address bits 31 to 12. Lowest 12 bits are forced to 0 as address must be 4KB aligned."]
    #[inline(always)]
    pub fn start_address_l(&self) -> StartAddressLR {
        StartAddressLR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Start address bits 11 to 0 are forced to 0 as address must be 4KB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_lsb(
        &mut self,
    ) -> StartAddressLsbW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec> {
        StartAddressLsbW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Start address bits 31 to 12. Lowest 12 bits are forced to 0 as address must be 4KB aligned."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_l(
        &mut self,
    ) -> StartAddressLW<FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec> {
        StartAddressLW::new(self, 12)
    }
}
#[doc = "The FW Region 5 Start Address Low Register defines the start address bits 31 to 0 for the slave br_scrm_64b_clk2_to_scrp_misc_clk2_l0 region 5 firewall.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec;
impl crate::RegisterSpec for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l::R`](R) reader structure"]
impl crate::Readable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec {}
#[doc = "`write(|w| ..)` method takes [`fw_regs_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l::W`](W) writer structure"]
impl crate::Writable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FW_REGS_br_scrm_64b_clk2_to_scrp_misc_clk2_l0_fw_region_5_start_address_l to value 0"]
impl crate::Resettable for FwRegsBrScrm64bClk2ToScrpMiscClk2L0FwRegion5StartAddressLSpec {
    const RESET_VALUE: u32 = 0;
}
