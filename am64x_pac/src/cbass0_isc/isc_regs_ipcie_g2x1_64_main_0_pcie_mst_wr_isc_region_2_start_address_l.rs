#[doc = "Register `ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l` reader"]
pub type R = crate::R<IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec>;
#[doc = "Register `ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l` writer"]
pub type W = crate::W<IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec>;
#[doc = "Field `START_ADDRESS_LSB` reader - 11:0\\]
Start address bits 11 to 0 must be 0 as address must be 4KB aligned in address mode. Can also be channel number in channel mode."]
pub type StartAddressLsbR = crate::FieldReader<u16>;
#[doc = "Field `START_ADDRESS_LSB` writer - 11:0\\]
Start address bits 11 to 0 must be 0 as address must be 4KB aligned in address mode. Can also be channel number in channel mode."]
pub type StartAddressLsbW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `START_ADDRESS_L` reader - 31:12\\]
Start address bits 31 to 12."]
pub type StartAddressLR = crate::FieldReader<u32>;
#[doc = "Field `START_ADDRESS_L` writer - 31:12\\]
Start address bits 31 to 12."]
pub type StartAddressLW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Start address bits 11 to 0 must be 0 as address must be 4KB aligned in address mode. Can also be channel number in channel mode."]
    #[inline(always)]
    pub fn start_address_lsb(&self) -> StartAddressLsbR {
        StartAddressLsbR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Start address bits 31 to 12."]
    #[inline(always)]
    pub fn start_address_l(&self) -> StartAddressLR {
        StartAddressLR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Start address bits 11 to 0 must be 0 as address must be 4KB aligned in address mode. Can also be channel number in channel mode."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_lsb(
        &mut self,
    ) -> StartAddressLsbW<IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec> {
        StartAddressLsbW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Start address bits 31 to 12."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_l(
        &mut self,
    ) -> StartAddressLW<IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec> {
        StartAddressLW::new(self, 12)
    }
}
#[doc = "The ISC Region 2 Start Address Low Register defines the start address bits 31 to 0 for the master Ipcie_g2x1_64_main_0.pcie_mst_wr region 2 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec;
impl crate::RegisterSpec for IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::R`](R) reader structure"]
impl crate::Readable for IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec {}
#[doc = "`write(|w| ..)` method takes [`isc_regs_ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l::W`](W) writer structure"]
impl crate::Writable for IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Ipcie_g2x1_64_main_0_pcie_mst_wr_isc_region_2_start_address_l to value 0"]
impl crate::Resettable for IscRegsIpcieG2x1_64Main0PcieMstWrIscRegion2StartAddressLSpec {
    const RESET_VALUE: u32 = 0;
}
