#[doc = "Register `REGS_LOCK` reader"]
pub type R = crate::R<RegsLockSpec>;
#[doc = "Register `REGS_LOCK` writer"]
pub type W = crate::W<RegsLockSpec>;
#[doc = "Field `TAKEN` reader - 0:0\\]
Lock Status Read 0 : Lock was previously free. The reader now has been granted the lock. Read 1 : Lock was previously taken. The reader has not been granted the lock and must retry. Write 0 : Free the lock by setting TAKEN to zero. Write 1 : No effect"]
pub type TakenR = crate::BitReader;
#[doc = "Field `TAKEN` writer - 0:0\\]
Lock Status Read 0 : Lock was previously free. The reader now has been granted the lock. Read 1 : Lock was previously taken. The reader has not been granted the lock and must retry. Write 0 : Free the lock by setting TAKEN to zero. Write 1 : No effect"]
pub type TakenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Lock Status Read 0 : Lock was previously free. The reader now has been granted the lock. Read 1 : Lock was previously taken. The reader has not been granted the lock and must retry. Write 0 : Free the lock by setting TAKEN to zero. Write 1 : No effect"]
    #[inline(always)]
    pub fn taken(&self) -> TakenR {
        TakenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Lock Status Read 0 : Lock was previously free. The reader now has been granted the lock. Read 1 : Lock was previously taken. The reader has not been granted the lock and must retry. Write 0 : Free the lock by setting TAKEN to zero. Write 1 : No effect"]
    #[inline(always)]
    #[must_use]
    pub fn taken(&mut self) -> TakenW<RegsLockSpec> {
        TakenW::new(self, 0)
    }
}
#[doc = "The Lock\\[a\\]
register is read and written to perform lock and unlock operations on lock 'a'\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegsLockSpec;
impl crate::RegisterSpec for RegsLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs_lock::R`](R) reader structure"]
impl crate::Readable for RegsLockSpec {}
#[doc = "`write(|w| ..)` method takes [`regs_lock::W`](W) writer structure"]
impl crate::Writable for RegsLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS_LOCK to value 0"]
impl crate::Resettable for RegsLockSpec {
    const RESET_VALUE: u32 = 0;
}
