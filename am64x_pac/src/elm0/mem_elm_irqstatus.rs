#[doc = "Register `MEM_ELM_IRQSTATUS` reader"]
pub type R = crate::R<MemElmIrqstatusSpec>;
#[doc = "Register `MEM_ELM_IRQSTATUS` writer"]
pub type W = crate::W<MemElmIrqstatusSpec>;
#[doc = "Field `LOC_VALID_0` reader - 0:0\\]
Error location status for syndrome polynomial 0"]
pub type LocValid0R = crate::BitReader;
#[doc = "Field `LOC_VALID_0` writer - 0:0\\]
Error location status for syndrome polynomial 0"]
pub type LocValid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_1` reader - 1:1\\]
Error location status for syndrome polynomial 1"]
pub type LocValid1R = crate::BitReader;
#[doc = "Field `LOC_VALID_1` writer - 1:1\\]
Error location status for syndrome polynomial 1"]
pub type LocValid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_2` reader - 2:2\\]
Error location status for syndrome polynomial 2"]
pub type LocValid2R = crate::BitReader;
#[doc = "Field `LOC_VALID_2` writer - 2:2\\]
Error location status for syndrome polynomial 2"]
pub type LocValid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_3` reader - 3:3\\]
Error location status for syndrome polynomial 3"]
pub type LocValid3R = crate::BitReader;
#[doc = "Field `LOC_VALID_3` writer - 3:3\\]
Error location status for syndrome polynomial 3"]
pub type LocValid3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_4` reader - 4:4\\]
Error location status for syndrome polynomial 4"]
pub type LocValid4R = crate::BitReader;
#[doc = "Field `LOC_VALID_4` writer - 4:4\\]
Error location status for syndrome polynomial 4"]
pub type LocValid4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_5` reader - 5:5\\]
Error location status for syndrome polynomial 5"]
pub type LocValid5R = crate::BitReader;
#[doc = "Field `LOC_VALID_5` writer - 5:5\\]
Error location status for syndrome polynomial 5"]
pub type LocValid5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_6` reader - 6:6\\]
Error location status for syndrome polynomial 6"]
pub type LocValid6R = crate::BitReader;
#[doc = "Field `LOC_VALID_6` writer - 6:6\\]
Error location status for syndrome polynomial 6"]
pub type LocValid6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOC_VALID_7` reader - 7:7\\]
Error location status for syndrome polynomial 7 Read 0x0: no syndrome processed or process in progress Read 0x1: error location process completed Write 0x0: no effect Write 0x1: clear interrupt"]
pub type LocValid7R = crate::BitReader;
#[doc = "Field `LOC_VALID_7` writer - 7:7\\]
Error location status for syndrome polynomial 7 Read 0x0: no syndrome processed or process in progress Read 0x1: error location process completed Write 0x0: no effect Write 0x1: clear interrupt"]
pub type LocValid7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE_VALID` reader - 8:8\\]
Error location status for a full page, based on the mask definition Read 0x0: error locations invalid for all polynomials enabled in the ECC_INTERRUPT_MASK register Read 0x1: all error locations valid Write 0x0: no effect Write 0x1: clear interrupt"]
pub type PageValidR = crate::BitReader;
#[doc = "Field `PAGE_VALID` writer - 8:8\\]
Error location status for a full page, based on the mask definition Read 0x0: error locations invalid for all polynomials enabled in the ECC_INTERRUPT_MASK register Read 0x1: all error locations valid Write 0x0: no effect Write 0x1: clear interrupt"]
pub type PageValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Error location status for syndrome polynomial 0"]
    #[inline(always)]
    pub fn loc_valid_0(&self) -> LocValid0R {
        LocValid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error location status for syndrome polynomial 1"]
    #[inline(always)]
    pub fn loc_valid_1(&self) -> LocValid1R {
        LocValid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Error location status for syndrome polynomial 2"]
    #[inline(always)]
    pub fn loc_valid_2(&self) -> LocValid2R {
        LocValid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Error location status for syndrome polynomial 3"]
    #[inline(always)]
    pub fn loc_valid_3(&self) -> LocValid3R {
        LocValid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Error location status for syndrome polynomial 4"]
    #[inline(always)]
    pub fn loc_valid_4(&self) -> LocValid4R {
        LocValid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Error location status for syndrome polynomial 5"]
    #[inline(always)]
    pub fn loc_valid_5(&self) -> LocValid5R {
        LocValid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Error location status for syndrome polynomial 6"]
    #[inline(always)]
    pub fn loc_valid_6(&self) -> LocValid6R {
        LocValid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Error location status for syndrome polynomial 7 Read 0x0: no syndrome processed or process in progress Read 0x1: error location process completed Write 0x0: no effect Write 0x1: clear interrupt"]
    #[inline(always)]
    pub fn loc_valid_7(&self) -> LocValid7R {
        LocValid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Error location status for a full page, based on the mask definition Read 0x0: error locations invalid for all polynomials enabled in the ECC_INTERRUPT_MASK register Read 0x1: all error locations valid Write 0x0: no effect Write 0x1: clear interrupt"]
    #[inline(always)]
    pub fn page_valid(&self) -> PageValidR {
        PageValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Error location status for syndrome polynomial 0"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_0(&mut self) -> LocValid0W<MemElmIrqstatusSpec> {
        LocValid0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error location status for syndrome polynomial 1"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_1(&mut self) -> LocValid1W<MemElmIrqstatusSpec> {
        LocValid1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Error location status for syndrome polynomial 2"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_2(&mut self) -> LocValid2W<MemElmIrqstatusSpec> {
        LocValid2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Error location status for syndrome polynomial 3"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_3(&mut self) -> LocValid3W<MemElmIrqstatusSpec> {
        LocValid3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Error location status for syndrome polynomial 4"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_4(&mut self) -> LocValid4W<MemElmIrqstatusSpec> {
        LocValid4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Error location status for syndrome polynomial 5"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_5(&mut self) -> LocValid5W<MemElmIrqstatusSpec> {
        LocValid5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Error location status for syndrome polynomial 6"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_6(&mut self) -> LocValid6W<MemElmIrqstatusSpec> {
        LocValid6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Error location status for syndrome polynomial 7 Read 0x0: no syndrome processed or process in progress Read 0x1: error location process completed Write 0x0: no effect Write 0x1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn loc_valid_7(&mut self) -> LocValid7W<MemElmIrqstatusSpec> {
        LocValid7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Error location status for a full page, based on the mask definition Read 0x0: error locations invalid for all polynomials enabled in the ECC_INTERRUPT_MASK register Read 0x1: all error locations valid Write 0x0: no effect Write 0x1: clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn page_valid(&mut self) -> PageValidW<MemElmIrqstatusSpec> {
        PageValidW::new(self, 8)
    }
}
#[doc = "Interrupt status. This register doubles as a status register for the error location processes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_irqstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_irqstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmIrqstatusSpec;
impl crate::RegisterSpec for MemElmIrqstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_irqstatus::R`](R) reader structure"]
impl crate::Readable for MemElmIrqstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_irqstatus::W`](W) writer structure"]
impl crate::Writable for MemElmIrqstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_IRQSTATUS to value 0"]
impl crate::Resettable for MemElmIrqstatusSpec {
    const RESET_VALUE: u32 = 0;
}
