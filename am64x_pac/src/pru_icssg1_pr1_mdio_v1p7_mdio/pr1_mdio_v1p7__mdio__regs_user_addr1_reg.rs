#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR1_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR1_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec>;
#[doc = "Field `USER_ADDR1` reader - 15:0\\]
MDIO USER Address 1"]
pub type UserAddr1R = crate::FieldReader<u16>;
#[doc = "Field `USER_ADDR1` writer - 15:0\\]
MDIO USER Address 1"]
pub type UserAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 1"]
    #[inline(always)]
    pub fn user_addr1(&self) -> UserAddr1R {
        UserAddr1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
MDIO USER Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn user_addr1(&mut self) -> UserAddr1W<Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec> {
        UserAddr1W::new(self, 0)
    }
}
#[doc = "MDIO USER Address 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_addr1_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_ADDR1_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserAddr1RegSpec {
    const RESET_VALUE: u32 = 0;
}
