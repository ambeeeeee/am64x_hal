#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR0_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR0_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec>;
#[doc = "Field `USER_ADDR0` reader - 15:0\\]
MDIO USER Address 0"]
pub type UserAddr0R = crate::FieldReader<u16>;
#[doc = "Field `USER_ADDR0` writer - 15:0\\]
MDIO USER Address 0"]
pub type UserAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 0"]
    #[inline(always)]
    pub fn user_addr0(&self) -> UserAddr0R {
        UserAddr0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn user_addr0(&mut self) -> UserAddr0W<Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec> {
        UserAddr0W::new(self, 0)
    }
}
#[doc = "MDIO USER Address 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_addr0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR0_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserAddr0RegSpec {
    const RESET_VALUE: u32 = 0;
}
