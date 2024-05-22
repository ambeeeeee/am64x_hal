#[doc = "Register `MCRC64_REGS_CRC_INTR` reader"]
pub type R = crate::R<Mcrc64RegsCrcIntrSpec>;
#[doc = "Register `MCRC64_REGS_CRC_INTR` writer"]
pub type W = crate::W<Mcrc64RegsCrcIntrSpec>;
#[doc = "Field `CH1_CCITENR` reader - 0:0\\]
Channel 1 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch1CcitenrR = crate::BitReader;
#[doc = "Field `CH1_CCITENR` writer - 0:0\\]
Channel 1 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch1CcitenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRC_FAILENR` reader - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcFailenrR = crate::BitReader;
#[doc = "Field `CH1_CRC_FAILENR` writer - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcFailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVERENR` reader - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverenrR = crate::BitReader;
#[doc = "Field `CH1_OVERENR` writer - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDERENR` reader - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderenrR = crate::BitReader;
#[doc = "Field `CH1_UNDERENR` writer - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIME_OUT_ENR_` reader - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeOutEnr_R = crate::BitReader;
#[doc = "Field `CH1_TIME_OUT_ENR_` writer - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeOutEnr_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CCITENR` reader - 8:8\\]
Channel 2 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch2CcitenrR = crate::BitReader;
#[doc = "Field `CH2_CCITENR` writer - 8:8\\]
Channel 2 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch2CcitenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CRC_FAILENR` reader - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcFailenrR = crate::BitReader;
#[doc = "Field `CH2_CRC_FAILENR` writer - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcFailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVERENR` reader - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverenrR = crate::BitReader;
#[doc = "Field `CH2_OVERENR` writer - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDERENR` reader - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderenrR = crate::BitReader;
#[doc = "Field `CH2_UNDERENR` writer - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIME_OUT_ENR_` reader - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeOutEnr_R = crate::BitReader;
#[doc = "Field `CH2_TIME_OUT_ENR_` writer - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeOutEnr_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CCITENR` reader - 16:16\\]
Channel 3 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch3CcitenrR = crate::BitReader;
#[doc = "Field `CH3_CCITENR` writer - 16:16\\]
Channel 3 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch3CcitenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CRC_FAILENR` reader - 17:17\\]
Channel 3 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch3CrcFailenrR = crate::BitReader;
#[doc = "Field `CH3_CRC_FAILENR` writer - 17:17\\]
Channel 3 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch3CrcFailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_OVERENR` reader - 18:18\\]
Channel 3 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch3OverenrR = crate::BitReader;
#[doc = "Field `CH3_OVERENR` writer - 18:18\\]
Channel 3 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch3OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_UNDERENR` reader - 19:19\\]
Channel 3 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch3UnderenrR = crate::BitReader;
#[doc = "Field `CH3_UNDERENR` writer - 19:19\\]
Channel 3 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch3UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TIME_OUT_ENR_` reader - 20:20\\]
Channel 3 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch3TimeOutEnr_R = crate::BitReader;
#[doc = "Field `CH3_TIME_OUT_ENR_` writer - 20:20\\]
Channel 3 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch3TimeOutEnr_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CCITENR` reader - 24:24\\]
Channel 4 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch4CcitenrR = crate::BitReader;
#[doc = "Field `CH4_CCITENR` writer - 24:24\\]
Channel 4 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
pub type Ch4CcitenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CRC_FAILENR` reader - 25:25\\]
Channel 4 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch4CrcFailenrR = crate::BitReader;
#[doc = "Field `CH4_CRC_FAILENR` writer - 25:25\\]
Channel 4 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch4CrcFailenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_OVERENR` reader - 26:26\\]
Channel 4 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch4OverenrR = crate::BitReader;
#[doc = "Field `CH4_OVERENR` writer - 26:26\\]
Channel 4 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch4OverenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_UNDERENR` reader - 27:27\\]
Channel 4 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch4UnderenrR = crate::BitReader;
#[doc = "Field `CH4_UNDERENR` writer - 27:27\\]
Channel 4 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch4UnderenrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_TIME_OUT_ENR` reader - 28:28\\]
Channel 4 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch4TimeOutEnrR = crate::BitReader;
#[doc = "Field `CH4_TIME_OUT_ENR` writer - 28:28\\]
Channel 4 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch4TimeOutEnrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    pub fn ch1_ccitenr(&self) -> Ch1CcitenrR {
        Ch1CcitenrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch1_crc_failenr(&self) -> Ch1CrcFailenrR {
        Ch1CrcFailenrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_overenr(&self) -> Ch1OverenrR {
        Ch1OverenrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_underenr(&self) -> Ch1UnderenrR {
        Ch1UnderenrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch1_time_out_enr_(&self) -> Ch1TimeOutEnr_R {
        Ch1TimeOutEnr_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    pub fn ch2_ccitenr(&self) -> Ch2CcitenrR {
        Ch2CcitenrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch2_crc_failenr(&self) -> Ch2CrcFailenrR {
        Ch2CrcFailenrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_overenr(&self) -> Ch2OverenrR {
        Ch2OverenrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_underenr(&self) -> Ch2UnderenrR {
        Ch2UnderenrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch2_time_out_enr_(&self) -> Ch2TimeOutEnr_R {
        Ch2TimeOutEnr_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    pub fn ch3_ccitenr(&self) -> Ch3CcitenrR {
        Ch3CcitenrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch3_crc_failenr(&self) -> Ch3CrcFailenrR {
        Ch3CrcFailenrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch3_overenr(&self) -> Ch3OverenrR {
        Ch3OverenrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch3_underenr(&self) -> Ch3UnderenrR {
        Ch3UnderenrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch3_time_out_enr_(&self) -> Ch3TimeOutEnr_R {
        Ch3TimeOutEnr_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    pub fn ch4_ccitenr(&self) -> Ch4CcitenrR {
        Ch4CcitenrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch4_crc_failenr(&self) -> Ch4CrcFailenrR {
        Ch4CrcFailenrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch4_overenr(&self) -> Ch4OverenrR {
        Ch4OverenrR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch4_underenr(&self) -> Ch4UnderenrR {
        Ch4UnderenrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch4_time_out_enr(&self) -> Ch4TimeOutEnrR {
        Ch4TimeOutEnrR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_ccitenr(&mut self) -> Ch1CcitenrW<Mcrc64RegsCrcIntrSpec> {
        Ch1CcitenrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crc_failenr(&mut self) -> Ch1CrcFailenrW<Mcrc64RegsCrcIntrSpec> {
        Ch1CrcFailenrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_overenr(&mut self) -> Ch1OverenrW<Mcrc64RegsCrcIntrSpec> {
        Ch1OverenrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_underenr(&mut self) -> Ch1UnderenrW<Mcrc64RegsCrcIntrSpec> {
        Ch1UnderenrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_time_out_enr_(&mut self) -> Ch1TimeOutEnr_W<Mcrc64RegsCrcIntrSpec> {
        Ch1TimeOutEnr_W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ccitenr(&mut self) -> Ch2CcitenrW<Mcrc64RegsCrcIntrSpec> {
        Ch2CcitenrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crc_failenr(&mut self) -> Ch2CrcFailenrW<Mcrc64RegsCrcIntrSpec> {
        Ch2CrcFailenrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_overenr(&mut self) -> Ch2OverenrW<Mcrc64RegsCrcIntrSpec> {
        Ch2OverenrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_underenr(&mut self) -> Ch2UnderenrW<Mcrc64RegsCrcIntrSpec> {
        Ch2UnderenrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_time_out_enr_(&mut self) -> Ch2TimeOutEnr_W<Mcrc64RegsCrcIntrSpec> {
        Ch2TimeOutEnr_W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_ccitenr(&mut self) -> Ch3CcitenrW<Mcrc64RegsCrcIntrSpec> {
        Ch3CcitenrW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_crc_failenr(&mut self) -> Ch3CrcFailenrW<Mcrc64RegsCrcIntrSpec> {
        Ch3CrcFailenrW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_overenr(&mut self) -> Ch3OverenrW<Mcrc64RegsCrcIntrSpec> {
        Ch3OverenrW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_underenr(&mut self) -> Ch3UnderenrW<Mcrc64RegsCrcIntrSpec> {
        Ch3UnderenrW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_time_out_enr_(&mut self) -> Ch3TimeOutEnr_W<Mcrc64RegsCrcIntrSpec> {
        Ch3TimeOutEnr_W::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 Compression Complete Interrupt Disable Bit Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable) User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt disable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_ccitenr(&mut self) -> Ch4CcitenrW<Mcrc64RegsCrcIntrSpec> {
        Ch4CcitenrW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Fail Interrupt Disable Bit. Writing a one to this bit disable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_crc_failenr(&mut self) -> Ch4CrcFailenrW<Mcrc64RegsCrcIntrSpec> {
        Ch4CrcFailenrW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 Overrun Interrupt Disable Bit Writing a one to this bit disable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_overenr(&mut self) -> Ch4OverenrW<Mcrc64RegsCrcIntrSpec> {
        Ch4OverenrW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 Underrun Interrupt Disable Bit Writing a one to this bit disable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_underenr(&mut self) -> Ch4UnderenrW<Mcrc64RegsCrcIntrSpec> {
        Ch4UnderenrW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 Timeout Interrupt Disable Bit Writing a one to this bit disable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_time_out_enr(&mut self) -> Ch4TimeOutEnrW<Mcrc64RegsCrcIntrSpec> {
        Ch4TimeOutEnrW::new(self, 28)
    }
}
#[doc = "CRC Interrupt Enable Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_intr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_intr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcIntrSpec;
impl crate::RegisterSpec for Mcrc64RegsCrcIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_intr::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_intr::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_INTR to value 0"]
impl crate::Resettable for Mcrc64RegsCrcIntrSpec {
    const RESET_VALUE: u32 = 0;
}
