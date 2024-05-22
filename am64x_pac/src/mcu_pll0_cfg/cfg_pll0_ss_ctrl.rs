#[doc = "Register `CFG_pll0_SS_CTRL` reader"]
pub type R = crate::R<CfgPll0SsCtrlSpec>;
#[doc = "Register `CFG_pll0_SS_CTRL` writer"]
pub type W = crate::W<CfgPll0SsCtrlSpec>;
#[doc = "Field `WAVE_SEL` reader - 0:0\\]
Wave pattern select External wave table should only be selected if PLL_CFG_ssm_wvtbl = 1 Field values (Others are reserved): 1'b0 - Use 128 point triangle wave table 1'b1 - Use external wave table"]
pub type WaveSelR = crate::BitReader;
#[doc = "Field `WAVE_SEL` writer - 0:0\\]
Wave pattern select External wave table should only be selected if PLL_CFG_ssm_wvtbl = 1 Field values (Others are reserved): 1'b0 - Use 128 point triangle wave table 1'b1 - Use external wave table"]
pub type WaveSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOWNSPREAD_EN` reader - 4:4\\]
Selects center spread or down spread clock variance 1'b0 - Center spread 1'b1 - Down spread"]
pub type DownspreadEnR = crate::BitReader;
#[doc = "Field `DOWNSPREAD_EN` writer - 4:4\\]
Selects center spread or down spread clock variance 1'b0 - Center spread 1'b1 - Down spread"]
pub type DownspreadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - 15:15\\]
SSM reset. When set to 1 the SSM modulator is in reset"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 15:15\\]
SSM reset. When set to 1 the SSM modulator is in reset"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WV_TBLE_MAXADDR` reader - 25:18\\]
Wave table max address. Indicates the maximum number of address bits used to access the external wave table. These bits are not used if PLL_CFG_ssm_wvtbl = 0"]
pub type WvTbleMaxaddrR = crate::FieldReader;
#[doc = "Field `WV_TBLE_MAXADDR` writer - 25:18\\]
Wave table max address. Indicates the maximum number of address bits used to access the external wave table. These bits are not used if PLL_CFG_ssm_wvtbl = 0"]
pub type WvTbleMaxaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYPASS_EN` reader - 31:31\\]
Bypass the SS modulator. 1'b0 - Spread spectrum modulation is enabled 1'b1 - SSMOD is bypassed. No modulation of PLL frequency"]
pub type BypassEnR = crate::BitReader;
#[doc = "Field `BYPASS_EN` writer - 31:31\\]
Bypass the SS modulator. 1'b0 - Spread spectrum modulation is enabled 1'b1 - SSMOD is bypassed. No modulation of PLL frequency"]
pub type BypassEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Wave pattern select External wave table should only be selected if PLL_CFG_ssm_wvtbl = 1 Field values (Others are reserved): 1'b0 - Use 128 point triangle wave table 1'b1 - Use external wave table"]
    #[inline(always)]
    pub fn wave_sel(&self) -> WaveSelR {
        WaveSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Selects center spread or down spread clock variance 1'b0 - Center spread 1'b1 - Down spread"]
    #[inline(always)]
    pub fn downspread_en(&self) -> DownspreadEnR {
        DownspreadEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
SSM reset. When set to 1 the SSM modulator is in reset"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:25 - 25:18\\]
Wave table max address. Indicates the maximum number of address bits used to access the external wave table. These bits are not used if PLL_CFG_ssm_wvtbl = 0"]
    #[inline(always)]
    pub fn wv_tble_maxaddr(&self) -> WvTbleMaxaddrR {
        WvTbleMaxaddrR::new(((self.bits >> 18) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Bypass the SS modulator. 1'b0 - Spread spectrum modulation is enabled 1'b1 - SSMOD is bypassed. No modulation of PLL frequency"]
    #[inline(always)]
    pub fn bypass_en(&self) -> BypassEnR {
        BypassEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Wave pattern select External wave table should only be selected if PLL_CFG_ssm_wvtbl = 1 Field values (Others are reserved): 1'b0 - Use 128 point triangle wave table 1'b1 - Use external wave table"]
    #[inline(always)]
    #[must_use]
    pub fn wave_sel(&mut self) -> WaveSelW<CfgPll0SsCtrlSpec> {
        WaveSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Selects center spread or down spread clock variance 1'b0 - Center spread 1'b1 - Down spread"]
    #[inline(always)]
    #[must_use]
    pub fn downspread_en(&mut self) -> DownspreadEnW<CfgPll0SsCtrlSpec> {
        DownspreadEnW::new(self, 4)
    }
    #[doc = "Bit 15 - 15:15\\]
SSM reset. When set to 1 the SSM modulator is in reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CfgPll0SsCtrlSpec> {
        ResetW::new(self, 15)
    }
    #[doc = "Bits 18:25 - 25:18\\]
Wave table max address. Indicates the maximum number of address bits used to access the external wave table. These bits are not used if PLL_CFG_ssm_wvtbl = 0"]
    #[inline(always)]
    #[must_use]
    pub fn wv_tble_maxaddr(&mut self) -> WvTbleMaxaddrW<CfgPll0SsCtrlSpec> {
        WvTbleMaxaddrW::new(self, 18)
    }
    #[doc = "Bit 31 - 31:31\\]
Bypass the SS modulator. 1'b0 - Spread spectrum modulation is enabled 1'b1 - SSMOD is bypassed. No modulation of PLL frequency"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_en(&mut self) -> BypassEnW<CfgPll0SsCtrlSpec> {
        BypassEnW::new(self, 31)
    }
}
#[doc = "CFG_pll0_SS_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll0_ss_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll0_ss_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll0SsCtrlSpec;
impl crate::RegisterSpec for CfgPll0SsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll0_ss_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgPll0SsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll0_ss_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgPll0SsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll0_SS_CTRL to value 0x8000_0000"]
impl crate::Resettable for CfgPll0SsCtrlSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
