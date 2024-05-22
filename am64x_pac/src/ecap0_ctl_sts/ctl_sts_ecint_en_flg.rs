#[doc = "Register `CTL_STS_ECINT_EN_FLG` reader"]
pub type R = crate::R<CtlStsEcintEnFlgSpec>;
#[doc = "Register `CTL_STS_ECINT_EN_FLG` writer"]
pub type W = crate::W<CtlStsEcintEnFlgSpec>;
#[doc = "Field `CEVT1_EN` reader - 1:1\\]
Capture Event 1 Interrupt Enable: 1'b0 Disabled Capture Event 1 as an Interrupt source: 1'b1 Enable Capture Event 1 as an Interrupt source"]
pub type Cevt1EnR = crate::BitReader;
#[doc = "Field `CEVT1_EN` writer - 1:1\\]
Capture Event 1 Interrupt Enable: 1'b0 Disabled Capture Event 1 as an Interrupt source: 1'b1 Enable Capture Event 1 as an Interrupt source"]
pub type Cevt1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2_EN` reader - 2:2\\]
Capture Event 2 Interrupt Enable: 1'b0 Disabled Capture Event 2 as an Interrupt source: 1'b1 Enable Capture Event 2 as an Interrupt source"]
pub type Cevt2EnR = crate::BitReader;
#[doc = "Field `CEVT2_EN` writer - 2:2\\]
Capture Event 2 Interrupt Enable: 1'b0 Disabled Capture Event 2 as an Interrupt source: 1'b1 Enable Capture Event 2 as an Interrupt source"]
pub type Cevt2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3_EN` reader - 3:3\\]
Capture Event 3 Interrupt Enable: 1'b0 Disabled Capture Event 3 as an Interrupt source: 1'b1 Enable Capture Event 3 as an Interrupt source"]
pub type Cevt3EnR = crate::BitReader;
#[doc = "Field `CEVT3_EN` writer - 3:3\\]
Capture Event 3 Interrupt Enable: 1'b0 Disabled Capture Event 3 as an Interrupt source: 1'b1 Enable Capture Event 3 as an Interrupt source"]
pub type Cevt3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT4_EN` reader - 4:4\\]
Capture Event 4 Interrupt Enable: 1'b0 Disabled Capture Event 4 as an Interrupt source: 1'b1 Enable Capture Event 4 as an Interrupt source"]
pub type Cevt4EnR = crate::BitReader;
#[doc = "Field `CEVT4_EN` writer - 4:4\\]
Capture Event 4 Interrupt Enable: 1'b0 Disabled Capture Event 4 as an Interrupt source: 1'b1 Enable Capture Event 4 as an Interrupt source"]
pub type Cevt4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOVF_EN` reader - 5:5\\]
Counter Overflow Interrupt Enable: 1'b0 Disabled Counter Overflow as an Interrupt source; 1'b1 Enable Counter Overflow as an Interrupt source"]
pub type CntovfEnR = crate::BitReader;
#[doc = "Field `CNTOVF_EN` writer - 5:5\\]
Counter Overflow Interrupt Enable: 1'b0 Disabled Counter Overflow as an Interrupt source; 1'b1 Enable Counter Overflow as an Interrupt source"]
pub type CntovfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDEQ_EN` reader - 6:6\\]
Period Equal Interrupt Enable: 1'b0 Disabled Period Equal as an Interrupt source; 1'b1 Enable Period Equal as an Interrupt source"]
pub type PrdeqEnR = crate::BitReader;
#[doc = "Field `PRDEQ_EN` writer - 6:6\\]
Period Equal Interrupt Enable: 1'b0 Disabled Period Equal as an Interrupt source; 1'b1 Enable Period Equal as an Interrupt source"]
pub type PrdeqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEQ_EN` reader - 7:7\\]
Compare Equal Interrupt Enable: 1'b0 Disabled Compare Equal as an Interrupt source; 1'b1 Enable Compare Equal as an Interrupt source"]
pub type CmpeqEnR = crate::BitReader;
#[doc = "Field `CMPEQ_EN` writer - 7:7\\]
Compare Equal Interrupt Enable: 1'b0 Disabled Compare Equal as an Interrupt source; 1'b1 Enable Compare Equal as an Interrupt source"]
pub type CmpeqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_FLG` reader - 16:16\\]
Global Interrupt Status Flag: Reading a 1 on this bit indicates that an interrupt was generated from one of the following events. Reading a 0 indicates no interrupt generated."]
pub type IntFlgR = crate::BitReader;
#[doc = "Field `INT_FLG` writer - 16:16\\]
Global Interrupt Status Flag: Reading a 1 on this bit indicates that an interrupt was generated from one of the following events. Reading a 0 indicates no interrupt generated."]
pub type IntFlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1_FLG` reader - 17:17\\]
Capture Event 1 Status Flag: Reading a 1 on this bit indicates the first event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt1FlgR = crate::BitReader;
#[doc = "Field `CEVT1_FLG` writer - 17:17\\]
Capture Event 1 Status Flag: Reading a 1 on this bit indicates the first event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt1FlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2_FLG` reader - 18:18\\]
Capture Event 2 Status Flag: Reading a 1 on this bit indicates the second event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt2FlgR = crate::BitReader;
#[doc = "Field `CEVT2_FLG` writer - 18:18\\]
Capture Event 2 Status Flag: Reading a 1 on this bit indicates the second event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt2FlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3_FLG` reader - 19:19\\]
Capture Event 3 Status Flag: Reading a 1 on this bit indicates the third event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt3FlgR = crate::BitReader;
#[doc = "Field `CEVT3_FLG` writer - 19:19\\]
Capture Event 3 Status Flag: Reading a 1 on this bit indicates the third event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt3FlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT4_FLG` reader - 20:20\\]
Capture Event 4 Status Flag: Reading a 1 on this bit indicates the fourth event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt4FlgR = crate::BitReader;
#[doc = "Field `CEVT4_FLG` writer - 20:20\\]
Capture Event 4 Status Flag: Reading a 1 on this bit indicates the fourth event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
pub type Cevt4FlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOVF_FLG` reader - 21:21\\]
Counter Overflow Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) has made the transition from 0xFFFFFFFF 0x00000000 Reading a 0 indicates no event occurred. Note: This flag is active in CAP &amp; APWM mode."]
pub type CntovfFlgR = crate::BitReader;
#[doc = "Field `CNTOVF_FLG` writer - 21:21\\]
Counter Overflow Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) has made the transition from 0xFFFFFFFF 0x00000000 Reading a 0 indicates no event occurred. Note: This flag is active in CAP &amp; APWM mode."]
pub type CntovfFlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDEQ_FLG` reader - 22:22\\]
Period Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Period register value (APER) and was reset. Reading a 0 indicates no event occurred Notes: This flag is only active in APWM mode."]
pub type PrdeqFlgR = crate::BitReader;
#[doc = "Field `PRDEQ_FLG` writer - 22:22\\]
Period Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Period register value (APER) and was reset. Reading a 0 indicates no event occurred Notes: This flag is only active in APWM mode."]
pub type PrdeqFlgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEQ_FLG` reader - 23:23\\]
Compare Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Compare register value (ACMP) Reading a 0 indicates no event occurred Note: This flag is only active in APWM mode."]
pub type CmpeqFlgR = crate::BitReader;
#[doc = "Field `CMPEQ_FLG` writer - 23:23\\]
Compare Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Compare register value (ACMP) Reading a 0 indicates no event occurred Note: This flag is only active in APWM mode."]
pub type CmpeqFlgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - 1:1\\]
Capture Event 1 Interrupt Enable: 1'b0 Disabled Capture Event 1 as an Interrupt source: 1'b1 Enable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    pub fn cevt1_en(&self) -> Cevt1EnR {
        Cevt1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Interrupt Enable: 1'b0 Disabled Capture Event 2 as an Interrupt source: 1'b1 Enable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    pub fn cevt2_en(&self) -> Cevt2EnR {
        Cevt2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture Event 3 Interrupt Enable: 1'b0 Disabled Capture Event 3 as an Interrupt source: 1'b1 Enable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    pub fn cevt3_en(&self) -> Cevt3EnR {
        Cevt3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 4 Interrupt Enable: 1'b0 Disabled Capture Event 4 as an Interrupt source: 1'b1 Enable Capture Event 4 as an Interrupt source"]
    #[inline(always)]
    pub fn cevt4_en(&self) -> Cevt4EnR {
        Cevt4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Overflow Interrupt Enable: 1'b0 Disabled Counter Overflow as an Interrupt source; 1'b1 Enable Counter Overflow as an Interrupt source"]
    #[inline(always)]
    pub fn cntovf_en(&self) -> CntovfEnR {
        CntovfEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Period Equal Interrupt Enable: 1'b0 Disabled Period Equal as an Interrupt source; 1'b1 Enable Period Equal as an Interrupt source"]
    #[inline(always)]
    pub fn prdeq_en(&self) -> PrdeqEnR {
        PrdeqEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Compare Equal Interrupt Enable: 1'b0 Disabled Compare Equal as an Interrupt source; 1'b1 Enable Compare Equal as an Interrupt source"]
    #[inline(always)]
    pub fn cmpeq_en(&self) -> CmpeqEnR {
        CmpeqEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Global Interrupt Status Flag: Reading a 1 on this bit indicates that an interrupt was generated from one of the following events. Reading a 0 indicates no interrupt generated."]
    #[inline(always)]
    pub fn int_flg(&self) -> IntFlgR {
        IntFlgR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Capture Event 1 Status Flag: Reading a 1 on this bit indicates the first event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    pub fn cevt1_flg(&self) -> Cevt1FlgR {
        Cevt1FlgR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Capture Event 2 Status Flag: Reading a 1 on this bit indicates the second event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    pub fn cevt2_flg(&self) -> Cevt2FlgR {
        Cevt2FlgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Capture Event 3 Status Flag: Reading a 1 on this bit indicates the third event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    pub fn cevt3_flg(&self) -> Cevt3FlgR {
        Cevt3FlgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Capture Event 4 Status Flag: Reading a 1 on this bit indicates the fourth event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    pub fn cevt4_flg(&self) -> Cevt4FlgR {
        Cevt4FlgR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Counter Overflow Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) has made the transition from 0xFFFFFFFF 0x00000000 Reading a 0 indicates no event occurred. Note: This flag is active in CAP &amp; APWM mode."]
    #[inline(always)]
    pub fn cntovf_flg(&self) -> CntovfFlgR {
        CntovfFlgR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Period Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Period register value (APER) and was reset. Reading a 0 indicates no event occurred Notes: This flag is only active in APWM mode."]
    #[inline(always)]
    pub fn prdeq_flg(&self) -> PrdeqFlgR {
        PrdeqFlgR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Compare Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Compare register value (ACMP) Reading a 0 indicates no event occurred Note: This flag is only active in APWM mode."]
    #[inline(always)]
    pub fn cmpeq_flg(&self) -> CmpeqFlgR {
        CmpeqFlgR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 1:1\\]
Capture Event 1 Interrupt Enable: 1'b0 Disabled Capture Event 1 as an Interrupt source: 1'b1 Enable Capture Event 1 as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cevt1_en(&mut self) -> Cevt1EnW<CtlStsEcintEnFlgSpec> {
        Cevt1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Interrupt Enable: 1'b0 Disabled Capture Event 2 as an Interrupt source: 1'b1 Enable Capture Event 2 as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cevt2_en(&mut self) -> Cevt2EnW<CtlStsEcintEnFlgSpec> {
        Cevt2EnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture Event 3 Interrupt Enable: 1'b0 Disabled Capture Event 3 as an Interrupt source: 1'b1 Enable Capture Event 3 as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cevt3_en(&mut self) -> Cevt3EnW<CtlStsEcintEnFlgSpec> {
        Cevt3EnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 4 Interrupt Enable: 1'b0 Disabled Capture Event 4 as an Interrupt source: 1'b1 Enable Capture Event 4 as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cevt4_en(&mut self) -> Cevt4EnW<CtlStsEcintEnFlgSpec> {
        Cevt4EnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Overflow Interrupt Enable: 1'b0 Disabled Counter Overflow as an Interrupt source; 1'b1 Enable Counter Overflow as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cntovf_en(&mut self) -> CntovfEnW<CtlStsEcintEnFlgSpec> {
        CntovfEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Period Equal Interrupt Enable: 1'b0 Disabled Period Equal as an Interrupt source; 1'b1 Enable Period Equal as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn prdeq_en(&mut self) -> PrdeqEnW<CtlStsEcintEnFlgSpec> {
        PrdeqEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Compare Equal Interrupt Enable: 1'b0 Disabled Compare Equal as an Interrupt source; 1'b1 Enable Compare Equal as an Interrupt source"]
    #[inline(always)]
    #[must_use]
    pub fn cmpeq_en(&mut self) -> CmpeqEnW<CtlStsEcintEnFlgSpec> {
        CmpeqEnW::new(self, 7)
    }
    #[doc = "Bit 16 - 16:16\\]
Global Interrupt Status Flag: Reading a 1 on this bit indicates that an interrupt was generated from one of the following events. Reading a 0 indicates no interrupt generated."]
    #[inline(always)]
    #[must_use]
    pub fn int_flg(&mut self) -> IntFlgW<CtlStsEcintEnFlgSpec> {
        IntFlgW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Capture Event 1 Status Flag: Reading a 1 on this bit indicates the first event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    #[must_use]
    pub fn cevt1_flg(&mut self) -> Cevt1FlgW<CtlStsEcintEnFlgSpec> {
        Cevt1FlgW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Capture Event 2 Status Flag: Reading a 1 on this bit indicates the second event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    #[must_use]
    pub fn cevt2_flg(&mut self) -> Cevt2FlgW<CtlStsEcintEnFlgSpec> {
        Cevt2FlgW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Capture Event 3 Status Flag: Reading a 1 on this bit indicates the third event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    #[must_use]
    pub fn cevt3_flg(&mut self) -> Cevt3FlgW<CtlStsEcintEnFlgSpec> {
        Cevt3FlgW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Capture Event 4 Status Flag: Reading a 1 on this bit indicates the fourth event occurred at ECAPx pin. Reading a 0 indicates no event occurred. Note: This flag is only active in CAP mode."]
    #[inline(always)]
    #[must_use]
    pub fn cevt4_flg(&mut self) -> Cevt4FlgW<CtlStsEcintEnFlgSpec> {
        Cevt4FlgW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Counter Overflow Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) has made the transition from 0xFFFFFFFF 0x00000000 Reading a 0 indicates no event occurred. Note: This flag is active in CAP &amp; APWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn cntovf_flg(&mut self) -> CntovfFlgW<CtlStsEcintEnFlgSpec> {
        CntovfFlgW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Period Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Period register value (APER) and was reset. Reading a 0 indicates no event occurred Notes: This flag is only active in APWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn prdeq_flg(&mut self) -> PrdeqFlgW<CtlStsEcintEnFlgSpec> {
        PrdeqFlgW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Compare Equal Status Flag: Reading a 1 on this bit indicates the Counter (TSCNT) reached the Compare register value (ACMP) Reading a 0 indicates no event occurred Note: This flag is only active in APWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn cmpeq_flg(&mut self) -> CmpeqFlgW<CtlStsEcintEnFlgSpec> {
        CmpeqFlgW::new(self, 23)
    }
}
#[doc = "CTL_STS_ECINT_EN_FLG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecint_en_flg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecint_en_flg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsEcintEnFlgSpec;
impl crate::RegisterSpec for CtlStsEcintEnFlgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_ecint_en_flg::R`](R) reader structure"]
impl crate::Readable for CtlStsEcintEnFlgSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_ecint_en_flg::W`](W) writer structure"]
impl crate::Writable for CtlStsEcintEnFlgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_ECINT_EN_FLG to value 0"]
impl crate::Resettable for CtlStsEcintEnFlgSpec {
    const RESET_VALUE: u32 = 0;
}
