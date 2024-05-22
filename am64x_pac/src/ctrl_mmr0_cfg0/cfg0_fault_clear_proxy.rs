#[doc = "Register `CFG0_fault_clear_PROXY` reader"]
pub type R = crate::R<Cfg0FaultClearProxySpec>;
#[doc = "Register `CFG0_fault_clear_PROXY` writer"]
pub type W = crate::W<Cfg0FaultClearProxySpec>;
#[doc = "Field `FAULT_CLR_PROXY` reader - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrProxyR = crate::BitReader;
#[doc = "Field `FAULT_CLR_PROXY` writer - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fault_clr_proxy(&self) -> FaultClrProxyR {
        FaultClrProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fault_clr_proxy(&mut self) -> FaultClrProxyW<Cfg0FaultClearProxySpec> {
        FaultClrProxyW::new(self, 0)
    }
}
#[doc = "CFG0_fault_clear_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_clear_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_clear_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultClearProxySpec;
impl crate::RegisterSpec for Cfg0FaultClearProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_clear_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultClearProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_clear_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultClearProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_clear_PROXY to value 0"]
impl crate::Resettable for Cfg0FaultClearProxySpec {
    const RESET_VALUE: u32 = 0;
}
