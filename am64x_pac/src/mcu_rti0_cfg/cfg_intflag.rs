#[doc = "Register `CFG_INTFLAG` reader"]
pub type R = crate::R<CfgIntflagSpec>;
#[doc = "Register `CFG_INTFLAG` writer"]
pub type W = crate::W<CfgIntflagSpec>;
#[doc = "Field `INT0` reader - 0:0\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int0R = crate::BitReader;
#[doc = "Field `INT0` writer - 0:0\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT1` reader - 1:1\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int1R = crate::BitReader;
#[doc = "Field `INT1` writer - 1:1\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT2` reader - 2:2\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int2R = crate::BitReader;
#[doc = "Field `INT2` writer - 2:2\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT3` reader - 3:3\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int3R = crate::BitReader;
#[doc = "Field `INT3` writer - 3:3\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Int3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBINT` reader - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type TbintR = crate::BitReader;
#[doc = "Field `TBINT` writer - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type TbintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVL0INT` reader - 17:17\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl0intR = crate::BitReader;
#[doc = "Field `OVL0INT` writer - 17:17\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl0intW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVL1INT` reader - 18:18\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl1intR = crate::BitReader;
#[doc = "Field `OVL1INT` writer - 18:18\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
pub type Ovl1intW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int0(&self) -> Int0R {
        Int0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int1(&self) -> Int1R {
        Int1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int2(&self) -> Int2R {
        Int2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn int3(&self) -> Int3R {
        Int3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn tbint(&self) -> TbintR {
        TbintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn ovl0int(&self) -> Ovl0intR {
        Ovl0intR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    pub fn ovl1int(&self) -> Ovl1intR {
        Ovl1intR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int0(&mut self) -> Int0W<CfgIntflagSpec> {
        Int0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> Int1W<CfgIntflagSpec> {
        Int1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> Int2W<CfgIntflagSpec> {
        Int2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
User and privilege mode (read): determines if a interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> Int3W<CfgIntflagSpec> {
        Int3W::new(self, 3)
    }
    #[doc = "Bit 16 - 16:16\\]
User and privilege mode (read): this flag is set when the TBEXT bit is cleared by detection of a missing external clockedge. It will not be set by clearing TBEXT by software. determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn tbint(&mut self) -> TbintW<CfgIntflagSpec> {
        TbintW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovl0int(&mut self) -> Ovl0intW<CfgIntflagSpec> {
        Ovl0intW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
User and privilege mode (read): determines if an interrupt is pending 0 = no interrupt pending 1 = interrupt pending Privilege mode (write): 0 = leaves the bit unchanged 1 = set the bit to 0"]
    #[inline(always)]
    #[must_use]
    pub fn ovl1int(&mut self) -> Ovl1intW<CfgIntflagSpec> {
        Ovl1intW::new(self, 18)
    }
}
#[doc = "CFG_INTFLAG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_intflag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_intflag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIntflagSpec;
impl crate::RegisterSpec for CfgIntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_intflag::R`](R) reader structure"]
impl crate::Readable for CfgIntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_intflag::W`](W) writer structure"]
impl crate::Writable for CfgIntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_INTFLAG to value 0"]
impl crate::Resettable for CfgIntflagSpec {
    const RESET_VALUE: u32 = 0;
}
