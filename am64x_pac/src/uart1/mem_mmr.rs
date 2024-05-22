#[doc = "Register `MEM_MMR` reader"]
pub type R = crate::R<MemMmrSpec>;
#[doc = "Register `MEM_MMR` writer"]
pub type W = crate::W<MemMmrSpec>;
#[doc = "Field `MASK` reader - 7:0\\]
Address match masking value ? writing a 0 to a bit means that the corresponding address bit will be ignored in matching"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - 7:0\\]
Address match masking value ? writing a 0 to a bit means that the corresponding address bit will be ignored in matching"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Address match masking value ? writing a 0 to a bit means that the corresponding address bit will be ignored in matching"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Address match masking value ? writing a 0 to a bit means that the corresponding address bit will be ignored in matching"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MaskW<MemMmrSpec> {
        MaskW::new(self, 0)
    }
}
#[doc = "Multidrop Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_mmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_mmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemMmrSpec;
impl crate::RegisterSpec for MemMmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_mmr::R`](R) reader structure"]
impl crate::Readable for MemMmrSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_mmr::W`](W) writer structure"]
impl crate::Writable for MemMmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_MMR to value 0"]
impl crate::Resettable for MemMmrSpec {
    const RESET_VALUE: u32 = 0;
}
