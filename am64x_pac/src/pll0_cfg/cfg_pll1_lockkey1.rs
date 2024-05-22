#[doc = "Register `CFG_pll1_LOCKKEY1` reader"]
pub type R = crate::R<CfgPll1Lockkey1Spec>;
#[doc = "Register `CFG_pll1_LOCKKEY1` writer"]
pub type W = crate::W<CfgPll1Lockkey1Spec>;
#[doc = "Field `LOCKKEY1_VAL` reader - 31:0\\]
Write the kick1 unlock value after the kick0 unlock value to unlock the write-protected Partition1 registers"]
pub type Lockkey1ValR = crate::FieldReader<u32>;
#[doc = "Field `LOCKKEY1_VAL` writer - 31:0\\]
Write the kick1 unlock value after the kick0 unlock value to unlock the write-protected Partition1 registers"]
pub type Lockkey1ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Write the kick1 unlock value after the kick0 unlock value to unlock the write-protected Partition1 registers"]
    #[inline(always)]
    pub fn lockkey1_val(&self) -> Lockkey1ValR {
        Lockkey1ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Write the kick1 unlock value after the kick0 unlock value to unlock the write-protected Partition1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey1_val(&mut self) -> Lockkey1ValW<CfgPll1Lockkey1Spec> {
        Lockkey1ValW::new(self, 0)
    }
}
#[doc = "CFG_pll1_LOCKKEY1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_lockkey1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_lockkey1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll1Lockkey1Spec;
impl crate::RegisterSpec for CfgPll1Lockkey1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll1_lockkey1::R`](R) reader structure"]
impl crate::Readable for CfgPll1Lockkey1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll1_lockkey1::W`](W) writer structure"]
impl crate::Writable for CfgPll1Lockkey1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll1_LOCKKEY1 to value 0"]
impl crate::Resettable for CfgPll1Lockkey1Spec {
    const RESET_VALUE: u32 = 0;
}
