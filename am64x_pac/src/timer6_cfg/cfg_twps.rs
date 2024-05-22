#[doc = "Register `CFG_TWPS` reader"]
pub type R = crate::R<CfgTwpsSpec>;
#[doc = "Register `CFG_TWPS` writer"]
pub type W = crate::W<CfgTwpsSpec>;
#[doc = "Field `W_PEND_TCLR` reader - 0:0\\]
Write pending for register TCLR"]
pub type WPendTclrR = crate::BitReader;
#[doc = "Field `W_PEND_TCLR` writer - 0:0\\]
Write pending for register TCLR"]
pub type WPendTclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TCRR` reader - 1:1\\]
Write pending for register TCRR"]
pub type WPendTcrrR = crate::BitReader;
#[doc = "Field `W_PEND_TCRR` writer - 1:1\\]
Write pending for register TCRR"]
pub type WPendTcrrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TLDR` reader - 2:2\\]
Write pending for register TLDR"]
pub type WPendTldrR = crate::BitReader;
#[doc = "Field `W_PEND_TLDR` writer - 2:2\\]
Write pending for register TLDR"]
pub type WPendTldrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TTGR` reader - 3:3\\]
Write pending for register TTGR"]
pub type WPendTtgrR = crate::BitReader;
#[doc = "Field `W_PEND_TTGR` writer - 3:3\\]
Write pending for register TTGR"]
pub type WPendTtgrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TMAR` reader - 4:4\\]
Write pending for register TMAR"]
pub type WPendTmarR = crate::BitReader;
#[doc = "Field `W_PEND_TMAR` writer - 4:4\\]
Write pending for register TMAR"]
pub type WPendTmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TPIR` reader - 5:5\\]
Write pending for register TPIR"]
pub type WPendTpirR = crate::BitReader;
#[doc = "Field `W_PEND_TPIR` writer - 5:5\\]
Write pending for register TPIR"]
pub type WPendTpirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TNIR` reader - 6:6\\]
Write pending for register TNIR"]
pub type WPendTnirR = crate::BitReader;
#[doc = "Field `W_PEND_TNIR` writer - 6:6\\]
Write pending for register TNIR"]
pub type WPendTnirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TCVR` reader - 7:7\\]
Write pending for register TCVR"]
pub type WPendTcvrR = crate::BitReader;
#[doc = "Field `W_PEND_TCVR` writer - 7:7\\]
Write pending for register TCVR"]
pub type WPendTcvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TOCR` reader - 8:8\\]
Write pending for register TOCR"]
pub type WPendTocrR = crate::BitReader;
#[doc = "Field `W_PEND_TOCR` writer - 8:8\\]
Write pending for register TOCR"]
pub type WPendTocrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `W_PEND_TOWR` reader - 9:9\\]
Write pending for register TOWR"]
pub type WPendTowrR = crate::BitReader;
#[doc = "Field `W_PEND_TOWR` writer - 9:9\\]
Write pending for register TOWR"]
pub type WPendTowrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write pending for register TCLR"]
    #[inline(always)]
    pub fn w_pend_tclr(&self) -> WPendTclrR {
        WPendTclrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write pending for register TCRR"]
    #[inline(always)]
    pub fn w_pend_tcrr(&self) -> WPendTcrrR {
        WPendTcrrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pending for register TLDR"]
    #[inline(always)]
    pub fn w_pend_tldr(&self) -> WPendTldrR {
        WPendTldrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write pending for register TTGR"]
    #[inline(always)]
    pub fn w_pend_ttgr(&self) -> WPendTtgrR {
        WPendTtgrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write pending for register TMAR"]
    #[inline(always)]
    pub fn w_pend_tmar(&self) -> WPendTmarR {
        WPendTmarR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write pending for register TPIR"]
    #[inline(always)]
    pub fn w_pend_tpir(&self) -> WPendTpirR {
        WPendTpirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write pending for register TNIR"]
    #[inline(always)]
    pub fn w_pend_tnir(&self) -> WPendTnirR {
        WPendTnirR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Write pending for register TCVR"]
    #[inline(always)]
    pub fn w_pend_tcvr(&self) -> WPendTcvrR {
        WPendTcvrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Write pending for register TOCR"]
    #[inline(always)]
    pub fn w_pend_tocr(&self) -> WPendTocrR {
        WPendTocrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Write pending for register TOWR"]
    #[inline(always)]
    pub fn w_pend_towr(&self) -> WPendTowrR {
        WPendTowrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write pending for register TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tclr(&mut self) -> WPendTclrW<CfgTwpsSpec> {
        WPendTclrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write pending for register TCRR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tcrr(&mut self) -> WPendTcrrW<CfgTwpsSpec> {
        WPendTcrrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write pending for register TLDR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tldr(&mut self) -> WPendTldrW<CfgTwpsSpec> {
        WPendTldrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Write pending for register TTGR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_ttgr(&mut self) -> WPendTtgrW<CfgTwpsSpec> {
        WPendTtgrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write pending for register TMAR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tmar(&mut self) -> WPendTmarW<CfgTwpsSpec> {
        WPendTmarW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write pending for register TPIR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tpir(&mut self) -> WPendTpirW<CfgTwpsSpec> {
        WPendTpirW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Write pending for register TNIR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tnir(&mut self) -> WPendTnirW<CfgTwpsSpec> {
        WPendTnirW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Write pending for register TCVR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tcvr(&mut self) -> WPendTcvrW<CfgTwpsSpec> {
        WPendTcvrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Write pending for register TOCR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_tocr(&mut self) -> WPendTocrW<CfgTwpsSpec> {
        WPendTocrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Write pending for register TOWR"]
    #[inline(always)]
    #[must_use]
    pub fn w_pend_towr(&mut self) -> WPendTowrW<CfgTwpsSpec> {
        WPendTowrW::new(self, 9)
    }
}
#[doc = "This register contains the write posting bits for all writ-able functional registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_twps::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_twps::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTwpsSpec;
impl crate::RegisterSpec for CfgTwpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_twps::R`](R) reader structure"]
impl crate::Readable for CfgTwpsSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_twps::W`](W) writer structure"]
impl crate::Writable for CfgTwpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TWPS to value 0"]
impl crate::Resettable for CfgTwpsSpec {
    const RESET_VALUE: u32 = 0;
}
