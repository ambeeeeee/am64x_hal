#[doc = "Register `ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h` reader"]
pub type R = crate::R<IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec>;
#[doc = "Register `ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h` writer"]
pub type W = crate::W<IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec>;
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
    ) -> StartAddressHW<IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec>
    {
        StartAddressHW::new(self, 0)
    }
}
#[doc = "The ISC Region 0 Start Address High Register defines the start address bits 47 to 32 for the master Isam64_a53_256kb_wrap_main_0.a53_dual_wrap_cba_axi_w region 0 ISC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec;
impl crate::RegisterSpec
    for IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::R`](R) reader structure"]
impl crate::Readable
    for IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec
{
}
#[doc = "`write(|w| ..)` method takes [`isc_regs_isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h::W`](W) writer structure"]
impl crate::Writable
    for IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec
{
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISC_REGS_Isam64_a53_256kb_wrap_main_0_a53_dual_wrap_cba_axi_w_isc_region_0_start_address_h to value 0"]
impl crate::Resettable
    for IscRegsIsam64A53_256kbWrapMain0A53DualWrapCbaAxiWIscRegion0StartAddressHSpec
{
    const RESET_VALUE: u32 = 0;
}
