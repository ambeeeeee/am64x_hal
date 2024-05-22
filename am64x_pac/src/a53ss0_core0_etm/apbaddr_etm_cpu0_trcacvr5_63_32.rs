#[doc = "Register `APBADDR_ETM_CPU0_TRCACVR5_63_32` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trcacvr5_63_32Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCACVR5_63_32` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trcacvr5_63_32Spec>;
#[doc = "Field `ADDRESS` reader - 31:0\\]
Address value.The address comparators can support implementations that use multiple address widths. When the trace unit compares the ADDRESS field with an address that has a width less than this field, then it must ignore those upper bits in the comparison. For example, in a system that supports both 32-bit and 64-bit addresses, when the processor is in 32-bit state the comparator must ignore the ADDRESS\\[63_32\\]
bits."]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - 31:0\\]
Address value.The address comparators can support implementations that use multiple address widths. When the trace unit compares the ADDRESS field with an address that has a width less than this field, then it must ignore those upper bits in the comparison. For example, in a system that supports both 32-bit and 64-bit addresses, when the processor is in 32-bit state the comparator must ignore the ADDRESS\\[63_32\\]
bits."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Address value.The address comparators can support implementations that use multiple address widths. When the trace unit compares the ADDRESS field with an address that has a width less than this field, then it must ignore those upper bits in the comparison. For example, in a system that supports both 32-bit and 64-bit addresses, when the processor is in 32-bit state the comparator must ignore the ADDRESS\\[63_32\\]
bits."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Address value.The address comparators can support implementations that use multiple address widths. When the trace unit compares the ADDRESS field with an address that has a width less than this field, then it must ignore those upper bits in the comparison. For example, in a system that supports both 32-bit and 64-bit addresses, when the processor is in 32-bit state the comparator must ignore the ADDRESS\\[63_32\\]
bits."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<ApbaddrEtmCpu0Trcacvr5_63_32Spec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Address Comparator Value Registers 5 (high word)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trcacvr5_63_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trcacvr5_63_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trcacvr5_63_32Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trcacvr5_63_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trcacvr5_63_32::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trcacvr5_63_32Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trcacvr5_63_32::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trcacvr5_63_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCACVR5_63_32 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trcacvr5_63_32Spec {
    const RESET_VALUE: u32 = 0;
}
