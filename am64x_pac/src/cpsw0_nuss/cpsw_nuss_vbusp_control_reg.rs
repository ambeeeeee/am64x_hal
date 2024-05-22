#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG` reader"]
pub type R = crate::R<CpswNussVbuspControlRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_CONTROL_REG` writer"]
pub type W = crate::W<CpswNussVbuspControlRegSpec>;
#[doc = "Field `EEE_EN` reader - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
pub type EeeEnR = crate::BitReader;
#[doc = "Field `EEE_EN` writer - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
pub type EeeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEE_PHY_ONLY` reader - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
pub type EeePhyOnlyR = crate::BitReader;
#[doc = "Field `EEE_PHY_ONLY` writer - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
pub type EeePhyOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
    #[inline(always)]
    pub fn eee_en(&self) -> EeeEnR {
        EeeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
    #[inline(always)]
    pub fn eee_phy_only(&self) -> EeePhyOnlyR {
        EeePhyOnlyR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Energy Efficient Ethernet Enable: 0=EEE is disabled, 1=EEE is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn eee_en(&mut self) -> EeeEnW<CpswNussVbuspControlRegSpec> {
        EeeEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Energy Efficient Enable Phy Only Mode: 0=The low power indicate state includes gating off the CPPI_GCLK to the CPSW, 1=The low power indicate state does not gate the clock to the CPSW"]
    #[inline(always)]
    #[must_use]
    pub fn eee_phy_only(&mut self) -> EeePhyOnlyW<CpswNussVbuspControlRegSpec> {
        EeePhyOnlyW::new(self, 1)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspControlRegSpec;
impl crate::RegisterSpec for CpswNussVbuspControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_control_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_control_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_CONTROL_REG to value 0"]
impl crate::Resettable for CpswNussVbuspControlRegSpec {
    const RESET_VALUE: u32 = 0;
}
