#[doc = "Register `CFG_CLEARINT` reader"]
pub type R = crate::R<CfgClearintSpec>;
#[doc = "Register `CFG_CLEARINT` writer"]
pub type W = crate::W<CfgClearintSpec>;
#[doc = "Field `CLEARINT0` reader - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint0R = crate::BitReader;
#[doc = "Field `CLEARINT0` writer - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT1` reader - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint1R = crate::BitReader;
#[doc = "Field `CLEARINT1` writer - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT2` reader - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint2R = crate::BitReader;
#[doc = "Field `CLEARINT2` writer - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARINT3` reader - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint3R = crate::BitReader;
#[doc = "Field `CLEARINT3` writer - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearint3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA0` reader - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma0R = crate::BitReader;
#[doc = "Field `CLEARDMA0` writer - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA1` reader - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma1R = crate::BitReader;
#[doc = "Field `CLEARDMA1` writer - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA2` reader - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma2R = crate::BitReader;
#[doc = "Field `CLEARDMA2` writer - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARDMA3` reader - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma3R = crate::BitReader;
#[doc = "Field `CLEARDMA3` writer - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
pub type Cleardma3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEARTBINT` reader - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type CleartbintR = crate::BitReader;
#[doc = "Field `CLEARTBINT` writer - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type CleartbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAROVL0INT` reader - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl0intR = crate::BitReader;
#[doc = "Field `CLEAROVL0INT` writer - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAROVL1INT` reader - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl1intR = crate::BitReader;
#[doc = "Field `CLEAROVL1INT` writer - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
pub type Clearovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint0(&self) -> Clearint0R {
        Clearint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint1(&self) -> Clearint1R {
        Clearint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint2(&self) -> Clearint2R {
        Clearint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearint3(&self) -> Clearint3R {
        Clearint3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma0(&self) -> Cleardma0R {
        Cleardma0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma1(&self) -> Cleardma1R {
        Cleardma1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma2(&self) -> Cleardma2R {
        Cleardma2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    pub fn cleardma3(&self) -> Cleardma3R {
        Cleardma3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn cleartbint(&self) -> CleartbintR {
        CleartbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearovl0int(&self) -> Clearovl0intR {
        Clearovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    pub fn clearovl1int(&self) -> Clearovl1intR {
        Clearovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint0(&mut self) -> Clearint0W<CfgClearintSpec> {
        Clearint0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint1(&mut self) -> Clearint1W<CfgClearintSpec> {
        Clearint1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint2(&mut self) -> Clearint2W<CfgClearintSpec> {
        Clearint2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearint3(&mut self) -> Clearint3W<CfgClearintSpec> {
        Clearint3W::new(self, 3)
    }
    #[doc = "Bit 8 - 8:8\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma0(&mut self) -> Cleardma0W<CfgClearintSpec> {
        Cleardma0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma1(&mut self) -> Cleardma1W<CfgClearintSpec> {
        Cleardma1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma2(&mut self) -> Cleardma2W<CfgClearintSpec> {
        Cleardma2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
User and privilege mode (read): 0 = DMA request is disabled 1 = DMA request is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable DMA request"]
    #[inline(always)]
    #[must_use]
    pub fn cleardma3(&mut self) -> Cleardma3W<CfgClearintSpec> {
        Cleardma3W::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cleartbint(&mut self) -> CleartbintW<CfgClearintSpec> {
        CleartbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearovl0int(&mut self) -> Clearovl0intW<CfgClearintSpec> {
        Clearovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): 0 = interrupt is disabled 1 = interrupt is enabled Privilege mode (write): 0 = leaves the corresponding bit unchanged 1 = disable interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clearovl1int(&mut self) -> Clearovl1intW<CfgClearintSpec> {
        Clearovl1intW::new(self, 18)
    }
}
#[doc = "CFG_CLEARINT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_clearint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_clearint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgClearintSpec;
impl crate::RegisterSpec for CfgClearintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_clearint::R`](R) reader structure"]
impl crate::Readable for CfgClearintSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_clearint::W`](W) writer structure"]
impl crate::Writable for CfgClearintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_CLEARINT to value 0"]
impl crate::Resettable for CfgClearintSpec {
    const RESET_VALUE: u32 = 0;
}
