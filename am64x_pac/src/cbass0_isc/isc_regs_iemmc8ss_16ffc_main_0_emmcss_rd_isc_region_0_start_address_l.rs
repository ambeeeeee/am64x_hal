#[doc = "Register `ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l` reader"]
pub type R = crate::R<IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec>;
#[doc = "Register `ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l` writer"]
pub type W = crate::W<IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec>;
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
    ) -> StartAddressLsbW<IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec> {
        StartAddressLsbW::new(self, 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Start address bits 31 to 12."]
    #[inline(always)]
    #[must_use]
    pub fn start_address_l(
        &mut self,
    ) -> StartAddressLW<IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec> {
        StartAddressLW::new(self, 12)
    }
}
#[doc = "The ISC Region 0 Start Address Low Register defines the start address bits 31 to 0 for the master Iemmc8ss_16ffc_main_0.emmcss_rd region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec;
impl crate::RegisterSpec for IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::R`](R) reader structure"]
impl crate::Readable for IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec {}
#[doc = "`write(|w| ..)` method takes [`isc_regs_iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l::W`](W) writer structure"]
impl crate::Writable for IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Iemmc8ss_16ffc_main_0_emmcss_rd_isc_region_0_start_address_l to value 0"]
impl crate::Resettable for IscRegsIemmc8ss16ffcMain0EmmcssRdIscRegion0StartAddressLSpec {
    const RESET_VALUE: u32 = 0;
}
