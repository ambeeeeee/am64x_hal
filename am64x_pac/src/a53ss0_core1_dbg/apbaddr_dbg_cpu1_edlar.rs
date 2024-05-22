#[doc = "Register `APBADDR_DBG_CPU1_EDLAR` reader"]
pub type R = crate::R<ApbaddrDbgCpu1EdlarSpec>;
#[doc = "Register `APBADDR_DBG_CPU1_EDLAR` writer"]
pub type W = crate::W<ApbaddrDbgCpu1EdlarSpec>;
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
    pub fn key(&mut self) -> KeyW<ApbaddrDbgCpu1EdlarSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "External Debug Lock Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_dbg_cpu1_edlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_dbg_cpu1_edlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrDbgCpu1EdlarSpec;
impl crate::RegisterSpec for ApbaddrDbgCpu1EdlarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_dbg_cpu1_edlar::R`](R) reader structure"]
impl crate::Readable for ApbaddrDbgCpu1EdlarSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_dbg_cpu1_edlar::W`](W) writer structure"]
impl crate::Writable for ApbaddrDbgCpu1EdlarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_DBG_CPU1_EDLAR to value 0"]
impl crate::Resettable for ApbaddrDbgCpu1EdlarSpec {
    const RESET_VALUE: u32 = 0;
}
