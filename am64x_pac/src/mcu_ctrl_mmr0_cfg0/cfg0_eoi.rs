#[doc = "Register `CFG0_eoi` reader"]
pub type R = crate::R<Cfg0EoiSpec>;
#[doc = "Register `CFG0_eoi` writer"]
pub type W = crate::W<Cfg0EoiSpec>;
#[doc = "Field `EOI_VECTOR` reader - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
pub type EoiVectorR = crate::FieldReader;
#[doc = "Field `EOI_VECTOR` writer - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
pub type EoiVectorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
    #[inline(always)]
    pub fn eoi_vector(&self) -> EoiVectorR {
        EoiVectorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
EOI vector value. Write this with interrupt distribution value in the chip."]
    #[inline(always)]
    #[must_use]
    pub fn eoi_vector(&mut self) -> EoiVectorW<Cfg0EoiSpec> {
        EoiVectorW::new(self, 0)
    }
}
#[doc = "CFG0_eoi\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_eoi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_eoi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0EoiSpec;
impl crate::RegisterSpec for Cfg0EoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_eoi::R`](R) reader structure"]
impl crate::Readable for Cfg0EoiSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_eoi::W`](W) writer structure"]
impl crate::Writable for Cfg0EoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_eoi to value 0"]
impl crate::Resettable for Cfg0EoiSpec {
    const RESET_VALUE: u32 = 0;
}
