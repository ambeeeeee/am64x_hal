#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IR` reader"]
pub type R = crate::R<McanWrap_McanCfgVbp_McanRegsIrSpec>;
#[doc = "Register `MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IR` writer"]
pub type W = crate::W<McanWrap_McanCfgVbp_McanRegsIrSpec>;
#[doc = "Field `RF0N` reader - 0:0\\]
Rx FIFO 0 New Message"]
pub type Rf0nR = crate::BitReader;
#[doc = "Field `RF0N` writer - 0:0\\]
Rx FIFO 0 New Message"]
pub type Rf0nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0W` reader - 1:1\\]
Rx FIFO 0 Watermark Reached"]
pub type Rf0wR = crate::BitReader;
#[doc = "Field `RF0W` writer - 1:1\\]
Rx FIFO 0 Watermark Reached"]
pub type Rf0wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0F` reader - 2:2\\]
Rx FIFO 0 Full"]
pub type Rf0fR = crate::BitReader;
#[doc = "Field `RF0F` writer - 2:2\\]
Rx FIFO 0 Full"]
pub type Rf0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - 3:3\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - 3:3\\]
Rx FIFO 0 Message Lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1N` reader - 4:4\\]
Rx FIFO 1 New Message"]
pub type Rf1nR = crate::BitReader;
#[doc = "Field `RF1N` writer - 4:4\\]
Rx FIFO 1 New Message"]
pub type Rf1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1W` reader - 5:5\\]
Rx FIFO 1 Watermark Reached"]
pub type Rf1wR = crate::BitReader;
#[doc = "Field `RF1W` writer - 5:5\\]
Rx FIFO 1 Watermark Reached"]
pub type Rf1wW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1F` reader - 6:6\\]
Rx FIFO 1 Full"]
pub type Rf1fR = crate::BitReader;
#[doc = "Field `RF1F` writer - 6:6\\]
Rx FIFO 1 Full"]
pub type Rf1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - 7:7\\]
Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - 7:7\\]
Rx FIFO 1 Message Lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPM` reader - 8:8\\]
High Priority Message"]
pub type HpmR = crate::BitReader;
#[doc = "Field `HPM` writer - 8:8\\]
High Priority Message"]
pub type HpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - 9:9\\]
Transmission Complete"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - 9:9\\]
Transmission Complete"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCF` reader - 10:10\\]
Transmission Cancellation Finished"]
pub type TcfR = crate::BitReader;
#[doc = "Field `TCF` writer - 10:10\\]
Transmission Cancellation Finished"]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - 11:11\\]
Tx FIFO Empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFE` writer - 11:11\\]
Tx FIFO Empty"]
pub type TfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFN` reader - 12:12\\]
Tx Event FIFO New Entry"]
pub type TefnR = crate::BitReader;
#[doc = "Field `TEFN` writer - 12:12\\]
Tx Event FIFO New Entry"]
pub type TefnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFW` reader - 13:13\\]
Tx Event FIFO Watermark Reached"]
pub type TefwR = crate::BitReader;
#[doc = "Field `TEFW` writer - 13:13\\]
Tx Event FIFO Watermark Reached"]
pub type TefwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFF` reader - 14:14\\]
Tx Event FIFO Full"]
pub type TeffR = crate::BitReader;
#[doc = "Field `TEFF` writer - 14:14\\]
Tx Event FIFO Full"]
pub type TeffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - 15:15\\]
Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - 15:15\\]
Tx Event FIFO Element Lost"]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSW` reader - 16:16\\]
Timestamp Wraparound"]
pub type TswR = crate::BitReader;
#[doc = "Field `TSW` writer - 16:16\\]
Timestamp Wraparound"]
pub type TswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRAF` reader - 17:17\\]
Message RAM Access Failure"]
pub type MrafR = crate::BitReader;
#[doc = "Field `MRAF` writer - 17:17\\]
Message RAM Access Failure"]
pub type MrafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOO` reader - 18:18\\]
Timeout Occurred"]
pub type TooR = crate::BitReader;
#[doc = "Field `TOO` writer - 18:18\\]
Timeout Occurred"]
pub type TooW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRX` reader - 19:19\\]
Message stored to Dedicated Rx Buffer"]
pub type DrxR = crate::BitReader;
#[doc = "Field `DRX` writer - 19:19\\]
Message stored to Dedicated Rx Buffer"]
pub type DrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEU` reader - 21:21\\]
Bit Error Uncorrected"]
pub type BeuR = crate::BitReader;
#[doc = "Field `BEU` writer - 21:21\\]
Bit Error Uncorrected"]
pub type BeuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELO` reader - 22:22\\]
Error Logging Overflow"]
pub type EloR = crate::BitReader;
#[doc = "Field `ELO` writer - 22:22\\]
Error Logging Overflow"]
pub type EloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP` reader - 23:23\\]
Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - 23:23\\]
Error Passive"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - 24:24\\]
Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - 24:24\\]
Warning Status"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - 25:25\\]
Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - 25:25\\]
Bus_Off Status"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDI` reader - 26:26\\]
Watchdog Interrupt"]
pub type WdiR = crate::BitReader;
#[doc = "Field `WDI` writer - 26:26\\]
Watchdog Interrupt"]
pub type WdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEA` reader - 27:27\\]
Protocol Error in Arbitration Phase"]
pub type PeaR = crate::BitReader;
#[doc = "Field `PEA` writer - 27:27\\]
Protocol Error in Arbitration Phase"]
pub type PeaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PED` reader - 28:28\\]
Protocol Error in data Phase"]
pub type PedR = crate::BitReader;
#[doc = "Field `PED` writer - 28:28\\]
Protocol Error in data Phase"]
pub type PedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARA` reader - 29:29\\]
Access to Reserved Address"]
pub type AraR = crate::BitReader;
#[doc = "Field `ARA` writer - 29:29\\]
Access to Reserved Address"]
pub type AraW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub fn rf0w(&self) -> Rf0wR {
        Rf0wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> Rf1wR {
        Rf1wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmission Complete"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TefwR {
        TefwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DrxR {
        DrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit Error Uncorrected"]
    #[inline(always)]
    pub fn beu(&self) -> BeuR {
        BeuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Protocol Error in Arbitration Phase"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Protocol Error in data Phase"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Rx FIFO 0 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> Rf0nW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf0nW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx FIFO 0 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> Rf0wW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf0wW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Rx FIFO 0 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> Rf0fW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf0fW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Rx FIFO 0 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l(&mut self) -> Rf0lW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf0lW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Rx FIFO 1 New Message"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> Rf1nW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf1nW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> Rf1wW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf1wW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Rx FIFO 1 Full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> Rf1fW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf1fW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Rx FIFO 1 Message Lost"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l(&mut self) -> Rf1lW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        Rf1lW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
High Priority Message"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HpmW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        HpmW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Transmission Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TcW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Transmission Cancellation Finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TcfW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TcfW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Tx FIFO Empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TfeW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TfeW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Tx Event FIFO New Entry"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TefnW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TefnW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TefwW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TefwW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Tx Event FIFO Full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TeffW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TeffW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Tx Event FIFO Element Lost"]
    #[inline(always)]
    #[must_use]
    pub fn tefl(&mut self) -> TeflW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TeflW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Timestamp Wraparound"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TswW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TswW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Message RAM Access Failure"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MrafW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        MrafW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Timeout Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TooW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        TooW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DrxW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        DrxW::new(self, 19)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit Error Uncorrected"]
    #[inline(always)]
    #[must_use]
    pub fn beu(&mut self) -> BeuW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        BeuW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Error Logging Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> EloW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        EloW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Error Passive"]
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EpW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        EpW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Warning Status"]
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EwW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        EwW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Bus_Off Status"]
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BoW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        BoW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Watchdog Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WdiW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        WdiW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Protocol Error in Arbitration Phase"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PeaW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        PeaW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Protocol Error in data Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PedW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        PedW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Access to Reserved Address"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> AraW<McanWrap_McanCfgVbp_McanRegsIrSpec> {
        AraW::new(self, 29)
    }
}
#[doc = "Interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanWrap_McanCfgVbp_McanRegsIrSpec;
impl crate::RegisterSpec for McanWrap_McanCfgVbp_McanRegsIrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::R`](R) reader structure"]
impl crate::Readable for McanWrap_McanCfgVbp_McanRegsIrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_wrap__mcan_cfg_vbp__mcan_regs_ir::W`](W) writer structure"]
impl crate::Writable for McanWrap_McanCfgVbp_McanRegsIrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_WRAP__MCAN_CFG_VBP__MCAN_REGS_IR to value 0"]
impl crate::Resettable for McanWrap_McanCfgVbp_McanRegsIrSpec {
    const RESET_VALUE: u32 = 0;
}
