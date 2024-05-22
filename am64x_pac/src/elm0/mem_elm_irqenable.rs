#[doc = "Register `MEM_ELM_IRQENABLE` reader"]
pub type R = crate::R<MemElmIrqenableSpec>;
#[doc = "Register `MEM_ELM_IRQENABLE` writer"]
pub type W = crate::W<MemElmIrqenableSpec>;
#[doc = "Field `LOCATION_MASK_0` reader - 0:0\\]
Error location interrupt mask bit for syndrome polynomial 0 0: disable interrupt 1: enable interrupt"]
pub type LocationMask0R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_0` writer - 0:0\\]
Error location interrupt mask bit for syndrome polynomial 0 0: disable interrupt 1: enable interrupt"]
pub type LocationMask0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_1` reader - 1:1\\]
Error location interrupt mask bit for syndrome polynomial 1"]
pub type LocationMask1R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_1` writer - 1:1\\]
Error location interrupt mask bit for syndrome polynomial 1"]
pub type LocationMask1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_2` reader - 2:2\\]
Error location interrupt mask bit for syndrome polynomial 2"]
pub type LocationMask2R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_2` writer - 2:2\\]
Error location interrupt mask bit for syndrome polynomial 2"]
pub type LocationMask2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_3` reader - 3:3\\]
Error location interrupt mask bit for syndrome polynomial 3"]
pub type LocationMask3R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_3` writer - 3:3\\]
Error location interrupt mask bit for syndrome polynomial 3"]
pub type LocationMask3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_4` reader - 4:4\\]
Error location interrupt mask bit for syndrome polynomial 4"]
pub type LocationMask4R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_4` writer - 4:4\\]
Error location interrupt mask bit for syndrome polynomial 4"]
pub type LocationMask4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_5` reader - 5:5\\]
Error location interrupt mask bit for syndrome polynomial 5"]
pub type LocationMask5R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_5` writer - 5:5\\]
Error location interrupt mask bit for syndrome polynomial 5"]
pub type LocationMask5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_6` reader - 6:6\\]
Error location interrupt mask bit for syndrome polynomial 6"]
pub type LocationMask6R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_6` writer - 6:6\\]
Error location interrupt mask bit for syndrome polynomial 6"]
pub type LocationMask6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCATION_MASK_7` reader - 7:7\\]
Error location interrupt mask bit for syndrome polynomial 7"]
pub type LocationMask7R = crate::BitReader;
#[doc = "Field `LOCATION_MASK_7` writer - 7:7\\]
Error location interrupt mask bit for syndrome polynomial 7"]
pub type LocationMask7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_MASK` reader - 8:8\\]
Page interrupt mask bit 0: disable interrupt 1: enable interrupt"]
pub type PageMaskR = crate::BitReader;
#[doc = "Field `PAGE_MASK` writer - 8:8\\]
Page interrupt mask bit 0: disable interrupt 1: enable interrupt"]
pub type PageMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Error location interrupt mask bit for syndrome polynomial 0 0: disable interrupt 1: enable interrupt"]
    #[inline(always)]
    pub fn location_mask_0(&self) -> LocationMask0R {
        LocationMask0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error location interrupt mask bit for syndrome polynomial 1"]
    #[inline(always)]
    pub fn location_mask_1(&self) -> LocationMask1R {
        LocationMask1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Error location interrupt mask bit for syndrome polynomial 2"]
    #[inline(always)]
    pub fn location_mask_2(&self) -> LocationMask2R {
        LocationMask2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Error location interrupt mask bit for syndrome polynomial 3"]
    #[inline(always)]
    pub fn location_mask_3(&self) -> LocationMask3R {
        LocationMask3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Error location interrupt mask bit for syndrome polynomial 4"]
    #[inline(always)]
    pub fn location_mask_4(&self) -> LocationMask4R {
        LocationMask4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Error location interrupt mask bit for syndrome polynomial 5"]
    #[inline(always)]
    pub fn location_mask_5(&self) -> LocationMask5R {
        LocationMask5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Error location interrupt mask bit for syndrome polynomial 6"]
    #[inline(always)]
    pub fn location_mask_6(&self) -> LocationMask6R {
        LocationMask6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Error location interrupt mask bit for syndrome polynomial 7"]
    #[inline(always)]
    pub fn location_mask_7(&self) -> LocationMask7R {
        LocationMask7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Page interrupt mask bit 0: disable interrupt 1: enable interrupt"]
    #[inline(always)]
    pub fn page_mask(&self) -> PageMaskR {
        PageMaskR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Error location interrupt mask bit for syndrome polynomial 0 0: disable interrupt 1: enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_0(&mut self) -> LocationMask0W<MemElmIrqenableSpec> {
        LocationMask0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error location interrupt mask bit for syndrome polynomial 1"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_1(&mut self) -> LocationMask1W<MemElmIrqenableSpec> {
        LocationMask1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Error location interrupt mask bit for syndrome polynomial 2"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_2(&mut self) -> LocationMask2W<MemElmIrqenableSpec> {
        LocationMask2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Error location interrupt mask bit for syndrome polynomial 3"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_3(&mut self) -> LocationMask3W<MemElmIrqenableSpec> {
        LocationMask3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Error location interrupt mask bit for syndrome polynomial 4"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_4(&mut self) -> LocationMask4W<MemElmIrqenableSpec> {
        LocationMask4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Error location interrupt mask bit for syndrome polynomial 5"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_5(&mut self) -> LocationMask5W<MemElmIrqenableSpec> {
        LocationMask5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Error location interrupt mask bit for syndrome polynomial 6"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_6(&mut self) -> LocationMask6W<MemElmIrqenableSpec> {
        LocationMask6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Error location interrupt mask bit for syndrome polynomial 7"]
    #[inline(always)]
    #[must_use]
    pub fn location_mask_7(&mut self) -> LocationMask7W<MemElmIrqenableSpec> {
        LocationMask7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Page interrupt mask bit 0: disable interrupt 1: enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn page_mask(&mut self) -> PageMaskW<MemElmIrqenableSpec> {
        PageMaskW::new(self, 8)
    }
}
#[doc = "Interrupt enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_irqenable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_irqenable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmIrqenableSpec;
impl crate::RegisterSpec for MemElmIrqenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_irqenable::R`](R) reader structure"]
impl crate::Readable for MemElmIrqenableSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_irqenable::W`](W) writer structure"]
impl crate::Writable for MemElmIrqenableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_IRQENABLE to value 0"]
impl crate::Resettable for MemElmIrqenableSpec {
    const RESET_VALUE: u32 = 0;
}
