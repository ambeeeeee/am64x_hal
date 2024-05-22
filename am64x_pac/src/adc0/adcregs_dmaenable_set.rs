#[doc = "Register `ADCREGS_DMAENABLE_SET` reader"]
pub type R = crate::R<AdcregsDmaenableSetSpec>;
#[doc = "Register `ADCREGS_DMAENABLE_SET` writer"]
pub type W = crate::W<AdcregsDmaenableSetSpec>;
#[doc = "Field `ENABLE0` reader - 0:0\\]
enable DMA reguest FIFO0"]
pub type Enable0R = crate::BitReader;
#[doc = "Field `ENABLE0` writer - 0:0\\]
enable DMA reguest FIFO0"]
pub type Enable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE1` reader - 1:1\\]
enable DMA reguest FIFO1"]
pub type Enable1R = crate::BitReader;
#[doc = "Field `ENABLE1` writer - 1:1\\]
enable DMA reguest FIFO1"]
pub type Enable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
enable DMA reguest FIFO0"]
    #[inline(always)]
    pub fn enable0(&self) -> Enable0R {
        Enable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
enable DMA reguest FIFO1"]
    #[inline(always)]
    pub fn enable1(&self) -> Enable1R {
        Enable1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
enable DMA reguest FIFO0"]
    #[inline(always)]
    #[must_use]
    pub fn enable0(&mut self) -> Enable0W<AdcregsDmaenableSetSpec> {
        Enable0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
enable DMA reguest FIFO1"]
    #[inline(always)]
    #[must_use]
    pub fn enable1(&mut self) -> Enable1W<AdcregsDmaenableSetSpec> {
        Enable1W::new(self, 1)
    }
}
#[doc = "The DMAENABLE_SET register allows the enabling of DMA requests\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcregs_dmaenable_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcregs_dmaenable_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcregsDmaenableSetSpec;
impl crate::RegisterSpec for AdcregsDmaenableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcregs_dmaenable_set::R`](R) reader structure"]
impl crate::Readable for AdcregsDmaenableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`adcregs_dmaenable_set::W`](W) writer structure"]
impl crate::Writable for AdcregsDmaenableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCREGS_DMAENABLE_SET to value 0"]
impl crate::Resettable for AdcregsDmaenableSetSpec {
    const RESET_VALUE: u32 = 0;
}
