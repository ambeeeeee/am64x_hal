#[doc = "Register `CFG0_fault_address_PROXY` reader"]
pub type R = crate::R<Cfg0FaultAddressProxySpec>;
#[doc = "Register `CFG0_fault_address_PROXY` writer"]
pub type W = crate::W<Cfg0FaultAddressProxySpec>;
#[doc = "Field `FAULT_ADDR_PROXY` reader - 31:0\\]
Fault Address."]
pub type FaultAddrProxyR = crate::FieldReader<u32>;
#[doc = "Field `FAULT_ADDR_PROXY` writer - 31:0\\]
Fault Address."]
pub type FaultAddrProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    pub fn fault_addr_proxy(&self) -> FaultAddrProxyR {
        FaultAddrProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Fault Address."]
    #[inline(always)]
    #[must_use]
    pub fn fault_addr_proxy(&mut self) -> FaultAddrProxyW<Cfg0FaultAddressProxySpec> {
        FaultAddrProxyW::new(self, 0)
    }
}
#[doc = "CFG0_fault_address_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_address_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_address_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultAddressProxySpec;
impl crate::RegisterSpec for Cfg0FaultAddressProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_address_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultAddressProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_address_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultAddressProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_address_PROXY to value 0"]
impl crate::Resettable for Cfg0FaultAddressProxySpec {
    const RESET_VALUE: u32 = 0;
}
