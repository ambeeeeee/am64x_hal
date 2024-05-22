#[doc = "Register `CFG_GPMC_CONFIG4` reader"]
pub type R = crate::R<CfgGpmcConfig4Spec>;
#[doc = "Register `CFG_GPMC_CONFIG4` writer"]
pub type W = crate::W<CfgGpmcConfig4Spec>;
#[doc = "Field `OEONTIME` reader - 3:0\\]
OE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type OeontimeR = crate::FieldReader;
#[doc = "Field `OEONTIME` writer - 3:0\\]
OE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type OeontimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OEAADMUXONTIME` reader - 6:4\\]
OE# assertion time for the first address phase in an AAD-Mux access"]
pub type OeaadmuxontimeR = crate::FieldReader;
#[doc = "Field `OEAADMUXONTIME` writer - 6:4\\]
OE# assertion time for the first address phase in an AAD-Mux access"]
pub type OeaadmuxontimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OEEXTRADELAY` reader - 7:7\\]
OE# Add Extra Half GPMC.FCLK cycle"]
pub type OeextradelayR = crate::BitReader;
#[doc = "Field `OEEXTRADELAY` writer - 7:7\\]
OE# Add Extra Half GPMC.FCLK cycle"]
pub type OeextradelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEOFFTIME` reader - 12:8\\]
OE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type OeofftimeR = crate::FieldReader;
#[doc = "Field `OEOFFTIME` writer - 12:8\\]
OE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type OeofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OEAADMUXOFFTIME` reader - 15:13\\]
OE# de-assertion time for the first address phase in an AAD-Mux access"]
pub type OeaadmuxofftimeR = crate::FieldReader;
#[doc = "Field `OEAADMUXOFFTIME` writer - 15:13\\]
OE# de-assertion time for the first address phase in an AAD-Mux access"]
pub type OeaadmuxofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WEONTIME` reader - 19:16\\]
WE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type WeontimeR = crate::FieldReader;
#[doc = "Field `WEONTIME` writer - 19:16\\]
WE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type WeontimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WEEXTRADELAY` reader - 23:23\\]
WE# Add Extra Half GPMC.FCLK cycle"]
pub type WeextradelayR = crate::BitReader;
#[doc = "Field `WEEXTRADELAY` writer - 23:23\\]
WE# Add Extra Half GPMC.FCLK cycle"]
pub type WeextradelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEOFFTIME` reader - 28:24\\]
WE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WeofftimeR = crate::FieldReader;
#[doc = "Field `WEOFFTIME` writer - 28:24\\]
WE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type WeofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
OE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn oeontime(&self) -> OeontimeR {
        OeontimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
OE# assertion time for the first address phase in an AAD-Mux access"]
    #[inline(always)]
    pub fn oeaadmuxontime(&self) -> OeaadmuxontimeR {
        OeaadmuxontimeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
OE# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    pub fn oeextradelay(&self) -> OeextradelayR {
        OeextradelayR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
OE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn oeofftime(&self) -> OeofftimeR {
        OeofftimeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
OE# de-assertion time for the first address phase in an AAD-Mux access"]
    #[inline(always)]
    pub fn oeaadmuxofftime(&self) -> OeaadmuxofftimeR {
        OeaadmuxofftimeR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
WE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn weontime(&self) -> WeontimeR {
        WeontimeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
WE# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    pub fn weextradelay(&self) -> WeextradelayR {
        WeextradelayR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
WE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn weofftime(&self) -> WeofftimeR {
        WeofftimeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
OE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn oeontime(&mut self) -> OeontimeW<CfgGpmcConfig4Spec> {
        OeontimeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
OE# assertion time for the first address phase in an AAD-Mux access"]
    #[inline(always)]
    #[must_use]
    pub fn oeaadmuxontime(&mut self) -> OeaadmuxontimeW<CfgGpmcConfig4Spec> {
        OeaadmuxontimeW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
OE# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    #[must_use]
    pub fn oeextradelay(&mut self) -> OeextradelayW<CfgGpmcConfig4Spec> {
        OeextradelayW::new(self, 7)
    }
    #[doc = "Bits 8:12 - 12:8\\]
OE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn oeofftime(&mut self) -> OeofftimeW<CfgGpmcConfig4Spec> {
        OeofftimeW::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
OE# de-assertion time for the first address phase in an AAD-Mux access"]
    #[inline(always)]
    #[must_use]
    pub fn oeaadmuxofftime(&mut self) -> OeaadmuxofftimeW<CfgGpmcConfig4Spec> {
        OeaadmuxofftimeW::new(self, 13)
    }
    #[doc = "Bits 16:19 - 19:16\\]
WE# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn weontime(&mut self) -> WeontimeW<CfgGpmcConfig4Spec> {
        WeontimeW::new(self, 16)
    }
    #[doc = "Bit 23 - 23:23\\]
WE# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    #[must_use]
    pub fn weextradelay(&mut self) -> WeextradelayW<CfgGpmcConfig4Spec> {
        WeextradelayW::new(self, 23)
    }
    #[doc = "Bits 24:28 - 28:24\\]
WE# de-assertion time from start cycle time \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn weofftime(&mut self) -> WeofftimeW<CfgGpmcConfig4Spec> {
        WeofftimeW::new(self, 24)
    }
}
#[doc = "WE# and OE# signals timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig4Spec;
impl crate::RegisterSpec for CfgGpmcConfig4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config4::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig4Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config4::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG4 to value 0x1605_7616"]
impl crate::Resettable for CfgGpmcConfig4Spec {
    const RESET_VALUE: u32 = 0x1605_7616;
}
