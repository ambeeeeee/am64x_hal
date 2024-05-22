#[doc = "Register `MEM_ETHR` reader"]
pub type R = crate::R<MemEthrSpec>;
#[doc = "Register `MEM_ETHR` writer"]
pub type W = crate::W<MemEthrSpec>;
#[doc = "Field `ETHR` reader - 8:0\\]
Extended Transmit Holding Register - allows writing the full 9bit RHR"]
pub type EthrR = crate::FieldReader<u16>;
#[doc = "Field `ETHR` writer - 8:0\\]
Extended Transmit Holding Register - allows writing the full 9bit RHR"]
pub type EthrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Extended Transmit Holding Register - allows writing the full 9bit RHR"]
    #[inline(always)]
    pub fn ethr(&self) -> EthrR {
        EthrR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Extended Transmit Holding Register - allows writing the full 9bit RHR"]
    #[inline(always)]
    #[must_use]
    pub fn ethr(&mut self) -> EthrW<MemEthrSpec> {
        EthrW::new(self, 0)
    }
}
#[doc = "Extended Transmit Holding Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ethr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ethr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemEthrSpec;
impl crate::RegisterSpec for MemEthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ethr::R`](R) reader structure"]
impl crate::Readable for MemEthrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_ethr::W`](W) writer structure"]
impl crate::Writable for MemEthrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ETHR to value 0"]
impl crate::Resettable for MemEthrSpec {
    const RESET_VALUE: u32 = 0;
}
