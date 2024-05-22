#[doc = "Register `PR1_PROTECT__SLV__REGS_unlock_key` reader"]
pub type R = crate::R<Pr1Protect_Slv_RegsUnlockKeySpec>;
#[doc = "Register `PR1_PROTECT__SLV__REGS_unlock_key` writer"]
pub type W = crate::W<Pr1Protect_Slv_RegsUnlockKeySpec>;
#[doc = "Field `UNLOCK_KEY` reader - 31:0\\]
UnLock Key Pattern 0x83E7_0B13 to UnLock 0x0000_0000 to Lock Must unlock to update MMRs"]
pub type UnlockKeyR = crate::FieldReader<u32>;
#[doc = "Field `UNLOCK_KEY` writer - 31:0\\]
UnLock Key Pattern 0x83E7_0B13 to UnLock 0x0000_0000 to Lock Must unlock to update MMRs"]
pub type UnlockKeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
UnLock Key Pattern 0x83E7_0B13 to UnLock 0x0000_0000 to Lock Must unlock to update MMRs"]
    #[inline(always)]
    pub fn unlock_key(&self) -> UnlockKeyR {
        UnlockKeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
UnLock Key Pattern 0x83E7_0B13 to UnLock 0x0000_0000 to Lock Must unlock to update MMRs"]
    #[inline(always)]
    #[must_use]
    pub fn unlock_key(&mut self) -> UnlockKeyW<Pr1Protect_Slv_RegsUnlockKeySpec> {
        UnlockKeyW::new(self, 0)
    }
}
#[doc = "PR1_PROTECT__SLV__REGS_unlock_key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_protect__slv__regs_unlock_key::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_protect__slv__regs_unlock_key::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Protect_Slv_RegsUnlockKeySpec;
impl crate::RegisterSpec for Pr1Protect_Slv_RegsUnlockKeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_protect__slv__regs_unlock_key::R`](R) reader structure"]
impl crate::Readable for Pr1Protect_Slv_RegsUnlockKeySpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_protect__slv__regs_unlock_key::W`](W) writer structure"]
impl crate::Writable for Pr1Protect_Slv_RegsUnlockKeySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_PROTECT__SLV__REGS_unlock_key to value 0"]
impl crate::Resettable for Pr1Protect_Slv_RegsUnlockKeySpec {
    const RESET_VALUE: u32 = 0;
}
