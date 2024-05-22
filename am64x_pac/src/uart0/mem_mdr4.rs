#[doc = "Register `MEM_MDR4` reader"]
pub type R = crate::R<MemMdr4Spec>;
#[doc = "Register `MEM_MDR4` writer"]
pub type W = crate::W<MemMdr4Spec>;
#[doc = "Field `MODE` reader - 2:0\\]
New modes \\[when set, overrides MDR1 modes\\]"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - 2:0\\]
New modes \\[when set, overrides MDR1 modes\\]"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FREQ_SEL_H` reader - 5:3\\]
Upper 3 bits of FREQ_SEL register for higher division values, as required for example for FI/Di in ISO7816 mode"]
pub type FreqSelHR = crate::FieldReader;
#[doc = "Field `FREQ_SEL_H` writer - 5:3\\]
Upper 3 bits of FREQ_SEL register for higher division values, as required for example for FI/Di in ISO7816 mode"]
pub type FreqSelHW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE9` reader - 6:6\\]
9-bit character length. When '1', overrides character length setting in LCR"]
pub type Mode9R = crate::BitReader;
#[doc = "Field `MODE9` writer - 6:6\\]
9-bit character length. When '1', overrides character length setting in LCR"]
pub type Mode9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - "]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - "]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
New modes \\[when set, overrides MDR1 modes\\]"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Upper 3 bits of FREQ_SEL register for higher division values, as required for example for FI/Di in ISO7816 mode"]
    #[inline(always)]
    pub fn freq_sel_h(&self) -> FreqSelHR {
        FreqSelHR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
9-bit character length. When '1', overrides character length setting in LCR"]
    #[inline(always)]
    pub fn mode9(&self) -> Mode9R {
        Mode9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
New modes \\[when set, overrides MDR1 modes\\]"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<MemMdr4Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Upper 3 bits of FREQ_SEL register for higher division values, as required for example for FI/Di in ISO7816 mode"]
    #[inline(always)]
    #[must_use]
    pub fn freq_sel_h(&mut self) -> FreqSelHW<MemMdr4Spec> {
        FreqSelHW::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
9-bit character length. When '1', overrides character length setting in LCR"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> Mode9W<MemMdr4Spec> {
        Mode9W::new(self, 6)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<MemMdr4Spec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Mode definition register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mdr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mdr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMdr4Spec;
impl crate::RegisterSpec for MemMdr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mdr4::R`](R) reader structure"]
impl crate::Readable for MemMdr4Spec {}
#[doc = "`write(|w| ..)` method takes [`mem_mdr4::W`](W) writer structure"]
impl crate::Writable for MemMdr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MDR4 to value 0"]
impl crate::Resettable for MemMdr4Spec {
    const RESET_VALUE: u32 = 0;
}
