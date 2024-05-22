#[doc = "Register `CFG0_fault_clear` reader"]
pub type R = crate::R<Cfg0FaultClearSpec>;
#[doc = "Register `CFG0_fault_clear` writer"]
pub type W = crate::W<Cfg0FaultClearSpec>;
#[doc = "Field `FAULT_CLR` reader - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrR = crate::BitReader;
#[doc = "Field `FAULT_CLR` writer - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
pub type FaultClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn fault_clr(&self) -> FaultClrR {
        FaultClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Fault clear. Writing a 1 clears the current fault. Writing a 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fault_clr(&mut self) -> FaultClrW<Cfg0FaultClearSpec> {
        FaultClrW::new(self, 0)
    }
}
#[doc = "CFG0_fault_clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_fault_clear::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_fault_clear::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0FaultClearSpec;
impl crate::RegisterSpec for Cfg0FaultClearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_fault_clear::R`](R) reader structure"]
impl crate::Readable for Cfg0FaultClearSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_fault_clear::W`](W) writer structure"]
impl crate::Writable for Cfg0FaultClearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_fault_clear to value 0"]
impl crate::Resettable for Cfg0FaultClearSpec {
    const RESET_VALUE: u32 = 0;
}
