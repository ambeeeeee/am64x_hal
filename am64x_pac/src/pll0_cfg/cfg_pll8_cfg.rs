#[doc = "Register `CFG_pll8_CFG` reader"]
pub type R = crate::R<CfgPll8CfgSpec>;
#[doc = "Register `CFG_pll8_CFG` writer"]
pub type W = crate::W<CfgPll8CfgSpec>;
#[doc = "Field `PLL_TYPE` reader - 1:0\\]
Indicates PLL type Field values (Others are reserved): 2'b00 - Fractional PLL 2'b01 - FractionalF PLL 2'b10 - De-Skew PLL"]
pub type PllTypeR = crate::FieldReader;
#[doc = "Field `PLL_TYPE` writer - 1:0\\]
Indicates PLL type Field values (Others are reserved): 2'b00 - Fractional PLL 2'b01 - FractionalF PLL 2'b10 - De-Skew PLL"]
pub type PllTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSM_WVTBL` reader - 8:8\\]
Spread spectrum wave table presence 1'b0 - SSM Wave table is not present 1'b1 - SSM Wave table is present"]
pub type SsmWvtblR = crate::BitReader;
#[doc = "Field `SSM_WVTBL` writer - 8:8\\]
Spread spectrum wave table presence 1'b0 - SSM Wave table is not present 1'b1 - SSM Wave table is present"]
pub type SsmWvtblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM_TYPE` reader - 12:11\\]
Spread spectrum module presence Field values (Others are reserved): 2'b00 - SSM is not present 2'b01 - SSM is present 2'b10 - Reserved 2'b11 - Reserved"]
pub type SsmTypeR = crate::FieldReader;
#[doc = "Field `SSM_TYPE` writer - 12:11\\]
Spread spectrum module presence Field values (Others are reserved): 2'b00 - SSM is not present 2'b01 - SSM is present 2'b10 - Reserved 2'b11 - Reserved"]
pub type SsmTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSDIV_PRSNT` reader - 31:16\\]
High Speed Divider Presence Each bit Indicates the presence of High Speed dividers 0-15. By definition HSDIV\\[4:0\\]
are connected the PLL FOUTVCO output clock and HSDIV\\[15:5\\]
are connected to the PLL FOUTPOSTDIV output clock"]
pub type HsdivPrsntR = crate::FieldReader<u16>;
#[doc = "Field `HSDIV_PRSNT` writer - 31:16\\]
High Speed Divider Presence Each bit Indicates the presence of High Speed dividers 0-15. By definition HSDIV\\[4:0\\]
are connected the PLL FOUTVCO output clock and HSDIV\\[15:5\\]
are connected to the PLL FOUTPOSTDIV output clock"]
pub type HsdivPrsntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates PLL type Field values (Others are reserved): 2'b00 - Fractional PLL 2'b01 - FractionalF PLL 2'b10 - De-Skew PLL"]
    #[inline(always)]
    pub fn pll_type(&self) -> PllTypeR {
        PllTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Spread spectrum wave table presence 1'b0 - SSM Wave table is not present 1'b1 - SSM Wave table is present"]
    #[inline(always)]
    pub fn ssm_wvtbl(&self) -> SsmWvtblR {
        SsmWvtblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Spread spectrum module presence Field values (Others are reserved): 2'b00 - SSM is not present 2'b01 - SSM is present 2'b10 - Reserved 2'b11 - Reserved"]
    #[inline(always)]
    pub fn ssm_type(&self) -> SsmTypeR {
        SsmTypeR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
High Speed Divider Presence Each bit Indicates the presence of High Speed dividers 0-15. By definition HSDIV\\[4:0\\]
are connected the PLL FOUTVCO output clock and HSDIV\\[15:5\\]
are connected to the PLL FOUTPOSTDIV output clock"]
    #[inline(always)]
    pub fn hsdiv_prsnt(&self) -> HsdivPrsntR {
        HsdivPrsntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Indicates PLL type Field values (Others are reserved): 2'b00 - Fractional PLL 2'b01 - FractionalF PLL 2'b10 - De-Skew PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_type(&mut self) -> PllTypeW<CfgPll8CfgSpec> {
        PllTypeW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Spread spectrum wave table presence 1'b0 - SSM Wave table is not present 1'b1 - SSM Wave table is present"]
    #[inline(always)]
    #[must_use]
    pub fn ssm_wvtbl(&mut self) -> SsmWvtblW<CfgPll8CfgSpec> {
        SsmWvtblW::new(self, 8)
    }
    #[doc = "Bits 11:12 - 12:11\\]
Spread spectrum module presence Field values (Others are reserved): 2'b00 - SSM is not present 2'b01 - SSM is present 2'b10 - Reserved 2'b11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ssm_type(&mut self) -> SsmTypeW<CfgPll8CfgSpec> {
        SsmTypeW::new(self, 11)
    }
    #[doc = "Bits 16:31 - 31:16\\]
High Speed Divider Presence Each bit Indicates the presence of High Speed dividers 0-15. By definition HSDIV\\[4:0\\]
are connected the PLL FOUTVCO output clock and HSDIV\\[15:5\\]
are connected to the PLL FOUTPOSTDIV output clock"]
    #[inline(always)]
    #[must_use]
    pub fn hsdiv_prsnt(&mut self) -> HsdivPrsntW<CfgPll8CfgSpec> {
        HsdivPrsntW::new(self, 16)
    }
}
#[doc = "CFG_pll8_CFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_pll8_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_pll8_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgPll8CfgSpec;
impl crate::RegisterSpec for CfgPll8CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_pll8_cfg::R`](R) reader structure"]
impl crate::Readable for CfgPll8CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_pll8_cfg::W`](W) writer structure"]
impl crate::Writable for CfgPll8CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_pll8_CFG to value 0x0001_0801"]
impl crate::Resettable for CfgPll8CfgSpec {
    const RESET_VALUE: u32 = 0x0001_0801;
}
