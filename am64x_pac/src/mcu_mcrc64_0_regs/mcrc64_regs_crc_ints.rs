#[doc = "Register `MCRC64_REGS_CRC_INTS` reader"]
pub type R = crate::R<Mcrc64RegsCrcIntsSpec>;
#[doc = "Register `MCRC64_REGS_CRC_INTS` writer"]
pub type W = crate::W<Mcrc64RegsCrcIntsSpec>;
#[doc = "Field `CH1_CCITENS` reader - 0:0\\]
Channel 1 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch1CcitensR = crate::BitReader;
#[doc = "Field `CH1_CCITENS` writer - 0:0\\]
Channel 1 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch1CcitensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CRC_FAILENS` reader - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcFailensR = crate::BitReader;
#[doc = "Field `CH1_CRC_FAILENS` writer - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch1CrcFailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVERENS` reader - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverensR = crate::BitReader;
#[doc = "Field `CH1_OVERENS` writer - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch1OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_UNDERENS` reader - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderensR = crate::BitReader;
#[doc = "Field `CH1_UNDERENS` writer - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch1UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TIME_OUT_ENS_` reader - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeOutEns_R = crate::BitReader;
#[doc = "Field `CH1_TIME_OUT_ENS_` writer - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch1TimeOutEns_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CCITENS` reader - 8:8\\]
Channel 2 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch2CcitensR = crate::BitReader;
#[doc = "Field `CH2_CCITENS` writer - 8:8\\]
Channel 2 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch2CcitensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_CRC_FAILENS` reader - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcFailensR = crate::BitReader;
#[doc = "Field `CH2_CRC_FAILENS` writer - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch2CrcFailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_OVERENS` reader - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverensR = crate::BitReader;
#[doc = "Field `CH2_OVERENS` writer - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch2OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_UNDERENS` reader - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderensR = crate::BitReader;
#[doc = "Field `CH2_UNDERENS` writer - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch2UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TIME_OUT_ENS_` reader - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeOutEns_R = crate::BitReader;
#[doc = "Field `CH2_TIME_OUT_ENS_` writer - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch2TimeOutEns_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CCITENS` reader - 16:16\\]
Channel 3 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch3CcitensR = crate::BitReader;
#[doc = "Field `CH3_CCITENS` writer - 16:16\\]
Channel 3 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch3CcitensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CRC_FAILENS` reader - 17:17\\]
Channel 3 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch3CrcFailensR = crate::BitReader;
#[doc = "Field `CH3_CRC_FAILENS` writer - 17:17\\]
Channel 3 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch3CrcFailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_OVERENS` reader - 18:18\\]
Channel 3 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch3OverensR = crate::BitReader;
#[doc = "Field `CH3_OVERENS` writer - 18:18\\]
Channel 3 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch3OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_UNDERENS` reader - 19:19\\]
Channel 3 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch3UnderensR = crate::BitReader;
#[doc = "Field `CH3_UNDERENS` writer - 19:19\\]
Channel 3 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch3UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TIME_OUT_ENS` reader - 20:20\\]
Channel 3 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch3TimeOutEnsR = crate::BitReader;
#[doc = "Field `CH3_TIME_OUT_ENS` writer - 20:20\\]
Channel 3 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch3TimeOutEnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CCITENS` reader - 24:24\\]
Channel 4 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch4CcitensR = crate::BitReader;
#[doc = "Field `CH4_CCITENS` writer - 24:24\\]
Channel 4 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
pub type Ch4CcitensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CRC_FAILENS` reader - 25:25\\]
Channel 4 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch4CrcFailensR = crate::BitReader;
#[doc = "Field `CH4_CRC_FAILENS` writer - 25:25\\]
Channel 4 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
pub type Ch4CrcFailensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_OVERENS` reader - 26:26\\]
Channel 4 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch4OverensR = crate::BitReader;
#[doc = "Field `CH4_OVERENS` writer - 26:26\\]
Channel 4 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
pub type Ch4OverensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_UNDERENS` reader - 27:27\\]
Channel 4 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch4UnderensR = crate::BitReader;
#[doc = "Field `CH4_UNDERENS` writer - 27:27\\]
Channel 4 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
pub type Ch4UnderensW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_TIME_OUT_ENS_` reader - 28:28\\]
Channel 4 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch4TimeOutEns_R = crate::BitReader;
#[doc = "Field `CH4_TIME_OUT_ENS_` writer - 28:28\\]
Channel 4 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
pub type Ch4TimeOutEns_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    pub fn ch1_ccitens(&self) -> Ch1CcitensR {
        Ch1CcitensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch1_crc_failens(&self) -> Ch1CrcFailensR {
        Ch1CrcFailensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_overens(&self) -> Ch1OverensR {
        Ch1OverensR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch1_underens(&self) -> Ch1UnderensR {
        Ch1UnderensR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch1_time_out_ens_(&self) -> Ch1TimeOutEns_R {
        Ch1TimeOutEns_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    pub fn ch2_ccitens(&self) -> Ch2CcitensR {
        Ch2CcitensR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch2_crc_failens(&self) -> Ch2CrcFailensR {
        Ch2CrcFailensR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_overens(&self) -> Ch2OverensR {
        Ch2OverensR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch2_underens(&self) -> Ch2UnderensR {
        Ch2UnderensR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch2_time_out_ens_(&self) -> Ch2TimeOutEns_R {
        Ch2TimeOutEns_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    pub fn ch3_ccitens(&self) -> Ch3CcitensR {
        Ch3CcitensR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch3_crc_failens(&self) -> Ch3CrcFailensR {
        Ch3CrcFailensR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch3_overens(&self) -> Ch3OverensR {
        Ch3OverensR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch3_underens(&self) -> Ch3UnderensR {
        Ch3UnderensR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch3_time_out_ens(&self) -> Ch3TimeOutEnsR {
        Ch3TimeOutEnsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    pub fn ch4_ccitens(&self) -> Ch4CcitensR {
        Ch4CcitensR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    pub fn ch4_crc_failens(&self) -> Ch4CrcFailensR {
        Ch4CrcFailensR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    pub fn ch4_overens(&self) -> Ch4OverensR {
        Ch4OverensR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    pub fn ch4_underens(&self) -> Ch4UnderensR {
        Ch4UnderensR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    pub fn ch4_time_out_ens_(&self) -> Ch4TimeOutEns_R {
        Ch4TimeOutEns_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel 1 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_ccitens(&mut self) -> Ch1CcitensW<Mcrc64RegsCrcIntsSpec> {
        Ch1CcitensW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel 1 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_crc_failens(&mut self) -> Ch1CrcFailensW<Mcrc64RegsCrcIntsSpec> {
        Ch1CrcFailensW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Channel 1 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_overens(&mut self) -> Ch1OverensW<Mcrc64RegsCrcIntsSpec> {
        Ch1OverensW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Channel 1 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_underens(&mut self) -> Ch1UnderensW<Mcrc64RegsCrcIntsSpec> {
        Ch1UnderensW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel 1 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_time_out_ens_(&mut self) -> Ch1TimeOutEns_W<Mcrc64RegsCrcIntsSpec> {
        Ch1TimeOutEns_W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 2 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ccitens(&mut self) -> Ch2CcitensW<Mcrc64RegsCrcIntsSpec> {
        Ch2CcitensW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 2 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_crc_failens(&mut self) -> Ch2CrcFailensW<Mcrc64RegsCrcIntsSpec> {
        Ch2CrcFailensW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_overens(&mut self) -> Ch2OverensW<Mcrc64RegsCrcIntsSpec> {
        Ch2OverensW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Channel 2 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_underens(&mut self) -> Ch2UnderensW<Mcrc64RegsCrcIntsSpec> {
        Ch2UnderensW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Channel 2 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_time_out_ens_(&mut self) -> Ch2TimeOutEns_W<Mcrc64RegsCrcIntsSpec> {
        Ch2TimeOutEns_W::new(self, 12)
    }
    #[doc = "Bit 16 - 16:16\\]
Channel 3 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_ccitens(&mut self) -> Ch3CcitensW<Mcrc64RegsCrcIntsSpec> {
        Ch3CcitensW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Channel 3 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_crc_failens(&mut self) -> Ch3CrcFailensW<Mcrc64RegsCrcIntsSpec> {
        Ch3CrcFailensW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Channel 3 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_overens(&mut self) -> Ch3OverensW<Mcrc64RegsCrcIntsSpec> {
        Ch3OverensW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Channel 3 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_underens(&mut self) -> Ch3UnderensW<Mcrc64RegsCrcIntsSpec> {
        Ch3UnderensW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Channel 3 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_time_out_ens(&mut self) -> Ch3TimeOutEnsW<Mcrc64RegsCrcIntsSpec> {
        Ch3TimeOutEnsW::new(self, 20)
    }
    #[doc = "Bit 24 - 24:24\\]
Channel 4 Compression Complete Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Compression Complete Interrupt disable 1 = Compression Complete Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Compression Complete Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_ccitens(&mut self) -> Ch4CcitensW<Mcrc64RegsCrcIntsSpec> {
        Ch4CcitensW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Channel 4 CRC Fail Interrupt Enable Bit. Writing a one to this bit enable the CRC fail interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = CRC Fail Interrupt disable 1 = CRC Fail Interrupt enable User and privileged mode write: 0 = Has no effect 1 = CRC Fail Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_crc_failens(&mut self) -> Ch4CrcFailensW<Mcrc64RegsCrcIntsSpec> {
        Ch4CrcFailensW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Channel 4 Overrun Interrupt Enable Bit Writing a one to this bit enable the overrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Overrun Interrupt disable 1 = Overrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Overrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_overens(&mut self) -> Ch4OverensW<Mcrc64RegsCrcIntsSpec> {
        Ch4OverensW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Channel 4 Underrun Interrupt Enable Bit Writing a one to this bit enable the underrun interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Underrun Interrupt disable 1 = Underrun Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Underrun Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_underens(&mut self) -> Ch4UnderensW<Mcrc64RegsCrcIntsSpec> {
        Ch4UnderensW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel 4 Timeout Interrupt Enable Bit Writing a one to this bit enable the timeout interrupt. Writing a zero has no effect. Reading from this bit gives the status (interrupt enable/disable). User and privileged mode read: 0 = Timeout Interrupt disable 1 = Timeout Interrupt enable User and privileged mode write: 0 = Has no effect 1 = Timeout Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_time_out_ens_(&mut self) -> Ch4TimeOutEns_W<Mcrc64RegsCrcIntsSpec> {
        Ch4TimeOutEns_W::new(self, 28)
    }
}
#[doc = "CRC Interrupt Enable Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcrc64_regs_crc_ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcrc64_regs_crc_ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mcrc64RegsCrcIntsSpec;
impl crate::RegisterSpec for Mcrc64RegsCrcIntsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc64_regs_crc_ints::R`](R) reader structure"]
impl crate::Readable for Mcrc64RegsCrcIntsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc64_regs_crc_ints::W`](W) writer structure"]
impl crate::Writable for Mcrc64RegsCrcIntsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC64_REGS_CRC_INTS to value 0"]
impl crate::Resettable for Mcrc64RegsCrcIntsSpec {
    const RESET_VALUE: u32 = 0;
}
