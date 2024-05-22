#[doc = "Register `CPTS_VBUSP_CONTROL_REG` reader"]
pub type R = crate::R<CptsVbuspControlRegSpec>;
#[doc = "Register `CPTS_VBUSP_CONTROL_REG` writer"]
pub type W = crate::W<CptsVbuspControlRegSpec>;
#[doc = "Field `CPTS_EN` reader - 0:0\\]
Time sync enable"]
pub type CptsEnR = crate::BitReader;
#[doc = "Field `CPTS_EN` writer - 0:0\\]
Time sync enable"]
pub type CptsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_TEST` reader - 1:1\\]
Interrupt test"]
pub type IntTestR = crate::BitReader;
#[doc = "Field `INT_TEST` writer - 1:1\\]
Interrupt test"]
pub type IntTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_COMP_POLARITY` reader - 2:2\\]
TS_COMP polarity"]
pub type TsCompPolarityR = crate::BitReader;
#[doc = "Field `TS_COMP_POLARITY` writer - 2:2\\]
TS_COMP polarity"]
pub type TsCompPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTAMP_EN` reader - 3:3\\]
Host Receive Timestamp Enable"]
pub type TstampEnR = crate::BitReader;
#[doc = "Field `TSTAMP_EN` writer - 3:3\\]
Host Receive Timestamp Enable"]
pub type TstampEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQUENCE_EN` reader - 4:4\\]
Sequence Enable"]
pub type SequenceEnR = crate::BitReader;
#[doc = "Field `SEQUENCE_EN` writer - 4:4\\]
Sequence Enable"]
pub type SequenceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - 5:5\\]
Timestamp mode"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - 5:5\\]
Timestamp mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_COMP_TOG` reader - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
pub type TsCompTogR = crate::BitReader;
#[doc = "Field `TS_COMP_TOG` writer - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
pub type TsCompTogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_PPM_DIR` reader - 7:7\\]
Timestamp PPM Direction"]
pub type TsPpmDirR = crate::BitReader;
#[doc = "Field `TS_PPM_DIR` writer - 7:7\\]
Timestamp PPM Direction"]
pub type TsPpmDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW1_TS_PUSH_EN` reader - 8:8\\]
Hardware push 1 enable"]
pub type Hw1TsPushEnR = crate::BitReader;
#[doc = "Field `HW1_TS_PUSH_EN` writer - 8:8\\]
Hardware push 1 enable"]
pub type Hw1TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW2_TS_PUSH_EN` reader - 9:9\\]
Hardware push 2 enable"]
pub type Hw2TsPushEnR = crate::BitReader;
#[doc = "Field `HW2_TS_PUSH_EN` writer - 9:9\\]
Hardware push 2 enable"]
pub type Hw2TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW3_TS_PUSH_EN` reader - 10:10\\]
Hardware push 3 enable"]
pub type Hw3TsPushEnR = crate::BitReader;
#[doc = "Field `HW3_TS_PUSH_EN` writer - 10:10\\]
Hardware push 3 enable"]
pub type Hw3TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW4_TS_PUSH_EN` reader - 11:11\\]
Hardware push 4 enable"]
pub type Hw4TsPushEnR = crate::BitReader;
#[doc = "Field `HW4_TS_PUSH_EN` writer - 11:11\\]
Hardware push 4 enable"]
pub type Hw4TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW5_TS_PUSH_EN` reader - 12:12\\]
Hardware push 5 enable"]
pub type Hw5TsPushEnR = crate::BitReader;
#[doc = "Field `HW5_TS_PUSH_EN` writer - 12:12\\]
Hardware push 5 enable"]
pub type Hw5TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW6_TS_PUSH_EN` reader - 13:13\\]
Hardware push 6 enable"]
pub type Hw6TsPushEnR = crate::BitReader;
#[doc = "Field `HW6_TS_PUSH_EN` writer - 13:13\\]
Hardware push 6 enable"]
pub type Hw6TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW7_TS_PUSH_EN` reader - 14:14\\]
Hardware push 7 enable"]
pub type Hw7TsPushEnR = crate::BitReader;
#[doc = "Field `HW7_TS_PUSH_EN` writer - 14:14\\]
Hardware push 7 enable"]
pub type Hw7TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HW8_TS_PUSH_EN` reader - 15:15\\]
Hardware push 8 enable"]
pub type Hw8TsPushEnR = crate::BitReader;
#[doc = "Field `HW8_TS_PUSH_EN` writer - 15:15\\]
Hardware push 8 enable"]
pub type Hw8TsPushEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_RX_NO_EVENT` reader - 16:16\\]
Receive Produces no Events"]
pub type TsRxNoEventR = crate::BitReader;
#[doc = "Field `TS_RX_NO_EVENT` writer - 16:16\\]
Receive Produces no Events"]
pub type TsRxNoEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_GENF_CLR_EN` reader - 17:17\\]
Enable for GENF clear when length is zero"]
pub type TsGenfClrEnR = crate::BitReader;
#[doc = "Field `TS_GENF_CLR_EN` writer - 17:17\\]
Enable for GENF clear when length is zero"]
pub type TsGenfClrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_SYNC_SEL` reader - 31:28\\]
TS_SYNC output timestamp counter bit select"]
pub type TsSyncSelR = crate::FieldReader;
#[doc = "Field `TS_SYNC_SEL` writer - 31:28\\]
TS_SYNC output timestamp counter bit select"]
pub type TsSyncSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Time sync enable"]
    #[inline(always)]
    pub fn cpts_en(&self) -> CptsEnR {
        CptsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt test"]
    #[inline(always)]
    pub fn int_test(&self) -> IntTestR {
        IntTestR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TS_COMP polarity"]
    #[inline(always)]
    pub fn ts_comp_polarity(&self) -> TsCompPolarityR {
        TsCompPolarityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Receive Timestamp Enable"]
    #[inline(always)]
    pub fn tstamp_en(&self) -> TstampEnR {
        TstampEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Sequence Enable"]
    #[inline(always)]
    pub fn sequence_en(&self) -> SequenceEnR {
        SequenceEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Timestamp mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
    #[inline(always)]
    pub fn ts_comp_tog(&self) -> TsCompTogR {
        TsCompTogR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Timestamp PPM Direction"]
    #[inline(always)]
    pub fn ts_ppm_dir(&self) -> TsPpmDirR {
        TsPpmDirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Hardware push 1 enable"]
    #[inline(always)]
    pub fn hw1_ts_push_en(&self) -> Hw1TsPushEnR {
        Hw1TsPushEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Hardware push 2 enable"]
    #[inline(always)]
    pub fn hw2_ts_push_en(&self) -> Hw2TsPushEnR {
        Hw2TsPushEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware push 3 enable"]
    #[inline(always)]
    pub fn hw3_ts_push_en(&self) -> Hw3TsPushEnR {
        Hw3TsPushEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Hardware push 4 enable"]
    #[inline(always)]
    pub fn hw4_ts_push_en(&self) -> Hw4TsPushEnR {
        Hw4TsPushEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Hardware push 5 enable"]
    #[inline(always)]
    pub fn hw5_ts_push_en(&self) -> Hw5TsPushEnR {
        Hw5TsPushEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Hardware push 6 enable"]
    #[inline(always)]
    pub fn hw6_ts_push_en(&self) -> Hw6TsPushEnR {
        Hw6TsPushEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Hardware push 7 enable"]
    #[inline(always)]
    pub fn hw7_ts_push_en(&self) -> Hw7TsPushEnR {
        Hw7TsPushEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Hardware push 8 enable"]
    #[inline(always)]
    pub fn hw8_ts_push_en(&self) -> Hw8TsPushEnR {
        Hw8TsPushEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive Produces no Events"]
    #[inline(always)]
    pub fn ts_rx_no_event(&self) -> TsRxNoEventR {
        TsRxNoEventR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable for GENF clear when length is zero"]
    #[inline(always)]
    pub fn ts_genf_clr_en(&self) -> TsGenfClrEnR {
        TsGenfClrEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
TS_SYNC output timestamp counter bit select"]
    #[inline(always)]
    pub fn ts_sync_sel(&self) -> TsSyncSelR {
        TsSyncSelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Time sync enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpts_en(&mut self) -> CptsEnW<CptsVbuspControlRegSpec> {
        CptsEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt test"]
    #[inline(always)]
    #[must_use]
    pub fn int_test(&mut self) -> IntTestW<CptsVbuspControlRegSpec> {
        IntTestW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TS_COMP polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ts_comp_polarity(&mut self) -> TsCompPolarityW<CptsVbuspControlRegSpec> {
        TsCompPolarityW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Receive Timestamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tstamp_en(&mut self) -> TstampEnW<CptsVbuspControlRegSpec> {
        TstampEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Sequence Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sequence_en(&mut self) -> SequenceEnW<CptsVbuspControlRegSpec> {
        SequenceEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Timestamp mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CptsVbuspControlRegSpec> {
        ModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Timestamp Compare Toggle mode: 0=TS_COMP is in non-toggle mode, 1=TS_COMP is in toggle mode"]
    #[inline(always)]
    #[must_use]
    pub fn ts_comp_tog(&mut self) -> TsCompTogW<CptsVbuspControlRegSpec> {
        TsCompTogW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Timestamp PPM Direction"]
    #[inline(always)]
    #[must_use]
    pub fn ts_ppm_dir(&mut self) -> TsPpmDirW<CptsVbuspControlRegSpec> {
        TsPpmDirW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Hardware push 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw1_ts_push_en(&mut self) -> Hw1TsPushEnW<CptsVbuspControlRegSpec> {
        Hw1TsPushEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Hardware push 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw2_ts_push_en(&mut self) -> Hw2TsPushEnW<CptsVbuspControlRegSpec> {
        Hw2TsPushEnW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Hardware push 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw3_ts_push_en(&mut self) -> Hw3TsPushEnW<CptsVbuspControlRegSpec> {
        Hw3TsPushEnW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Hardware push 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw4_ts_push_en(&mut self) -> Hw4TsPushEnW<CptsVbuspControlRegSpec> {
        Hw4TsPushEnW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Hardware push 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw5_ts_push_en(&mut self) -> Hw5TsPushEnW<CptsVbuspControlRegSpec> {
        Hw5TsPushEnW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Hardware push 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw6_ts_push_en(&mut self) -> Hw6TsPushEnW<CptsVbuspControlRegSpec> {
        Hw6TsPushEnW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Hardware push 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw7_ts_push_en(&mut self) -> Hw7TsPushEnW<CptsVbuspControlRegSpec> {
        Hw7TsPushEnW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Hardware push 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn hw8_ts_push_en(&mut self) -> Hw8TsPushEnW<CptsVbuspControlRegSpec> {
        Hw8TsPushEnW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Receive Produces no Events"]
    #[inline(always)]
    #[must_use]
    pub fn ts_rx_no_event(&mut self) -> TsRxNoEventW<CptsVbuspControlRegSpec> {
        TsRxNoEventW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Enable for GENF clear when length is zero"]
    #[inline(always)]
    #[must_use]
    pub fn ts_genf_clr_en(&mut self) -> TsGenfClrEnW<CptsVbuspControlRegSpec> {
        TsGenfClrEnW::new(self, 17)
    }
    #[doc = "Bits 28:31 - 31:28\\]
TS_SYNC output timestamp counter bit select"]
    #[inline(always)]
    #[must_use]
    pub fn ts_sync_sel(&mut self) -> TsSyncSelW<CptsVbuspControlRegSpec> {
        TsSyncSelW::new(self, 28)
    }
}
#[doc = "Time Sync Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpts_vbusp_control_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpts_vbusp_control_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptsVbuspControlRegSpec;
impl crate::RegisterSpec for CptsVbuspControlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpts_vbusp_control_reg::R`](R) reader structure"]
impl crate::Readable for CptsVbuspControlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpts_vbusp_control_reg::W`](W) writer structure"]
impl crate::Writable for CptsVbuspControlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTS_VBUSP_CONTROL_REG to value 0x04"]
impl crate::Resettable for CptsVbuspControlRegSpec {
    const RESET_VALUE: u32 = 0x04;
}
