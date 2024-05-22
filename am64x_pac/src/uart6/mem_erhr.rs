#[doc = "Register `MEM_ERHR` reader"]
pub type R = crate::R<MemErhrSpec>;
#[doc = "Register `MEM_ERHR` writer"]
pub type W = crate::W<MemErhrSpec>;
#[doc = "Field `ERHR` reader - 8:0\\]
Extended Receive Holding Register - allows accessing the full 9bit RHR"]
pub type ErhrR = crate::FieldReader<u16>;
#[doc = "Field `ERHR` writer - 8:0\\]
Extended Receive Holding Register - allows accessing the full 9bit RHR"]
pub type ErhrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Extended Receive Holding Register - allows accessing the full 9bit RHR"]
    #[inline(always)]
    pub fn erhr(&self) -> ErhrR {
        ErhrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Extended Receive Holding Register - allows accessing the full 9bit RHR"]
    #[inline(always)]
    #[must_use]
    pub fn erhr(&mut self) -> ErhrW<MemErhrSpec> {
        ErhrW::new(self, 0)
    }
}
#[doc = "Extended Receive Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_erhr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_erhr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemErhrSpec;
impl crate::RegisterSpec for MemErhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_erhr::R`](R) reader structure"]
impl crate::Readable for MemErhrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_erhr::W`](W) writer structure"]
impl crate::Writable for MemErhrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ERHR to value 0"]
impl crate::Resettable for MemErhrSpec {
    const RESET_VALUE: u32 = 0;
}
