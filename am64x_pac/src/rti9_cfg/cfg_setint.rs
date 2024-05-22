#[doc = "Register `CFG_SETINT` reader"]
pub type R = crate::R<CfgSetintSpec>;
#[doc = "Register `CFG_SETINT` writer"]
pub type W = crate::W<CfgSetintSpec>;
#[doc = "Field `SETINT0` reader - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint0R = crate::BitReader;
#[doc = "Field `SETINT0` writer - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT1` reader - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint1R = crate::BitReader;
#[doc = "Field `SETINT1` writer - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT2` reader - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint2R = crate::BitReader;
#[doc = "Field `SETINT2` writer - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETINT3` reader - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint3R = crate::BitReader;
#[doc = "Field `SETINT3` writer - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA0` reader - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma0R = crate::BitReader;
#[doc = "Field `SETDMA0` writer - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA1` reader - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma1R = crate::BitReader;
#[doc = "Field `SETDMA1` writer - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA2` reader - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma2R = crate::BitReader;
#[doc = "Field `SETDMA2` writer - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETDMA3` reader - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma3R = crate::BitReader;
#[doc = "Field `SETDMA3` writer - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
pub type Setdma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETTBINT` reader - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SettbintR = crate::BitReader;
#[doc = "Field `SETTBINT` writer - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type SettbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOVL0INT` reader - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl0intR = crate::BitReader;
#[doc = "Field `SETOVL0INT` writer - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETOVL1INT` reader - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl1intR = crate::BitReader;
#[doc = "Field `SETOVL1INT` writer - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
pub type Setovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint0(&self) -> Setint0R {
        Setint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint1(&self) -> Setint1R {
        Setint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint2(&self) -> Setint2R {
        Setint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setint3(&self) -> Setint3R {
        Setint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma0(&self) -> Setdma0R {
        Setdma0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma1(&self) -> Setdma1R {
        Setdma1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma2(&self) -> Setdma2R {
        Setdma2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    pub fn setdma3(&self) -> Setdma3R {
        Setdma3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn settbint(&self) -> SettbintR {
        SettbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setovl0int(&self) -> Setovl0intR {
        Setovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    pub fn setovl1int(&self) -> Setovl1intR {
        Setovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint0(&mut self) -> Setint0W<CfgSetintSpec> {
        Setint0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint1(&mut self) -> Setint1W<CfgSetintSpec> {
        Setint1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint2(&mut self) -> Setint2W<CfgSetintSpec> {
        Setint2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setint3(&mut self) -> Setint3W<CfgSetintSpec> {
        Setint3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma0(&mut self) -> Setdma0W<CfgSetintSpec> {
        Setdma0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma1(&mut self) -> Setdma1W<CfgSetintSpec> {
        Setdma1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma2(&mut self) -> Setdma2W<CfgSetintSpec> {
        Setdma2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn setdma3(&mut self) -> Setdma3W<CfgSetintSpec> {
        Setdma3W::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn settbint(&mut self) -> SettbintW<CfgSetintSpec> {
        SettbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setovl0int(&mut self) -> Setovl0intW<CfgSetintSpec> {
        Setovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = enable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn setovl1int(&mut self) -> Setovl1intW<CfgSetintSpec> {
        Setovl1intW::new(self, 18)
    }
}
#[doc = "CFG_SETINT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_setint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_setint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSetintSpec;
impl crate::RegisterSpec for CfgSetintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_setint::R`](R) reader structure"]
impl crate::Readable for CfgSetintSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_setint::W`](W) writer structure"]
impl crate::Writable for CfgSetintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_SETINT to value 0"]
impl crate::Resettable for CfgSetintSpec {
    const RESET_VALUE: u32 = 0;
}
