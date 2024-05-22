#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec>;
#[doc = "Field `WRT_STALL_EVENT_CNT` reader - 13:0\\]
wrt stall event do to lack of eng"]
pub type WrtStallEventCntR = crate::FieldReader<u16>;
#[doc = "Field `WRT_STALL_EVENT_CNT` writer - 13:0\\]
wrt stall event do to lack of eng"]
pub type WrtStallEventCntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `RD_STALL_EVENT_CNT` reader - 29:16\\]
rd stall event do to lack of eng"]
pub type RdStallEventCntR = crate::FieldReader<u16>;
#[doc = "Field `RD_STALL_EVENT_CNT` writer - 29:16\\]
rd stall event do to lack of eng"]
pub type RdStallEventCntW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `CRYPTO_BUSY` reader - 30:30\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
pub type CryptoBusyR = crate::BitReader;
#[doc = "Field `CRYPTO_BUSY` writer - 30:30\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
pub type CryptoBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - 31:31\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 31:31\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
wrt stall event do to lack of eng"]
    #[inline(always)]
    pub fn wrt_stall_event_cnt(&self) -> WrtStallEventCntR {
        WrtStallEventCntR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - 29:16\\]
rd stall event do to lack of eng"]
    #[inline(always)]
    pub fn rd_stall_event_cnt(&self) -> RdStallEventCntR {
        RdStallEventCntR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
    #[inline(always)]
    pub fn crypto_busy(&self) -> CryptoBusyR {
        CryptoBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
wrt stall event do to lack of eng"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_stall_event_cnt(
        &mut self,
    ) -> WrtStallEventCntW<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec> {
        WrtStallEventCntW::new(self, 0)
    }
    #[doc = "Bits 16:29 - 29:16\\]
rd stall event do to lack of eng"]
    #[inline(always)]
    #[must_use]
    pub fn rd_stall_event_cnt(
        &mut self,
    ) -> RdStallEventCntW<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec> {
        RdStallEventCntW::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_busy(&mut self) -> CryptoBusyW<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec> {
        CryptoBusyW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
0 No transactions are active, crypto or none crypto 1 One or more transactions are active, crypto or none crypto"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec> {
        BusyW::new(self, 31)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_cstatus::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_cstatus to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsCstatusSpec {
    const RESET_VALUE: u32 = 0;
}
