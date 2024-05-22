#[doc = "Register `CFG_pll12_LOCKKEY0` reader"]
pub type R = crate::R<CfgPll12Lockkey0Spec>;
#[doc = "Register `CFG_pll12_LOCKKEY0` writer"]
pub type W = crate::W<CfgPll12Lockkey0Spec>;
#[doc = "Field `UNLOCKED` reader - 0:0\\]
Unlock status. When set, indicates that the proper unlock sequence has been performed and the partition is unlocked for writing."]
pub type UnlockedR = crate::BitReader;
#[doc = "Field `UNLOCKED` writer - 0:0\\]
Unlock status. When set, indicates that the proper unlock sequence has been performed and the partition is unlocked for writing."]
pub type UnlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - 31:1\\]
Write the kick0 unlock value followed by the kick1 unlock value to unlock the write-protected Partition12 registers"]
pub type KeyR = crate::FieldReader<u32>;
#[doc = "Field `KEY` writer - 31:1\\]
Write the kick0 unlock value followed by the kick1 unlock value to unlock the write-protected Partition12 registers"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Unlock status. When set, indicates that the proper unlock sequence has been performed and the partition is unlocked for writing."]
    #[inline(always)]
    pub fn unlocked(&self) -> UnlockedR {
        UnlockedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Write the kick0 unlock value followed by the kick1 unlock value to unlock the write-protected Partition12 registers"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Unlock status. When set, indicates that the proper unlock sequence has been performed and the partition is unlocked for writing."]
    #[inline(always)]
    #[must_use]
    pub fn unlocked(&mut self) -> UnlockedW<CfgPll12Lockkey0Spec> {
        UnlockedW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Write the kick0 unlock value followed by the kick1 unlock value to unlock the write-protected Partition12 registers"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgPll12Lockkey0Spec> {
        KeyW::new(self, 1)
    }
}
#[doc = "CFG_pll12_LOCKKEY0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll12_lockkey0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll12_lockkey0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll12Lockkey0Spec;
impl crate::RegisterSpec for CfgPll12Lockkey0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll12_lockkey0::R`](R) reader structure"]
impl crate::Readable for CfgPll12Lockkey0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll12_lockkey0::W`](W) writer structure"]
impl crate::Writable for CfgPll12Lockkey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll12_LOCKKEY0 to value 0"]
impl crate::Resettable for CfgPll12Lockkey0Spec {
    const RESET_VALUE: u32 = 0;
}
