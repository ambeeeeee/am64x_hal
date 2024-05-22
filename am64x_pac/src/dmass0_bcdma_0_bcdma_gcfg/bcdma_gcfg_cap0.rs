#[doc = "Register `BCDMA_GCFG_CAP0` reader"]
pub type R = crate::R<BcdmaGcfgCap0Spec>;
#[doc = "Register `BCDMA_GCFG_CAP0` writer"]
pub type W = crate::W<BcdmaGcfgCap0Spec>;
#[doc = "Field `TYPE0` reader - 0:0\\]
Type 0 TR is supported"]
pub type Type0R = crate::BitReader;
#[doc = "Field `TYPE0` writer - 0:0\\]
Type 0 TR is supported"]
pub type Type0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE1` reader - 1:1\\]
Type 1 TR is supported"]
pub type Type1R = crate::BitReader;
#[doc = "Field `TYPE1` writer - 1:1\\]
Type 1 TR is supported"]
pub type Type1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE2` reader - 2:2\\]
Type 2 TR is supported"]
pub type Type2R = crate::BitReader;
#[doc = "Field `TYPE2` writer - 2:2\\]
Type 2 TR is supported"]
pub type Type2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE3` reader - 3:3\\]
Type 3 TR is supported"]
pub type Type3R = crate::BitReader;
#[doc = "Field `TYPE3` writer - 3:3\\]
Type 3 TR is supported"]
pub type Type3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE4` reader - 4:4\\]
Type 4 TR is supported"]
pub type Type4R = crate::BitReader;
#[doc = "Field `TYPE4` writer - 4:4\\]
Type 4 TR is supported"]
pub type Type4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE5` reader - 5:5\\]
Type 5 TR is supported"]
pub type Type5R = crate::BitReader;
#[doc = "Field `TYPE5` writer - 5:5\\]
Type 5 TR is supported"]
pub type Type5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE6` reader - 6:6\\]
Type 6 TR is supported"]
pub type Type6R = crate::BitReader;
#[doc = "Field `TYPE6` writer - 6:6\\]
Type 6 TR is supported"]
pub type Type6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE7` reader - 7:7\\]
Type 7 TR is supported"]
pub type Type7R = crate::BitReader;
#[doc = "Field `TYPE7` writer - 7:7\\]
Type 7 TR is supported"]
pub type Type7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE8` reader - 8:8\\]
Type 8 TR is supported"]
pub type Type8R = crate::BitReader;
#[doc = "Field `TYPE8` writer - 8:8\\]
Type 8 TR is supported"]
pub type Type8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE9` reader - 9:9\\]
Type 9 TR is supported"]
pub type Type9R = crate::BitReader;
#[doc = "Field `TYPE9` writer - 9:9\\]
Type 9 TR is supported"]
pub type Type9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE10` reader - 10:10\\]
Type 10 TR is supported"]
pub type Type10R = crate::BitReader;
#[doc = "Field `TYPE10` writer - 10:10\\]
Type 10 TR is supported"]
pub type Type10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE11` reader - 11:11\\]
Type 11 TR is supported"]
pub type Type11R = crate::BitReader;
#[doc = "Field `TYPE11` writer - 11:11\\]
Type 11 TR is supported"]
pub type Type11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE12` reader - 12:12\\]
Type 12 TR is supported"]
pub type Type12R = crate::BitReader;
#[doc = "Field `TYPE12` writer - 12:12\\]
Type 12 TR is supported"]
pub type Type12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE13` reader - 13:13\\]
Type 13 TR is supported"]
pub type Type13R = crate::BitReader;
#[doc = "Field `TYPE13` writer - 13:13\\]
Type 13 TR is supported"]
pub type Type13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE14` reader - 14:14\\]
Type 14 TR is supported"]
pub type Type14R = crate::BitReader;
#[doc = "Field `TYPE14` writer - 14:14\\]
Type 14 TR is supported"]
pub type Type14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE15` reader - 15:15\\]
Type 15 TR is supported"]
pub type Type15R = crate::BitReader;
#[doc = "Field `TYPE15` writer - 15:15\\]
Type 15 TR is supported"]
pub type Type15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATIC` reader - 16:16\\]
STATIC field is supported"]
pub type StaticR = crate::BitReader;
#[doc = "Field `STATIC` writer - 16:16\\]
STATIC field is supported"]
pub type StaticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOL` reader - 17:17\\]
EOL field is supported"]
pub type EolR = crate::BitReader;
#[doc = "Field `EOL` writer - 17:17\\]
EOL field is supported"]
pub type EolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCAL_TRIG` reader - 18:18\\]
Dedicated local trigger is supported"]
pub type LocalTrigR = crate::BitReader;
#[doc = "Field `LOCAL_TRIG` writer - 18:18\\]
Dedicated local trigger is supported"]
pub type LocalTrigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLOBAL_TRIG` reader - 19:19\\]
Global triggers 0 and 1 are supported"]
pub type GlobalTrigR = crate::BitReader;
#[doc = "Field `GLOBAL_TRIG` writer - 19:19\\]
Global triggers 0 and 1 are supported"]
pub type GlobalTrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Type 0 TR is supported"]
    #[inline(always)]
    pub fn type0(&self) -> Type0R {
        Type0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Type 1 TR is supported"]
    #[inline(always)]
    pub fn type1(&self) -> Type1R {
        Type1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Type 2 TR is supported"]
    #[inline(always)]
    pub fn type2(&self) -> Type2R {
        Type2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Type 3 TR is supported"]
    #[inline(always)]
    pub fn type3(&self) -> Type3R {
        Type3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Type 4 TR is supported"]
    #[inline(always)]
    pub fn type4(&self) -> Type4R {
        Type4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Type 5 TR is supported"]
    #[inline(always)]
    pub fn type5(&self) -> Type5R {
        Type5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Type 6 TR is supported"]
    #[inline(always)]
    pub fn type6(&self) -> Type6R {
        Type6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Type 7 TR is supported"]
    #[inline(always)]
    pub fn type7(&self) -> Type7R {
        Type7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Type 8 TR is supported"]
    #[inline(always)]
    pub fn type8(&self) -> Type8R {
        Type8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Type 9 TR is supported"]
    #[inline(always)]
    pub fn type9(&self) -> Type9R {
        Type9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Type 10 TR is supported"]
    #[inline(always)]
    pub fn type10(&self) -> Type10R {
        Type10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Type 11 TR is supported"]
    #[inline(always)]
    pub fn type11(&self) -> Type11R {
        Type11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Type 12 TR is supported"]
    #[inline(always)]
    pub fn type12(&self) -> Type12R {
        Type12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Type 13 TR is supported"]
    #[inline(always)]
    pub fn type13(&self) -> Type13R {
        Type13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Type 14 TR is supported"]
    #[inline(always)]
    pub fn type14(&self) -> Type14R {
        Type14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Type 15 TR is supported"]
    #[inline(always)]
    pub fn type15(&self) -> Type15R {
        Type15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
STATIC field is supported"]
    #[inline(always)]
    pub fn static_(&self) -> StaticR {
        StaticR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
EOL field is supported"]
    #[inline(always)]
    pub fn eol(&self) -> EolR {
        EolR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Dedicated local trigger is supported"]
    #[inline(always)]
    pub fn local_trig(&self) -> LocalTrigR {
        LocalTrigR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Global triggers 0 and 1 are supported"]
    #[inline(always)]
    pub fn global_trig(&self) -> GlobalTrigR {
        GlobalTrigR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Type 0 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type0(&mut self) -> Type0W<BcdmaGcfgCap0Spec> {
        Type0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Type 1 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type1(&mut self) -> Type1W<BcdmaGcfgCap0Spec> {
        Type1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Type 2 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type2(&mut self) -> Type2W<BcdmaGcfgCap0Spec> {
        Type2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Type 3 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type3(&mut self) -> Type3W<BcdmaGcfgCap0Spec> {
        Type3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Type 4 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type4(&mut self) -> Type4W<BcdmaGcfgCap0Spec> {
        Type4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Type 5 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type5(&mut self) -> Type5W<BcdmaGcfgCap0Spec> {
        Type5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Type 6 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type6(&mut self) -> Type6W<BcdmaGcfgCap0Spec> {
        Type6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Type 7 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type7(&mut self) -> Type7W<BcdmaGcfgCap0Spec> {
        Type7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Type 8 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type8(&mut self) -> Type8W<BcdmaGcfgCap0Spec> {
        Type8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Type 9 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type9(&mut self) -> Type9W<BcdmaGcfgCap0Spec> {
        Type9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Type 10 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type10(&mut self) -> Type10W<BcdmaGcfgCap0Spec> {
        Type10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Type 11 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type11(&mut self) -> Type11W<BcdmaGcfgCap0Spec> {
        Type11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Type 12 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type12(&mut self) -> Type12W<BcdmaGcfgCap0Spec> {
        Type12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Type 13 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type13(&mut self) -> Type13W<BcdmaGcfgCap0Spec> {
        Type13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Type 14 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type14(&mut self) -> Type14W<BcdmaGcfgCap0Spec> {
        Type14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Type 15 TR is supported"]
    #[inline(always)]
    #[must_use]
    pub fn type15(&mut self) -> Type15W<BcdmaGcfgCap0Spec> {
        Type15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
STATIC field is supported"]
    #[inline(always)]
    #[must_use]
    pub fn static_(&mut self) -> StaticW<BcdmaGcfgCap0Spec> {
        StaticW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
EOL field is supported"]
    #[inline(always)]
    #[must_use]
    pub fn eol(&mut self) -> EolW<BcdmaGcfgCap0Spec> {
        EolW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Dedicated local trigger is supported"]
    #[inline(always)]
    #[must_use]
    pub fn local_trig(&mut self) -> LocalTrigW<BcdmaGcfgCap0Spec> {
        LocalTrigW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Global triggers 0 and 1 are supported"]
    #[inline(always)]
    #[must_use]
    pub fn global_trig(&mut self) -> GlobalTrigW<BcdmaGcfgCap0Spec> {
        GlobalTrigW::new(self, 19)
    }
}
#[doc = "The Capabilities Register 0 specifies which standard features this BCDMA instance supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_gcfg_cap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_gcfg_cap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaGcfgCap0Spec;
impl crate::RegisterSpec for BcdmaGcfgCap0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_gcfg_cap0::R`](R) reader structure"]
impl crate::Readable for BcdmaGcfgCap0Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_gcfg_cap0::W`](W) writer structure"]
impl crate::Writable for BcdmaGcfgCap0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_GCFG_CAP0 to value 0x000a_800f"]
impl crate::Resettable for BcdmaGcfgCap0Spec {
    const RESET_VALUE: u32 = 0x000a_800f;
}
