#[doc = "Register `ADCREGS_DMAENABLE_CLR` reader"]
pub type R = crate::R<AdcregsDmaenableClrSpec>;
#[doc = "Register `ADCREGS_DMAENABLE_CLR` writer"]
pub type W = crate::W<AdcregsDmaenableClrSpec>;
#[doc = "Field `ENABLE0` reader - 0:0\\]
clears the enable of the DMA reguest FIFO0. Disables DMA request when writing 1"]
pub type Enable0R = crate::BitReader;
#[doc = "Field `ENABLE0` writer - 0:0\\]
clears the enable of the DMA reguest FIFO0. Disables DMA request when writing 1"]
pub type Enable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE1` reader - 1:1\\]
clears the enable of the DMA reguest FIFO1. Disables DMA request when writing 1"]
pub type Enable1R = crate::BitReader;
#[doc = "Field `ENABLE1` writer - 1:1\\]
clears the enable of the DMA reguest FIFO1. Disables DMA request when writing 1"]
pub type Enable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
clears the enable of the DMA reguest FIFO0. Disables DMA request when writing 1"]
    #[inline(always)]
    pub fn enable0(&self) -> Enable0R {
        Enable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
clears the enable of the DMA reguest FIFO1. Disables DMA request when writing 1"]
    #[inline(always)]
    pub fn enable1(&self) -> Enable1R {
        Enable1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
clears the enable of the DMA reguest FIFO0. Disables DMA request when writing 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable0(&mut self) -> Enable0W<AdcregsDmaenableClrSpec> {
        Enable0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
clears the enable of the DMA reguest FIFO1. Disables DMA request when writing 1"]
    #[inline(always)]
    #[must_use]
    pub fn enable1(&mut self) -> Enable1W<AdcregsDmaenableClrSpec> {
        Enable1W::new(self, 1)
    }
}
#[doc = "The DMAENABLE_CLR register allows the disabling of DMA requests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_dmaenable_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_dmaenable_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsDmaenableClrSpec;
impl crate::RegisterSpec for AdcregsDmaenableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_dmaenable_clr::R`](R) reader structure"]
impl crate::Readable for AdcregsDmaenableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_dmaenable_clr::W`](W) writer structure"]
impl crate::Writable for AdcregsDmaenableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_DMAENABLE_CLR to value 0"]
impl crate::Resettable for AdcregsDmaenableClrSpec {
    const RESET_VALUE: u32 = 0;
}
