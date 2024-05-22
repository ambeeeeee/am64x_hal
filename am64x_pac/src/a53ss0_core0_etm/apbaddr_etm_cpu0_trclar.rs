#[doc = "Register `APBADDR_ETM_CPU0_TRCLAR` reader"]
pub type R = crate::R<ApbaddrEtmCpu0TrclarSpec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCLAR` writer"]
pub type W = crate::W<ApbaddrEtmCpu0TrclarSpec>;
#[doc = "Field `KEY` reader - 31:0\\]
Writing the key value 0xC5ACCE55 to this field clears the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register sets the lock, disabling write accesses to this component's registers through a memory mapped interface.Software can use the Software Lock to prevent accidental modification of the trace unit registers by software being debugged. For example, software that accidentally initializes an incorrect region of memory might disable the trace unit and make it impossible to trace the software."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - 31:0\\]
Writing the key value 0xC5ACCE55 to this field clears the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register sets the lock, disabling write accesses to this component's registers through a memory mapped interface.Software can use the Software Lock to prevent accidental modification of the trace unit registers by software being debugged. For example, software that accidentally initializes an incorrect region of memory might disable the trace unit and make it impossible to trace the software."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Writing the key value 0xC5ACCE55 to this field clears the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register sets the lock, disabling write accesses to this component's registers through a memory mapped interface.Software can use the Software Lock to prevent accidental modification of the trace unit registers by software being debugged. For example, software that accidentally initializes an incorrect region of memory might disable the trace unit and make it impossible to trace the software."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Writing the key value 0xC5ACCE55 to this field clears the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register sets the lock, disabling write accesses to this component's registers through a memory mapped interface.Software can use the Software Lock to prevent accidental modification of the trace unit registers by software being debugged. For example, software that accidentally initializes an incorrect region of memory might disable the trace unit and make it impossible to trace the software."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<ApbaddrEtmCpu0TrclarSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Software Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trclar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trclar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0TrclarSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu0TrclarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trclar::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0TrclarSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trclar::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0TrclarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCLAR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0TrclarSpec {
    const RESET_VALUE: u32 = 0;
}
