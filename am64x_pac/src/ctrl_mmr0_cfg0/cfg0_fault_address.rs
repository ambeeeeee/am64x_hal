#[doc = "Register `CFG0_fault_address` reader"]
pub type R = crate::R<Cfg0FaultAddressSpec>;
#[doc = "Register `CFG0_fault_address` writer"]
pub type W = crate::W<Cfg0FaultAddressSpec>;
#[doc = "Field `FAULT_ADDR` reader - 31:0\\]
Fault Address."]
pub type FaultAddrR = crate::FieldReader<u32>;
#[doc = "Field `FAULT_ADDR` writer - 31:0\\]
Fault Address."]
pub type FaultAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    pub fn fault_addr(&self) -> FaultAddrR {
        FaultAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    #[must_use]
    pub fn fault_addr(&mut self) -> FaultAddrW<Cfg0FaultAddressSpec> {
        FaultAddrW::new(self, 0)
    }
}
#[doc = "CFG0_fault_address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultAddressSpec;
impl crate::RegisterSpec for Cfg0FaultAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_address::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_address::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_address to value 0"]
impl crate::Resettable for Cfg0FaultAddressSpec {
    const RESET_VALUE: u32 = 0;
}
