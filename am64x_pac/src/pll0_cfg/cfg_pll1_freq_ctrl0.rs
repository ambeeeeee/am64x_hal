#[doc = "Register `CFG_pll1_FREQ_CTRL0` reader"]
pub type R = crate::R<CfgPll1FreqCtrl0Spec>;
#[doc = "Register `CFG_pll1_FREQ_CTRL0` writer"]
pub type W = crate::W<CfgPll1FreqCtrl0Spec>;
#[doc = "Field `FB_DIV_INT` reader - 11:0\\]
PLL feedback divider (integer portion) In Integer mode values of 16 - 3200 (dec) are supported. In Fractional mode values of 20 to 320 are supported. 12'h010 - Divide by 16 12'h011 - Divide by 17 : 12'h140 - Divide by 320 : 12'hC80 - Divide by 3200 12'hC81 - 12'hFFF - Not supported"]
pub type FbDivIntR = crate::FieldReader<u16>;
#[doc = "Field `FB_DIV_INT` writer - 11:0\\]
PLL feedback divider (integer portion) In Integer mode values of 16 - 3200 (dec) are supported. In Fractional mode values of 20 to 320 are supported. 12'h010 - Divide by 16 12'h011 - Divide by 17 : 12'h140 - Divide by 320 : 12'hC80 - Divide by 3200 12'hC81 - 12'hFFF - Not supported"]
pub type FbDivIntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
PLL feedback divider (integer portion) In Integer mode values of 16 - 3200 (dec) are supported. In Fractional mode values of 20 to 320 are supported. 12'h010 - Divide by 16 12'h011 - Divide by 17 : 12'h140 - Divide by 320 : 12'hC80 - Divide by 3200 12'hC81 - 12'hFFF - Not supported"]
    #[inline(always)]
    pub fn fb_div_int(&self) -> FbDivIntR {
        FbDivIntR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
PLL feedback divider (integer portion) In Integer mode values of 16 - 3200 (dec) are supported. In Fractional mode values of 20 to 320 are supported. 12'h010 - Divide by 16 12'h011 - Divide by 17 : 12'h140 - Divide by 320 : 12'hC80 - Divide by 3200 12'hC81 - 12'hFFF - Not supported"]
    #[inline(always)]
    #[must_use]
    pub fn fb_div_int(&mut self) -> FbDivIntW<CfgPll1FreqCtrl0Spec> {
        FbDivIntW::new(self, 0)
    }
}
#[doc = "CFG_pll1_FREQ_CTRL0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll1_freq_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll1_freq_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll1FreqCtrl0Spec;
impl crate::RegisterSpec for CfgPll1FreqCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll1_freq_ctrl0::R`](R) reader structure"]
impl crate::Readable for CfgPll1FreqCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll1_freq_ctrl0::W`](W) writer structure"]
impl crate::Writable for CfgPll1FreqCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll1_FREQ_CTRL0 to value 0x16"]
impl crate::Resettable for CfgPll1FreqCtrl0Spec {
    const RESET_VALUE: u32 = 0x16;
}
