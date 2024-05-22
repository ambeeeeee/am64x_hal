#[doc = "Register `CFG_pll2_STAT` reader"]
pub type R = crate::R<CfgPll2StatSpec>;
#[doc = "Register `CFG_pll2_STAT` writer"]
pub type W = crate::W<CfgPll2StatSpec>;
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
    pub fn lock(&mut self) -> LockW<CfgPll2StatSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "CFG_pll2_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll2StatSpec;
impl crate::RegisterSpec for CfgPll2StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll2_stat::R`](R) reader structure"]
impl crate::Readable for CfgPll2StatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll2_stat::W`](W) writer structure"]
impl crate::Writable for CfgPll2StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll2_STAT to value 0"]
impl crate::Resettable for CfgPll2StatSpec {
    const RESET_VALUE: u32 = 0;
}
