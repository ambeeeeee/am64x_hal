#[doc = "Register `CPSW_NUSS_VBUSP_STATUS_REG` reader"]
pub type R = crate::R<CpswNussVbuspStatusRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_STATUS_REG` writer"]
pub type W = crate::W<CpswNussVbuspStatusRegSpec>;
#[doc = "Field `LINK` reader - 0:0\\]
Link indicator"]
pub type LinkR = crate::BitReader;
#[doc = "Field `LINK` writer - 0:0\\]
Link indicator"]
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN_ERROR` reader - 1:1\\]
Auto-negotiation error"]
pub type AnErrorR = crate::BitReader;
#[doc = "Field `AN_ERROR` writer - 1:1\\]
Auto-negotiation error"]
pub type AnErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_AN_COMPLETE` reader - 2:2\\]
Auto-negotiation complete"]
pub type MrAnCompleteR = crate::BitReader;
#[doc = "Field `MR_AN_COMPLETE` writer - 2:2\\]
Auto-negotiation complete"]
pub type MrAnCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_PAGE_RX` reader - 3:3\\]
Next page received"]
pub type MrPageRxR = crate::BitReader;
#[doc = "Field `MR_PAGE_RX` writer - 3:3\\]
Next page received"]
pub type MrPageRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - 4:4\\]
Lock"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - 4:4\\]
Lock"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIB_SIG_DETECT` reader - 5:5\\]
Fiber signal detect"]
pub type FibSigDetectR = crate::BitReader;
#[doc = "Field `FIB_SIG_DETECT` writer - 5:5\\]
Fiber signal detect"]
pub type FibSigDetectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Link indicator"]
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-negotiation error"]
    #[inline(always)]
    pub fn an_error(&self) -> AnErrorR {
        AnErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Auto-negotiation complete"]
    #[inline(always)]
    pub fn mr_an_complete(&self) -> MrAnCompleteR {
        MrAnCompleteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Next page received"]
    #[inline(always)]
    pub fn mr_page_rx(&self) -> MrPageRxR {
        MrPageRxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Fiber signal detect"]
    #[inline(always)]
    pub fn fib_sig_detect(&self) -> FibSigDetectR {
        FibSigDetectR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Link indicator"]
    #[inline(always)]
    #[must_use]
    pub fn link(&mut self) -> LinkW<CpswNussVbuspStatusRegSpec> {
        LinkW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Auto-negotiation error"]
    #[inline(always)]
    #[must_use]
    pub fn an_error(&mut self) -> AnErrorW<CpswNussVbuspStatusRegSpec> {
        AnErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Auto-negotiation complete"]
    #[inline(always)]
    #[must_use]
    pub fn mr_an_complete(&mut self) -> MrAnCompleteW<CpswNussVbuspStatusRegSpec> {
        MrAnCompleteW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Next page received"]
    #[inline(always)]
    #[must_use]
    pub fn mr_page_rx(&mut self) -> MrPageRxW<CpswNussVbuspStatusRegSpec> {
        MrPageRxW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<CpswNussVbuspStatusRegSpec> {
        LockW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Fiber signal detect"]
    #[inline(always)]
    #[must_use]
    pub fn fib_sig_detect(&mut self) -> FibSigDetectW<CpswNussVbuspStatusRegSpec> {
        FibSigDetectW::new(self, 5)
    }
}
#[doc = "SGMII Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspStatusRegSpec;
impl crate::RegisterSpec for CpswNussVbuspStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_status_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_status_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_STATUS_REG to value 0"]
impl crate::Resettable for CpswNussVbuspStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
