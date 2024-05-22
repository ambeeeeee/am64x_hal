#[doc = "Register `CFG_pll1_STAT` reader"]
pub type R = crate::R<CfgPll1StatSpec>;
#[doc = "Register `CFG_pll1_STAT` writer"]
pub type W = crate::W<CfgPll1StatSpec>;
#[doc = "Field `LOCK` reader - 0:0\\]
PLL lock status. Software should wait for lock to be asserted before clearing the PLL_CTRL_bypass_en bit 1'b0 - PLL is not locked 1'b1 - PLL is locked"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 0:0\\]
PLL lock status. Software should wait for lock to be asserted before clearing the PLL_CTRL_bypass_en bit 1'b0 - PLL is not locked 1'b1 - PLL is locked"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
PLL lock status. Software should wait for lock to be asserted before clearing the PLL_CTRL_bypass_en bit 1'b0 - PLL is not locked 1'b1 - PLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
PLL lock status. Software should wait for lock to be asserted before clearing the PLL_CTRL_bypass_en bit 1'b0 - PLL is not locked 1'b1 - PLL is locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<CfgPll1StatSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "CFG_pll1_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll1StatSpec;
impl crate::RegisterSpec for CfgPll1StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll1_stat::R`](R) reader structure"]
impl crate::Readable for CfgPll1StatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll1_stat::W`](W) writer structure"]
impl crate::Writable for CfgPll1StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll1_STAT to value 0"]
impl crate::Resettable for CfgPll1StatSpec {
    const RESET_VALUE: u32 = 0;
}
