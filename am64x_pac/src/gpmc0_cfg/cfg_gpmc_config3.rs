#[doc = "Register `CFG_GPMC_CONFIG3` reader"]
pub type R = crate::R<CfgGpmcConfig3Spec>;
#[doc = "Register `CFG_GPMC_CONFIG3` writer"]
pub type W = crate::W<CfgGpmcConfig3Spec>;
#[doc = "Field `ADVONTIME` reader - 3:0\\]
ADV# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type AdvontimeR = crate::FieldReader;
#[doc = "Field `ADVONTIME` writer - 3:0\\]
ADV# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
pub type AdvontimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADVAADMUXONTIME` reader - 6:4\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxontimeR = crate::FieldReader;
#[doc = "Field `ADVAADMUXONTIME` writer - 6:4\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxontimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADVEXTRADELAY` reader - 7:7\\]
ADV# Add Extra Half GPMC.FCLK cycle"]
pub type AdvextradelayR = crate::BitReader;
#[doc = "Field `ADVEXTRADELAY` writer - 7:7\\]
ADV# Add Extra Half GPMC.FCLK cycle"]
pub type AdvextradelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVRDOFFTIME` reader - 12:8\\]
ADV# de-assertion time from start cycle time for read accesses\\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type AdvrdofftimeR = crate::FieldReader;
#[doc = "Field `ADVRDOFFTIME` writer - 12:8\\]
ADV# de-assertion time from start cycle time for read accesses\\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type AdvrdofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADVWROFFTIME` reader - 20:16\\]
ADV# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type AdvwrofftimeR = crate::FieldReader;
#[doc = "Field `ADVWROFFTIME` writer - 20:16\\]
ADV# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
pub type AdvwrofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADVAADMUXRDOFFTIME` reader - 26:24\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxrdofftimeR = crate::FieldReader;
#[doc = "Field `ADVAADMUXRDOFFTIME` writer - 26:24\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxrdofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED_0` reader - 27:27\\]
Write 0's for future compatibility. Read returns 0"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED_0` writer - 27:27\\]
Write 0's for future compatibility. Read returns 0"]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVAADMUXWROFFTIME` reader - 30:28\\]
ADV# de-assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxwrofftimeR = crate::FieldReader;
#[doc = "Field `ADVAADMUXWROFFTIME` writer - 30:28\\]
ADV# de-assertion for first address phase when using the AAD-Mux protocol"]
pub type AdvaadmuxwrofftimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED_1` reader - 31:31\\]
Write 0's for future compatibility. Read returns 0"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED_1` writer - 31:31\\]
Write 0's for future compatibility. Read returns 0"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
ADV# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn advontime(&self) -> AdvontimeR {
        AdvontimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    pub fn advaadmuxontime(&self) -> AdvaadmuxontimeR {
        AdvaadmuxontimeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
ADV# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    pub fn advextradelay(&self) -> AdvextradelayR {
        AdvextradelayR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ADV# de-assertion time from start cycle time for read accesses\\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn advrdofftime(&self) -> AdvrdofftimeR {
        AdvrdofftimeR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
ADV# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    pub fn advwrofftime(&self) -> AdvwrofftimeR {
        AdvwrofftimeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    pub fn advaadmuxrdofftime(&self) -> AdvaadmuxrdofftimeR {
        AdvaadmuxrdofftimeR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Write 0's for future compatibility. Read returns 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
ADV# de-assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    pub fn advaadmuxwrofftime(&self) -> AdvaadmuxwrofftimeR {
        AdvaadmuxwrofftimeR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Write 0's for future compatibility. Read returns 0"]
    #[inline(always)]
    pub fn reserved_1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
ADV# assertion time from start cycle time \\[0x0 corresponds to 0 GPMC.FCLK cycle, 0x1 corresponds to 1 GPMC.FCLK cycle, &amp;, 0xF corresponds to 15 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn advontime(&mut self) -> AdvontimeW<CfgGpmcConfig3Spec> {
        AdvontimeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    #[must_use]
    pub fn advaadmuxontime(&mut self) -> AdvaadmuxontimeW<CfgGpmcConfig3Spec> {
        AdvaadmuxontimeW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
ADV# Add Extra Half GPMC.FCLK cycle"]
    #[inline(always)]
    #[must_use]
    pub fn advextradelay(&mut self) -> AdvextradelayW<CfgGpmcConfig3Spec> {
        AdvextradelayW::new(self, 7)
    }
    #[doc = "Bits 8:12 - 12:8\\]
ADV# de-assertion time from start cycle time for read accesses\\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn advrdofftime(&mut self) -> AdvrdofftimeW<CfgGpmcConfig3Spec> {
        AdvrdofftimeW::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
ADV# de-assertion time from start cycle time for write accesses \\[0x00 corresponds to 0 GPMC.FCLK cycle, 0x01 corresponds to 1 GPMC.FCLK cycle, &amp;, 0x1F corresponds to 31 GPMC.FCLK cycles\\]"]
    #[inline(always)]
    #[must_use]
    pub fn advwrofftime(&mut self) -> AdvwrofftimeW<CfgGpmcConfig3Spec> {
        AdvwrofftimeW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
ADV# assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    #[must_use]
    pub fn advaadmuxrdofftime(&mut self) -> AdvaadmuxrdofftimeW<CfgGpmcConfig3Spec> {
        AdvaadmuxrdofftimeW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Write 0's for future compatibility. Read returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_0(&mut self) -> Reserved0W<CfgGpmcConfig3Spec> {
        Reserved0W::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
ADV# de-assertion for first address phase when using the AAD-Mux protocol"]
    #[inline(always)]
    #[must_use]
    pub fn advaadmuxwrofftime(&mut self) -> AdvaadmuxwrofftimeW<CfgGpmcConfig3Spec> {
        AdvaadmuxwrofftimeW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Write 0's for future compatibility. Read returns 0"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_1(&mut self) -> Reserved1W<CfgGpmcConfig3Spec> {
        Reserved1W::new(self, 31)
    }
}
#[doc = "ADV# signal timing parameter configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_config3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_config3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcConfig3Spec;
impl crate::RegisterSpec for CfgGpmcConfig3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_config3::R`](R) reader structure"]
impl crate::Readable for CfgGpmcConfig3Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_config3::W`](W) writer structure"]
impl crate::Writable for CfgGpmcConfig3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_CONFIG3 to value 0x2206_0514"]
impl crate::Resettable for CfgGpmcConfig3Spec {
    const RESET_VALUE: u32 = 0x2206_0514;
}
