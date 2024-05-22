#[doc = "Register `APBADDR_CTI_CPU1_CTILAR` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtilarSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTILAR` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtilarSpec>;
#[doc = "Field `KEY` reader - 31:0\\]
Lock Access control. Writing the key value 0xC5ACCE55 to this field unlocks the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register locks the lock, disabling write accesses to this component's registers through a memory mapped interface."]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - 31:0\\]
Lock Access control. Writing the key value 0xC5ACCE55 to this field unlocks the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register locks the lock, disabling write accesses to this component's registers through a memory mapped interface."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Lock Access control. Writing the key value 0xC5ACCE55 to this field unlocks the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register locks the lock, disabling write accesses to this component's registers through a memory mapped interface."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Lock Access control. Writing the key value 0xC5ACCE55 to this field unlocks the lock, enabling write accesses to this component's registers through a memory-mapped interface.Writing any other value to this register locks the lock, disabling write accesses to this component's registers through a memory mapped interface."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<ApbaddrCtiCpu1CtilarSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "CTI Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctilar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctilar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtilarSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtilarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctilar::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtilarSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctilar::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtilarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTILAR to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtilarSpec {
    const RESET_VALUE: u32 = 0;
}
