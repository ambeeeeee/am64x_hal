#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg2RegSpec>;
#[doc = "Field `TIMEOUTCLKFREQ` reader - 5:0\\]
Timeout Clock Frequency. Suggested Value is 1 KHz. Internally the 1msec Timer is used for Timeout Detection. The 1msec Timer is generated from the xin_clk."]
pub type TimeoutclkfreqR = crate::FieldReader;
#[doc = "Field `TIMEOUTCLKFREQ` writer - 5:0\\]
Timeout Clock Frequency. Suggested Value is 1 KHz. Internally the 1msec Timer is used for Timeout Detection. The 1msec Timer is generated from the xin_clk."]
pub type TimeoutclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TIMEOUTCLKUNIT` reader - 7:7\\]
Timeout Clock Unit. Suggested Value is 1'b0 (KHz)."]
pub type TimeoutclkunitR = crate::BitReader;
#[doc = "Field `TIMEOUTCLKUNIT` writer - 7:7\\]
Timeout Clock Unit. Suggested Value is 1'b0 (KHz)."]
pub type TimeoutclkunitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BASECLKFREQ` reader - 15:8\\]
Base Clock Frequency for SD Clock. This is the frequency of the xin_clk."]
pub type BaseclkfreqR = crate::FieldReader;
#[doc = "Field `BASECLKFREQ` writer - 15:8\\]
Base Clock Frequency for SD Clock. This is the frequency of the xin_clk."]
pub type BaseclkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAXBLKLENGTH` reader - 17:16\\]
Max Block Length. Maximum Block Length supported by the Core/Device. 00: 512 (Bytes), 01: 1024, 10: 2048, 11: Reserved."]
pub type MaxblklengthR = crate::FieldReader;
#[doc = "Field `MAXBLKLENGTH` writer - 17:16\\]
Max Block Length. Maximum Block Length supported by the Core/Device. 00: 512 (Bytes), 01: 1024, 10: 2048, 11: Reserved."]
pub type MaxblklengthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUPPORT8BIT` reader - 18:18\\]
8-bit Support for Embedded Device. Suggested Value is 1'b1 (The Core supports 8-bit Interface). Optionally an be set to 1'b0 if the Application supports only 4-bit SD Interface."]
pub type Support8bitR = crate::BitReader;
#[doc = "Field `SUPPORT8BIT` writer - 18:18\\]
8-bit Support for Embedded Device. Suggested Value is 1'b1 (The Core supports 8-bit Interface). Optionally an be set to 1'b0 if the Application supports only 4-bit SD Interface."]
pub type Support8bitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA2SUPPORT` reader - 19:19\\]
ADMA2 Support. Suggested Value is 1'b1 (The ADMA2 is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support ADMA2 Mode."]
pub type Adma2supportR = crate::BitReader;
#[doc = "Field `ADMA2SUPPORT` writer - 19:19\\]
ADMA2 Support. Suggested Value is 1'b1 (The ADMA2 is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support ADMA2 Mode."]
pub type Adma2supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHSPEEDSUPPORT` reader - 21:21\\]
High Speed Support. Suggested Value is 1'b1 (The High Speed mode is supported by Core)."]
pub type HighspeedsupportR = crate::BitReader;
#[doc = "Field `HIGHSPEEDSUPPORT` writer - 21:21\\]
High Speed Support. Suggested Value is 1'b1 (The High Speed mode is supported by Core)."]
pub type HighspeedsupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMASUPPORT` reader - 22:22\\]
SDMA Support. Suggested Value is 1'b1 (The SDMA is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support SDMA Mode."]
pub type SdmasupportR = crate::BitReader;
#[doc = "Field `SDMASUPPORT` writer - 22:22\\]
SDMA Support. Suggested Value is 1'b1 (The SDMA is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support SDMA Mode."]
pub type SdmasupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPRESSUPPORT` reader - 23:23\\]
Suspend/Resume Support. Suggested Value is 1'b1 (The Suspend/Resume is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support Suspend/Resume Mode."]
pub type SuspressupportR = crate::BitReader;
#[doc = "Field `SUSPRESSUPPORT` writer - 23:23\\]
Suspend/Resume Support. Suggested Value is 1'b1 (The Suspend/Resume is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support Suspend/Resume Mode."]
pub type SuspressupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT3P3VOLT` reader - 24:24\\]
3.3V Support. Suggested Value is 1'b1 as the 3.3 V is the default voltage on the SD Interface."]
pub type Support3p3voltR = crate::BitReader;
#[doc = "Field `SUPPORT3P3VOLT` writer - 24:24\\]
3.3V Support. Suggested Value is 1'b1 as the 3.3 V is the default voltage on the SD Interface."]
pub type Support3p3voltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT3P0VOLT` reader - 25:25\\]
3.0V Support. Should be set based on whether 3.0V is supported on the SD Interface."]
pub type Support3p0voltR = crate::BitReader;
#[doc = "Field `SUPPORT3P0VOLT` writer - 25:25\\]
3.0V Support. Should be set based on whether 3.0V is supported on the SD Interface."]
pub type Support3p0voltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT1P8VOLT` reader - 26:26\\]
1.8V Support. Suggested Value is 1'b1 (The 1.8 Volt Switching is supported by Core). Optionally can be set to 1'b0 if the application doesn't want 1.8V switching (SD3.0)."]
pub type Support1p8voltR = crate::BitReader;
#[doc = "Field `SUPPORT1P8VOLT` writer - 26:26\\]
1.8V Support. Suggested Value is 1'b1 (The 1.8 Volt Switching is supported by Core). Optionally can be set to 1'b0 if the application doesn't want 1.8V switching (SD3.0)."]
pub type Support1p8voltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASYNCHINTRSUPPORT` reader - 29:29\\]
Asynchronous Interrupt Support. Suggested Value is 1'b1 (The Core supports monitoring of Asynchronous Interrupt)."]
pub type AsynchintrsupportR = crate::BitReader;
#[doc = "Field `ASYNCHINTRSUPPORT` writer - 29:29\\]
Asynchronous Interrupt Support. Suggested Value is 1'b1 (The Core supports monitoring of Asynchronous Interrupt)."]
pub type AsynchintrsupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTTYPE` reader - 31:30\\]
Slot Type. Should be set based on the final product usage. 00 - Removable SCard Slot, 01 - Embedded Slot for One Device, 10 - Shared Bus Slot, 11 - Reserved."]
pub type SlottypeR = crate::FieldReader;
#[doc = "Field `SLOTTYPE` writer - 31:30\\]
Slot Type. Should be set based on the final product usage. 00 - Removable SCard Slot, 01 - Embedded Slot for One Device, 10 - Shared Bus Slot, 11 - Reserved."]
pub type SlottypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Timeout Clock Frequency. Suggested Value is 1 KHz. Internally the 1msec Timer is used for Timeout Detection. The 1msec Timer is generated from the xin_clk."]
    #[inline(always)]
    pub fn timeoutclkfreq(&self) -> TimeoutclkfreqR {
        TimeoutclkfreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Timeout Clock Unit. Suggested Value is 1'b0 (KHz)."]
    #[inline(always)]
    pub fn timeoutclkunit(&self) -> TimeoutclkunitR {
        TimeoutclkunitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Base Clock Frequency for SD Clock. This is the frequency of the xin_clk."]
    #[inline(always)]
    pub fn baseclkfreq(&self) -> BaseclkfreqR {
        BaseclkfreqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Max Block Length. Maximum Block Length supported by the Core/Device. 00: 512 (Bytes), 01: 1024, 10: 2048, 11: Reserved."]
    #[inline(always)]
    pub fn maxblklength(&self) -> MaxblklengthR {
        MaxblklengthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - 18:18\\]
8-bit Support for Embedded Device. Suggested Value is 1'b1 (The Core supports 8-bit Interface). Optionally an be set to 1'b0 if the Application supports only 4-bit SD Interface."]
    #[inline(always)]
    pub fn support8bit(&self) -> Support8bitR {
        Support8bitR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
ADMA2 Support. Suggested Value is 1'b1 (The ADMA2 is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support ADMA2 Mode."]
    #[inline(always)]
    pub fn adma2support(&self) -> Adma2supportR {
        Adma2supportR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
High Speed Support. Suggested Value is 1'b1 (The High Speed mode is supported by Core)."]
    #[inline(always)]
    pub fn highspeedsupport(&self) -> HighspeedsupportR {
        HighspeedsupportR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
SDMA Support. Suggested Value is 1'b1 (The SDMA is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support SDMA Mode."]
    #[inline(always)]
    pub fn sdmasupport(&self) -> SdmasupportR {
        SdmasupportR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Suspend/Resume Support. Suggested Value is 1'b1 (The Suspend/Resume is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support Suspend/Resume Mode."]
    #[inline(always)]
    pub fn suspressupport(&self) -> SuspressupportR {
        SuspressupportR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
3.3V Support. Suggested Value is 1'b1 as the 3.3 V is the default voltage on the SD Interface."]
    #[inline(always)]
    pub fn support3p3volt(&self) -> Support3p3voltR {
        Support3p3voltR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
3.0V Support. Should be set based on whether 3.0V is supported on the SD Interface."]
    #[inline(always)]
    pub fn support3p0volt(&self) -> Support3p0voltR {
        Support3p0voltR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
1.8V Support. Suggested Value is 1'b1 (The 1.8 Volt Switching is supported by Core). Optionally can be set to 1'b0 if the application doesn't want 1.8V switching (SD3.0)."]
    #[inline(always)]
    pub fn support1p8volt(&self) -> Support1p8voltR {
        Support1p8voltR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Asynchronous Interrupt Support. Suggested Value is 1'b1 (The Core supports monitoring of Asynchronous Interrupt)."]
    #[inline(always)]
    pub fn asynchintrsupport(&self) -> AsynchintrsupportR {
        AsynchintrsupportR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Slot Type. Should be set based on the final product usage. 00 - Removable SCard Slot, 01 - Embedded Slot for One Device, 10 - Shared Bus Slot, 11 - Reserved."]
    #[inline(always)]
    pub fn slottype(&self) -> SlottypeR {
        SlottypeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Timeout Clock Frequency. Suggested Value is 1 KHz. Internally the 1msec Timer is used for Timeout Detection. The 1msec Timer is generated from the xin_clk."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutclkfreq(&mut self) -> TimeoutclkfreqW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        TimeoutclkfreqW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Timeout Clock Unit. Suggested Value is 1'b0 (KHz)."]
    #[inline(always)]
    #[must_use]
    pub fn timeoutclkunit(&mut self) -> TimeoutclkunitW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        TimeoutclkunitW::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Base Clock Frequency for SD Clock. This is the frequency of the xin_clk."]
    #[inline(always)]
    #[must_use]
    pub fn baseclkfreq(&mut self) -> BaseclkfreqW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        BaseclkfreqW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Max Block Length. Maximum Block Length supported by the Core/Device. 00: 512 (Bytes), 01: 1024, 10: 2048, 11: Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn maxblklength(&mut self) -> MaxblklengthW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        MaxblklengthW::new(self, 16)
    }
    #[doc = "Bit 18 - 18:18\\]
8-bit Support for Embedded Device. Suggested Value is 1'b1 (The Core supports 8-bit Interface). Optionally an be set to 1'b0 if the Application supports only 4-bit SD Interface."]
    #[inline(always)]
    #[must_use]
    pub fn support8bit(&mut self) -> Support8bitW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        Support8bitW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
ADMA2 Support. Suggested Value is 1'b1 (The ADMA2 is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support ADMA2 Mode."]
    #[inline(always)]
    #[must_use]
    pub fn adma2support(&mut self) -> Adma2supportW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        Adma2supportW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
High Speed Support. Suggested Value is 1'b1 (The High Speed mode is supported by Core)."]
    #[inline(always)]
    #[must_use]
    pub fn highspeedsupport(&mut self) -> HighspeedsupportW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        HighspeedsupportW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
SDMA Support. Suggested Value is 1'b1 (The SDMA is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support SDMA Mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdmasupport(&mut self) -> SdmasupportW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        SdmasupportW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Suspend/Resume Support. Suggested Value is 1'b1 (The Suspend/Resume is supported by Core). Optionally can be set to 1'b0 if the application doesn't want to support Suspend/Resume Mode."]
    #[inline(always)]
    #[must_use]
    pub fn suspressupport(&mut self) -> SuspressupportW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        SuspressupportW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
3.3V Support. Suggested Value is 1'b1 as the 3.3 V is the default voltage on the SD Interface."]
    #[inline(always)]
    #[must_use]
    pub fn support3p3volt(&mut self) -> Support3p3voltW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        Support3p3voltW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
3.0V Support. Should be set based on whether 3.0V is supported on the SD Interface."]
    #[inline(always)]
    #[must_use]
    pub fn support3p0volt(&mut self) -> Support3p0voltW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        Support3p0voltW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
1.8V Support. Suggested Value is 1'b1 (The 1.8 Volt Switching is supported by Core). Optionally can be set to 1'b0 if the application doesn't want 1.8V switching (SD3.0)."]
    #[inline(always)]
    #[must_use]
    pub fn support1p8volt(&mut self) -> Support1p8voltW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        Support1p8voltW::new(self, 26)
    }
    #[doc = "Bit 29 - 29:29\\]
Asynchronous Interrupt Support. Suggested Value is 1'b1 (The Core supports monitoring of Asynchronous Interrupt)."]
    #[inline(always)]
    #[must_use]
    pub fn asynchintrsupport(&mut self) -> AsynchintrsupportW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        AsynchintrsupportW::new(self, 29)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Slot Type. Should be set based on the final product usage. 00 - Removable SCard Slot, 01 - Embedded Slot for One Device, 10 - Shared Bus Slot, 11 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn slottype(&mut self) -> SlottypeW<Regs_SsCfg_SscfgCtlCfg2RegSpec> {
        SlottypeW::new(self, 30)
    }
}
#[doc = "The Controller Config 2 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_2_REG to value 0x24ee_0001"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg2RegSpec {
    const RESET_VALUE: u32 = 0x24ee_0001;
}
