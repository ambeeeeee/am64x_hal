#[doc = "Register `MMR__VBUSP__CFG2_VTM_CLK_CTRL` reader"]
pub type R = crate::R<Mmr_Vbusp_Cfg2VtmClkCtrlSpec>;
#[doc = "Register `MMR__VBUSP__CFG2_VTM_CLK_CTRL` writer"]
pub type W = crate::W<Mmr_Vbusp_Cfg2VtmClkCtrlSpec>;
#[doc = "Field `TSENS_CLK_DIV` reader - 4:0\\]
Temperature sensor clock source divider selector. Device specific. Default set by e-fuse or tie-off. Divider uses select reference clock as source. 0 = 1x divide. 1 = 2x divide. ... 15 = 16x divide. ... 63 = 64x divide. Reset value is from e-fuse at POR, efuse_tsens_clk_src_div."]
pub type TsensClkDivR = crate::FieldReader;
#[doc = "Field `TSENS_CLK_DIV` writer - 4:0\\]
Temperature sensor clock source divider selector. Device specific. Default set by e-fuse or tie-off. Divider uses select reference clock as source. 0 = 1x divide. 1 = 2x divide. ... 15 = 16x divide. ... 63 = 64x divide. Reset value is from e-fuse at POR, efuse_tsens_clk_src_div."]
pub type TsensClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSENS_CLK_SEL` reader - 31:31\\]
Temperature sensor clock source selector. Device specific. 0 = fix_ref_clk as source. 1 = fix_ref2_clk as source. Reset value is at POR only."]
pub type TsensClkSelR = crate::BitReader;
#[doc = "Field `TSENS_CLK_SEL` writer - 31:31\\]
Temperature sensor clock source selector. Device specific. 0 = fix_ref_clk as source. 1 = fix_ref2_clk as source. Reset value is at POR only."]
pub type TsensClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Temperature sensor clock source divider selector. Device specific. Default set by e-fuse or tie-off. Divider uses select reference clock as source. 0 = 1x divide. 1 = 2x divide. ... 15 = 16x divide. ... 63 = 64x divide. Reset value is from e-fuse at POR, efuse_tsens_clk_src_div."]
    #[inline(always)]
    pub fn tsens_clk_div(&self) -> TsensClkDivR {
        TsensClkDivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Temperature sensor clock source selector. Device specific. 0 = fix_ref_clk as source. 1 = fix_ref2_clk as source. Reset value is at POR only."]
    #[inline(always)]
    pub fn tsens_clk_sel(&self) -> TsensClkSelR {
        TsensClkSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Temperature sensor clock source divider selector. Device specific. Default set by e-fuse or tie-off. Divider uses select reference clock as source. 0 = 1x divide. 1 = 2x divide. ... 15 = 16x divide. ... 63 = 64x divide. Reset value is from e-fuse at POR, efuse_tsens_clk_src_div."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_div(&mut self) -> TsensClkDivW<Mmr_Vbusp_Cfg2VtmClkCtrlSpec> {
        TsensClkDivW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Temperature sensor clock source selector. Device specific. 0 = fix_ref_clk as source. 1 = fix_ref2_clk as source. Reset value is at POR only."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_sel(&mut self) -> TsensClkSelW<Mmr_Vbusp_Cfg2VtmClkCtrlSpec> {
        TsensClkSelW::new(self, 31)
    }
}
#[doc = "VTM clock related control MMR. The default reset values will not be necessarily overwritten. The write capability in the MMR is for having the option to debug and have software driven adjustments if necessary. The e-fuse value is sampled from input port efuse_tsens_clk_src_div. The tsens_clks_src_div field is Device specific.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmr__vbusp__cfg2_vtm_clk_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmr__vbusp__cfg2_vtm_clk_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mmr_Vbusp_Cfg2VtmClkCtrlSpec;
impl crate::RegisterSpec for Mmr_Vbusp_Cfg2VtmClkCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmr__vbusp__cfg2_vtm_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for Mmr_Vbusp_Cfg2VtmClkCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmr__vbusp__cfg2_vtm_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for Mmr_Vbusp_Cfg2VtmClkCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMR__VBUSP__CFG2_VTM_CLK_CTRL to value 0"]
impl crate::Resettable for Mmr_Vbusp_Cfg2VtmClkCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
