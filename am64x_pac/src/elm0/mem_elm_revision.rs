#[doc = "Register `MEM_ELM_REVISION` reader"]
pub type R = crate::R<MemElmRevisionSpec>;
#[doc = "Register `MEM_ELM_REVISION` writer"]
pub type W = crate::W<MemElmRevisionSpec>;
#[doc = "Field `REV_NUMBER` reader - 7:0\\]
IP revision number \\[RTL\\]
\\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
pub type RevNumberR = crate::FieldReader;
#[doc = "Field `REV_NUMBER` writer - 7:0\\]
IP revision number \\[RTL\\]
\\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
pub type RevNumberW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED_0` reader - 31:8\\]
Read returns 0"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_0` writer - 31:8\\]
Read returns 0"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision number \\[RTL\\]
\\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
    #[inline(always)]
    pub fn rev_number(&self) -> RevNumberR {
        RevNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Read returns 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IP revision number \\[RTL\\]
\\[7:4\\]
Major revision \\[3:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev_number(&mut self) -> RevNumberW<MemElmRevisionSpec> {
        RevNumberW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Read returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_0(&mut self) -> Reserved0W<MemElmRevisionSpec> {
        Reserved0W::new(self, 8)
    }
}
#[doc = "This register contains the IP revision code. (A write to this register has no effect, the same as the reset)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_revision::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_revision::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmRevisionSpec;
impl crate::RegisterSpec for MemElmRevisionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_revision::R`](R) reader structure"]
impl crate::Readable for MemElmRevisionSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_revision::W`](W) writer structure"]
impl crate::Writable for MemElmRevisionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_REVISION to value 0x32"]
impl crate::Resettable for MemElmRevisionSpec {
    const RESET_VALUE: u32 = 0x32;
}
