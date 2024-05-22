#[doc = "Register `MEM_FREQ_SEL` reader"]
pub type R = crate::R<MemFreqSelSpec>;
#[doc = "Register `MEM_FREQ_SEL` writer"]
pub type W = crate::W<MemFreqSelSpec>;
#[doc = "Field `FREQ_SEL` reader - 7:0\\]
Sets the sample per bit if non default frequency is used. MDR3\\[1\\]
must be set to 1 after this value is set. Must be equal or higher then 6."]
pub type FreqSelR = crate::FieldReader;
#[doc = "Field `FREQ_SEL` writer - 7:0\\]
Sets the sample per bit if non default frequency is used. MDR3\\[1\\]
must be set to 1 after this value is set. Must be equal or higher then 6."]
pub type FreqSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Sets the sample per bit if non default frequency is used. MDR3\\[1\\]
must be set to 1 after this value is set. Must be equal or higher then 6."]
    #[inline(always)]
    pub fn freq_sel(&self) -> FreqSelR {
        FreqSelR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Sets the sample per bit if non default frequency is used. MDR3\\[1\\]
must be set to 1 after this value is set. Must be equal or higher then 6."]
    #[inline(always)]
    #[must_use]
    pub fn freq_sel(&mut self) -> FreqSelW<MemFreqSelSpec> {
        FreqSelW::new(self, 0)
    }
}
#[doc = "Sample per bit value selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_freq_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_freq_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemFreqSelSpec;
impl crate::RegisterSpec for MemFreqSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_freq_sel::R`](R) reader structure"]
impl crate::Readable for MemFreqSelSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_freq_sel::W`](W) writer structure"]
impl crate::Writable for MemFreqSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_FREQ_SEL to value 0x26"]
impl crate::Resettable for MemFreqSelSpec {
    const RESET_VALUE: u32 = 0x26;
}
