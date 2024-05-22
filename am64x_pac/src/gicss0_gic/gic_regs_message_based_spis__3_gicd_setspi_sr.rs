#[doc = "Register `GIC_REGS_Message_based_SPIs__3_GICD_SETSPI_SR` reader"]
pub type R = crate::R<GicRegsMessageBasedSpis_3GicdSetspiSrSpec>;
#[doc = "Register `GIC_REGS_Message_based_SPIs__3_GICD_SETSPI_SR` writer"]
pub type W = crate::W<GicRegsMessageBasedSpis_3GicdSetspiSrSpec>;
#[doc = "Field `MESSAGE_BASED_SPIS__3_GICD_SETSPI_SR__0_10` reader - 9:0\\]
SPI ID"]
pub type MessageBasedSpis_3GicdSetspiSr_0_10R = crate::FieldReader<u16>;
#[doc = "Field `MESSAGE_BASED_SPIS__3_GICD_SETSPI_SR__0_10` writer - 9:0\\]
SPI ID"]
pub type MessageBasedSpis_3GicdSetspiSr_0_10W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
SPI ID"]
    #[inline(always)]
    pub fn message_based_spis__3_gicd_setspi_sr__0_10(
        &self,
    ) -> MessageBasedSpis_3GicdSetspiSr_0_10R {
        MessageBasedSpis_3GicdSetspiSr_0_10R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
SPI ID"]
    #[inline(always)]
    #[must_use]
    pub fn message_based_spis__3_gicd_setspi_sr__0_10(
        &mut self,
    ) -> MessageBasedSpis_3GicdSetspiSr_0_10W<GicRegsMessageBasedSpis_3GicdSetspiSrSpec> {
        MessageBasedSpis_3GicdSetspiSr_0_10W::new(self, 0)
    }
}
#[doc = "GICD_SETSPI_SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_message_based_spis__3_gicd_setspi_sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_message_based_spis__3_gicd_setspi_sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsMessageBasedSpis_3GicdSetspiSrSpec;
impl crate::RegisterSpec for GicRegsMessageBasedSpis_3GicdSetspiSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_message_based_spis__3_gicd_setspi_sr::R`](R) reader structure"]
impl crate::Readable for GicRegsMessageBasedSpis_3GicdSetspiSrSpec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_message_based_spis__3_gicd_setspi_sr::W`](W) writer structure"]
impl crate::Writable for GicRegsMessageBasedSpis_3GicdSetspiSrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_Message_based_SPIs__3_GICD_SETSPI_SR to value 0"]
impl crate::Resettable for GicRegsMessageBasedSpis_3GicdSetspiSrSpec {
    const RESET_VALUE: u32 = 0;
}
