#[doc = "Register `PR1_IEP1__SLV__REGS_rxipg0_reg` reader"]
pub type R = crate::R<Pr1Iep1_Slv_RegsRxipg0RegSpec>;
#[doc = "Register `PR1_IEP1__SLV__REGS_rxipg0_reg` writer"]
pub type W = crate::W<Pr1Iep1_Slv_RegsRxipg0RegSpec>;
#[doc = "Field `RX_IPG0` reader - "]
pub type RxIpg0R = crate::FieldReader<u16>;
#[doc = "Field `RX_IPG0` writer - "]
pub type RxIpg0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RX_MIN_IPG0` reader - "]
pub type RxMinIpg0R = crate::FieldReader<u16>;
#[doc = "Field `RX_MIN_IPG0` writer - "]
pub type RxMinIpg0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_ipg0(&self) -> RxIpg0R {
        RxIpg0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rx_min_ipg0(&self) -> RxMinIpg0R {
        RxMinIpg0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rx_ipg0(&mut self) -> RxIpg0W<Pr1Iep1_Slv_RegsRxipg0RegSpec> {
        RxIpg0W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_min_ipg0(&mut self) -> RxMinIpg0W<Pr1Iep1_Slv_RegsRxipg0RegSpec> {
        RxMinIpg0W::new(self, 16)
    }
}
#[doc = "PR1_IEP1__SLV__REGS_rxipg0_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep1__slv__regs_rxipg0_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep1__slv__regs_rxipg0_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep1_Slv_RegsRxipg0RegSpec;
impl crate::RegisterSpec for Pr1Iep1_Slv_RegsRxipg0RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep1__slv__regs_rxipg0_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep1_Slv_RegsRxipg0RegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep1__slv__regs_rxipg0_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep1_Slv_RegsRxipg0RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP1__SLV__REGS_rxipg0_reg to value 0"]
impl crate::Resettable for Pr1Iep1_Slv_RegsRxipg0RegSpec {
    const RESET_VALUE: u32 = 0;
}
