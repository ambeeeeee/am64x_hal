#[doc = "Register `CFG0_SOCB_SEL` reader"]
pub type R = crate::R<Cfg0SocbSelSpec>;
#[doc = "Register `CFG0_SOCB_SEL` writer"]
pub type W = crate::W<Cfg0SocbSelSpec>;
#[doc = "Field `SOCB_SEL_SOCB_SEL` reader - 1:0\\]
Selects the SOC B output source"]
pub type SocbSelSocbSelR = crate::FieldReader;
#[doc = "Field `SOCB_SEL_SOCB_SEL` writer - 1:0\\]
Selects the SOC B output source"]
pub type SocbSelSocbSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC B output source"]
    #[inline(always)]
    pub fn socb_sel_socb_sel(&self) -> SocbSelSocbSelR {
        SocbSelSocbSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Selects the SOC B output source"]
    #[inline(always)]
    #[must_use]
    pub fn socb_sel_socb_sel(&mut self) -> SocbSelSocbSelW<Cfg0SocbSelSpec> {
        SocbSelSocbSelW::new(self, 0)
    }
}
#[doc = "CFG0_SOCB_SEL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_socb_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_socb_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0SocbSelSpec;
impl crate::RegisterSpec for Cfg0SocbSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_socb_sel::R`](R) reader structure"]
impl crate::Readable for Cfg0SocbSelSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_socb_sel::W`](W) writer structure"]
impl crate::Writable for Cfg0SocbSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_SOCB_SEL to value 0"]
impl crate::Resettable for Cfg0SocbSelSpec {
    const RESET_VALUE: u32 = 0;
}
