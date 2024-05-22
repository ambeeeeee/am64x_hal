#[doc = "Register `MEM_ELM_PAGE_CTRL` reader"]
pub type R = crate::R<MemElmPageCtrlSpec>;
#[doc = "Register `MEM_ELM_PAGE_CTRL` writer"]
pub type W = crate::W<MemElmPageCtrlSpec>;
#[doc = "Field `SECTOR_0` reader - 0:0\\]
Set to 1 if syndrome polynomial 0 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector0R = crate::BitReader;
#[doc = "Field `SECTOR_0` writer - 0:0\\]
Set to 1 if syndrome polynomial 0 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_1` reader - 1:1\\]
Set to 1 if syndrome polynomial 1 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector1R = crate::BitReader;
#[doc = "Field `SECTOR_1` writer - 1:1\\]
Set to 1 if syndrome polynomial 1 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_2` reader - 2:2\\]
Set to 1 if syndrome polynomial 2 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector2R = crate::BitReader;
#[doc = "Field `SECTOR_2` writer - 2:2\\]
Set to 1 if syndrome polynomial 2 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_3` reader - 3:3\\]
Set to 1 if syndrome polynomial 3 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector3R = crate::BitReader;
#[doc = "Field `SECTOR_3` writer - 3:3\\]
Set to 1 if syndrome polynomial 3 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_4` reader - 4:4\\]
Set to 1 if syndrome polynomial 4 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector4R = crate::BitReader;
#[doc = "Field `SECTOR_4` writer - 4:4\\]
Set to 1 if syndrome polynomial 4 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_5` reader - 5:5\\]
Set to 1 if syndrome polynomial 5 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector5R = crate::BitReader;
#[doc = "Field `SECTOR_5` writer - 5:5\\]
Set to 1 if syndrome polynomial 5 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_6` reader - 6:6\\]
Set to 1 if syndrome polynomial 6 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector6R = crate::BitReader;
#[doc = "Field `SECTOR_6` writer - 6:6\\]
Set to 1 if syndrome polynomial 6 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECTOR_7` reader - 7:7\\]
Set to 1 if syndrome polynomial 7 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector7R = crate::BitReader;
#[doc = "Field `SECTOR_7` writer - 7:7\\]
Set to 1 if syndrome polynomial 7 is part of the page in page mode Must be 0 in continuous mode"]
pub type Sector7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set to 1 if syndrome polynomial 0 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_0(&self) -> Sector0R {
        Sector0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set to 1 if syndrome polynomial 1 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_1(&self) -> Sector1R {
        Sector1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to 1 if syndrome polynomial 2 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_2(&self) -> Sector2R {
        Sector2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set to 1 if syndrome polynomial 3 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_3(&self) -> Sector3R {
        Sector3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set to 1 if syndrome polynomial 4 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_4(&self) -> Sector4R {
        Sector4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set to 1 if syndrome polynomial 5 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_5(&self) -> Sector5R {
        Sector5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Set to 1 if syndrome polynomial 6 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_6(&self) -> Sector6R {
        Sector6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set to 1 if syndrome polynomial 7 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    pub fn sector_7(&self) -> Sector7R {
        Sector7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set to 1 if syndrome polynomial 0 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_0(&mut self) -> Sector0W<MemElmPageCtrlSpec> {
        Sector0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set to 1 if syndrome polynomial 1 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_1(&mut self) -> Sector1W<MemElmPageCtrlSpec> {
        Sector1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to 1 if syndrome polynomial 2 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_2(&mut self) -> Sector2W<MemElmPageCtrlSpec> {
        Sector2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set to 1 if syndrome polynomial 3 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_3(&mut self) -> Sector3W<MemElmPageCtrlSpec> {
        Sector3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Set to 1 if syndrome polynomial 4 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_4(&mut self) -> Sector4W<MemElmPageCtrlSpec> {
        Sector4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set to 1 if syndrome polynomial 5 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_5(&mut self) -> Sector5W<MemElmPageCtrlSpec> {
        Sector5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Set to 1 if syndrome polynomial 6 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_6(&mut self) -> Sector6W<MemElmPageCtrlSpec> {
        Sector6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Set to 1 if syndrome polynomial 7 is part of the page in page mode Must be 0 in continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn sector_7(&mut self) -> Sector7W<MemElmPageCtrlSpec> {
        Sector7W::new(self, 7)
    }
}
#[doc = "Page definition\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_elm_page_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_elm_page_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemElmPageCtrlSpec;
impl crate::RegisterSpec for MemElmPageCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_elm_page_ctrl::R`](R) reader structure"]
impl crate::Readable for MemElmPageCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mem_elm_page_ctrl::W`](W) writer structure"]
impl crate::Writable for MemElmPageCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_ELM_PAGE_CTRL to value 0"]
impl crate::Resettable for MemElmPageCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
