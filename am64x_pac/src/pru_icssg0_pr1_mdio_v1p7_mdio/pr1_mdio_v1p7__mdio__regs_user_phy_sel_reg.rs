#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_PHY_SEL_REG` reader"]
pub type R = crate::R<Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec>;
#[doc = "Register `PR1_MDIO_V1P7__MDIO__REGS_USER_PHY_SEL_REG` writer"]
pub type W = crate::W<Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec>;
#[doc = "Field `PHYADR_MON` reader - 4:0\\]
PHY address whose link status is monitored"]
pub type PhyadrMonR = crate::FieldReader;
#[doc = "Field `PHYADR_MON` writer - 4:0\\]
PHY address whose link status is monitored"]
pub type PhyadrMonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LINKINT_ENABLE` reader - 6:6\\]
Link change interrupt enable"]
pub type LinkintEnableR = crate::BitReader;
#[doc = "Field `LINKINT_ENABLE` writer - 6:6\\]
Link change interrupt enable"]
pub type LinkintEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKSEL` reader - 7:7\\]
Link status determination select"]
pub type LinkselR = crate::BitReader;
#[doc = "Field `LINKSEL` writer - 7:7\\]
Link status determination select"]
pub type LinkselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
PHY address whose link status is monitored"]
    #[inline(always)]
    pub fn phyadr_mon(&self) -> PhyadrMonR {
        PhyadrMonR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Link change interrupt enable"]
    #[inline(always)]
    pub fn linkint_enable(&self) -> LinkintEnableR {
        LinkintEnableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Link status determination select"]
    #[inline(always)]
    pub fn linksel(&self) -> LinkselR {
        LinkselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
PHY address whose link status is monitored"]
    #[inline(always)]
    #[must_use]
    pub fn phyadr_mon(&mut self) -> PhyadrMonW<Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec> {
        PhyadrMonW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Link change interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn linkint_enable(&mut self) -> LinkintEnableW<Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec> {
        LinkintEnableW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Link status determination select"]
    #[inline(always)]
    #[must_use]
    pub fn linksel(&mut self) -> LinkselW<Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec> {
        LinkselW::new(self, 7)
    }
}
#[doc = "user_phy_sel_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec;
impl crate::RegisterSpec for Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::R`](R) reader structure"]
impl crate::Readable for Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mdio_v1p7__mdio__regs_user_phy_sel_reg::W`](W) writer structure"]
impl crate::Writable for Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MDIO_V1P7__MDIO__REGS_USER_PHY_SEL_REG to value 0"]
impl crate::Resettable for Pr1MdioV1p7_Mdio_RegsUserPhySelRegSpec {
    const RESET_VALUE: u32 = 0;
}
