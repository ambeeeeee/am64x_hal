#[doc = "Register `CFG_pll2_FREQ_CTRL1` reader"]
pub type R = crate::R<CfgPll2FreqCtrl1Spec>;
#[doc = "Register `CFG_pll2_FREQ_CTRL1` writer"]
pub type W = crate::W<CfgPll2FreqCtrl1Spec>;
#[doc = "Field `FB_DIV_FRAC` reader - 23:0\\]
PLL feedback divider (fractional portion) Supports values of 0 to 0.999999940395. The total feedback divide value is (fb_div_int + fb_div_frac / (2^24)) 24'h000000 - 0 24'h000001 - .000000059605 (1/(2^24)) 24'h000002 - .000000119209 (2/(2^24)) : 24'h800000 - .500000000000 : 24'hFFFFFF - .999999940395 (1677215/(2^24))"]
pub type FbDivFracR = crate::FieldReader<u32>;
#[doc = "Field `FB_DIV_FRAC` writer - 23:0\\]
PLL feedback divider (fractional portion) Supports values of 0 to 0.999999940395. The total feedback divide value is (fb_div_int + fb_div_frac / (2^24)) 24'h000000 - 0 24'h000001 - .000000059605 (1/(2^24)) 24'h000002 - .000000119209 (2/(2^24)) : 24'h800000 - .500000000000 : 24'hFFFFFF - .999999940395 (1677215/(2^24))"]
pub type FbDivFracW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
PLL feedback divider (fractional portion) Supports values of 0 to 0.999999940395. The total feedback divide value is (fb_div_int + fb_div_frac / (2^24)) 24'h000000 - 0 24'h000001 - .000000059605 (1/(2^24)) 24'h000002 - .000000119209 (2/(2^24)) : 24'h800000 - .500000000000 : 24'hFFFFFF - .999999940395 (1677215/(2^24))"]
    #[inline(always)]
    pub fn fb_div_frac(&self) -> FbDivFracR {
        FbDivFracR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
PLL feedback divider (fractional portion) Supports values of 0 to 0.999999940395. The total feedback divide value is (fb_div_int + fb_div_frac / (2^24)) 24'h000000 - 0 24'h000001 - .000000059605 (1/(2^24)) 24'h000002 - .000000119209 (2/(2^24)) : 24'h800000 - .500000000000 : 24'hFFFFFF - .999999940395 (1677215/(2^24))"]
    #[inline(always)]
    #[must_use]
    pub fn fb_div_frac(&mut self) -> FbDivFracW<CfgPll2FreqCtrl1Spec> {
        FbDivFracW::new(self, 0)
    }
}
#[doc = "CFG_pll2_FREQ_CTRL1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll2_freq_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll2_freq_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll2FreqCtrl1Spec;
impl crate::RegisterSpec for CfgPll2FreqCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll2_freq_ctrl1::R`](R) reader structure"]
impl crate::Readable for CfgPll2FreqCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll2_freq_ctrl1::W`](W) writer structure"]
impl crate::Writable for CfgPll2FreqCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll2_FREQ_CTRL1 to value 0"]
impl crate::Resettable for CfgPll2FreqCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}
