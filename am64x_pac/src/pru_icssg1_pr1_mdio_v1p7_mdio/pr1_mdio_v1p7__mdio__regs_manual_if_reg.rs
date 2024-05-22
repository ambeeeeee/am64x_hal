#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_MANUAL_IF_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsManualIfRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_MANUAL_IF_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsManualIfRegSpec>;
#[doc = "Field `MDIO_PIN` reader - 0:0\\]
MDIO Pin"]
pub type MdioPinR = crate::BitReader;
#[doc = "Field `MDIO_PIN` writer - 0:0\\]
MDIO Pin"]
pub type MdioPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_OE` reader - 1:1\\]
MDIO Output Enable"]
pub type MdioOeR = crate::BitReader;
#[doc = "Field `MDIO_OE` writer - 1:1\\]
MDIO Output Enable"]
pub type MdioOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIO_MDCLK_O` reader - 2:2\\]
MDIO Clock Output"]
pub type MdioMdclkOR = crate::BitReader;
#[doc = "Field `MDIO_MDCLK_O` writer - 2:2\\]
MDIO Clock Output"]
pub type MdioMdclkOW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
MDIO Pin"]
    #[inline(always)]
    pub fn mdio_pin(&self) -> MdioPinR {
        MdioPinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MDIO Output Enable"]
    #[inline(always)]
    pub fn mdio_oe(&self) -> MdioOeR {
        MdioOeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
MDIO Clock Output"]
    #[inline(always)]
    pub fn mdio_mdclk_o(&self) -> MdioMdclkOR {
        MdioMdclkOR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
MDIO Pin"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_pin(&mut self) -> MdioPinW<Pr1MdioV1p7_Mdio_RegsManualIfRegSpec> {
        MdioPinW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MDIO Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_oe(&mut self) -> MdioOeW<Pr1MdioV1p7_Mdio_RegsManualIfRegSpec> {
        MdioOeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
MDIO Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_mdclk_o(&mut self) -> MdioMdclkOW<Pr1MdioV1p7_Mdio_RegsManualIfRegSpec> {
        MdioMdclkOW::new(self, 2)
    }
}
#[doc = "manual_if_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsManualIfRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsManualIfRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsManualIfRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_manual_if_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsManualIfRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_MANUAL_IF_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsManualIfRegSpec {
    const RESET_VALUE: u32 = 0;
}
