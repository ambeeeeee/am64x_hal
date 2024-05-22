#[doc = "Register `CPTS_VBUSP_TS_ADD_VAL_REG` reader"]
pub type R = crate::R<CptsVbuspTsAddValRegSpec>;
#[doc = "Register `CPTS_VBUSP_TS_ADD_VAL_REG` writer"]
pub type W = crate::W<CptsVbuspTsAddValRegSpec>;
#[doc = "Field `ADD_VAL` reader - 2:0\\]
Add Value"]
pub type AddValR = crate::FieldReader;
#[doc = "Field `ADD_VAL` writer - 2:0\\]
Add Value"]
pub type AddValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Add Value"]
    #[inline(always)]
    pub fn add_val(&self) -> AddValR {
        AddValR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Add Value"]
    #[inline(always)]
    #[must_use]
    pub fn add_val(&mut self) -> AddValW<CptsVbuspTsAddValRegSpec> {
        AddValW::new(self, 0)
    }
}
#[doc = "TS Add Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_ts_add_val_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_ts_add_val_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspTsAddValRegSpec;
impl crate::RegisterSpec for CptsVbuspTsAddValRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_ts_add_val_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspTsAddValRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_ts_add_val_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspTsAddValRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_TS_ADD_VAL_REG to value 0"]
impl crate::Resettable for CptsVbuspTsAddValRegSpec {
    const RESET_VALUE: u32 = 0;
}
