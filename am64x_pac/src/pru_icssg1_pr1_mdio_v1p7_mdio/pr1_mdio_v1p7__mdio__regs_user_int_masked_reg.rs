#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASKED_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASKED_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec>;
#[doc = "Field `USERINTMASKED` reader - 1:0\\]
User interrupt masked"]
pub type UserintmaskedR = crate::FieldReader;
#[doc = "Field `USERINTMASKED` writer - 1:0\\]
User interrupt masked"]
pub type UserintmaskedW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    pub fn userintmasked(&self) -> UserintmaskedR {
        UserintmaskedR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt masked"]
    #[inline(always)]
    #[must_use]
    pub fn userintmasked(&mut self) -> UserintmaskedW<Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec> {
        UserintmaskedW::new(self, 0)
    }
}
#[doc = "user_int_masked_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_int_masked_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_INT_MASKED_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserIntMaskedRegSpec {
    const RESET_VALUE: u32 = 0;
}
