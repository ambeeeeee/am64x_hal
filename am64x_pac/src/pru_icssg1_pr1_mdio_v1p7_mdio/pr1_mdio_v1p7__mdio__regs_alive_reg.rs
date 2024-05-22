#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_ALIVE_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsAliveRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_ALIVE_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsAliveRegSpec>;
#[doc = "Field `ALIVE` reader - 31:0\\]
MDIO alive"]
pub type AliveR = crate::FieldReader<u32>;
#[doc = "Field `ALIVE` writer - 31:0\\]
MDIO alive"]
pub type AliveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    pub fn alive(&self) -> AliveR {
        AliveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MDIO alive"]
    #[inline(always)]
    #[must_use]
    pub fn alive(&mut self) -> AliveW<Pr1MdioV1p7_Mdio_RegsAliveRegSpec> {
        AliveW::new(self, 0)
    }
}
#[doc = "alive_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_alive_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_alive_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsAliveRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsAliveRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_alive_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsAliveRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_alive_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsAliveRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_ALIVE_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsAliveRegSpec {
    const RESET_VALUE: u32 = 0;
}
