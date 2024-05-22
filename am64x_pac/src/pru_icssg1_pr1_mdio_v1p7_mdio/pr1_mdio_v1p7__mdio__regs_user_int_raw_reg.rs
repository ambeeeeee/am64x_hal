#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_INT_RAW_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_INT_RAW_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec>;
#[doc = "Field `USERINTRAW` reader - 1:0\\]
User interrupt raw"]
pub type UserintrawR = crate::FieldReader;
#[doc = "Field `USERINTRAW` writer - 1:0\\]
User interrupt raw"]
pub type UserintrawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt raw"]
    #[inline(always)]
    pub fn userintraw(&self) -> UserintrawR {
        UserintrawR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
User interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn userintraw(&mut self) -> UserintrawW<Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec> {
        UserintrawW::new(self, 0)
    }
}
#[doc = "user_int_raw_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_int_raw_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_INT_RAW_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserIntRawRegSpec {
    const RESET_VALUE: u32 = 0;
}
