#[doc = "Register `CPSW_NUSS_VBUSP_MANUAL_IF_REG` reader"]
pub type R = crate::R<CpswNussVbuspManualIfRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_MANUAL_IF_REG` writer"]
pub type W = crate::W<CpswNussVbuspManualIfRegSpec>;
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
    pub fn mdio_pin(&mut self) -> MdioPinW<CpswNussVbuspManualIfRegSpec> {
        MdioPinW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
MDIO Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_oe(&mut self) -> MdioOeW<CpswNussVbuspManualIfRegSpec> {
        MdioOeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
MDIO Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn mdio_mdclk_o(&mut self) -> MdioMdclkOW<CpswNussVbuspManualIfRegSpec> {
        MdioMdclkOW::new(self, 2)
    }
}
#[doc = "MDIO Manual Interface Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_manual_if_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_manual_if_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspManualIfRegSpec;
impl crate::RegisterSpec for CpswNussVbuspManualIfRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_manual_if_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspManualIfRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_manual_if_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspManualIfRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_MANUAL_IF_REG to value 0"]
impl crate::Resettable for CpswNussVbuspManualIfRegSpec {
    const RESET_VALUE: u32 = 0;
}
