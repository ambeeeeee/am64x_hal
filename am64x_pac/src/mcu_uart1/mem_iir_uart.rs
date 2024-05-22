#[doc = "Register `MEM_IIR_UART` reader"]
pub type R = crate::R<MemIirUartSpec>;
#[doc = "Register `MEM_IIR_UART` writer"]
pub type W = crate::W<MemIirUartSpec>;
#[doc = "Field `IT_PENDING` reader - "]
pub type ItPendingR = crate::BitReader;
#[doc = "Field `IT_PENDING` writer - "]
pub type ItPendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IT_TYPE` reader - "]
pub type ItTypeR = crate::FieldReader;
#[doc = "Field `IT_TYPE` writer - "]
pub type ItTypeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FCR_MIRROR` reader - 7:6\\]
Mirror the contents of FCR\\[0\\]
on both bits."]
pub type FcrMirrorR = crate::FieldReader;
#[doc = "Field `FCR_MIRROR` writer - 7:6\\]
Mirror the contents of FCR\\[0\\]
on both bits."]
pub type FcrMirrorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn it_pending(&self) -> ItPendingR {
        ItPendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn it_type(&self) -> ItTypeR {
        ItTypeR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Mirror the contents of FCR\\[0\\]
on both bits."]
    #[inline(always)]
    pub fn fcr_mirror(&self) -> FcrMirrorR {
        FcrMirrorR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn it_pending(&mut self) -> ItPendingW<MemIirUartSpec> {
        ItPendingW::new(self, 0)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    #[must_use]
    pub fn it_type(&mut self) -> ItTypeW<MemIirUartSpec> {
        ItTypeW::new(self, 1)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Mirror the contents of FCR\\[0\\]
on both bits."]
    #[inline(always)]
    #[must_use]
    pub fn fcr_mirror(&mut self) -> FcrMirrorW<MemIirUartSpec> {
        FcrMirrorW::new(self, 6)
    }
}
#[doc = "The IIR is a read-only register, which provides the source of the interrupt in a prioritized manner.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_iir_uart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_iir_uart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemIirUartSpec;
impl crate::RegisterSpec for MemIirUartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_iir_uart::R`](R) reader structure"]
impl crate::Readable for MemIirUartSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_iir_uart::W`](W) writer structure"]
impl crate::Writable for MemIirUartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_IIR_UART to value 0x01"]
impl crate::Resettable for MemIirUartSpec {
    const RESET_VALUE: u32 = 0x01;
}
