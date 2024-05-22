#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECFLG_ECEINT` reader"]
pub type R = crate::R<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec>;
#[doc = "Register `PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECFLG_ECEINT` writer"]
pub type W = crate::W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec>;
#[doc = "Field `EN_RESV0` reader - "]
pub type EnResv0R = crate::BitReader;
#[doc = "Field `EN_RESV0` writer - "]
pub type EnResv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CEVT1` reader - 1:1\\]
CAPTURE EVENT 1 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt1R = crate::BitReader;
#[doc = "Field `EN_CEVT1` writer - 1:1\\]
CAPTURE EVENT 1 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CEVT2` reader - 2:2\\]
CAPTURE EVENT 2 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt2R = crate::BitReader;
#[doc = "Field `EN_CEVT2` writer - 2:2\\]
CAPTURE EVENT 2 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CEVT3` reader - 3:3\\]
CAPTURE EVENT 3 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt3R = crate::BitReader;
#[doc = "Field `EN_CEVT3` writer - 3:3\\]
CAPTURE EVENT 3 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CEVT4` reader - 4:4\\]
CAPTURE EVENT 4 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt4R = crate::BitReader;
#[doc = "Field `EN_CEVT4` writer - 4:4\\]
CAPTURE EVENT 4 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
pub type EnCevt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CNTOVF` reader - 5:5\\]
COUNTER OVERFLOW INTERRUPT ENABLE: 0DISABLED COUNTER OVERFLOW AS AN INTERRUPT SOURCE1ENABLE COUNTER OVERFLOW AS AN INTERRUPT SOURCE"]
pub type EnCntovfR = crate::BitReader;
#[doc = "Field `EN_CNTOVF` writer - 5:5\\]
COUNTER OVERFLOW INTERRUPT ENABLE: 0DISABLED COUNTER OVERFLOW AS AN INTERRUPT SOURCE1ENABLE COUNTER OVERFLOW AS AN INTERRUPT SOURCE"]
pub type EnCntovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_PRDEQ` reader - 6:6\\]
PERIOD EQUAL INTERRUPT ENABLE: 0DISABLED PERIOD EQUAL AS AN INTERRUPT SOURCE1ENABLE PERIOD EQUAL AS AN INTERRUPT SOURCE"]
pub type EnPrdeqR = crate::BitReader;
#[doc = "Field `EN_PRDEQ` writer - 6:6\\]
PERIOD EQUAL INTERRUPT ENABLE: 0DISABLED PERIOD EQUAL AS AN INTERRUPT SOURCE1ENABLE PERIOD EQUAL AS AN INTERRUPT SOURCE"]
pub type EnPrdeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CMPEQ` reader - 7:7\\]
COMPARE EQUAL INTERRUPT ENABLE: 0DISABLED COMPARE EQUAL AS AN INTERRUPT SOURCE1ENABLE COMPARE EQUAL AS AN INTERRUPT SOURCE"]
pub type EnCmpeqR = crate::BitReader;
#[doc = "Field `EN_CMPEQ` writer - 7:7\\]
COMPARE EQUAL INTERRUPT ENABLE: 0DISABLED COMPARE EQUAL AS AN INTERRUPT SOURCE1ENABLE COMPARE EQUAL AS AN INTERRUPT SOURCE"]
pub type EnCmpeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN__RESV1` reader - "]
pub type En_Resv1R = crate::FieldReader;
#[doc = "Field `EN__RESV1` writer - "]
pub type En_Resv1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLAG_INT` reader - 16:16\\]
GLOBAL INTERRUPT STATUS FLAG: READING A 1 ON THIS BIT INDICATES THAT AN INTERRUPT WAS GENERATED FROM ONE OF THE FOLLOWING EVENTSREADING A 0 INDICATES NO INTERRUPT GENERATED"]
pub type FlagIntR = crate::BitReader;
#[doc = "Field `FLAG_INT` writer - 16:16\\]
GLOBAL INTERRUPT STATUS FLAG: READING A 1 ON THIS BIT INDICATES THAT AN INTERRUPT WAS GENERATED FROM ONE OF THE FOLLOWING EVENTSREADING A 0 INDICATES NO INTERRUPT GENERATED"]
pub type FlagIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CEVT1` reader - 17:17\\]
CAPTURE EVENT 1 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FIRST EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt1R = crate::BitReader;
#[doc = "Field `FLAG_CEVT1` writer - 17:17\\]
CAPTURE EVENT 1 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FIRST EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CEVT2` reader - 18:18\\]
CAPTURE EVENT 2 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE SECOND EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt2R = crate::BitReader;
#[doc = "Field `FLAG_CEVT2` writer - 18:18\\]
CAPTURE EVENT 2 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE SECOND EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CEVT3` reader - 19:19\\]
CAPTURE EVENT 3 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE THIRD EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt3R = crate::BitReader;
#[doc = "Field `FLAG_CEVT3` writer - 19:19\\]
CAPTURE EVENT 3 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE THIRD EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CEVT4` reader - 20:20\\]
CAPTURE EVENT 4 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FOURTH EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt4R = crate::BitReader;
#[doc = "Field `FLAG_CEVT4` writer - 20:20\\]
CAPTURE EVENT 4 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FOURTH EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
pub type FlagCevt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CNTOVF` reader - 21:21\\]
COUNTER OVERFLOW STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
HAS MADE THE TRANSITION FROM FFFFFFFF 00000000READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ACTIVE IN CAP &amp;AMP"]
pub type FlagCntovfR = crate::BitReader;
#[doc = "Field `FLAG_CNTOVF` writer - 21:21\\]
COUNTER OVERFLOW STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
HAS MADE THE TRANSITION FROM FFFFFFFF 00000000READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ACTIVE IN CAP &amp;AMP"]
pub type FlagCntovfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_PRDEQ` reader - 22:22\\]
PERIOD EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE PERIOD REGISTER VALUE \\[APER\\]
AND WAS RESETREADING A 0 INDICATES NO EVENT OCCURREDNOTES: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
pub type FlagPrdeqR = crate::BitReader;
#[doc = "Field `FLAG_PRDEQ` writer - 22:22\\]
PERIOD EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE PERIOD REGISTER VALUE \\[APER\\]
AND WAS RESETREADING A 0 INDICATES NO EVENT OCCURREDNOTES: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
pub type FlagPrdeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_CMPEQ` reader - 23:23\\]
COMPARE EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE COMPARE REGISTER VALUE \\[ACMP\\]READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
pub type FlagCmpeqR = crate::BitReader;
#[doc = "Field `FLAG_CMPEQ` writer - 23:23\\]
COMPARE EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE COMPARE REGISTER VALUE \\[ACMP\\]READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
pub type FlagCmpeqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLAG_RESV0` reader - "]
pub type FlagResv0R = crate::FieldReader;
#[doc = "Field `FLAG_RESV0` writer - "]
pub type FlagResv0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en_resv0(&self) -> EnResv0R {
        EnResv0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CAPTURE EVENT 1 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cevt1(&self) -> EnCevt1R {
        EnCevt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
CAPTURE EVENT 2 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cevt2(&self) -> EnCevt2R {
        EnCevt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
CAPTURE EVENT 3 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cevt3(&self) -> EnCevt3R {
        EnCevt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
CAPTURE EVENT 4 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cevt4(&self) -> EnCevt4R {
        EnCevt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
COUNTER OVERFLOW INTERRUPT ENABLE: 0DISABLED COUNTER OVERFLOW AS AN INTERRUPT SOURCE1ENABLE COUNTER OVERFLOW AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cntovf(&self) -> EnCntovfR {
        EnCntovfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
PERIOD EQUAL INTERRUPT ENABLE: 0DISABLED PERIOD EQUAL AS AN INTERRUPT SOURCE1ENABLE PERIOD EQUAL AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_prdeq(&self) -> EnPrdeqR {
        EnPrdeqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
COMPARE EQUAL INTERRUPT ENABLE: 0DISABLED COMPARE EQUAL AS AN INTERRUPT SOURCE1ENABLE COMPARE EQUAL AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    pub fn en_cmpeq(&self) -> EnCmpeqR {
        EnCmpeqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn en__resv1(&self) -> En_Resv1R {
        En_Resv1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
GLOBAL INTERRUPT STATUS FLAG: READING A 1 ON THIS BIT INDICATES THAT AN INTERRUPT WAS GENERATED FROM ONE OF THE FOLLOWING EVENTSREADING A 0 INDICATES NO INTERRUPT GENERATED"]
    #[inline(always)]
    pub fn flag_int(&self) -> FlagIntR {
        FlagIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
CAPTURE EVENT 1 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FIRST EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    pub fn flag_cevt1(&self) -> FlagCevt1R {
        FlagCevt1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
CAPTURE EVENT 2 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE SECOND EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    pub fn flag_cevt2(&self) -> FlagCevt2R {
        FlagCevt2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
CAPTURE EVENT 3 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE THIRD EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    pub fn flag_cevt3(&self) -> FlagCevt3R {
        FlagCevt3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
CAPTURE EVENT 4 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FOURTH EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    pub fn flag_cevt4(&self) -> FlagCevt4R {
        FlagCevt4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
COUNTER OVERFLOW STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
HAS MADE THE TRANSITION FROM FFFFFFFF 00000000READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ACTIVE IN CAP &amp;AMP"]
    #[inline(always)]
    pub fn flag_cntovf(&self) -> FlagCntovfR {
        FlagCntovfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
PERIOD EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE PERIOD REGISTER VALUE \\[APER\\]
AND WAS RESETREADING A 0 INDICATES NO EVENT OCCURREDNOTES: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
    #[inline(always)]
    pub fn flag_prdeq(&self) -> FlagPrdeqR {
        FlagPrdeqR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
COMPARE EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE COMPARE REGISTER VALUE \\[ACMP\\]READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
    #[inline(always)]
    pub fn flag_cmpeq(&self) -> FlagCmpeqR {
        FlagCmpeqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn flag_resv0(&self) -> FlagResv0R {
        FlagResv0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn en_resv0(&mut self) -> EnResv0W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnResv0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
CAPTURE EVENT 1 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cevt1(&mut self) -> EnCevt1W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCevt1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
CAPTURE EVENT 2 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cevt2(&mut self) -> EnCevt2W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCevt2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
CAPTURE EVENT 3 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cevt3(&mut self) -> EnCevt3W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCevt3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
CAPTURE EVENT 4 INTERRUPT ENABLE: 0DISABLED CAPTURE EVENT 1 AS AN INTERRUPT SOURCE1ENABLE CAPTURE EVENT 1 AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cevt4(&mut self) -> EnCevt4W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCevt4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
COUNTER OVERFLOW INTERRUPT ENABLE: 0DISABLED COUNTER OVERFLOW AS AN INTERRUPT SOURCE1ENABLE COUNTER OVERFLOW AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cntovf(&mut self) -> EnCntovfW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCntovfW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
PERIOD EQUAL INTERRUPT ENABLE: 0DISABLED PERIOD EQUAL AS AN INTERRUPT SOURCE1ENABLE PERIOD EQUAL AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_prdeq(&mut self) -> EnPrdeqW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnPrdeqW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
COMPARE EQUAL INTERRUPT ENABLE: 0DISABLED COMPARE EQUAL AS AN INTERRUPT SOURCE1ENABLE COMPARE EQUAL AS AN INTERRUPT SOURCE"]
    #[inline(always)]
    #[must_use]
    pub fn en_cmpeq(&mut self) -> EnCmpeqW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        EnCmpeqW::new(self, 7)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn en__resv1(&mut self) -> En_Resv1W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        En_Resv1W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
GLOBAL INTERRUPT STATUS FLAG: READING A 1 ON THIS BIT INDICATES THAT AN INTERRUPT WAS GENERATED FROM ONE OF THE FOLLOWING EVENTSREADING A 0 INDICATES NO INTERRUPT GENERATED"]
    #[inline(always)]
    #[must_use]
    pub fn flag_int(&mut self) -> FlagIntW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagIntW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
CAPTURE EVENT 1 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FIRST EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cevt1(&mut self) -> FlagCevt1W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCevt1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
CAPTURE EVENT 2 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE SECOND EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cevt2(&mut self) -> FlagCevt2W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCevt2W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
CAPTURE EVENT 3 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE THIRD EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cevt3(&mut self) -> FlagCevt3W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCevt3W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
CAPTURE EVENT 4 STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE FOURTH EVENT OCCURRED AT ECAPX PINREADING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN CAP MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cevt4(&mut self) -> FlagCevt4W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCevt4W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
COUNTER OVERFLOW STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
HAS MADE THE TRANSITION FROM FFFFFFFF 00000000READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ACTIVE IN CAP &amp;AMP"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cntovf(&mut self) -> FlagCntovfW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCntovfW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
PERIOD EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE PERIOD REGISTER VALUE \\[APER\\]
AND WAS RESETREADING A 0 INDICATES NO EVENT OCCURREDNOTES: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_prdeq(&mut self) -> FlagPrdeqW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagPrdeqW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
COMPARE EQUAL STATUS FLAG: READING A 1 ON THIS BIT INDICATES THE COUNTER \\[TSCNT\\]
REACHED THE COMPARE REGISTER VALUE \\[ACMP\\]READING A 0 INDICATES NO EVENT OCCURREDNOTE: THIS FLAG IS ONLY ACTIVE IN APWM MODE"]
    #[inline(always)]
    #[must_use]
    pub fn flag_cmpeq(&mut self) -> FlagCmpeqW<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagCmpeqW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn flag_resv0(&mut self) -> FlagResv0W<Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec> {
        FlagResv0W::new(self, 24)
    }
}
#[doc = "ECAP INTERRUPT ENABLE REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec;
impl crate::RegisterSpec for Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::R`](R) reader structure"]
impl crate::Readable for Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_ecap0__ecap_slv__regs_ecflg_eceint::W`](W) writer structure"]
impl crate::Writable for Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_ECAP0__ECAP_SLV__REGS_ECFLG_ECEINT to value 0"]
impl crate::Resettable for Pr1IcssEcap0_EcapSlv_RegsEcflgEceintSpec {
    const RESET_VALUE: u32 = 0;
}
