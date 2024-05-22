#[doc = "Register `REGS_SPLOCK_SYSTATUS` reader"]
pub type R = crate::R<RegsSplockSystatusSpec>;
#[doc = "Register `REGS_SPLOCK_SYSTATUS` writer"]
pub type W = crate::W<RegsSplockSystatusSpec>;
#[doc = "Field `IN_USE0` reader - 0:0\\]
In-Use flag 0 covering lock registers 0 - 31. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 0 - 31 are in the Not Taken state Read 1 : At least one of the lock registers 0 - 31 are in the Taken state"]
pub type InUse0R = crate::BitReader;
#[doc = "Field `IN_USE0` writer - 0:0\\]
In-Use flag 0 covering lock registers 0 - 31. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 0 - 31 are in the Not Taken state Read 1 : At least one of the lock registers 0 - 31 are in the Taken state"]
pub type InUse0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE1` reader - 1:1\\]
In-Use flag 1 covering lock registers 32 - 63. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 32 - 63 are in the Not Taken state Read 1 : At least one of the lock registers 32 - 63 are in the Taken state"]
pub type InUse1R = crate::BitReader;
#[doc = "Field `IN_USE1` writer - 1:1\\]
In-Use flag 1 covering lock registers 32 - 63. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 32 - 63 are in the Not Taken state Read 1 : At least one of the lock registers 32 - 63 are in the Taken state"]
pub type InUse1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE2` reader - 2:2\\]
In-Use flag 2 covering lock registers 64 - 95. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 64 - 95 are in the Not Taken state Read 1 : At least one of the lock registers 64 - 95 are in the Taken state"]
pub type InUse2R = crate::BitReader;
#[doc = "Field `IN_USE2` writer - 2:2\\]
In-Use flag 2 covering lock registers 64 - 95. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 64 - 95 are in the Not Taken state Read 1 : At least one of the lock registers 64 - 95 are in the Taken state"]
pub type InUse2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE3` reader - 3:3\\]
In-Use flag 3 covering lock registers 96 - 127. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 96 - 127 are in the Not Taken state Read 1 : At least one of the lock registers 96 - 127 are in the Taken state"]
pub type InUse3R = crate::BitReader;
#[doc = "Field `IN_USE3` writer - 3:3\\]
In-Use flag 3 covering lock registers 96 - 127. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 96 - 127 are in the Not Taken state Read 1 : At least one of the lock registers 96 - 127 are in the Taken state"]
pub type InUse3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE4` reader - 4:4\\]
In-Use flag 4 covering lock registers 128 - 159. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 128 - 159 are in the Not Taken state Read 1 : At least one of the lock registers 128 - 159 are in the Taken state"]
pub type InUse4R = crate::BitReader;
#[doc = "Field `IN_USE4` writer - 4:4\\]
In-Use flag 4 covering lock registers 128 - 159. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 128 - 159 are in the Not Taken state Read 1 : At least one of the lock registers 128 - 159 are in the Taken state"]
pub type InUse4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE5` reader - 5:5\\]
In-Use flag 5 covering lock registers 160 - 191. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 160 - 191 are in the Not Taken state Read 1 : At least one of the lock registers 160 - 191 are in the Taken state"]
pub type InUse5R = crate::BitReader;
#[doc = "Field `IN_USE5` writer - 5:5\\]
In-Use flag 5 covering lock registers 160 - 191. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 160 - 191 are in the Not Taken state Read 1 : At least one of the lock registers 160 - 191 are in the Taken state"]
pub type InUse5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE6` reader - 6:6\\]
In-Use flag 6 covering lock registers 192 - 223. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 192 - 223 are in the Not Taken state Read 1 : At least one of the lock registers 192 - 223 are in the Taken state"]
pub type InUse6R = crate::BitReader;
#[doc = "Field `IN_USE6` writer - 6:6\\]
In-Use flag 6 covering lock registers 192 - 223. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 192 - 223 are in the Not Taken state Read 1 : At least one of the lock registers 192 - 223 are in the Taken state"]
pub type InUse6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_USE7` reader - 7:7\\]
In-Use flag 7 covering lock registers 224 - 255. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 224 - 255 are in the Not Taken state Read 1 : At least one of the lock registers 224 - 255 are in the Taken state"]
pub type InUse7R = crate::BitReader;
#[doc = "Field `IN_USE7` writer - 7:7\\]
In-Use flag 7 covering lock registers 224 - 255. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 224 - 255 are in the Not Taken state Read 1 : At least one of the lock registers 224 - 255 are in the Taken state"]
pub type InUse7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUM_LOCKS` reader - 31:24\\]
Module configuration parameter n, the total number of spinlocks divided by 32. e.g. For 256 spin locks, this will return the number 0x08"]
pub type NumLocksR = crate::FieldReader;
#[doc = "Field `NUM_LOCKS` writer - 31:24\\]
Module configuration parameter n, the total number of spinlocks divided by 32. e.g. For 256 spin locks, this will return the number 0x08"]
pub type NumLocksW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
In-Use flag 0 covering lock registers 0 - 31. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 0 - 31 are in the Not Taken state Read 1 : At least one of the lock registers 0 - 31 are in the Taken state"]
    #[inline(always)]
    pub fn in_use0(&self) -> InUse0R {
        InUse0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
In-Use flag 1 covering lock registers 32 - 63. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 32 - 63 are in the Not Taken state Read 1 : At least one of the lock registers 32 - 63 are in the Taken state"]
    #[inline(always)]
    pub fn in_use1(&self) -> InUse1R {
        InUse1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
In-Use flag 2 covering lock registers 64 - 95. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 64 - 95 are in the Not Taken state Read 1 : At least one of the lock registers 64 - 95 are in the Taken state"]
    #[inline(always)]
    pub fn in_use2(&self) -> InUse2R {
        InUse2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
In-Use flag 3 covering lock registers 96 - 127. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 96 - 127 are in the Not Taken state Read 1 : At least one of the lock registers 96 - 127 are in the Taken state"]
    #[inline(always)]
    pub fn in_use3(&self) -> InUse3R {
        InUse3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
In-Use flag 4 covering lock registers 128 - 159. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 128 - 159 are in the Not Taken state Read 1 : At least one of the lock registers 128 - 159 are in the Taken state"]
    #[inline(always)]
    pub fn in_use4(&self) -> InUse4R {
        InUse4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
In-Use flag 5 covering lock registers 160 - 191. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 160 - 191 are in the Not Taken state Read 1 : At least one of the lock registers 160 - 191 are in the Taken state"]
    #[inline(always)]
    pub fn in_use5(&self) -> InUse5R {
        InUse5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
In-Use flag 6 covering lock registers 192 - 223. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 192 - 223 are in the Not Taken state Read 1 : At least one of the lock registers 192 - 223 are in the Taken state"]
    #[inline(always)]
    pub fn in_use6(&self) -> InUse6R {
        InUse6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
In-Use flag 7 covering lock registers 224 - 255. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 224 - 255 are in the Not Taken state Read 1 : At least one of the lock registers 224 - 255 are in the Taken state"]
    #[inline(always)]
    pub fn in_use7(&self) -> InUse7R {
        InUse7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Module configuration parameter n, the total number of spinlocks divided by 32. e.g. For 256 spin locks, this will return the number 0x08"]
    #[inline(always)]
    pub fn num_locks(&self) -> NumLocksR {
        NumLocksR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
In-Use flag 0 covering lock registers 0 - 31. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 0 - 31 are in the Not Taken state Read 1 : At least one of the lock registers 0 - 31 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use0(&mut self) -> InUse0W<RegsSplockSystatusSpec> {
        InUse0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
In-Use flag 1 covering lock registers 32 - 63. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 32 - 63 are in the Not Taken state Read 1 : At least one of the lock registers 32 - 63 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use1(&mut self) -> InUse1W<RegsSplockSystatusSpec> {
        InUse1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
In-Use flag 2 covering lock registers 64 - 95. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 64 - 95 are in the Not Taken state Read 1 : At least one of the lock registers 64 - 95 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use2(&mut self) -> InUse2W<RegsSplockSystatusSpec> {
        InUse2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
In-Use flag 3 covering lock registers 96 - 127. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 96 - 127 are in the Not Taken state Read 1 : At least one of the lock registers 96 - 127 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use3(&mut self) -> InUse3W<RegsSplockSystatusSpec> {
        InUse3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
In-Use flag 4 covering lock registers 128 - 159. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 128 - 159 are in the Not Taken state Read 1 : At least one of the lock registers 128 - 159 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use4(&mut self) -> InUse4W<RegsSplockSystatusSpec> {
        InUse4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
In-Use flag 5 covering lock registers 160 - 191. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 160 - 191 are in the Not Taken state Read 1 : At least one of the lock registers 160 - 191 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use5(&mut self) -> InUse5W<RegsSplockSystatusSpec> {
        InUse5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
In-Use flag 6 covering lock registers 192 - 223. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 192 - 223 are in the Not Taken state Read 1 : At least one of the lock registers 192 - 223 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use6(&mut self) -> InUse6W<RegsSplockSystatusSpec> {
        InUse6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
In-Use flag 7 covering lock registers 224 - 255. If no lock registers are implemented in this range, then this flag always reads as 0 Read 0 : All lock registers 224 - 255 are in the Not Taken state Read 1 : At least one of the lock registers 224 - 255 are in the Taken state"]
    #[inline(always)]
    #[must_use]
    pub fn in_use7(&mut self) -> InUse7W<RegsSplockSystatusSpec> {
        InUse7W::new(self, 7)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Module configuration parameter n, the total number of spinlocks divided by 32. e.g. For 256 spin locks, this will return the number 0x08"]
    #[inline(always)]
    #[must_use]
    pub fn num_locks(&mut self) -> NumLocksW<RegsSplockSystatusSpec> {
        NumLocksW::new(self, 24)
    }
}
#[doc = "Provides information about the Spinlock module\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_splock_systatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_splock_systatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsSplockSystatusSpec;
impl crate::RegisterSpec for RegsSplockSystatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_splock_systatus::R`](R) reader structure"]
impl crate::Readable for RegsSplockSystatusSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_splock_systatus::W`](W) writer structure"]
impl crate::Writable for RegsSplockSystatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_SPLOCK_SYSTATUS to value 0x0800_0000"]
impl crate::Resettable for RegsSplockSystatusSpec {
    const RESET_VALUE: u32 = 0x0800_0000;
}
