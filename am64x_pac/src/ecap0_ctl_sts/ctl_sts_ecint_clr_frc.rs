#[doc = "Register `CTL_STS_ECINT_CLR_FRC` reader"]
pub type R = crate::R<CtlStsEcintClrFrcSpec>;
#[doc = "Register `CTL_STS_ECINT_CLR_FRC` writer"]
pub type W = crate::W<CtlStsEcintClrFrcSpec>;
#[doc = "Field `INT_CLR` reader - 0:0\\]
Global Interrupt Clear Flag: Writing a 1 will clear the INT flag and enable further interrupts to be generated if any of the event flags are set to 1. Writing a 0 will have no effect. Always reads back a 0."]
pub type IntClrR = crate::BitReader;
#[doc = "Field `INT_CLR` writer - 0:0\\]
Global Interrupt Clear Flag: Writing a 1 will clear the INT flag and enable further interrupts to be generated if any of the event flags are set to 1. Writing a 0 will have no effect. Always reads back a 0."]
pub type IntClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1_CLR` reader - 1:1\\]
Capture Event 1 Status Flag: Writing a 1 will clear the CEVT1 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt1ClrR = crate::BitReader;
#[doc = "Field `CEVT1_CLR` writer - 1:1\\]
Capture Event 1 Status Flag: Writing a 1 will clear the CEVT1 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2_CLR` reader - 2:2\\]
Capture Event 2 Status Flag: Writing a 1 will clear the CEVT2 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt2ClrR = crate::BitReader;
#[doc = "Field `CEVT2_CLR` writer - 2:2\\]
Capture Event 2 Status Flag: Writing a 1 will clear the CEVT2 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3_CLR` reader - 3:3\\]
Capture Event 3 Status Flag: Writing a 1 will clear the CEVT3 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt3ClrR = crate::BitReader;
#[doc = "Field `CEVT3_CLR` writer - 3:3\\]
Capture Event 3 Status Flag: Writing a 1 will clear the CEVT3 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt3ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT4_CLR` reader - 4:4\\]
Capture Event 4 Status Flag: Writing a 1 will clear the CEVT4 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt4ClrR = crate::BitReader;
#[doc = "Field `CEVT4_CLR` writer - 4:4\\]
Capture Event 4 Status Flag: Writing a 1 will clear the CEVT4 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type Cevt4ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOVF_CLR` reader - 5:5\\]
Counter Overflow Status Flag: Writing a 1 will clear the CNTOVF flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type CntovfClrR = crate::BitReader;
#[doc = "Field `CNTOVF_CLR` writer - 5:5\\]
Counter Overflow Status Flag: Writing a 1 will clear the CNTOVF flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type CntovfClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDEQ_CLR` reader - 6:6\\]
Period Equal Status Flag: Writing a 1 will clear the PRDEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type PrdeqClrR = crate::BitReader;
#[doc = "Field `PRDEQ_CLR` writer - 6:6\\]
Period Equal Status Flag: Writing a 1 will clear the PRDEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type PrdeqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEQ_CLR` reader - 7:7\\]
Compare Equal Status Flag: Writing a 1 will clear the CMPEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type CmpeqClrR = crate::BitReader;
#[doc = "Field `CMPEQ_CLR` writer - 7:7\\]
Compare Equal Status Flag: Writing a 1 will clear the CMPEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
pub type CmpeqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT1_FRC` reader - 17:17\\]
Force Capture Event 1: Writing a 1 to this bit will set the CEVT1 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt1FrcR = crate::BitReader;
#[doc = "Field `CEVT1_FRC` writer - 17:17\\]
Force Capture Event 1: Writing a 1 to this bit will set the CEVT1 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt1FrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT2_FRC` reader - 18:18\\]
Force Capture Event 2: Writing a 1 to this bit will set the CEVT2 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt2FrcR = crate::BitReader;
#[doc = "Field `CEVT2_FRC` writer - 18:18\\]
Force Capture Event 2: Writing a 1 to this bit will set the CEVT2 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt2FrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT3_FRC` reader - 19:19\\]
Force Capture Event 3: Writing a 1 to this bit will set the CEVT3 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt3FrcR = crate::BitReader;
#[doc = "Field `CEVT3_FRC` writer - 19:19\\]
Force Capture Event 3: Writing a 1 to this bit will set the CEVT3 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt3FrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEVT4_FRC` reader - 20:20\\]
Force Capture Event 4: Writing a 1 to this bit will set the CEVT4 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt4FrcR = crate::BitReader;
#[doc = "Field `CEVT4_FRC` writer - 20:20\\]
Force Capture Event 4: Writing a 1 to this bit will set the CEVT4 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type Cevt4FrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTOVF_FRC` reader - 21:21\\]
Force Counter Overflow: Writing a 1 to this bit will set the CNTOVF flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type CntovfFrcR = crate::BitReader;
#[doc = "Field `CNTOVF_FRC` writer - 21:21\\]
Force Counter Overflow: Writing a 1 to this bit will set the CNTOVF flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type CntovfFrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRDEQ_FRC` reader - 22:22\\]
Force Period Equal: Writing a 1 to this bit will set the PRDEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type PrdeqFrcR = crate::BitReader;
#[doc = "Field `PRDEQ_FRC` writer - 22:22\\]
Force Period Equal: Writing a 1 to this bit will set the PRDEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type PrdeqFrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEQ_FRC` reader - 23:23\\]
Force Compare Equal: Writing a 1 to this bit will set the CMPEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type CmpeqFrcR = crate::BitReader;
#[doc = "Field `CMPEQ_FRC` writer - 23:23\\]
Force Compare Equal: Writing a 1 to this bit will set the CMPEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
pub type CmpeqFrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Global Interrupt Clear Flag: Writing a 1 will clear the INT flag and enable further interrupts to be generated if any of the event flags are set to 1. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn int_clr(&self) -> IntClrR {
        IntClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Capture Event 1 Status Flag: Writing a 1 will clear the CEVT1 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt1_clr(&self) -> Cevt1ClrR {
        Cevt1ClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Status Flag: Writing a 1 will clear the CEVT2 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt2_clr(&self) -> Cevt2ClrR {
        Cevt2ClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture Event 3 Status Flag: Writing a 1 will clear the CEVT3 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt3_clr(&self) -> Cevt3ClrR {
        Cevt3ClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 4 Status Flag: Writing a 1 will clear the CEVT4 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt4_clr(&self) -> Cevt4ClrR {
        Cevt4ClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Overflow Status Flag: Writing a 1 will clear the CNTOVF flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cntovf_clr(&self) -> CntovfClrR {
        CntovfClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Period Equal Status Flag: Writing a 1 will clear the PRDEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn prdeq_clr(&self) -> PrdeqClrR {
        PrdeqClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Compare Equal Status Flag: Writing a 1 will clear the CMPEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    pub fn cmpeq_clr(&self) -> CmpeqClrR {
        CmpeqClrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Force Capture Event 1: Writing a 1 to this bit will set the CEVT1 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt1_frc(&self) -> Cevt1FrcR {
        Cevt1FrcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Force Capture Event 2: Writing a 1 to this bit will set the CEVT2 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt2_frc(&self) -> Cevt2FrcR {
        Cevt2FrcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Force Capture Event 3: Writing a 1 to this bit will set the CEVT3 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt3_frc(&self) -> Cevt3FrcR {
        Cevt3FrcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Force Capture Event 4: Writing a 1 to this bit will set the CEVT4 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cevt4_frc(&self) -> Cevt4FrcR {
        Cevt4FrcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Force Counter Overflow: Writing a 1 to this bit will set the CNTOVF flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cntovf_frc(&self) -> CntovfFrcR {
        CntovfFrcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Force Period Equal: Writing a 1 to this bit will set the PRDEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn prdeq_frc(&self) -> PrdeqFrcR {
        PrdeqFrcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Force Compare Equal: Writing a 1 to this bit will set the CMPEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    pub fn cmpeq_frc(&self) -> CmpeqFrcR {
        CmpeqFrcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Global Interrupt Clear Flag: Writing a 1 will clear the INT flag and enable further interrupts to be generated if any of the event flags are set to 1. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn int_clr(&mut self) -> IntClrW<CtlStsEcintClrFrcSpec> {
        IntClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Capture Event 1 Status Flag: Writing a 1 will clear the CEVT1 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt1_clr(&mut self) -> Cevt1ClrW<CtlStsEcintClrFrcSpec> {
        Cevt1ClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Event 2 Status Flag: Writing a 1 will clear the CEVT2 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt2_clr(&mut self) -> Cevt2ClrW<CtlStsEcintClrFrcSpec> {
        Cevt2ClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Capture Event 3 Status Flag: Writing a 1 will clear the CEVT3 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt3_clr(&mut self) -> Cevt3ClrW<CtlStsEcintClrFrcSpec> {
        Cevt3ClrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Capture Event 4 Status Flag: Writing a 1 will clear the CEVT4 flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt4_clr(&mut self) -> Cevt4ClrW<CtlStsEcintClrFrcSpec> {
        Cevt4ClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Counter Overflow Status Flag: Writing a 1 will clear the CNTOVF flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cntovf_clr(&mut self) -> CntovfClrW<CtlStsEcintClrFrcSpec> {
        CntovfClrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Period Equal Status Flag: Writing a 1 will clear the PRDEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn prdeq_clr(&mut self) -> PrdeqClrW<CtlStsEcintClrFrcSpec> {
        PrdeqClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Compare Equal Status Flag: Writing a 1 will clear the CMPEQ flag condition. Writing a 0 will have no effect. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmpeq_clr(&mut self) -> CmpeqClrW<CtlStsEcintClrFrcSpec> {
        CmpeqClrW::new(self, 7)
    }
    #[doc = "Bit 17 - 17:17\\]
Force Capture Event 1: Writing a 1 to this bit will set the CEVT1 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt1_frc(&mut self) -> Cevt1FrcW<CtlStsEcintClrFrcSpec> {
        Cevt1FrcW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Force Capture Event 2: Writing a 1 to this bit will set the CEVT2 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt2_frc(&mut self) -> Cevt2FrcW<CtlStsEcintClrFrcSpec> {
        Cevt2FrcW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Force Capture Event 3: Writing a 1 to this bit will set the CEVT3 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt3_frc(&mut self) -> Cevt3FrcW<CtlStsEcintClrFrcSpec> {
        Cevt3FrcW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Force Capture Event 4: Writing a 1 to this bit will set the CEVT4 flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cevt4_frc(&mut self) -> Cevt4FrcW<CtlStsEcintClrFrcSpec> {
        Cevt4FrcW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Force Counter Overflow: Writing a 1 to this bit will set the CNTOVF flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cntovf_frc(&mut self) -> CntovfFrcW<CtlStsEcintClrFrcSpec> {
        CntovfFrcW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Force Period Equal: Writing a 1 to this bit will set the PRDEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn prdeq_frc(&mut self) -> PrdeqFrcW<CtlStsEcintClrFrcSpec> {
        PrdeqFrcW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Force Compare Equal: Writing a 1 to this bit will set the CMPEQ flag bit. Writing of 0 will be ignored. Always reads back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmpeq_frc(&mut self) -> CmpeqFrcW<CtlStsEcintClrFrcSpec> {
        CmpeqFrcW::new(self, 23)
    }
}
#[doc = "CTL_STS_ECINT_CLR_FRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl_sts_ecint_clr_frc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl_sts_ecint_clr_frc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlStsEcintClrFrcSpec;
impl crate::RegisterSpec for CtlStsEcintClrFrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl_sts_ecint_clr_frc::R`](R) reader structure"]
impl crate::Readable for CtlStsEcintClrFrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl_sts_ecint_clr_frc::W`](W) writer structure"]
impl crate::Writable for CtlStsEcintClrFrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL_STS_ECINT_CLR_FRC to value 0"]
impl crate::Resettable for CtlStsEcintClrFrcSpec {
    const RESET_VALUE: u32 = 0;
}
