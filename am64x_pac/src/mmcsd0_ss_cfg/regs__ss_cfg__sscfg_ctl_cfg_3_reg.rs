#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_3_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg3RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_3_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg3RegSpec>;
#[doc = "Field `SDR50SUPPORT` reader - 0:0\\]
SDR50 Support. Suggested Value is 1'b1 (The Core supports SDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR50."]
pub type Sdr50supportR = crate::BitReader;
#[doc = "Field `SDR50SUPPORT` writer - 0:0\\]
SDR50 Support. Suggested Value is 1'b1 (The Core supports SDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR50."]
pub type Sdr50supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR104SUPPORT` reader - 1:1\\]
SDR104 Support. Suggested Value is 1'b1 (The Core supports SDR104 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR104."]
pub type Sdr104supportR = crate::BitReader;
#[doc = "Field `SDR104SUPPORT` writer - 1:1\\]
SDR104 Support. Suggested Value is 1'b1 (The Core supports SDR104 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR104."]
pub type Sdr104supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDR50SUPPORT` reader - 2:2\\]
DDR50 Support. Suggested Value is 1'b1 (The Core supports DDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support DDR50."]
pub type Ddr50supportR = crate::BitReader;
#[doc = "Field `DDR50SUPPORT` writer - 2:2\\]
DDR50 Support. Suggested Value is 1'b1 (The Core supports DDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support DDR50."]
pub type Ddr50supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADRIVERSUPPORT` reader - 4:4\\]
Driver Type A Support. This bit should be set based on whether Driver Type A for 1.8 Signalling is supported or not."]
pub type AdriversupportR = crate::BitReader;
#[doc = "Field `ADRIVERSUPPORT` writer - 4:4\\]
Driver Type A Support. This bit should be set based on whether Driver Type A for 1.8 Signalling is supported or not."]
pub type AdriversupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDRIVERSUPPORT` reader - 5:5\\]
Driver Type C Support. This bit should be set based on whether Driver Type C for 1.8 Signalling is supported or not."]
pub type CdriversupportR = crate::BitReader;
#[doc = "Field `CDRIVERSUPPORT` writer - 5:5\\]
Driver Type C Support. This bit should be set based on whether Driver Type C for 1.8 Signalling is supported or not."]
pub type CdriversupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRIVERSUPPORT` reader - 6:6\\]
Driver Type D Support. This bit should be set based on whether Driver Type D for 1.8 Signalling is supported or not."]
pub type DdriversupportR = crate::BitReader;
#[doc = "Field `DDRIVERSUPPORT` writer - 6:6\\]
Driver Type D Support. This bit should be set based on whether Driver Type D for 1.8 Signalling is supported or not."]
pub type DdriversupportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TYPE4SUPPORT` reader - 7:7\\]
Driver Type 4 Support. This bit should be set based on whether Driver Type 4 for 1.8 Signalling is supported or not."]
pub type Type4supportR = crate::BitReader;
#[doc = "Field `TYPE4SUPPORT` writer - 7:7\\]
Driver Type 4 Support. This bit should be set based on whether Driver Type 4 for 1.8 Signalling is supported or not."]
pub type Type4supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGTIMERCNT` reader - 11:8\\]
Timer Count for Re-Tuning. This is the Timer Count for Re-Tuning Timer for Re-Tuning Mode 1 to 3. Setting to 4'b0 disables Re-Tuning Timer."]
pub type RetuningtimercntR = crate::FieldReader;
#[doc = "Field `RETUNINGTIMERCNT` writer - 11:8\\]
Timer Count for Re-Tuning. This is the Timer Count for Re-Tuning Timer for Re-Tuning Mode 1 to 3. Setting to 4'b0 disables Re-Tuning Timer."]
pub type RetuningtimercntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TUNINGFORSDR50` reader - 13:13\\]
Use Tuning for SDR50. This bit should be set if the Application wants Tuning be used for SDR50 Modes. The Core operates with or with out tuning for SDR50 mode as long as the Clock can be manually tuned using tap delay."]
pub type Tuningforsdr50R = crate::BitReader;
#[doc = "Field `TUNINGFORSDR50` writer - 13:13\\]
Use Tuning for SDR50. This bit should be set if the Application wants Tuning be used for SDR50 Modes. The Core operates with or with out tuning for SDR50 mode as long as the Clock can be manually tuned using tap delay."]
pub type Tuningforsdr50W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNINGMODES` reader - 15:14\\]
Re-Tuning Modes. Should be set to 2'b00 as the Core supports only the Software Timer based Re-Tuning."]
pub type RetuningmodesR = crate::FieldReader;
#[doc = "Field `RETUNINGMODES` writer - 15:14\\]
Re-Tuning Modes. Should be set to 2'b00 as the Core supports only the Software Timer based Re-Tuning."]
pub type RetuningmodesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLOCKMULTIPLIER` reader - 23:16\\]
Clock Multiplier. This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. FFh Clock Multiplier M = 256, ...., 02h Clock Multiplier M = 3, 01h Clock Multiplier M = 2, 00h Clock Multiplier is Not Supported."]
pub type ClockmultiplierR = crate::FieldReader;
#[doc = "Field `CLOCKMULTIPLIER` writer - 23:16\\]
Clock Multiplier. This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. FFh Clock Multiplier M = 256, ...., 02h Clock Multiplier M = 3, 01h Clock Multiplier M = 2, 00h Clock Multiplier is Not Supported."]
pub type ClockmultiplierW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADMA3SUPPORT` reader - 27:27\\]
ADMA3 Support."]
pub type Adma3supportR = crate::BitReader;
#[doc = "Field `ADMA3SUPPORT` writer - 27:27\\]
ADMA3 Support."]
pub type Adma3supportW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPPORT1P8VDD2` reader - 28:28\\]
1.8V VDD2 Support."]
pub type Support1p8vdd2R = crate::BitReader;
#[doc = "Field `SUPPORT1P8VDD2` writer - 28:28\\]
1.8V VDD2 Support."]
pub type Support1p8vdd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS400SUPPORT` reader - 31:31\\]
HS400 Support. Suggested Value is 1'b1 (The Core supports HS400 Mode). This applies only to eMMC5.0 mode. This should be set to 1'b0 for SD3.0 mode Optionally can be set to 1'b0 if the application doesn't want to support HS400."]
pub type Hs400supportR = crate::BitReader;
#[doc = "Field `HS400SUPPORT` writer - 31:31\\]
HS400 Support. Suggested Value is 1'b1 (The Core supports HS400 Mode). This applies only to eMMC5.0 mode. This should be set to 1'b0 for SD3.0 mode Optionally can be set to 1'b0 if the application doesn't want to support HS400."]
pub type Hs400supportW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SDR50 Support. Suggested Value is 1'b1 (The Core supports SDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR50."]
    #[inline(always)]
    pub fn sdr50support(&self) -> Sdr50supportR {
        Sdr50supportR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SDR104 Support. Suggested Value is 1'b1 (The Core supports SDR104 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR104."]
    #[inline(always)]
    pub fn sdr104support(&self) -> Sdr104supportR {
        Sdr104supportR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
DDR50 Support. Suggested Value is 1'b1 (The Core supports DDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support DDR50."]
    #[inline(always)]
    pub fn ddr50support(&self) -> Ddr50supportR {
        Ddr50supportR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Driver Type A Support. This bit should be set based on whether Driver Type A for 1.8 Signalling is supported or not."]
    #[inline(always)]
    pub fn adriversupport(&self) -> AdriversupportR {
        AdriversupportR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Driver Type C Support. This bit should be set based on whether Driver Type C for 1.8 Signalling is supported or not."]
    #[inline(always)]
    pub fn cdriversupport(&self) -> CdriversupportR {
        CdriversupportR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Driver Type D Support. This bit should be set based on whether Driver Type D for 1.8 Signalling is supported or not."]
    #[inline(always)]
    pub fn ddriversupport(&self) -> DdriversupportR {
        DdriversupportR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Driver Type 4 Support. This bit should be set based on whether Driver Type 4 for 1.8 Signalling is supported or not."]
    #[inline(always)]
    pub fn type4support(&self) -> Type4supportR {
        Type4supportR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Timer Count for Re-Tuning. This is the Timer Count for Re-Tuning Timer for Re-Tuning Mode 1 to 3. Setting to 4'b0 disables Re-Tuning Timer."]
    #[inline(always)]
    pub fn retuningtimercnt(&self) -> RetuningtimercntR {
        RetuningtimercntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Use Tuning for SDR50. This bit should be set if the Application wants Tuning be used for SDR50 Modes. The Core operates with or with out tuning for SDR50 mode as long as the Clock can be manually tuned using tap delay."]
    #[inline(always)]
    pub fn tuningforsdr50(&self) -> Tuningforsdr50R {
        Tuningforsdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Re-Tuning Modes. Should be set to 2'b00 as the Core supports only the Software Timer based Re-Tuning."]
    #[inline(always)]
    pub fn retuningmodes(&self) -> RetuningmodesR {
        RetuningmodesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock Multiplier. This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. FFh Clock Multiplier M = 256, ...., 02h Clock Multiplier M = 3, 01h Clock Multiplier M = 2, 00h Clock Multiplier is Not Supported."]
    #[inline(always)]
    pub fn clockmultiplier(&self) -> ClockmultiplierR {
        ClockmultiplierR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
ADMA3 Support."]
    #[inline(always)]
    pub fn adma3support(&self) -> Adma3supportR {
        Adma3supportR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
1.8V VDD2 Support."]
    #[inline(always)]
    pub fn support1p8vdd2(&self) -> Support1p8vdd2R {
        Support1p8vdd2R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
HS400 Support. Suggested Value is 1'b1 (The Core supports HS400 Mode). This applies only to eMMC5.0 mode. This should be set to 1'b0 for SD3.0 mode Optionally can be set to 1'b0 if the application doesn't want to support HS400."]
    #[inline(always)]
    pub fn hs400support(&self) -> Hs400supportR {
        Hs400supportR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SDR50 Support. Suggested Value is 1'b1 (The Core supports SDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR50."]
    #[inline(always)]
    #[must_use]
    pub fn sdr50support(&mut self) -> Sdr50supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Sdr50supportW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SDR104 Support. Suggested Value is 1'b1 (The Core supports SDR104 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support SDR104."]
    #[inline(always)]
    #[must_use]
    pub fn sdr104support(&mut self) -> Sdr104supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Sdr104supportW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
DDR50 Support. Suggested Value is 1'b1 (The Core supports DDR50 mode of operation). Optionally can be set to 1'b0 if the application doesn't want to support DDR50."]
    #[inline(always)]
    #[must_use]
    pub fn ddr50support(&mut self) -> Ddr50supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Ddr50supportW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
Driver Type A Support. This bit should be set based on whether Driver Type A for 1.8 Signalling is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn adriversupport(&mut self) -> AdriversupportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        AdriversupportW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Driver Type C Support. This bit should be set based on whether Driver Type C for 1.8 Signalling is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn cdriversupport(&mut self) -> CdriversupportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        CdriversupportW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Driver Type D Support. This bit should be set based on whether Driver Type D for 1.8 Signalling is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn ddriversupport(&mut self) -> DdriversupportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        DdriversupportW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Driver Type 4 Support. This bit should be set based on whether Driver Type 4 for 1.8 Signalling is supported or not."]
    #[inline(always)]
    #[must_use]
    pub fn type4support(&mut self) -> Type4supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Type4supportW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Timer Count for Re-Tuning. This is the Timer Count for Re-Tuning Timer for Re-Tuning Mode 1 to 3. Setting to 4'b0 disables Re-Tuning Timer."]
    #[inline(always)]
    #[must_use]
    pub fn retuningtimercnt(&mut self) -> RetuningtimercntW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        RetuningtimercntW::new(self, 8)
    }
    #[doc = "Bit 13 - 13:13\\]
Use Tuning for SDR50. This bit should be set if the Application wants Tuning be used for SDR50 Modes. The Core operates with or with out tuning for SDR50 mode as long as the Clock can be manually tuned using tap delay."]
    #[inline(always)]
    #[must_use]
    pub fn tuningforsdr50(&mut self) -> Tuningforsdr50W<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Tuningforsdr50W::new(self, 13)
    }
    #[doc = "Bits 14:15 - 15:14\\]
Re-Tuning Modes. Should be set to 2'b00 as the Core supports only the Software Timer based Re-Tuning."]
    #[inline(always)]
    #[must_use]
    pub fn retuningmodes(&mut self) -> RetuningmodesW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        RetuningmodesW::new(self, 14)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Clock Multiplier. This field indicates clock multiplier value of programmable clock generator. Refer to Clock Control register. Setting 00h means that Host Controller does not support programmable clock generator. FFh Clock Multiplier M = 256, ...., 02h Clock Multiplier M = 3, 01h Clock Multiplier M = 2, 00h Clock Multiplier is Not Supported."]
    #[inline(always)]
    #[must_use]
    pub fn clockmultiplier(&mut self) -> ClockmultiplierW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        ClockmultiplierW::new(self, 16)
    }
    #[doc = "Bit 27 - 27:27\\]
ADMA3 Support."]
    #[inline(always)]
    #[must_use]
    pub fn adma3support(&mut self) -> Adma3supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Adma3supportW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
1.8V VDD2 Support."]
    #[inline(always)]
    #[must_use]
    pub fn support1p8vdd2(&mut self) -> Support1p8vdd2W<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Support1p8vdd2W::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
HS400 Support. Suggested Value is 1'b1 (The Core supports HS400 Mode). This applies only to eMMC5.0 mode. This should be set to 1'b0 for SD3.0 mode Optionally can be set to 1'b0 if the application doesn't want to support HS400."]
    #[inline(always)]
    #[must_use]
    pub fn hs400support(&mut self) -> Hs400supportW<Regs_SsCfg_SscfgCtlCfg3RegSpec> {
        Hs400supportW::new(self, 31)
    }
}
#[doc = "The Controller Config 3 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg3RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_3_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_3_REG to value 0x9800_0407"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg3RegSpec {
    const RESET_VALUE: u32 = 0x9800_0407;
}
